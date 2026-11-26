/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Instantaneous power of existing complex FFT bins, without another transform or PSD scaling.
use crate::{error::invalid, SimulationError};

/// Squared complex magnitude is a power ratio, whose dB conversion is 10 log10.
const POWER_DECIBEL_FACTOR: f64 = 10.0;

/// Computes squared magnitude of an existing FFT bin without PSD scaling.
/// # Arguments
/// * `value` - Original complex I/Q; None denotes unavailable history.
/// # Returns
/// Optional linear/dB power; known zero has linear zero and no finite dB.
/// # Errors
/// Rejects nonfinite components or overflowing squared magnitude.
pub(crate) fn bin_power(
    value: Option<[f64; 2]>,
) -> Result<(Option<f64>, Option<f64>), SimulationError> {
    let Some([real, imaginary]) = value else {
        return Ok((None, None));
    };
    if !real.is_finite() || !imaginary.is_finite() {
        return Err(invalid("FFT-bin I/Q must be finite"));
    }
    let power = real * real + imaginary * imaginary;
    if !power.is_finite() {
        return Err(invalid("FFT-bin power overflows finite units"));
    }
    Ok((
        Some(power),
        (power > 0.0).then(|| POWER_DECIBEL_FACTOR * power.log10()),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_bin_power_preserves_known_zero_and_missing_history() {
        let (linear, decibels) = bin_power(Some([3.0, 4.0])).unwrap();
        assert_eq!(linear, Some(25.0));
        assert!((decibels.unwrap() - 13.979400086720377).abs() < 1e-12);
        assert_eq!(bin_power(Some([0.0, -0.0])).unwrap(), (Some(0.0), None));
        assert_eq!(bin_power(None).unwrap(), (None, None));
    }

    #[test]
    fn nonfinite_or_overflowing_bin_power_is_rejected() {
        for value in [[f64::NAN, 0.0], [0.0, f64::INFINITY], [f64::MAX, 1.0]] {
            assert!(bin_power(Some(value)).is_err());
        }
    }
}
