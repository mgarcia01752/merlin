/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Electrical references and scalar channel conditioning; no device capability inference.
use crate::{error::invalid, transmission::settings::SAMPLE_RATE_HZ, SimulationError};
use serde::{Deserialize, Serialize};

/// Explicit conditioning, independent of captured relative channel response.
/// One normalized data FFT cell has unit mean squared complex magnitude. The
/// optional input level calibrates that data density over6 MHz, including the
/// unitary FFT/sample-clock conversion. Complex envelope power is rms mV squared;
/// there is no additional real-carrier factor of two. Pilot boosts remain actual.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PowerConditioning {
    pub amplifier_gain_db: f64,
    pub attenuation_db: f64,
    /// Absent preserves legacy relative units; not a measured electrical level.
    pub input_data_level_dbmv_per_6mhz: Option<f64>,
    /// Alternative to gain: target nominal DATA density after the amplifier.
    pub amplifier_target_dbmv_per_6mhz: Option<f64>,
    /// Independent white receiver-input noise, after the variable attenuator.
    /// Explicitly additional to the measured post-EQ RxMER residual, not its replacement.
    pub receiver_noise_dbmv_hz: Option<f64>,
}
impl PowerConditioning {
    /// Validates finite numerical model limits and required electrical references.
    /// # Returns
    /// Unit for supported scalar settings; no CMTS port capability is inferred.
    /// # Errors
    /// Rejects invalid ranges, target gain outside0..60 dB or noise without calibration.
    pub fn validate(&self) -> Result<(), SimulationError> {
        let valid_level = |value: f64| value.is_finite() && (-120.0..=60.0).contains(&value);
        if !self.amplifier_gain_db.is_finite()
            || !(0.0..=60.0).contains(&self.amplifier_gain_db)
            || !self.attenuation_db.is_finite()
            || !(0.0..=60.0).contains(&self.attenuation_db)
            || self
                .input_data_level_dbmv_per_6mhz
                .is_some_and(|v| !valid_level(v))
            || self
                .amplifier_target_dbmv_per_6mhz
                .is_some_and(|v| !valid_level(v))
            || self
                .receiver_noise_dbmv_hz
                .is_some_and(|v| !v.is_finite() || !(-200.0..=-30.0).contains(&v))
        {
            return Err(invalid("Power conditioning exceeds finite model limits."));
        }
        if (self.amplifier_target_dbmv_per_6mhz.is_some() || self.receiver_noise_dbmv_hz.is_some())
            && self.input_data_level_dbmv_per_6mhz.is_none()
        {
            return Err(invalid(
                "Target level and receiver noise require an explicit TX data reference.",
            ));
        }
        if self.receiver_noise_power() > super::MAX_NORMALIZED_NOISE_POWER {
            return Err(invalid(
                "Receiver noise exceeds normalized simulation limits.",
            ));
        }
        let gain = self.gain_db();
        if !(0.0..=60.0).contains(&gain) {
            return Err(invalid("Target requires amplifier gain outside0..60 dB."));
        }
        Ok(())
    }
    /// Resolves the accepted gain or alternative nominal-data level target.
    /// # Returns
    /// Gain in dB; call validate before using external settings.
    pub fn gain_db(&self) -> f64 {
        self.amplifier_target_dbmv_per_6mhz
            .zip(self.input_data_level_dbmv_per_6mhz)
            .map_or(self.amplifier_gain_db, |(target, input)| target - input)
    }
    /// Returns a scalar complex-amplitude multiplier for a validated net path.
    /// # Returns
    /// Amplifier followed by receiver attenuation, without imported-H normalization.
    pub fn net_amplitude(&self) -> f64 {
        10.0_f64.powf((self.gain_db() - self.attenuation_db) / 20.0)
    }
    /// Converts normalized sample power to rms millivolt squared power.
    /// # Returns
    /// Calibration factor or None when the workbench explicitly uses relative units.
    pub fn millivolts_squared_per_unit(&self) -> Option<f64> {
        self.input_data_level_dbmv_per_6mhz
            .map(|level| 10.0_f64.powf(level / 10.0) * f64::from(SAMPLE_RATE_HZ) / 6_000_000.0)
    }
    /// Converts the optional independent white receiver density into normalized variance.
    /// # Returns
    /// Complex time-sample and unitary FFT-bin noise power after attenuation.
    pub fn receiver_noise_power(&self) -> f64 {
        self.receiver_noise_dbmv_hz
            .zip(self.millivolts_squared_per_unit())
            .map_or(0.0, |(density, scale)| {
                10.0_f64.powf(density / 10.0) * f64::from(SAMPLE_RATE_HZ) / scale
            })
    }
}

/// Integrated linear PSD over a fixed inclusive physical FFT-bin band.
#[derive(Debug, Serialize)]
pub struct ChannelPower {
    pub normalized_power: f64,
    pub dbmv: Option<f64>,
    pub equivalent_dbmv_per_6mhz: Option<f64>,
    pub bandwidth_hz: f64,
    pub first_bin: usize,
    pub last_bin: usize,
}
/// Integrates full-resolution linear PSD independently of display reduction/zoom.
/// # Arguments
/// * `spectrum` - Actual window-normalized complex-sample PSD.
/// * `first` - First included physical bin, not a viewport selection.
/// * `last` - Last included physical bin.
/// * `conditioning` - Explicit electrical calibration or relative defaults.
/// # Returns
/// Total power with optional rms dBmV and band-averaged6 MHz equivalent level.
/// # Errors
/// Rejects malformed PSDs, ranges, calibration and nonfinite integration.
pub fn channel_power(
    spectrum: &super::super::waveform::spectrum::PowerSpectrum,
    first: usize,
    last: usize,
    conditioning: &PowerConditioning,
) -> Result<ChannelPower, SimulationError> {
    conditioning.validate()?;
    if ![4096, 8192].contains(&spectrum.fft_samples)
        || spectrum.density.len() != spectrum.fft_samples
        || spectrum
            .fft_samples
            .checked_mul(spectrum.spacing_hz as usize)
            != Some(SAMPLE_RATE_HZ as usize)
        || first > last
        || last >= spectrum.density.len()
        || spectrum.spacing_hz == 0
    {
        return Err(invalid("Invalid channel integration band."));
    }
    let densities = &spectrum.density[first..=last];
    if densities.iter().any(|p| !p.is_finite() || *p < 0.0) {
        return Err(invalid("Invalid channel power density."));
    }
    let power = densities.iter().sum::<f64>() * f64::from(spectrum.spacing_hz);
    if !power.is_finite() {
        return Err(invalid("Channel power integration overflow."));
    }
    let bandwidth_hz = (last - first + 1) as f64 * f64::from(spectrum.spacing_hz);
    let dbmv = conditioning
        .millivolts_squared_per_unit()
        .filter(|_| power > 0.0)
        .map(|scale| 10.0 * power.log10() + 10.0 * scale.log10());
    Ok(ChannelPower {
        normalized_power: power,
        dbmv,
        equivalent_dbmv_per_6mhz: dbmv
            .map(|level| level + 10.0 * (6_000_000.0 / bandwidth_hz).log10()),
        bandwidth_hz,
        first_bin: first,
        last_bin: last,
    })
}
