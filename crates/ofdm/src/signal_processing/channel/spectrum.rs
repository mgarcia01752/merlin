/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Electrical spectrum projection and scalar amplifier observations over existing numerical data.
use super::power::{channel_power, ChannelPower, PowerConditioning};
use crate::{
    error::invalid,
    signal_processing::waveform::spectrum::PowerSpectrum,
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        settings::SAMPLE_RATE_HZ,
    },
    types::IqPair,
    SimulationError,
};
use serde::Serialize;

/// Base of the conventional logarithmic decibel scale, independent of power versus amplitude.
const DECIBEL_BASE: f64 = 10.0;
/// Complex amplitude ratios use twenty times the base-ten logarithm.
const AMPLITUDE_DECIBEL_FACTOR: f64 = 20.0;

/// One bounded original-bin PSD bucket; its parent declares the logarithmic reference.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ChannelSpectrumBucket {
    pub first_bin: usize,
    pub last_bin: usize,
    pub minimum_db: Option<f64>,
    pub maximum_db: Option<f64>,
}

/// Existing PSD with explicit electrical units and full-band power independent of viewport.
#[derive(Serialize)]
#[non_exhaustive]
pub struct ChannelSpectrum {
    pub reference_plane: &'static str,
    pub density_units: &'static str,
    pub bucket_units: &'static str,
    /// None means relative units; no electrical reference is inferred from the samples.
    pub millivolts_squared_per_unit: Option<f64>,
    pub sample_rate_hz: u32,
    pub zero_frequency_hz: u32,
    pub spacing_hz: u32,
    pub fft_samples: usize,
    pub averages: usize,
    pub equivalent_noise_bandwidth_hz: f64,
    pub channel_power: ChannelPower,
    pub buckets: Vec<ChannelSpectrumBucket>,
}

/// Projects a full-resolution PSD without altering it or its electrical settings.
/// # Arguments
/// * `spectrum` - Existing normalized Welch output with its original complete FFT grid.
/// * `layout` - Accepted physical roles; first through last nonexcluded bins define the band.
/// * `power` - Validated electrical reference; gain/noise are not applied by this projection.
/// * `reference_plane` - Actual or explicitly scalar-derived scientific boundary.
/// * `start`, `count` - Original-bin viewport; zero count selects the remaining FFT span.
/// # Returns
/// Owned at-most-2048 extrema and zoom-independent full-band integrated power.
/// # Errors
/// Rejects malformed grids, absent bands, invalid/overflowing densities, calibration or viewport.
pub(crate) fn view(
    spectrum: &PowerSpectrum,
    layout: &CarrierLayout,
    power: &PowerConditioning,
    reference_plane: &'static str,
    start: usize,
    count: usize,
) -> Result<ChannelSpectrum, SimulationError> {
    if spectrum.fft_samples != layout.static_roles().len()
        || spectrum.spacing_hz != layout.subcarrier_spacing().frequency_hz()
        || Some(spectrum.zero_frequency_hz) != layout.frequency_hz(0)
        || spectrum
            .density
            .iter()
            .any(|value| !value.is_finite() || *value < 0.0)
    {
        return Err(invalid(
            "Channel spectrum has incompatible geometry or density",
        ));
    }
    let mut active = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, role)| **role != PhysicalCarrierRole::Excluded)
        .map(|(index, _)| index);
    let first = active
        .next()
        .ok_or_else(|| invalid("Channel has no occupied physical band"))?;
    let last = active.last().unwrap_or(first);
    let reading = channel_power(spectrum, first, last, power)?;
    let scale = power.millivolts_squared_per_unit();
    let mut calibrated = spectrum.clone();
    if let Some(scale) = scale {
        for density in &mut calibrated.density {
            *density *= scale;
            if !density.is_finite() {
                return Err(invalid("Calibrated spectrum density overflows"));
            }
        }
    }
    let count = if count == 0 {
        calibrated
            .fft_samples
            .checked_sub(start)
            .ok_or_else(|| invalid("Channel spectrum starts outside original FFT"))?
    } else {
        count
    };
    let buckets = calibrated
        .buckets(start, count)?
        .into_iter()
        .map(|bucket| ChannelSpectrumBucket {
            first_bin: bucket.first_bin,
            last_bin: bucket.last_bin,
            minimum_db: bucket.minimum_db,
            maximum_db: bucket.maximum_db,
        })
        .collect();
    Ok(ChannelSpectrum {
        reference_plane,
        density_units: if scale.is_some() {
            "millivolts_squared_per_hz"
        } else {
            "normalized_sample_units_squared_per_hz"
        },
        bucket_units: if scale.is_some() {
            "dbmv_per_hz"
        } else {
            "db_re_normalized_sample_unit_squared_per_hz"
        },
        millivolts_squared_per_unit: scale,
        sample_rate_hz: SAMPLE_RATE_HZ,
        zero_frequency_hz: spectrum.zero_frequency_hz,
        spacing_hz: spectrum.spacing_hz,
        fft_samples: spectrum.fft_samples,
        averages: spectrum.averages,
        equivalent_noise_bandwidth_hz: spectrum.equivalent_noise_bandwidth_hz,
        channel_power: reading,
        buckets,
    })
}

