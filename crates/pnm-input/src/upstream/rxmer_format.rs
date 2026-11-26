/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! CCAP-OSSI I18 Table 410, printed p419: one owner for decoder/encoder fields.
use docsis_ofdma::configuration::Numerology;

pub(super) const FILE_TYPE: [u8; 4] = [0x50, 0x4e, 0x4e, 0x69];
pub(super) const MAJOR_VERSION: u8 = 2;
pub(super) const MINOR_VERSION: u8 = 0;
pub(super) const HEADER_BYTES: usize = 297;
pub(super) const MAJOR_OFFSET: usize = 4;
pub(super) const MINOR_OFFSET: usize = 5;
pub(super) const TIME_OFFSET: usize = 6;
pub(super) const INTERFACE_OFFSET: usize = 10;
pub(super) const CCAP_ID_START: usize = 14;
pub(super) const CCAP_ID_END: usize = 270;
pub(super) const CCAP_ID_BYTES: usize = CCAP_ID_END - CCAP_ID_START;
pub(super) const RPD_ID_START: usize = 270;
pub(super) const RPD_ID_END: usize = 276;
pub(super) const RPD_PORT_OFFSET: usize = 276;
pub(super) const CM_MAC_START: usize = 277;
pub(super) const CM_MAC_END: usize = 283;
pub(super) const RPD_ID_BYTES: usize = RPD_ID_END - RPD_ID_START;
pub(super) const CM_MAC_BYTES: usize = CM_MAC_END - CM_MAC_START;
pub(super) const AVERAGES_OFFSET: usize = 283;
pub(super) const PRE_EQ_OFFSET: usize = 285;
pub(super) const ZERO_FREQUENCY_OFFSET: usize = 286;
pub(super) const FIRST_CARRIER_OFFSET: usize = 290;
pub(super) const SPACING_OFFSET: usize = 292;
pub(super) const DATA_LENGTH_OFFSET: usize = 293;
pub(super) const SPACING_25_KHZ: u8 = 25;
pub(super) const SPACING_50_KHZ: u8 = 50;

// Application storage budget, independently bounded from a particular FFT mode.
pub(super) const MAX_DATA_BYTES: usize = 4096;
pub(super) const MAX_FILE_BYTES: usize = HEADER_BYTES + MAX_DATA_BYTES;

/// Looks up supported upstream geometry without reconstructing PHY dimensions.
///
/// # Arguments
/// * `spacing_khz` - Original one-octet file spacing, in kHz.
///
/// # Returns
/// The authoritative PHY FFT length, or None for an unsupported file profile.
pub(super) fn fft_len(spacing_khz: u8) -> Option<usize> {
    match spacing_khz {
        SPACING_25_KHZ => Some(Numerology::Khz25.fft_len()),
        SPACING_50_KHZ => Some(Numerology::Khz50.fft_len()),
        _ => None,
    }
}
