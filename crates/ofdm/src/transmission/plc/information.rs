/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.5/Figure63:36-byte blocks become288 MSB-first information bits.
use super::geometry::{PLC_INFORMATION_BITS, PLC_INFORMATION_BYTES};
use crate::{error::invalid, SimulationError};
use serde::Serialize;
/// Unencoded information block; no parity or puncturing is implied by this type.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlcInformationBlock {
    bytes: Vec<u8>,
}
impl PlcInformationBlock {
    /// Copies one bounded information block before FEC processing.
    /// # Arguments
    /// * `bytes` - Exactly36 message bytes in transmission order.
    /// # Returns
    /// Owned immutable information bytes.
    /// # Errors
    /// Rejects any size other than36 before copying.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SimulationError> {
        if bytes.len() != PLC_INFORMATION_BYTES {
            return Err(invalid("PLC FEC information requires exactly36 bytes."));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }
    /// Borrows the information bytes in original order.
    /// # Returns
    /// Read-only36-byte slice with no header stripping or byte reversal.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Expands the information bytes into the encoder's bit order.
    /// # Returns
    ///288 binary values, byte0 bit7 first, then bit6 through bit0, then byte1.
    /// This is byte-to-bit conversion only, not FEC encoding.
    pub fn bits(&self) -> [u8; PLC_INFORMATION_BITS] {
        std::array::from_fn(|index| (self.bytes[index / 8] >> (7 - index % 8)) & 1)
    }
}
