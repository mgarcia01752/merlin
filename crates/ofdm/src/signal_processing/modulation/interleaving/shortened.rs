/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.4.3.3: parity first,24 twisted columns, discard unused holes.
use super::mixed::{MIXED_COLUMNS, MIXED_TWISTS};
use crate::{error::invalid, SimulationError};
use fec_codec::shortening::Shortening;
/// Immutable output-bit to transmitted-short-word permutation; no padding is emitted.
pub struct ShortenedBitOrder {
    geometry: Shortening,
    output_to_codeword: Vec<usize>,
}
impl ShortenedBitOrder {
    /// Builds the shortened-word map independently of uniform or mixed loading.
    /// # Arguments
    /// * `geometry` - Checked original payload size; must actually omit mother bits.
    /// # Returns
    /// A bounded bijection over K+1968 coordinates; unused matrix entries are skipped.
    /// # Errors
    /// Rejects full words, which must select uniform/mixed rules instead.
    pub fn new(geometry: Shortening) -> Result<Self, SimulationError> {
        if geometry.omitted_bits() == 0 {
            return Err(invalid(
                "Select full-word bit rules for an unshortened payload.",
            ));
        }
        let count = geometry.transmitted_bits();
        let rows = count.div_ceil(MIXED_COLUMNS);
        let mut matrix = vec![None; rows * MIXED_COLUMNS];
        let parity = super::parity::with_systematic_bits(geometry.bch_bits());
        for (position, original) in parity.iter().enumerate() {
            let column = position / rows;
            let row = (position % rows + MIXED_TWISTS[column]) % rows;
            matrix[row * MIXED_COLUMNS + column] = Some(*original);
        }
        Ok(Self {
            geometry,
            output_to_codeword: matrix.into_iter().flatten().collect(),
        })
    }
    /// Borrows mapper-bit to short-word indices, excluding unused matrix holes.
    /// # Returns
    /// Exactly K+1968 unique coordinate indices.
    pub fn permutation(&self) -> &[usize] {
        &self.output_to_codeword
    }
    /// Applies shortened parity and twist ordering to a binary transmitted word.
    /// # Arguments
    /// * `bits` - Exactly K+1968 binary values, without final-cell padding.
    /// # Returns
    /// Mapper-order bits; input remains unchanged.
    /// # Errors
    /// Rejects malformed length/values before indexing.
    pub fn encode(&self, bits: &[u8]) -> Result<Vec<u8>, SimulationError> {
        if bits.len() != self.geometry.transmitted_bits() || bits.iter().any(|bit| *bit > 1) {
            return Err(invalid("Invalid shortened coded bits."));
        }
        Ok(self
            .output_to_codeword
            .iter()
            .map(|index| bits[*index])
            .collect())
    }
    /// Reverses shortened ordering on likelihoods without hard slicing.
    /// # Arguments
    /// * `llrs` - Finite mapper-order values, with final-cell padding already removed.
    /// # Returns
    /// K+1968 codeword-order LLRs for known-zero mother restoration.
    /// # Errors
    /// Rejects malformed shape/nonfinite values.
    pub fn decode(&self, llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
        if llrs.len() != self.geometry.transmitted_bits()
            || llrs.iter().any(|value| !value.is_finite())
        {
            return Err(invalid("Invalid shortened likelihoods."));
        }
        let mut result = vec![0.0; llrs.len()];
        for (value, index) in llrs.iter().zip(&self.output_to_codeword) {
            result[*index] = *value;
        }
        Ok(result)
    }
}
