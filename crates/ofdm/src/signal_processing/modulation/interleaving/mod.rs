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
    types::ofdm_modulation::ConstellationOrder, SimulationError,
};
use fec_codec::parameters::LDPC_CODEWORD_BITS;
use parameters::Parameters;

/// Immutable short8/9 uniform-modulation bit permutation, reusable across words.
///
/// Owned state has no symbol/channel timing. All downstream data orders through16384-QAM
/// use their pinned full-word rules.
pub struct BitInterleaver {
    bits_per_cell: usize,
    output_to_codeword: Vec<usize>,
}
impl BitInterleaver {
    /// Builds a pinned16200-bit parity/twist/demultiplexer permutation.
    /// # Arguments
    /// * `order` - Supported uniform data order through16384; no shortening/mixed loading.
    /// # Returns
    /// Reusable mapping from MSB-first cell bits back to original codeword indices.
    /// # Errors
    /// Returns UnsupportedCodingMode for other orders; never substitutes a mode.
    pub fn new(order: ConstellationOrder) -> Result<Self, SimulationError> {
        let parameters = parameters::require(order)?;
        Ok(Self {
            bits_per_cell: parameters.bits_per_cell,
            output_to_codeword: build_permutation(parameters),
        })
    }

    /// Borrows the full output-bit→original-codeword-bit permutation.
    /// # Returns
    /// MSB-first bit indices; entries >=16200 identify appended zero-pad coordinates.
    pub fn output_to_codeword(&self) -> &[usize] {
        &self.output_to_codeword
    }

    /// Converts one encoded codeword to cell-word labels without changing input.
    /// # Arguments
    /// * `codeword` - Exactly16200 binary bytes, one0or1 per bit.
    /// # Returns
    /// MSB-first labels, including PHY-specified zero extension for applicable orders.
    /// # Errors
    /// Rejects wrong length or nonbinary values before creating labels.
    pub fn encode_labels(&self, codeword: &[u8]) -> Result<Vec<SymbolLabel>, SimulationError> {
        if codeword.len() != LDPC_CODEWORD_BITS || codeword.iter().any(|bit| *bit > 1) {
            return Err(invalid("Expected16200 binary LDPC bits."));
        }
        Ok(self
            .output_to_codeword
            .chunks_exact(self.bits_per_cell)
            .map(|indices| {
                indices.iter().fold(0, |label, index| {
                    (label << 1) | SymbolLabel::from(codeword.get(*index).copied().unwrap_or(0))
                })
            })
            .collect())
    }

    /// Reverses cell demultiplexing, column twists and parity interleaving on LLRs.
    /// # Arguments
    /// * `cell_llrs` - Exactly the padded output-bit count of finite MSB-first LLRs, after inverse randomization.
    /// # Returns
    /// Owned codeword-order LLRs with signs/magnitudes preserved, ready for LDPC.
    /// # Errors
    /// Rejects wrong length or nonfinite values; input remains unchanged.
    pub fn decode_llrs(&self, cell_llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
        if cell_llrs.len() != self.output_to_codeword.len()
            || cell_llrs.iter().any(|value| !value.is_finite())
        {
            return Err(invalid(
                "Expected finite LLRs matching the padded uniform bit count.",
            ));
        }
        let mut codeword_llrs = vec![0.0; LDPC_CODEWORD_BITS];
        for (llr, original) in cell_llrs.iter().zip(&self.output_to_codeword) {
            if *original < LDPC_CODEWORD_BITS {
                codeword_llrs[*original] = *llr;
            }
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
    let mut parity = parity::permutation();
    parity.extend(LDPC_CODEWORD_BITS..parameters.rows * parameters.twists.len());
    let twist = twist::for_parameters(parameters);
    demultiplexing::for_parameters(parameters)
        .into_iter()
        .map(|position| parity[twist[position]])
        .collect()
}

pub mod mixed;

pub mod shortened;
