/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 sections7.5.4.3.2 and7.5.5.2.2, PDF117/122–123: non-shortened mixed words.
use super::{parity, BitInterleaver};
use crate::{error::invalid, SimulationError};
use fec_codec::parameters::LDPC_CODEWORD_BITS;
pub(super) const MIXED_COLUMNS: usize = 24;
const MIXED_ROWS: usize = LDPC_CODEWORD_BITS / MIXED_COLUMNS;
// Transcribed Tables27/28; mixed mode bypasses the bit-to-cell demultiplexer.
pub(super) const MIXED_TWISTS: [usize; MIXED_COLUMNS] = [
    0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 7, 9, 9, 9, 10, 10, 10, 10, 10, 11,
];
/// Common immutable coded-bit ordering, before per-subcarrier variable grouping.
pub struct PayloadBitOrder {
    output_to_codeword: Vec<usize>,
}
impl PayloadBitOrder {
    /// Selects normative full uniform or mixed-profile bit processing.
    /// # Arguments
    /// * `loading` - Validated nonempty source-rank bit counts; zeros are ignored
    ///   only when deciding whether the profile is mixed, as PHY I20 requires.
    /// # Returns
    /// Full-word permutation including explicit uniform padding when required.
    /// # Errors
    /// Rejects invalid/all-zero loading or unsupported uniform coding modes.
    pub fn new(loading: &[u8]) -> Result<Self, SimulationError> {
        if loading.is_empty() || loading.iter().any(|bits| !matches!(bits, 0 | 4 | 6..=14)) {
            return Err(invalid("Invalid data bit loading."));
        }
        let mut distinct: Vec<_> = loading.iter().copied().filter(|bits| *bits > 0).collect();
        distinct.sort_unstable();
        distinct.dedup();
        let output_to_codeword = if distinct.len() > 1 {
            let parity = parity::permutation();
            (0..MIXED_ROWS)
                .flat_map(|row| {
                    (0..MIXED_COLUMNS).map(move |column| {
                        column * MIXED_ROWS + (row + MIXED_ROWS - MIXED_TWISTS[column]) % MIXED_ROWS
                    })
                })
                .map(|position| parity[position])
                .collect()
        } else if let Some(bits) = distinct.first() {
            BitInterleaver::new(1u16 << bits)?
                .output_to_codeword()
                .to_vec()
        } else {
            return Err(invalid("All-zero data profile cannot transport payload."));
        };
        Ok(Self { output_to_codeword })
    }
    /// Borrows the MSB-first mapper-bit to original LDPC-bit permutation.
    /// # Returns
    /// Output bit indices, including appended-zero indices >=16200 in uniform modes.
    pub fn permutation(&self) -> &[usize] {
        &self.output_to_codeword
    }
    /// Orders an encoded codeword before variable-width cell grouping.
    /// # Arguments
    /// * `bits` - Exactly16200 binary encoded LDPC bits.
    /// # Returns
    /// Owned ordered bits, with no final-cell padding yet.
    /// # Errors
    /// Rejects malformed binary input before indexing.
    pub fn encode(&self, bits: &[u8]) -> Result<Vec<u8>, SimulationError> {
        if bits.len() != LDPC_CODEWORD_BITS || bits.iter().any(|bit| *bit > 1) {
            return Err(invalid("Expected16200 binary LDPC bits."));
        }
        Ok(self
            .output_to_codeword
            .iter()
            .map(|index| bits.get(*index).copied().unwrap_or(0))
            .collect())
    }
    /// Reverses grouping-order LLRs to LDPC codeword order.
    /// # Arguments
    /// * `llrs` - Finite likelihoods matching permutation length, with final-cell padding removed.
    /// # Returns
    /// Original codeword-order likelihoods without changing signs/magnitudes.
    /// # Errors
    /// Rejects incorrect length or nonfinite values.
    pub fn decode(&self, llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
        if llrs.len() != self.output_to_codeword.len()
            || llrs.iter().any(|value| !value.is_finite())
        {
            return Err(invalid(
                "Expected finite likelihoods matching profile-coded length.",
            ));
        }
        let mut result = vec![0.0; LDPC_CODEWORD_BITS];
        for (value, index) in llrs.iter().zip(&self.output_to_codeword) {
            if *index < LDPC_CODEWORD_BITS {
                result[*index] = *value;
            }
        }
        Ok(result)
    }
}
