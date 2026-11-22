/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Configured deterministic transfers and explicitly periodic impulse diagnostics.
use super::transfer::{amplitude_db, frequency_hz, phase};
use crate::{
    configuration::SAMPLE_RATE_HZ,
    error::{invalid, limit},
    signal_processing::Transform,
    simulation::{BatchReport, ExperimentConfig},
    types::Iq,
    PhyError, PhyResult,
};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, f64::consts::TAU};

/// Frequency-display application budget, independent of impulse support or normative FFT size.
pub const MAX_CONFIGURED_RESPONSE_POINTS: usize = 4096;
/// Complete periodic-impulse application budget, independent of the frequency viewport.
pub const MAX_CONFIGURED_IMPULSE_SAMPLES: usize = 4096;

/// Admitted deterministic diagnostic source; none denotes a received-pilot estimate.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ConfiguredResponseSource {
    /// Actual admitted corrective C selected at the original base-grant symbol.
    CmPreEq,
    /// Original configured causal FIR, delay and constant phase, excluding noise.
    #[default]
    PhysicalChannel,
    /// Configured C times deterministic H, without receiver-estimate semantics.
    Composite,
}

/// Original symbol/frequency viewport and an explicit complete-period impulse opt-in.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConfiguredResponseRequest {
    /// C, deterministic H or their composite; received estimates are a separate operation.
    pub source: ConfiguredResponseSource,
    /// Original zero-based symbol across the configured base grant's frames.
    pub grant_symbol: usize,
    /// First original physical FFT carrier, without compaction.
    pub first_carrier: usize,
    /// Positive span, or zero for the remaining original FFT extent.
    pub carrier_count: usize,
    /// Positive frequency-point budget, at most MAX_CONFIGURED_RESPONSE_POINTS.
    pub maximum_points: usize,
    /// Explicitly request all periodic impulse samples, independently of frequency zoom.
    pub include_impulse: bool,
}
impl Default for ConfiguredResponseRequest {
    /// Selects deterministic H at base-grant symbol zero over the original full FFT.
    /// # Returns
    /// A finite read-only query with impulse generation disabled; no simulation starts.
    fn default() -> Self {
        Self {
            source: ConfiguredResponseSource::PhysicalChannel,
            grant_symbol: 0,
            first_carrier: 0,
            carrier_count: 0,
            maximum_points: MAX_CONFIGURED_RESPONSE_POINTS,
            include_impulse: false,
        }
    }
}

/// One original physical-frequency transfer point, with explicit zero logarithmic readings.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConfiguredResponsePoint {
    /// Original centered FFT index in increasing-frequency order.
    pub physical_carrier: usize,
    /// Centered complex-baseband frequency in hertz, without an inferred RF origin.
    pub frequency_hz: f64,
    /// Original signal-carrier exclusion; physical H is still defined there.
    pub excluded: bool,
    /// Admitted deterministic dimensionless complex sample-amplitude transfer.
    pub value: Iq,
    /// Twenty log10 of magnitude, absent for an exact zero.
    pub amplitude_db: Option<f64>,
    /// Principal phase in radians, absent for an exact zero.
    pub phase_radians: Option<f64>,
}

/// One original sample in the complete periodic impulse diagnostic.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PeriodicImpulsePoint {
    /// Original zero-based index inside one N-sample period; delays can wrap.
    pub sample_index: usize,
    /// Original periodic index divided by the actual upstream sample clock, in seconds.
    pub time_seconds: f64,
    /// Inverse unitary transform divided by sqrt(N), without an arbitrary floor.
    pub value: Iq,
    /// Magnitude relative to the complete-period peak in dB, absent for an exact zero.
    pub relative_power_db: Option<f64>,
}

/// Complete periodic response, independent of the selected frequency viewport.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct PeriodicImpulse {
    /// Complete N-sample period; this is not unaliased causal echo support.
    pub period_samples: usize,
    /// Actual complex-baseband sample clock in samples per second.
    pub sample_rate_hz: u32,
    /// Explicit inverse-transform normalization, separate from waveform scaling.
    pub normalization: &'static str,
    /// Maximum finite magnitude over the complete period; zero if the entire response is zero.
    pub peak_amplitude: f64,
    /// All original periodic samples, never recomputed from a cropped frequency viewport.
    pub points: Vec<PeriodicImpulsePoint>,
}

