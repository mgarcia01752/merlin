/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Validated mother-code selection, shortening and job-owned soft decoding.
use super::{DocsisLdpcCode, LdpcError};
use crate::sparse_encoder::SparseEncoder;
use ldpc_toolbox::{
    decoder::{
        factory::{DecoderFactory, DecoderImplementation},
        LdpcDecoder,
    },
    sparse::SparseMatrix,
};
use serde::Serialize;

/// Application work bound, independent of normative device requirements.
pub const MAX_ITERATIONS: usize = 100;
// Finite saturation avoids numerical instability and restores known-zero bits.
const LLR_SATURATION: f64 = 50.0;

/// Validated immutable standalone FEC settings; not a DOCSIS grant selector.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct LdpcSettings {
    code: DocsisLdpcCode,
    information_bits: usize,
    max_iterations: usize,
}

/// Owned decoder decisions, preserving convergence independently of source truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DecodeResult {
    /// Information decisions after dropping shortened zero coordinates.
    pub information_bits: Vec<u8>,
    /// Full mother-code decisions, including restored zero positions and parity.
    pub mother_codeword: Vec<u8>,
    /// Actual layered decoder iteration count; an initial valid word may use zero.
    pub iterations: usize,
    /// Unmodified engine convergence flag; does not assert correct source recovery.
    pub converged: bool,
    /// Independently checked syndrome of the final mother decisions.
    pub parity_valid: bool,
    /// Whether every shortened known-zero coordinate still decoded as zero.
    pub shortened_zeros_valid: bool,
    /// Changes from signs of nonzero transmitted LLRs, excluding restored positions.
    pub hard_decision_changes: usize,
}

/// Reusable codec with independent decoder scratch space and a sparse encoder.
///
/// Construction owns all state; operations borrow inputs without changing them.
/// Mutable decoding requires exclusive access; use separate codecs for parallel jobs.
pub struct LdpcCodec {
    settings: LdpcSettings,
    matrix: SparseMatrix,
    encoder: SparseEncoder,
    decoder: Box<dyn LdpcDecoder>,
}

impl LdpcSettings {
    /// Validates selected mother code, shortening and bounded decoder work.
    ///
    /// # Arguments
    /// * `code` - Upstream long, medium or short data code.
    /// * `information_bits` - Retained count in 1..=K; K disables shortening.
    /// * `max_iterations` - Application budget in 1..=MAX_ITERATIONS.
    /// # Returns
    /// Immutable settings; normal-grant selection remains a separate future policy.
    /// # Errors
    /// Rejects zero/excessive information or iteration counts.
    pub fn new(
        code: DocsisLdpcCode,
        information_bits: usize,
        max_iterations: usize,
    ) -> Result<Self, LdpcError> {
        if information_bits == 0 || information_bits > code.parameters().information_bits {
            return Err(LdpcError::InformationLength);
        }
        if !(1..=MAX_ITERATIONS).contains(&max_iterations) {
            return Err(LdpcError::IterationLimit);
        }
        Ok(Self {
            code,
            information_bits,
            max_iterations,
        })
    }

    /// Reports the selected upstream mother-code identity without mutation.
    ///
    /// # Returns
    /// The matrix identity used by encoding and decoding.
    pub fn code(self) -> DocsisLdpcCode {
        self.code
    }

    /// Reports the required retained information-bit count.
    ///
    /// # Returns
    /// Exact binary input count after shortening.
    pub fn information_bits(self) -> usize {
        self.information_bits
    }

    /// Reports the finite decoder work budget.
    ///
    /// # Returns
    /// Maximum iterations per decode call, not an actual measured count.
    pub fn max_iterations(self) -> usize {
        self.max_iterations
    }

    /// Reports the number of bits sent after removing appended zero coordinates.
    ///
    /// # Returns
    /// Retained information plus all mother-code parity bits.
    pub fn transmitted_bits(self) -> usize {
        let parameters = self.code.parameters();
        self.information_bits + parameters.codeword_bits - parameters.information_bits
    }
}

impl LdpcCodec {
    /// Builds a reusable upstream matrix, sparse plan and layered soft decoder.
    ///
    /// # Arguments
    /// * `settings` - Previously validated immutable code and work configuration.
    /// # Returns
    /// Job-owned state; no filesystem, Python or downstream BCH dependency.
    /// # Errors
    /// Reports malformed or non-peelable pinned matrix definitions.
    pub fn new(settings: LdpcSettings) -> Result<Self, LdpcError> {
        let matrix = settings.code.matrix()?;
        let encoder = SparseEncoder::from_h(&matrix).map_err(|_| LdpcError::MatrixDefinition)?;
        let decoder = DecoderImplementation::HLPhif64.build_decoder(matrix.clone());
        Ok(Self {
            settings,
            matrix,
            encoder,
            decoder,
        })
    }

