/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.2,.6–.7 and MULPI I25 Table74: fixed PLC frame dimensions.
use crate::transmission::SubcarrierSpacing;
use serde::Serialize;
/// Information bytes entering one PLC FEC encoder, before puncturing.
pub const PLC_INFORMATION_BYTES: usize = 36;
/// Information bits entering the (480,288) mother encoder.
pub const PLC_INFORMATION_BITS: usize = PLC_INFORMATION_BYTES * 8;
/// Bytes represented by one transmitted punctured384-bit codeword.
pub const PLC_CODEWORD_BYTES: usize = 48;
/// Total number of preamble symbols at the end of the post-preamble index cycle.
pub const PLC_PREAMBLE_SYMBOLS: usize = 8;
/// Number of data symbols per128-symbol PLC frame.
pub const PLC_DATA_SYMBOLS: usize = crate::types::PLC_FRAME_SYMBOLS as usize - PLC_PREAMBLE_SYMBOLS;
/// Header, eight-byte timestamp and three-byte CRC-24-D.
pub const TIMESTAMP_BLOCK_BYTES: usize = 12;
/// Maximum PLC information payload, excluding preamble and FEC redundancy.
pub const MAX_PLC_PAYLOAD_BYTES: usize = 720;
/// Largest MC block when the mandatory timestamp is the only preceding block.
pub const MAX_MESSAGE_CHANNEL_BYTES: usize = MAX_PLC_PAYLOAD_BYTES - TIMESTAMP_BLOCK_BYTES;
/// Immutable fixed dimensions, derived from spacing rather than supplied separately.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PlcFrameGeometry {
    spacing: SubcarrierSpacing,
    pub plc_subcarriers: usize,
    pub data_symbols_per_codeword: usize,
    pub codewords_per_frame: usize,
    pub payload_bytes: usize,
    pub encoded_bytes: usize,
}
impl PlcFrameGeometry {
    /// Derives PLC frame dimensions for one fixed downstream numerology.
    /// # Arguments
    /// * `spacing` -25kHz/8K or50kHz/4K spacing.
    /// # Returns
    ///20 blocks/720 information bytes or10 blocks/360 information bytes;
    /// encoded byte counts describe the future encoder output, not generated parity.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        let plc_subcarriers = match spacing {
            SubcarrierSpacing::Khz25 => 16,
            SubcarrierSpacing::Khz50 => 8,
        };
        let data_symbols_per_codeword = PLC_CODEWORD_BYTES * 2 / plc_subcarriers;
        let codewords_per_frame = PLC_DATA_SYMBOLS / data_symbols_per_codeword;
        Self {
            spacing,
            plc_subcarriers,
            data_symbols_per_codeword,
            codewords_per_frame,
            payload_bytes: codewords_per_frame * PLC_INFORMATION_BYTES,
            encoded_bytes: codewords_per_frame * PLC_CODEWORD_BYTES,
        }
    }
}