/// Owned configured diagnostic, explicitly separate from actual received observations.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ConfiguredResponseSnapshot {
    /// Configured C, deterministic H or their composite.
    pub source: ConfiguredResponseSource,
    /// Explicit configured diagnostic plane, never a claimed received estimate.
    pub reference_plane: &'static str,
    /// Dimensionless complex sample-amplitude transfer, without RF calibration.
    pub units: &'static str,
    /// Accepted session generation when retained; absent in a report with no started grant.
    pub configuration_generation: Option<u64>,
    /// Original report run identity when retained; idle and never-started reports have none.
    pub run_id: Option<u64>,
    /// Original configured base-grant identity, not a later batch grant identity.
    pub grant_id: u64,
    /// Original selected profile identity, independent of viewport position.
    pub profile_id: u16,
    /// Original selected IUC, without deriving QAM order from it.
    pub iuc: u8,
    /// Original selected base-grant symbol, before any source or waveform execution.
    pub grant_symbol: usize,
    /// Actual selected corrective bank generation; absent for a pure physical-H query.
    pub pre_eq_generation: Option<u64>,
    /// Complete upstream centered FFT size, independent of display reduction.
    pub fft_samples: usize,
    /// Actual upstream sample clock in samples per second.
    pub sample_rate_hz: u32,
    /// Original physical FFT carrier spacing in hertz.
    pub spacing_hz: u32,
    /// First original physical carrier in this viewport.
    pub first_carrier: usize,
    /// Resolved original frequency-point count.
    pub carrier_count: usize,
    /// Original configured physical delay, which can exceed one periodic diagnostic period.
    pub physical_delay_samples: usize,
    /// Original configured constant emitted-clock phase in radians.
    pub physical_phase_radians: f64,
    /// Original configured frequency offset; nonzero rejects static H/composite queries.
    pub physical_frequency_offset_hz: f64,
    /// Original sample-noise variance, explicitly excluded from the deterministic transfer.
    pub configured_sample_noise_variance: f64,
    /// Whether separate admitted RxMER residual impairment is configured, also excluded here.
    pub rxmer_residual_configured: bool,
    /// Original physical-frequency observations of the configured deterministic transfer.
    pub points: Vec<ConfiguredResponsePoint>,
    /// Complete periodic inverse response when explicitly requested, otherwise absent.
    pub impulse: Option<PeriodicImpulse>,
}

impl BatchReport {
    /// Queries the frozen original configuration's deterministic C, H or composite response.
    /// # Arguments
    /// * `request` - Original base-grant symbol/frequency window, finite budget and impulse opt-in.
    /// # Returns
    /// Owned configured diagnostic; no source, waveform, receiver, channel stream or RNG runs.
    /// Optional run/generation metadata stays absent if this report retained no started grant.
    /// # Errors
    /// Shared extent, budget, configuration and finite-arithmetic failures propagate.
    /// Static H/composite rejects a nonzero configured frequency offset explicitly.
    pub fn configured_response(
        &self,
        request: ConfiguredResponseRequest,
    ) -> PhyResult<ConfiguredResponseSnapshot> {
        let mut response = configured(&self.configuration, request)?;
        if let Some(grant) = self.grants.first() {
            grant.identity.validate()?;
            response.configuration_generation = Some(grant.identity.generation);
            response.run_id = Some(grant.identity.run);
        }
        Ok(response)
    }
}

