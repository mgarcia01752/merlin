/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ideal AWGN Shannon capacity bounds; these are not measured DOCSIS MER limits.
use crate::pnm::data_type::BitsPerSymbol;

/// Ideal minimum signal-to-noise ratio in decibels.
pub type ShannonSnrDb = f64;

/// Computes the ideal SNR bound for a spectral efficiency in bits per symbol.
/// # Arguments
/// * `bits_per_symbol` - Positive modulation bit count; zero is not a data signal.
/// # Returns
/// `10 log10(2^bits - 1)` in dB, or None for zero (no finite logarithm).
/// Every positive u8 input yields a finite result. This ignores implementation
/// margin, coding overhead and real receiver requirements.
pub fn minimum_snr_db(bits_per_symbol: BitsPerSymbol) -> Option<ShannonSnrDb> {
    if bits_per_symbol == 0 {
        return None;
    }
    Some(10.0 * (2.0_f64.powi(i32::from(bits_per_symbol)) - 1.0).log10())
}