/// Resolves the amplifier alone, before CPA, physical response and receiver noise.
/// # Arguments
/// * `power` - Accepted gain or alternative target/reference settings.
/// # Returns
/// Finite complex-amplitude ratio; unrelated channel effects remain excluded.
/// # Errors
/// Preserves original electrical-reference and finite model admission.
pub(crate) fn amplifier_amplitude(power: &PowerConditioning) -> Result<f64, SimulationError> {
    power.validate()?;
    Ok(DECIBEL_BASE.powf(power.gain_db() / AMPLITUDE_DECIBEL_FACTOR))
}

/// Applies an explicit scalar to one copied complex observation without changing its source.
/// # Arguments
/// * `iq` - Original finite complex sample or bin.
/// * `amplitude` - Finite nonnegative complex-amplitude ratio from the validated power owner.
/// # Returns
/// Exact scalar-derived I/Q, retaining zeros and component ordering.
/// # Errors
/// Rejects nonfinite input, negative/nonfinite scale or overflowing output.
pub(crate) fn scale_iq(iq: IqPair, amplitude: f64) -> Result<IqPair, SimulationError> {
    if !amplitude.is_finite() || amplitude < 0.0 || iq.iter().any(|value| !value.is_finite()) {
        return Err(invalid("Invalid scalar amplifier observation"));
    }
    let scaled = [iq[0] * amplitude, iq[1] * amplitude];
    if scaled.iter().any(|value| !value.is_finite()) {
        return Err(invalid("Scalar amplifier observation overflows"));
    }
    Ok(scaled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transmission::{
        layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings,
    };

    const FLAT_NORMALIZED_DENSITY: f64 = 1.0 / 204_800_000.0;
    const EXPECTED_BAND_POWER: f64 = 0.927734375;
    const EXPECTED_BANDWIDTH_HZ: f64 = 190_000_000.0;
    const EXPECTED_CALIBRATED_DBMV_HZ: f64 = -67.78151250383644;
    const ELECTRICAL_TOLERANCE: f64 = 1e-10;
    const AMPLIFIER_GAIN_DB: f64 = 20.0;
    const CPA_ATTENUATION_DB: f64 = 6.0;
    const EXPECTED_AMPLITUDE: f64 = 10.0;
    const DEFAULT_INPUT_LEVEL_DBMV: f64 = 0.0;

    fn fixture(spacing: SubcarrierSpacing) -> (PowerSpectrum, CarrierLayout) {
        // Independent physical test geometry contains one internal exclusion in both modes.
        let (fft_samples, spacing_hz, divisor) = match spacing {
            SubcarrierSpacing::Khz25 => (8192, 25_000, 1),
            SubcarrierSpacing::Khz50 => (4096, 50_000, 2),
        };
        let settings = TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        };
        let layout = CarrierLayout::new(
            &settings,
            &CarrierLayoutSettings {
                zero_frequency_hz: 100_000_000,
                first_active_bin: 296 / divisor,
                active_end_exclusive: 7896 / divisor,
                plc_first_bin: 4112 / divisor,
                excluded_bins: vec![6000 / divisor],
                additional_continuous_pilots: [480, 600, 720, 840, 960, 1080, 1200, 1320]
                    .into_iter()
                    .map(|bin| bin / divisor)
                    .collect(),
            },
        )
        .unwrap();
        (
            PowerSpectrum {
                zero_frequency_hz: 100_000_000,
                spacing_hz,
                fft_samples,
                averages: 1,
                equivalent_noise_bandwidth_hz: 1.5 * f64::from(spacing_hz),
                density: vec![FLAT_NORMALIZED_DENSITY; fft_samples],
            },
            layout,
        )
    }

    #[test]
    fn independent_flat_density_calibration_and_full_band_integration_ignore_zoom_and_internal_holes(
    ) {
        let power = PowerConditioning {
            input_data_level_dbmv_per_6mhz: Some(DEFAULT_INPUT_LEVEL_DBMV),
            ..Default::default()
        };
        for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
            let (spectrum, layout) = fixture(spacing);
            let full = view(&spectrum, &layout, &power, "test_input", 0, 0).unwrap();
            let zoom = view(&spectrum, &layout, &power, "test_input", 300, 7).unwrap();
            assert_eq!(full.density_units, "millivolts_squared_per_hz");
            assert_eq!(full.bucket_units, "dbmv_per_hz");
            assert!(
                (full.channel_power.normalized_power - EXPECTED_BAND_POWER).abs()
                    < ELECTRICAL_TOLERANCE
            );
            assert_eq!(full.channel_power.bandwidth_hz, EXPECTED_BANDWIDTH_HZ);
            assert!(
                full.channel_power.equivalent_dbmv_per_6mhz.unwrap().abs() < ELECTRICAL_TOLERANCE
            );
            assert_eq!(
                full.channel_power.normalized_power,
                zoom.channel_power.normalized_power
            );
            assert_eq!(full.channel_power.dbmv, zoom.channel_power.dbmv);
            for bucket in &zoom.buckets {
                assert!(
                    (bucket.minimum_db.unwrap() - EXPECTED_CALIBRATED_DBMV_HZ).abs()
                        < ELECTRICAL_TOLERANCE
                );
                assert_eq!(bucket.minimum_db, bucket.maximum_db);
            }
            assert!(spectrum
                .density
                .iter()
                .all(|value| *value == FLAT_NORMALIZED_DENSITY));
        }
    }

    #[test]
    fn relative_zero_and_invalid_spectra_never_invent_electrical_levels_or_finite_missing_power() {
        let (mut spectrum, layout) = fixture(SubcarrierSpacing::Khz25);
        spectrum.density.fill(0.0);
        let power = PowerConditioning::default();
        let zero = view(&spectrum, &layout, &power, "zero_input", 0, 0).unwrap();
        assert!(zero.millivolts_squared_per_unit.is_none());
        assert!(zero.channel_power.dbmv.is_none());
        assert_eq!(zero.channel_power.normalized_power, 0.0);
        assert!(zero
            .buckets
            .iter()
            .all(|bucket| bucket.minimum_db.is_none() && bucket.maximum_db.is_none()));
        assert!(view(
            &spectrum,
            &layout,
            &power,
            "bad_window",
            spectrum.fft_samples,
            0
        )
        .is_err());
        assert!(view(&spectrum, &layout, &power, "bad_window", 0, usize::MAX).is_err());
        spectrum.density[0] = f64::NAN;
        assert!(view(&spectrum, &layout, &power, "bad_density", 300, 7).is_err());
        spectrum.density[0] = 0.0;
        spectrum.zero_frequency_hz += 1;
        assert!(view(&spectrum, &layout, &power, "bad_frequency", 300, 7).is_err());
        spectrum.zero_frequency_hz -= 1;
        spectrum.density[0] = -1.0;
        assert!(view(&spectrum, &layout, &power, "negative_density", 300, 7).is_err());
        spectrum.density[0] = f64::MAX;
        let calibrated = PowerConditioning {
            input_data_level_dbmv_per_6mhz: Some(DEFAULT_INPUT_LEVEL_DBMV),
            ..Default::default()
        };
        assert!(view(&spectrum, &layout, &calibrated, "overflow_density", 300, 7).is_err());
    }

    #[test]
    fn amplifier_alone_has_literal_complex_gain_and_keeps_cpa_target_and_overflow_semantics_explicit(
    ) {
        let mut power = PowerConditioning {
            amplifier_gain_db: AMPLIFIER_GAIN_DB,
            cpa_attenuation_db: CPA_ATTENUATION_DB,
            input_data_level_dbmv_per_6mhz: Some(DEFAULT_INPUT_LEVEL_DBMV),
            ..Default::default()
        };
        assert_eq!(amplifier_amplitude(&power).unwrap(), EXPECTED_AMPLITUDE);
        assert_eq!(
            scale_iq([3.0, -4.0], EXPECTED_AMPLITUDE).unwrap(),
            [30.0, -40.0]
        );
        assert_eq!(
            scale_iq([0.0, -0.0], EXPECTED_AMPLITUDE).unwrap(),
            [0.0, -0.0]
        );
        power.amplifier_target_dbmv_per_6mhz = Some(AMPLIFIER_GAIN_DB);
        assert_eq!(amplifier_amplitude(&power).unwrap(), EXPECTED_AMPLITUDE);
        for pair in [[f64::NAN, 0.0], [f64::INFINITY, 0.0], [f64::MAX, 0.0]] {
            assert!(scale_iq(pair, EXPECTED_AMPLITUDE).is_err());
        }
        for amplitude in [-1.0, f64::NAN, f64::INFINITY] {
            assert!(scale_iq([1.0, 0.0], amplitude).is_err());
        }
        power.input_data_level_dbmv_per_6mhz = None;
        assert!(amplifier_amplitude(&power).is_err());
    }
}
