/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Parity interleaving, column twists and cell demultiplexing, plus soft inverse.
mod boundary;
pub mod demultiplexing;
mod parameters;
pub mod parity;
pub mod twist;
use crate::{
    error::invalid, signal_processing::modulation::qam::SymbolLabel,
    types::ofdm_modulation::ConstellationOrder, LabError,
};
use fec_codec::parameters::LDPC_CODEWORD_BITS;
use parameters::Parameters;

/// Immutable short8/9 uniform-modulation bit permutation, reusable across words.
///
/// Owned state has no symbol/channel timing.16and256-QAM are supported; other
/// mapped orders still need their separately verified coding-bit rules.
pub struct BitInterleaver {
    bits_per_cell: usize,
    output_to_codeword: Vec<usize>,
}
impl BitInterleaver {
    /// Builds a pinned16200-bit parity/twist/demultiplexer permutation.
    /// # Arguments
    /// * `order` - Uniform data constellation16or256; no shortening/mixed loading.
    /// # Returns
    /// Reusable mapping from MSB-first cell bits back to original codeword indices.
    /// # Errors
    /// Returns UnsupportedCodingMode for other orders; never substitutes a mode.
    pub fn new(order: ConstellationOrder) -> Result<Self, LabError> {
        let parameters = parameters::require(order)?;
        Ok(Self {
            bits_per_cell: parameters.bits_per_cell,
            output_to_codeword: build_permutation(parameters),
        })
    }

    /// Borrows the full output-bit→original-codeword-bit permutation.
    /// # Returns
    /// Read-only16200-element bijection in MSB-first cell-bit order.
    pub fn output_to_codeword(&self) -> &[usize] {
        &self.output_to_codeword
    }

    /// Converts one encoded codeword to cell-word labels without changing input.
    /// # Arguments
    /// * `codeword` - Exactly16200 binary bytes, one0or1 per bit.
    /// # Returns
    /// MSB-first labels:4050for16-QAM or2025for256-QAM; no padding for these modes.
    /// # Errors
    /// Rejects wrong length or nonbinary values before creating labels.
    pub fn encode_labels(&self, codeword: &[u8]) -> Result<Vec<SymbolLabel>, LabError> {
        if codeword.len() != LDPC_CODEWORD_BITS || codeword.iter().any(|bit| *bit > 1) {
            return Err(invalid("Expected16200 binary LDPC bits."));
        }
        Ok(self
            .output_to_codeword
            .chunks_exact(self.bits_per_cell)
            .map(|indices| {
                indices.iter().fold(0, |label, index| {
                    (label << 1) | SymbolLabel::from(codeword[*index])
                })
            })
            .collect())
    }

    /// Reverses cell demultiplexing, column twists and parity interleaving on LLRs.
    /// # Arguments
    /// * `cell_llrs` - Exactly16200 finite MSB-first LLRs, after inverse randomization.
    /// # Returns
    /// Owned codeword-order LLRs with signs/magnitudes preserved, ready for LDPC.
    /// # Errors
    /// Rejects wrong length or nonfinite values; input remains unchanged.
    pub fn decode_llrs(&self, cell_llrs: &[f64]) -> Result<Vec<f64>, LabError> {
        if cell_llrs.len() != LDPC_CODEWORD_BITS || cell_llrs.iter().any(|value| !value.is_finite())
        {
            return Err(invalid("Expected16200 finite cell-order LLRs."));
        }
        let mut codeword_llrs = vec![0.0; LDPC_CODEWORD_BITS];
        for (llr, original) in cell_llrs.iter().zip(&self.output_to_codeword) {
            codeword_llrs[*original] = *llr;
        }
        Ok(codeword_llrs)
    }
}

/// Returns the established16-QAM output-bit→codeword-bit ordering.
/// # Returns
/// A full16200-element bijection, unchanged from the original coding experiment.
pub fn transmit_bit_order() -> Vec<usize> {
    build_permutation(&parameters::SHORT16)
}

fn build_permutation(parameters: &Parameters) -> Vec<usize> {
    let parity = parity::permutation();
    let twist = twist::for_parameters(parameters);
    demultiplexing::for_parameters(parameters)
        .into_iter()
        .map(|position| parity[twist[position]])
        .collect()
}
