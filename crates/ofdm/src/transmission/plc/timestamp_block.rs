/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! MULPI I25 §6.5.2/Table75: timestamp block header0x10 and CRC-24-D.
use super::{geometry::TIMESTAMP_BLOCK_BYTES, ExtendedTimestamp};
use crate::{error::invalid, LabError};
use fec_codec::crc::crc24d;
use serde::Serialize;
const TIMESTAMP_HEADER: u8 = 0x10;
const CRC_OFFSET: usize = TIMESTAMP_BLOCK_BYTES - 3;
/// Exactly one validated timestamp block; scheduling is owned by frame construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TimestampMessageBlock {
    pub timestamp: ExtendedTimestamp,
}
impl TimestampMessageBlock {
    /// Serializes the header, timestamp and CRC without modifying the timestamp.
    /// # Returns
    /// Twelve bytes; CRC covers header plus eight timestamp bytes.
    pub fn to_bytes(self) -> [u8; TIMESTAMP_BLOCK_BYTES] {
        let mut bytes = [0; TIMESTAMP_BLOCK_BYTES];
        bytes[0] = TIMESTAMP_HEADER;
        bytes[1..CRC_OFFSET].copy_from_slice(&self.timestamp.to_bytes());
        let crc = crc24d(&bytes[..CRC_OFFSET]);
        bytes[CRC_OFFSET..].copy_from_slice(&crc);
        bytes
    }
    /// Parses and verifies a timestamp block before exposing any timestamp fields.
    /// # Arguments
    /// * `bytes` - Twelve unencoded message bytes, in transmission order.
    /// # Returns
    /// Owned timestamp with validated modulo20 fields.
    /// # Errors
    /// Rejects wrong size/type, nonzero reserved bits, CRC corruption or bad time.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, LabError> {
        if bytes.len() != TIMESTAMP_BLOCK_BYTES || bytes[0] != TIMESTAMP_HEADER {
            return Err(invalid(
                "Expected a12-byte timestamp block with reserved bits zero.",
            ));
        }
        if bytes[CRC_OFFSET..] != crc24d(&bytes[..CRC_OFFSET]) {
            return Err(LabError::new(
                "PlcTimestampCrcMismatch",
                "PLC timestamp CRC-24-D mismatch.",
            ));
        }
        let timestamp_bytes = bytes[1..CRC_OFFSET]
            .try_into()
            .map_err(|_| invalid("Invalid timestamp byte count."))?;
        Ok(Self {
            timestamp: ExtendedTimestamp::from_bytes(timestamp_bytes)?,
        })
    }
}
