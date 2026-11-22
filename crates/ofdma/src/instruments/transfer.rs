/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Identical complex-transfer display math shared by configured and received observations.
use crate::{error::invalid, types::Iq, PhyResult};

/// Dimensionless complex-amplitude ratios use twenty times the base-ten magnitude logarithm.
const AMPLITUDE_DECIBEL_FACTOR: f64 = 20.0;

/// Projects an original centered physical FFT `carrier` into complex-baseband hertz.
/// # Arguments
/// * `carrier` - Original physical index in increasing centered-FFT frequency order.
/// * `length`, `spacing_hz` - Original even FFT dimension and physical carrier spacing.
/// # Returns
/// Baseband frequency without guessing any absolute RF origin or compacting excluded bins.
pub(super) fn frequency_hz(carrier: usize, length: usize, spacing_hz: u32) -> f64 {
    (carrier as f64 - length as f64 / 2.0) * f64::from(spacing_hz)
}

/// Projects one finite complex transfer to amplitude dB without an arbitrary floor.
/// # Arguments
/// * `value` - Original configured or actually estimated dimensionless amplitude transfer.
/// * `field` - Caller-owned stable error context, preserving configured/received distinctions.
/// # Returns
/// Twenty-log magnitude, or None for an exact zero; no source/receiver state changes.
/// # Errors
/// Rejects nonrepresentable magnitude through the caller's existing numerical error context.
pub(super) fn amplitude_db(value: Iq, field: &'static str) -> PhyResult<Option<f64>> {
    let magnitude = value.re.hypot(value.im);
    if !magnitude.is_finite() {
        return Err(invalid(field));
    }
    if magnitude == 0.0 {
        return Ok(None);
    }
    Ok(Some(AMPLITUDE_DECIBEL_FACTOR * magnitude.log10()))
}

/// Projects finite complex `value` to its principal phase without a zero-phase fiction.
/// # Returns
/// Radians from atan2, or None for an exact zero; missing estimates are handled by their owner.
pub(super) fn phase(value: Iq) -> Option<f64> {
    if value.re == 0.0 && value.im == 0.0 {
        // An exact zero has no defined transfer phase.
        None
    } else {
        Some(value.im.atan2(value.re))
    }
}
