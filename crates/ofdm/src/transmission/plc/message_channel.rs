/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! MULPI I25 §6.5.4/Table77: bounded MC bytes and optional packet start pointer.
use super::geometry::MAX_MESSAGE_CHANNEL_BYTES;
use crate::{error::invalid, SimulationError};
use serde::Serialize;
const MESSAGE_CHANNEL_HEADER: u8 = 0x30;
const POINTER_PRESENT: u8 = 1;
const IDLE_BYTE: u8 = 0xff;
/// MC transport bytes; embedded MAC management packets are explicitly opaque here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct MessageChannelBlock {
    packet_start_pointer: Option<u16>,
    segment: Vec<u8>,
}
impl MessageChannelBlock {
    /// Copies a bounded MC segment without interpreting embedded MAC messages.
    /// # Arguments
    /// * `segment` - Transmission-order MAC packet fragments or0xff idle bytes.
    /// * `packet_start_pointer` - Offset from the byte after the pointer to a new
    ///   packet's first byte, or None for no packet start in this block.
    /// # Returns
    /// Owned block; packet CRC, OCD/DPD contents and cross-frame reassembly are not validated.
    /// # Errors
    /// Rejects blocks over708 bytes and pointers outside the actual segment.
    /// Pointer bounds are a strict laboratory policy, not MAC packet validation.
    pub fn new(segment: &[u8], packet_start_pointer: Option<u16>) -> Result<Self, SimulationError> {
        let header_bytes = 1 + usize::from(packet_start_pointer.is_some()) * 2;
        if segment.len() > MAX_MESSAGE_CHANNEL_BYTES - header_bytes
            || packet_start_pointer.is_some_and(|offset| usize::from(offset) >= segment.len())
        {
            return Err(invalid(
                "PLC message channel exceeds its bound or has an invalid packet pointer.",
            ));
        }
        Ok(Self {
            packet_start_pointer,
            segment: segment.to_vec(),
        })
    }
    /// Builds a complete MC block containing only the specified idle pattern.
    /// # Arguments
    /// * `block_bytes` - Total encoded size, including the one-byte MC header.
    /// # Returns
    /// Owned header0x30 followed by0xff bytes; no packet start pointer.
    /// # Errors
    /// Rejects sizes outside1..708 before allocating.
    pub fn idle(block_bytes: usize) -> Result<Self, SimulationError> {
        if !(1..=MAX_MESSAGE_CHANNEL_BYTES).contains(&block_bytes) {
            return Err(invalid("Idle PLC message channel requires1..708 bytes."));
        }
        Self::new(&vec![IDLE_BYTE; block_bytes - 1], None)
    }
    /// Borrows the MC payload without its header or optional pointer bytes.
    /// # Returns
    /// Read-only packet/idle bytes tied to this block's lifetime.
    pub fn segment(&self) -> &[u8] {
        &self.segment
    }
    /// Reports a packet start relative to the first segment byte.
    /// # Returns
    /// Known in-bounds offset or None; no fabricated packet identity.
    pub fn packet_start_pointer(&self) -> Option<u16> {
        self.packet_start_pointer
    }
    /// Returns total block length including header and optional two-byte pointer.
    /// # Returns
    /// At most708 bytes.
    pub fn encoded_len(&self) -> usize {
        1 + usize::from(self.packet_start_pointer.is_some()) * 2 + self.segment.len()
    }
    /// Serializes the MC block without a block-level CRC.
    /// # Returns
    /// Owned bytes; packet CRCs belong to embedded MAC management packets.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.encoded_len());
        bytes.push(MESSAGE_CHANNEL_HEADER | u8::from(self.packet_start_pointer.is_some()));
        if let Some(offset) = self.packet_start_pointer {
            bytes.extend_from_slice(&offset.to_be_bytes());
        }
        bytes.extend_from_slice(&self.segment);
        bytes
    }
    /// Parses a bounded MC block without retaining or mutating the source buffer.
    /// # Arguments
    /// * `bytes` - Entire MC block through the end of this frame's information payload.
    /// # Returns
    /// Owned segment and optional pointer, preserving opaque payload octets.
    /// # Errors
    /// Rejects empty/oversized blocks, wrong type, reserved bits, truncated pointer
    /// or a pointer outside the segment. Does not validate embedded packet CRCs.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SimulationError> {
        if bytes.is_empty() || bytes.len() > MAX_MESSAGE_CHANNEL_BYTES {
            return Err(invalid("Invalid PLC message-channel length."));
        }
        if bytes[0] & !POINTER_PRESENT != MESSAGE_CHANNEL_HEADER {
            return Err(SimulationError::new(
                "UnsupportedPlcMessageBlock",
                "Expected an MC block with reserved bits zero.",
            ));
        }
        if bytes[0] & POINTER_PRESENT != 0 {
            if bytes.len() < 3 {
                return Err(invalid("Truncated PLC packet-start pointer."));
            }
            Self::new(&bytes[3..], Some(u16::from_be_bytes([bytes[1], bytes[2]])))
        } else {
            Self::new(&bytes[1..], None)
        }
    }
}
