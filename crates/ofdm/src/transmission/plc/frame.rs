/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded TS+MC frame subset; no descriptor synthesis, EM/TR or FEC implementation.
use super::{
    geometry::TIMESTAMP_BLOCK_BYTES, ExtendedTimestamp, MessageChannelBlock, PlcFrameGeometry,
    PlcInformationBlock, TimestampMessageBlock,
};
use crate::{error::invalid, transmission::SubcarrierSpacing, LabError};
use serde::Serialize;
/// A full unencoded PLC information frame with exactly one TS followed by one MC.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlcMessageFrame {
    spacing: SubcarrierSpacing,
    timestamp: TimestampMessageBlock,
    message_channel: MessageChannelBlock,
}
impl PlcMessageFrame {
    /// Assembles the supported message-block subset at an exact frame boundary.
    /// # Arguments
    /// * `spacing` - Fixed25/50kHz numerology determining frame capacity.
    /// * `timestamp` - Timestamp at the end of this frame's preceding PLC preamble.
    /// * `message_channel` - Complete final MC block, including any caller padding.
    /// # Returns
    /// Owned TS+MC frame; retains no duplicate full-frame byte buffer.
    /// # Errors
    /// Rejects an MC block that does not exactly fill the remaining payload.
    pub fn new(
        spacing: SubcarrierSpacing,
        timestamp: ExtendedTimestamp,
        message_channel: MessageChannelBlock,
    ) -> Result<Self, LabError> {
        let capacity = PlcFrameGeometry::new(spacing).payload_bytes;
        if TIMESTAMP_BLOCK_BYTES + message_channel.encoded_len() != capacity {
            return Err(invalid(
                "PLC timestamp and message channel must exactly fill the frame payload.",
            ));
        }
        Ok(Self {
            spacing,
            timestamp: TimestampMessageBlock { timestamp },
            message_channel,
        })
    }
    /// Builds a valid timestamp plus idle MC frame for testing the PHY boundary.
    /// # Arguments
    /// * `spacing` - Fixed spacing;720-byte8K or360-byte4K information payload.
    /// * `timestamp` - Explicit DOCSIS reference timestamp, never host time.
    /// # Returns
    /// Complete unencoded frame with0xff idle fill and no acquisition descriptors.
    /// # Errors
    /// Propagates unexpected capacity validation failures.
    pub fn idle(
        spacing: SubcarrierSpacing,
        timestamp: ExtendedTimestamp,
    ) -> Result<Self, LabError> {
        let channel = MessageChannelBlock::idle(
            PlcFrameGeometry::new(spacing).payload_bytes - TIMESTAMP_BLOCK_BYTES,
        )?;
        Self::new(spacing, timestamp, channel)
    }
    /// Borrows the decoded MC block for receiver-owned packet reassembly.
    /// # Returns
    /// Immutable message-channel bytes and pointer, without serializing a duplicate frame.
    pub fn message_channel(&self) -> &MessageChannelBlock {
        &self.message_channel
    }

    /// Returns the fixed numerology without inferring it from encoded byte counts.
    /// # Returns
    /// The spacing supplied when this validated message frame was constructed.
    pub fn spacing(&self) -> SubcarrierSpacing {
        self.spacing
    }

    /// Reports the frame's immutable numerology-derived dimensions.
    /// # Returns
    /// Small owned geometry record; no signal or parity bytes generated.
    pub fn geometry(&self) -> PlcFrameGeometry {
        PlcFrameGeometry::new(self.spacing)
    }
    /// Serializes information bytes in message transmission order.
    /// # Returns
    /// Exactly360or720 bytes; TS CRC is recomputed from its validated timestamp.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.geometry().payload_bytes);
        bytes.extend_from_slice(&self.timestamp.to_bytes());
        bytes.extend_from_slice(&self.message_channel.to_bytes());
        bytes
    }
    /// Splits the message byte stream into consecutive FEC input blocks.
    /// # Returns
    /// Ten or twenty owned36-byte information blocks. MB boundaries do not reset FEC.
    /// # Errors
    /// Propagates unexpected information-block validation failures.
    pub fn information_blocks(&self) -> Result<Vec<PlcInformationBlock>, LabError> {
        self.to_bytes()
            .chunks_exact(super::geometry::PLC_INFORMATION_BYTES)
            .map(PlcInformationBlock::from_bytes)
            .collect()
    }
    /// Parses the supported timestamp-plus-MC subset without hidden transmitter IDs.
    /// # Arguments
    /// * `spacing` - Known fixed channel numerology; not inferred from a filename.
    /// * `bytes` - Complete360or720-byte information payload, not encoded parity.
    /// # Returns
    /// Owned validated frame; original bytes remain unchanged.
    /// # Errors
    /// Rejects size, timestamp/CRC errors, unsupported intervening blocks,
    /// reserved MC fields or invalid pointers. Does not validate MAC packet content.
    pub fn from_bytes(spacing: SubcarrierSpacing, bytes: &[u8]) -> Result<Self, LabError> {
        if bytes.len() != PlcFrameGeometry::new(spacing).payload_bytes {
            return Err(invalid(
                "PLC information payload length does not match spacing.",
            ));
        }
        let timestamp = TimestampMessageBlock::from_bytes(&bytes[..TIMESTAMP_BLOCK_BYTES])?;
        let channel = MessageChannelBlock::from_bytes(&bytes[TIMESTAMP_BLOCK_BYTES..])?;
        Self::new(spacing, timestamp.timestamp, channel)
    }
}
