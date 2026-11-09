/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Figure65: MSB-first grouping before the separate PLC permutation.
use super::geometry::{PLC_BITS_PER_CELL, PLC_CODEWORD_BITS, PLC_CODEWORD_NIBBLES};
use crate::{error::invalid, SimulationError};
use serde::Serialize;
/// Four-bit PLC constellation label; validated containers restrict values to0..15.
pub type PlcNibble = u8;
/// Exactly96 validated labels in punctured-codeword order, before interleaving.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlcCodewordNibbles {
    values: Vec<PlcNibble>,
}
impl PlcCodewordNibbles {
    /// Groups borrowed binary bits without reversing codeword or bit significance.
    /// # Arguments
    /// * `bits` -384 binary values in retained mother-coordinate order.
    /// # Returns
    /// Owned96-label block; first bit of each group is its label MSB.
    /// # Errors
    /// Rejects incorrect size or nonbinary input before constructing output.
    pub fn from_bits(bits: &[u8]) -> Result<Self, SimulationError> {
        if bits.len() != PLC_CODEWORD_BITS || bits.iter().any(|bit| *bit > 1) {
            log::debug!("plc-nibble-bits-rejected actual-bits={}", bits.len());
            return Err(invalid("PLC nibble grouping requires384 binary bits."));
        }
        let values = bits
            .chunks_exact(PLC_BITS_PER_CELL)
            .map(|bits| bits.iter().fold(0, |value, bit| (value << 1) | bit))
            .collect();
        Ok(Self { values })
    }
    /// Copies one validated label block for direct stage use or receiver inversion.
    /// # Arguments
    /// * `values` - Exactly96 labels in0..15, not arbitrary subcarrier indices.
    /// # Returns
    /// Owned codeword labels; source remains unchanged.
    /// # Errors
    /// Rejects invalid count or labels before copying.
    pub fn from_values(values: &[PlcNibble]) -> Result<Self, SimulationError> {
        if values.len() != PLC_CODEWORD_NIBBLES
            || values.iter().any(|value| *value >= 1 << PLC_BITS_PER_CELL)
        {
            log::debug!("plc-nibble-values-rejected actual-labels={}", values.len());
            return Err(invalid("PLC codeword requires96 labels in0..15."));
        }
        Ok(Self {
            values: values.to_vec(),
        })
    }
    /// Borrows labels in the encoder's original codeword order.
    /// # Returns
    /// Read-only96-element slice; lifetime is tied to this container.
    pub fn values(&self) -> &[PlcNibble] {
        &self.values
    }
    /// Expands labels into their original MSB-first binary stream.
    /// # Returns
    ///384 binary values; no randomizer or soft decision is applied.
    pub fn bits(&self) -> [u8; PLC_CODEWORD_BITS] {
        std::array::from_fn(|index| {
            (self.values[index / PLC_BITS_PER_CELL]
                >> (PLC_BITS_PER_CELL - 1 - index % PLC_BITS_PER_CELL))
                & 1
        })
    }
}