    /// Encodes retained information with trailing-zero shortening and full parity.
    ///
    /// # Arguments
    /// * `information` - Exactly the configured number of binary bits, in wire order.
    /// # Returns
    /// Owned systematic information followed by parity; omitted zeros are not sent.
    /// Inputs and codec remain unchanged.
    /// # Errors
    /// Rejects unexpected length or nonbinary elements before allocating a word.
    pub fn encode(&self, information: &[u8]) -> Result<Vec<u8>, LdpcError> {
        check_bits(information, self.settings.information_bits)?;
        let mother_information = self.settings.code.parameters().information_bits;
        let mut padded = vec![0; mother_information];
        padded[..information.len()].copy_from_slice(information);
        let mother_word = self.encoder.encode_bits(&padded);
        Ok(mother_word[..information.len()]
            .iter()
            .chain(&mother_word[mother_information..])
            .copied()
            .collect())
    }

    /// Decodes finite natural-log P(0)/P(1) LLRs using horizontal layered sum-product.
    ///
    /// # Arguments
    /// * `llrs` - Exactly transmitted_bits() finite values; positive favors zero.
    /// # Returns
    /// Owned final decisions, including on nonconvergence; input remains unchanged.
    /// Finite inputs saturate to ±50; omitted zeros receive a +50 prior.
    /// Zero is an erasure; an engine tie decision may pass parity at iteration zero.
    /// Erasures are excluded from hard-decision changes; actual decisions are kept.
    /// Decoder scratch state is reused but reset by the engine per call.
    /// # Errors
    /// Rejects wrong size and nonfinite LLRs before changing decoder state.
    pub fn decode(&mut self, llrs: &[f64]) -> Result<DecodeResult, LdpcError> {
        check_length(llrs.len(), self.settings.transmitted_bits())?;
        if llrs.iter().any(|value| !value.is_finite()) {
            return Err(LdpcError::NonFiniteLlr);
        }
        let parameters = self.settings.code.parameters();
        let information_bits = self.settings.information_bits;
        let mut restored = vec![LLR_SATURATION; parameters.codeword_bits];
        for (target, value) in restored[..information_bits].iter_mut().zip(llrs) {
            *target = value.clamp(-LLR_SATURATION, LLR_SATURATION);
        }
        for (target, value) in restored[parameters.information_bits..]
            .iter_mut()
            .zip(&llrs[information_bits..])
        {
            *target = value.clamp(-LLR_SATURATION, LLR_SATURATION);
        }
        let (converged, output) = match self.decoder.decode(&restored, self.settings.max_iterations)
        {
            Ok(output) => (true, output),
            Err(output) => (false, output),
        };
        let hard_decision_changes = output.codeword[..information_bits]
            .iter()
            .chain(&output.codeword[parameters.information_bits..])
            .zip(llrs)
            .filter(|(bit, llr)| **llr != 0.0 && **bit != u8::from(**llr < 0.0))
            .count();
        Ok(DecodeResult {
            information_bits: output.codeword[..information_bits].to_vec(),
            parity_valid: self.mother_valid_syndrome(&output.codeword),
            shortened_zeros_valid: output.codeword[information_bits..parameters.information_bits]
                .iter()
                .all(|bit| *bit == 0),
            mother_codeword: output.codeword,
            iterations: output.iterations,
            converged,
            hard_decision_changes,
        })
    }

    /// Checks transmitted systematic/parity bits with shortened zeros restored.
    ///
    /// # Arguments
    /// * `codeword` - A complete transmitted word for the configured shortening.
    /// # Returns
    /// True only for correct size, binary values and all-zero mother syndrome.
    /// This check does not compare against the original source.
    pub fn valid_syndrome(&self, codeword: &[u8]) -> bool {
        if check_bits(codeword, self.settings.transmitted_bits()).is_err() {
            return false;
        }
        let information_bits = self.settings.information_bits;
        let parameters = self.settings.code.parameters();
        let mut restored = vec![0; parameters.codeword_bits];
        restored[..information_bits].copy_from_slice(&codeword[..information_bits]);
        restored[parameters.information_bits..].copy_from_slice(&codeword[information_bits..]);
        self.mother_valid_syndrome(&restored)
    }

    fn mother_valid_syndrome(&self, codeword: &[u8]) -> bool {
        (0..self.matrix.num_rows()).all(|row| {
            self.matrix
                .iter_row(row)
                .fold(0, |parity, column| parity ^ codeword[*column])
                == 0
        })
    }
}

fn check_bits(bits: &[u8], expected: usize) -> Result<(), LdpcError> {
    check_length(bits.len(), expected)?;
    if bits.iter().any(|bit| *bit > 1) {
        return Err(LdpcError::NonBinary);
    }
    Ok(())
}

fn check_length(actual: usize, expected: usize) -> Result<(), LdpcError> {
    if actual != expected {
        return Err(LdpcError::InputLength { expected, actual });
    }
    Ok(())
}