/// Derives a bounded configured transfer using existing C replay and shared unitary transforms.
/// # Arguments
/// * `configuration` - Borrowed admitted scientific settings; public report copies are revalidated.
/// * `request` - Original base-grant symbol and carrier coordinates with finite application limits.
/// # Returns
/// Owned deterministic response and optional complete periodic inverse, without execution identities.
/// No random draws or receiver/source/channel-stream operations occur.
/// # Errors
/// Rejects invalid original windows, exceeded budgets, time-varying H and nonfinite arithmetic.
pub(crate) fn configured(
    configuration: &ExperimentConfig,
    request: ConfiguredResponseRequest,
) -> PhyResult<ConfiguredResponseSnapshot> {
    let us = &configuration.us_configuration;
    let mode = us.waveform().numerology();
    let length = mode.fft_len();
    if request.maximum_points == 0 || request.maximum_points > MAX_CONFIGURED_RESPONSE_POINTS {
        return Err(limit("configured_response.maximum_points"));
    }
    let total_symbols = us
        .grant()
        .frames
        .checked_mul(us.waveform().frame_symbols())
        .ok_or_else(|| limit("configured_response.symbols"))?;
    if request.grant_symbol >= total_symbols || request.first_carrier >= length {
        return Err(invalid("configured_response.start"));
    }
    let remaining = length - request.first_carrier;
    let count = if request.carrier_count == 0 {
        // Zero selects the original remaining physical extent, not an empty transfer.
        remaining
    } else {
        request.carrier_count
    };
    if count > remaining {
        return Err(invalid("configured_response.extent"));
    }
    if count > request.maximum_points
        || (request.include_impulse && length > MAX_CONFIGURED_IMPULSE_SAMPLES)
    {
        return Err(limit("configured_response.points"));
    }
    configuration.validate()?;
    let needs_h = request.source != ConfiguredResponseSource::CmPreEq;
    if needs_h && configuration.channel.frequency_offset_hz != 0.0 {
        return Err(PhyError::Unsupported {
            field: "configured_response.time_varying_channel",
        });
    }
    let bank = if request.source == ConfiguredResponseSource::PhysicalChannel {
        None
    } else {
        Some(match &configuration.pre_eq_updates {
            Some(playback) => playback.at(request.grant_symbol)?,
            None => &configuration.pre_eq,
        })
    };
    let transform = Transform::new(mode);
    let transfer = if needs_h {
        let mut impulse = vec![Iq::default(); length];
        impulse[..configuration.channel.taps.len()].copy_from_slice(&configuration.channel.taps);
        let scale = (length as f64).sqrt();
        let initial_phase = configuration.channel.phase_radians.rem_euclid(TAU);
        let mut response = transform.forward(&impulse)?;
        for (carrier, value) in response.iter_mut().enumerate() {
            let frequency = frequency_hz(carrier, length, mode.spacing_hz());
            let phase = initial_phase
                - TAU * frequency * configuration.channel.delay_samples as f64
                    / f64::from(SAMPLE_RATE_HZ);
            let (imaginary, real) = phase.sin_cos();
            *value = *value
                * Iq {
                    re: real * scale,
                    im: imaginary * scale,
                };
            if let Some(bank) = bank {
                *value = *value * bank.coefficients()[carrier];
            }
        }
        response
    } else {
        bank.ok_or_else(|| invalid("configured_response.c_bank"))?
            .coefficients()
            .to_vec()
    };
    if transfer.iter().any(|value| !value.is_finite()) {
        return Err(invalid("configured_response.transfer"));
    }
    let impulse = if request.include_impulse {
        Some(periodic_impulse(&transform, &transfer)?)
    } else {
        None
    };
    let exclusions: BTreeSet<_> = us.excluded().iter().copied().collect();
    let points = transfer
        .iter()
        .enumerate()
        .skip(request.first_carrier)
        .take(count)
        .map(|(carrier, &value)| {
            Ok(ConfiguredResponsePoint {
                physical_carrier: carrier,
                frequency_hz: frequency_hz(carrier, length, mode.spacing_hz()),
                excluded: exclusions.contains(&carrier),
                value,
                amplitude_db: amplitude_db(value, "configured_response.magnitude")?,
                phase_radians: phase(value),
            })
        })
        .collect::<PhyResult<Vec<_>>>()?;
    let reference_plane = match request.source {
        ConfiguredResponseSource::CmPreEq => "configured_cm_transmit_c",
        ConfiguredResponseSource::PhysicalChannel => "configured_deterministic_physical_h",
        ConfiguredResponseSource::Composite => "configured_deterministic_c_times_h",
    };
    let profile = us.selected_profile();
    Ok(ConfiguredResponseSnapshot {
        source: request.source,
        reference_plane,
        units: "dimensionless-complex-sample-amplitude-transfer",
        configuration_generation: None,
        run_id: None,
        grant_id: us.grant().id,
        profile_id: profile.id(),
        iuc: profile.iuc(),
        grant_symbol: request.grant_symbol,
        pre_eq_generation: bank.map(|bank| bank.generation()),
        fft_samples: length,
        sample_rate_hz: SAMPLE_RATE_HZ,
        spacing_hz: mode.spacing_hz(),
        first_carrier: request.first_carrier,
        carrier_count: count,
        physical_delay_samples: configuration.channel.delay_samples,
        physical_phase_radians: configuration.channel.phase_radians,
        physical_frequency_offset_hz: configuration.channel.frequency_offset_hz,
        configured_sample_noise_variance: configuration.channel.noise_variance,
        rxmer_residual_configured: configuration.rxmer.is_some(),
        points,
        impulse,
    })
}

fn periodic_impulse(transform: &Transform, transfer: &[Iq]) -> PhyResult<PeriodicImpulse> {
    let scale = (transfer.len() as f64).sqrt().recip();
    let samples: Vec<_> = transform
        .inverse(transfer)?
        .into_iter()
        .map(|value| value * scale)
        .collect();
    let mut peak_amplitude = 0.0_f64;
    for value in &samples {
        let magnitude = value.re.hypot(value.im);
        if !value.is_finite() || !magnitude.is_finite() {
            return Err(invalid("configured_response.impulse"));
        }
        peak_amplitude = peak_amplitude.max(magnitude);
    }
    let peak_db = amplitude_db(
        Iq {
            re: peak_amplitude,
            im: 0.0,
        },
        "configured_response.magnitude",
    )?;
    let points = samples
        .into_iter()
        .enumerate()
        .map(|(sample_index, value)| {
            let relative_power_db = amplitude_db(value, "configured_response.magnitude")?
                .zip(peak_db)
                .map(|(value_db, peak_db)| value_db - peak_db);
            Ok(PeriodicImpulsePoint {
                sample_index,
                time_seconds: sample_index as f64 / f64::from(SAMPLE_RATE_HZ),
                value,
                relative_power_db,
            })
        })
        .collect::<PhyResult<Vec<_>>>()?;
    Ok(PeriodicImpulse {
        period_samples: transfer.len(),
        sample_rate_hz: SAMPLE_RATE_HZ,
        normalization: "ifft_unitary_div_sqrt_n",
        peak_amplitude,
        points,
    })
}
