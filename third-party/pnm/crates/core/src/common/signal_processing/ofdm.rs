/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ideal useful-symbol OFDM rates; no cyclic-prefix, FEC or MAC assumptions.
use crate::pnm::data_type::{BitCount, BitRateBps, CarrierCount, FrequencyHz};

/// Calculates ideal gross rate using reciprocal useful-symbol duration.
/// # Arguments
/// * `data_bits` - Sum of data-carrier bits per useful symbol.
/// * `spacing_hz` - Positive subcarrier spacing; 25/50 kHz give 40/20 µs.
/// # Returns
/// Bits per second before cyclic-prefix, FEC and MAC overhead, or None for
/// zero spacing or integer overflow. This is not measured net throughput.
pub fn useful_symbol_rate(data_bits: BitCount, spacing_hz: FrequencyHz) -> Option<BitRateBps> {
    if spacing_hz == 0 {
        return None;
    }
    data_bits.checked_mul(u64::from(spacing_hz))
}

/// Sums nominal widths of data carriers, including disjoint frequency regions.
/// # Arguments
/// * `data_carriers` - Count of data assignments only.
/// * `spacing_hz` - Positive subcarrier spacing in Hz.
/// # Returns
/// Nominal data-carrier bandwidth in Hz, or None for zero spacing or u32 overflow.
/// This is not the span between the lowest and highest assigned carrier.
pub fn data_bandwidth(data_carriers: CarrierCount, spacing_hz: FrequencyHz) -> Option<FrequencyHz> {
    if spacing_hz == 0 {
        return None;
    }
    data_carriers
        .checked_mul(u64::from(spacing_hz))
        .and_then(|hz| FrequencyHz::try_from(hz).ok())
}
