/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded message-channel receive reassembly; MC segmentation is not MMM multipart.
use super::{
    mac_packets::{
        ManagementPacket, MAC_HEADER_BYTES, MANAGEMENT_FC, MAX_MANAGEMENT_PDU_BYTES,
        MIN_MANAGEMENT_FRAME_BYTES,
    },
    MessageChannelBlock,
};
use crate::{error::invalid, LabError};
/// Receiver-owned incomplete MAC frame, bounded to2006 bytes.
#[derive(Debug, Clone, Default)]
pub struct PlcMessageReassembler {
    partial: Vec<u8>,
}
impl PlcMessageReassembler {
    /// Creates empty receive history without transmitter packet boundaries.
    /// # Returns
    /// Receiver accepting pointer-aligned packets and idle bytes.
    pub fn new() -> Self {
        Self::default()
    }
    /// Borrows incomplete packet bytes for a direct receive test point.
    /// # Returns
    /// Empty when all received packets are complete; no fabricated padding.
    pub fn pending(&self) -> &[u8] {
        &self.partial
    }
    /// Reassembles one consecutive MC block using only pointer, lengths and CRCs.
    /// # Arguments
    /// * `block` - Decoded validated MC bytes, including optional first-start pointer.
    /// # Returns
    /// Complete CRC-validated packets. At startup, an unknown continuation prefix
    /// before a known pointer is discarded. State commits only if the whole block passes.
    /// # Errors
    /// Rejects invalid lengths/checksums, missing starts, inconsistent pointers or
    /// unexpected nonidle tails; original pending history is retained on rejection.
    pub fn push(&mut self, block: &MessageChannelBlock) -> Result<Vec<ManagementPacket>, LabError> {
        let mut candidate = self.clone();
        let mut output = Vec::new();
        if let Some(pointer) = block.packet_start_pointer() {
            let split = usize::from(pointer);
            if !candidate.partial.is_empty() {
                candidate.consume(&block.segment()[..split], false, &mut output)?;
                if !candidate.partial.is_empty() {
                    return Err(invalid(
                        "New PLC packet starts before its predecessor ends.",
                    ));
                }
            }
            candidate.consume(&block.segment()[split..], true, &mut output)?;
        } else {
            candidate.consume(block.segment(), false, &mut output)?;
        }
        *self = candidate;
        Ok(output)
    }
    fn consume(
        &mut self,
        mut bytes: &[u8],
        allow_starts: bool,
        output: &mut Vec<ManagementPacket>,
    ) -> Result<(), LabError> {
        while !bytes.is_empty() {
            if self.partial.is_empty() {
                if bytes[0] == 0xff {
                    bytes = &bytes[1..];
                    continue;
                }
                if !allow_starts {
                    return Err(invalid("PLC packet start has no matching pointer."));
                }
            }
            let needed = if self.partial.len() < 4 {
                4 - self.partial.len()
            } else {
                let total = MAC_HEADER_BYTES
                    + usize::from(u16::from_be_bytes([self.partial[2], self.partial[3]]));
                if self.partial[0..2] != [MANAGEMENT_FC, 0]
                    || !(MIN_MANAGEMENT_FRAME_BYTES..=MAC_HEADER_BYTES + MAX_MANAGEMENT_PDU_BYTES)
                        .contains(&total)
                {
                    return Err(invalid("Invalid segmented PLC MAC header."));
                }
                total - self.partial.len()
            };
            let take = needed.min(bytes.len());
            self.partial.extend_from_slice(&bytes[..take]);
            bytes = &bytes[take..];
            if self.partial.len() >= 4 {
                let total = MAC_HEADER_BYTES
                    + usize::from(u16::from_be_bytes([self.partial[2], self.partial[3]]));
                if !(MIN_MANAGEMENT_FRAME_BYTES..=MAC_HEADER_BYTES + MAX_MANAGEMENT_PDU_BYTES)
                    .contains(&total)
                    || self.partial[0..2] != [MANAGEMENT_FC, 0]
                {
                    return Err(invalid("Invalid segmented PLC packet length."));
                }
                if self.partial.len() == total {
                    output.push(ManagementPacket::from_bytes(&self.partial)?);
                    self.partial.clear();
                }
            }
        }
        Ok(())
    }
}
