/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin composition of inspected shortening boundaries and the existing mother codecs.
use super::Shortening;
use crate::{
    bch::{decode_bch, encode_bch, BchResult},
    binary_bits,
    concatenated::PayloadComparison,
    ldpc::{LdpcCodec, LdpcResult},
    parameters::{BCH_INPUT_BITS, NOISELESS_LLR_MAGNITUDE},
};
/// Bounded encoding test points; every vector belongs to this one word.
pub struct EncodedShortenedBlock {
    pub shortening: Shortening,
    pub bch_mother: Vec<u8>,
    pub ldpc_input: Vec<u8>,
    pub ldpc_mother: Vec<u8>,
    /// Before shortened bit interleaving and QAM cell grouping.
    pub transmitted: Vec<u8>,
}
/// Actual stage decisions; failed decoding still retains a full original-length payload.
pub struct DecodedShortenedBlock {
    pub shortening: Shortening,
    pub restored_llrs: Vec<f64>,
    pub ldpc: LdpcResult,
    pub bch: BchResult,
    pub payload: Vec<u8>,
    pub known_zeros_consistent: bool,
}
impl DecodedShortenedBlock {
    /// Combines actual decoder flags with omitted-coordinate consistency.
    /// # Returns
    /// Overall decode acceptance, independent of original-versus-recovered BER.
    pub fn success(&self) -> bool {
        self.ldpc.success && self.bch.success && self.known_zeros_consistent
    }
}
/// Reusable job-owned mother codec at one explicit shortened information length.
pub struct ShortenedCodec {
    shortening: Shortening,
    ldpc: LdpcCodec,
}
impl ShortenedCodec {
    /// Constructs reusable downstream mother coding state for one validated geometry.
    /// # Arguments
    /// * `shortening` - Checked K and derived stage sizes; no other mother code is selected.
    /// # Returns
    /// Independent codec with no QAM, files, source generator or channel assumptions.
    /// # Errors
    /// Propagates mother parity-matrix/setup failures.
    pub fn new(shortening: Shortening) -> Result<Self, String> {
        Ok(Self {
            shortening,
            ldpc: LdpcCodec::new()?,
        })
    }
    /// Borrows the frozen original-length configuration by value.
    /// # Returns
    /// Checked geometry, without advancing any coding stage.
    pub fn shortening(&self) -> Shortening {
        self.shortening
    }
    /// Performs all six PHY shortening encoder operations without changing source bits.
    /// # Arguments
    /// * `payload` - Exactly K binary original information bits.
    /// # Returns
    /// Encoded mother stages and K+1968 transmitted pre-interleaver bits.
    /// # Errors
    /// Rejects malformed payloads or mother-code/stage failures.
    pub fn encode(&self, payload: &[u8]) -> Result<EncodedShortenedBlock, String> {
        let bch_input = self.shortening.prepend_bch_zeros(payload)?;
        let bch_mother = encode_bch(&bch_input)?;
        let bch = self.shortening.remove_bch_zeros(&bch_mother)?;
        let ldpc_input = self.shortening.append_ldpc_zeros(&bch)?;
        let ldpc_mother = self.ldpc.encode(&ldpc_input)?;
        let transmitted = self.shortening.remove_ldpc_zeros(&ldpc_mother)?;
        Ok(EncodedShortenedBlock {
            shortening: self.shortening,
            bch_mother,
            ldpc_input,
            ldpc_mother,
            transmitted,
        })
    }
    /// Restores known zeros and decodes through the existing mother LDPC/BCH stages.
    /// # Arguments
    /// * `wire_llrs` - Exactly K+1968 finite ln(P0/P1) values after inverse bit processing.
    /// * `max_iterations` - Actual LDPC work cap in1..=100.
    /// # Returns
    /// Original-length decisions, restored LLR test point, actual iteration/status
    /// results and consistency. Non-convergence is an outcome, not an input error.
    /// # Errors
    /// Rejects malformed LLRs/work limits or invalid mother-stage outputs.
    pub fn decode(
        &mut self,
        wire_llrs: &[f64],
        max_iterations: usize,
    ) -> Result<DecodedShortenedBlock, String> {
        let restored_llrs = self.shortening.restore_ldpc_llrs(wire_llrs)?;
        let mut ldpc = self.ldpc.decode(&restored_llrs, max_iterations)?;
        // Only wire coordinates are observations; restored shortening zeros are synthetic.
        ldpc.hard_decision_changes = ldpc.codeword[..self.shortening.bch_bits()]
            .iter()
            .chain(&ldpc.codeword[crate::parameters::LDPC_INPUT_BITS..])
            .zip(wire_llrs)
            .filter(|(bit, llr)| **llr != 0.0 && **bit != u8::from(**llr < 0.0))
            .count();
        let bch_input = self.shortening.restore_bch_input(&ldpc.codeword)?;
        let bch = decode_bch(&bch_input)?;
        let known_zeros_consistent = self
            .shortening
            .known_zeros_consistent(&ldpc.codeword, &bch.codeword);
        if !known_zeros_consistent {
            log::debug!(
                "payload-shortening-inconsistent payload-bits={}",
                self.shortening.payload_bits()
            );
        }
        let payload = bch.codeword[self.shortening.omitted_bits()..BCH_INPUT_BITS].to_vec();
        Ok(DecodedShortenedBlock {
            shortening: self.shortening,
            restored_llrs,
            ldpc,
            bch,
            payload,
            known_zeros_consistent,
        })
    }
    /// Decodes binary transmitted coordinates using finite noiseless evidence.
    /// # Arguments
    /// * `transmitted` - Exactly K+1968 binary bits before bit interleaving.
    /// * `max_iterations` - LDPC cap in1..=100; not a requested convergence count.
    /// # Returns
    /// Actual reverse-decoding stages and original-length payload decisions.
    /// # Errors
    /// Rejects malformed binary input or invalid receiver policy.
    pub fn decode_noiseless(
        &mut self,
        transmitted: &[u8],
        max_iterations: usize,
    ) -> Result<DecodedShortenedBlock, String> {
        binary_bits(transmitted, self.shortening.transmitted_bits())?;
        let llrs: Vec<_> = transmitted
            .iter()
            .map(|bit| {
                if *bit == 0 {
                    NOISELESS_LLR_MAGNITUDE
                } else {
                    -NOISELESS_LLR_MAGNITUDE
                }
            })
            .collect();
        self.decode(&llrs, max_iterations)
    }
}
/// Counts original-payload mismatches without trusting success flags.
/// # Arguments
/// * `original` - Exactly K binary source bits retained by the experiment.
/// * `decoded` - Actual original-length receive decisions and declared geometry.
/// # Returns
/// Exactly K comparisons; neither BCH/LDPC parity nor omitted zeros enter BER.
/// # Errors
/// Rejects mismatched geometry or malformed source/decision vectors.
pub fn compare_payload(
    original: &[u8],
    decoded: &DecodedShortenedBlock,
) -> Result<PayloadComparison, String> {
    binary_bits(original, decoded.shortening.payload_bits())?;
    binary_bits(&decoded.payload, decoded.shortening.payload_bits())?;
    Ok(PayloadComparison {
        bits_tested: original.len(),
        bit_errors: original
            .iter()
            .zip(&decoded.payload)
            .filter(|(left, right)| left != right)
            .count(),
    })
}
