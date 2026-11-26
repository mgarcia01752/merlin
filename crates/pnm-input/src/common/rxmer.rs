/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Quarter-dB semantics shared by CM-OSSI I27 and CCAP-OSSI I18 RxMER output.
use crate::rxmer_export::RangePolicy;

/// Smallest reportable RxMER, in dB; this is a wire range, not a simulation limit.
pub(crate) const MINIMUM_MER_DB: f64 = 0.0;
/// Largest finite reading; the next octet is reserved for unavailable data.
pub(crate) const MAXIMUM_MER_DB: f64 = 63.5;
/// Four quarter-dB wire steps per dB.
pub(crate) const STEPS_PER_DB: f64 = 4.0;
/// Both OSSI formats reserve this octet for an unavailable reading.
pub(crate) const MISSING_VALUE: u8 = 0xff;

pub(crate) enum ReadingFailure {
    Nonfinite,
    OutOfRange,
}

pub(crate) struct QuantizedReading {
    pub(crate) byte: u8,
    pub(crate) bounded_db: f64,
    pub(crate) decoded_db: f64,
}

/// Converts one finite reading without changing the caller's value.
///
/// # Arguments
/// * `value` - Requested RxMER, in dB; unavailable readings are handled separately.
/// * `policy` - Reject finite range violations or explicitly clamp them.
///
/// # Returns
/// Encoded octet and pre/post-rounding values, with positive half-ties up.
///
/// # Errors
/// Rejects nonfinite values and out-of-range values under the Reject policy.
pub(crate) fn quantize(
    value: f64,
    policy: RangePolicy,
) -> Result<QuantizedReading, ReadingFailure> {
    if !value.is_finite() {
        return Err(ReadingFailure::Nonfinite);
    }
    let bounded_db = value.clamp(MINIMUM_MER_DB, MAXIMUM_MER_DB);
    if value != bounded_db && matches!(policy, RangePolicy::Reject) {
        return Err(ReadingFailure::OutOfRange);
    }
    let byte = (bounded_db * STEPS_PER_DB).round() as u8;
    Ok(QuantizedReading {
        byte,
        bounded_db,
        decoded_db: f64::from(byte) / STEPS_PER_DB,
    })
}
