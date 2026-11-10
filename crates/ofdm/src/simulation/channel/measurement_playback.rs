/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit residual-power combination without changing original capture epochs.
use crate::{
    error::invalid,
    profile::candidate::CapturedProfile,
    scenario::{ChannelFrame, RxMerFrame},
    signal_processing::channel::{
        impairments::{ChannelImpairments, NoiseModel},
        MAX_NORMALIZED_NOISE_POWER,
    },
    simulation::channel::measurement::captured_layout,
    transmission::{layout::CarrierLayout, TransmissionSettings},
    SimulationError,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Application bound on selected captures, independent of protocol limits.
pub const MAX_PLAYBACK_FRAMES: usize = 64;
/// Shared temporal observation safety limit.
pub const MAX_PLAYBACK_VALUES: usize = 4_000_000;

/// How compatible per-carrier residual observations condition the simulated channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackMode {
    Single,
    Average,
    Hold,
    Interpolate,
}

/// One immutable query result, with original source indexes and interpolation weight.
pub struct ResidualSnapshot {
    pub power: Vec<Option<f64>>,
    pub first_source: usize,
    pub second_source: usize,
    pub second_weight: f64,
}

/// Validated frozen observations; no RNG, filesystem or UI dependency.
#[derive(Clone)]
pub struct MeasurementPlayback {
    frames: Arc<Vec<RxMerFrame>>,
    powers: Arc<Vec<Vec<Option<f64>>>>,
    average: Arc<Vec<Option<f64>>>,
    mode: PlaybackMode,
    interval_symbols: u64,
    settings: TransmissionSettings,
}

/// Validates unity or imported H against the accepted DPD/OCD carrier layout.
/// # Arguments
/// * `settings` - Frozen transmission geometry.
/// * `profile` - Accepted captured layout/identity.
/// * `response` - Supplied H, or None for explicitly synthetic unity.
/// # Returns
/// Unit for an admissible response; does not mutate inputs or infer timestamps.
/// # Errors
/// Rejects inconsistent identity/grid/coverage, missing/zero active gains or geometry.
pub fn validate_response(
    settings: &TransmissionSettings,
    profile: &CapturedProfile,
    response: Option<&ChannelFrame>,
) -> Result<(), SimulationError> {
    if response.is_some_and(|frame| frame.key != profile.key) {
        return Err(invalid(
            "Channel response device/channel/frequency grid differs from DPD.",
        ));
    }
    let layout_settings = captured_layout(profile, settings)?;
    let layout = CarrierLayout::new(settings, &layout_settings)?;
    ChannelImpairments {
        response: response.cloned(),
        ..Default::default()
    }
    .plan(settings, &layout, 1.0)?;
    Ok(())
}

impl MeasurementPlayback {
    /// Validates and freezes bounded, explicitly ordered compatible measurements.
    /// # Arguments
    /// * `settings` - Accepted geometry, copied without modifying the caller.
    /// * `profile` - Exact device/channel/grid association and required carrier roles.
    /// * `frames` - Ordered original captures; missing required values reject.
    /// * `mode` - Static, equal-capture-weight average, hold or power interpolation.
    /// * `interval_symbols` - Positive physical-symbol interval for sequence modes.
    /// * `tolerance_seconds` - Explicit maximum capture distance from the DPD epoch.
    /// # Returns
    /// Immutable reusable plan with final capture held indefinitely.
    /// # Errors
    /// Rejects resource bounds, missing inputs, inconsistent grids/epochs/active values,
    /// invalid mode/count, zero intervals and overflowing symbol schedules.
    pub fn new(
        settings: &TransmissionSettings,
        profile: &CapturedProfile,
        frames: Vec<RxMerFrame>,
        mode: PlaybackMode,
        interval_symbols: u64,
        tolerance_seconds: u32,
    ) -> Result<Self, SimulationError> {
        if frames.is_empty()
            || frames.len() > MAX_PLAYBACK_FRAMES
            || frames.iter().map(|frame| frame.mer_db.len()).sum::<usize>() > MAX_PLAYBACK_VALUES
        {
            return Err(invalid(
                "Choose 1–64 captures within the 4-million-value limit.",
            ));
        }
        if (mode == PlaybackMode::Single && frames.len() != 1)
            || (matches!(mode, PlaybackMode::Hold | PlaybackMode::Interpolate)
                && (frames.len() < 2 || interval_symbols == 0))
        {
            return Err(invalid("Single requires one capture; playback requires at least two and a positive symbol interval."));
        }
        interval_symbols
            .checked_mul((frames.len() - 1) as u64)
            .ok_or_else(|| invalid("Playback symbol schedule overflows."))?;
        let layout_settings = captured_layout(profile, settings)?;
        let layout = CarrierLayout::new(settings, &layout_settings)?;
        let mut powers = Vec::with_capacity(frames.len());
        for frame in &frames {
            frame.validate()?;
            if frame.key != profile.key
                || frame.capture_time.abs_diff(profile.capture_time) > tolerance_seconds
            {
                return Err(invalid("RxMER device/channel/frequency grid or capture-time tolerance differs from DPD."));
            }
            ChannelImpairments {
                noise: NoiseModel::Rxmer {
                    frame: frame.clone(),
                },
                ..Default::default()
            }
            .plan(settings, &layout, 1.0)?;
            let values = frame
                .mer_db
                .iter()
                .map(|reading| reading.map(|mer| 10.0_f64.powf(-mer / 10.0)))
                .collect::<Vec<_>>();
            if values
                .iter()
                .flatten()
                .any(|power| !power.is_finite() || *power > MAX_NORMALIZED_NOISE_POWER)
            {
                return Err(invalid(
                    "RxMER residual power exceeds the supported finite range.",
                ));
            }
            powers.push(values);
        }
        let average = (0..profile.key.grid.sample_count)
            .map(|index| {
                let values = powers
                    .iter()
                    .map(|power| power[index])
                    .collect::<Option<Vec<_>>>()?;
                Some(values.iter().sum::<f64>() / values.len() as f64)
            })
            .collect();
        Ok(Self {
            frames: Arc::new(frames),
            powers: Arc::new(powers),
            average: Arc::new(average),
            mode,
            interval_symbols,
            settings: settings.clone(),
        })
    }

    /// Borrows unchanged original captures, including their epochs and provenance.
    /// # Returns
    /// Ordered observations; lifetime is tied to this plan.
    pub fn frames(&self) -> &[RxMerFrame] {
        &self.frames
    }

    /// Reports the final transition symbol and its elapsed simulation time.
    /// # Returns
    /// Symbol offset and milliseconds; static modes report zero span.
    pub fn span(&self) -> (u64, f64) {
        let last = if matches!(self.mode, PlaybackMode::Hold | PlaybackMode::Interpolate) {
            self.interval_symbols * (self.frames.len() - 1) as u64
        } else {
            0
        };
        let period = f64::from(self.settings.subcarrier_spacing.fft_samples())
            + f64::from(self.settings.cyclic_prefix_samples);
        (
            last,
            last as f64 * period * 1000.0
                / f64::from(crate::transmission::settings::SAMPLE_RATE_HZ),
        )
    }

    /// Reports elapsed milliseconds between consecutive playback captures.
    /// # Returns
    /// Explicit interval using the frozen symbol cadence, independent of recorded epochs.
    pub fn interval_milliseconds(&self) -> f64 {
        let period = f64::from(self.settings.subcarrier_spacing.fft_samples())
            + f64::from(self.settings.cyclic_prefix_samples);
        self.interval_symbols as f64 * period * 1000.0
            / f64::from(crate::transmission::settings::SAMPLE_RATE_HZ)
    }

    /// Queries actual residual power at an explicit physical observation symbol.
    /// # Arguments
    /// * `symbol` - Zero-based symbol relative to playback start; last frame is held.
    /// # Returns
    /// Bounded per-carrier power, never dB interpolation; no source or RNG mutation.
    /// Original nulls remain null; no frequency resampling occurs.
    pub fn at(&self, symbol: u64) -> ResidualSnapshot {
        if self.mode == PlaybackMode::Average {
            return ResidualSnapshot {
                power: self.average.as_ref().clone(),
                first_source: 0,
                second_source: self.frames.len() - 1,
                second_weight: 0.0,
            };
        }
        let first = if self.mode == PlaybackMode::Single {
            0
        } else {
            (symbol / self.interval_symbols).min((self.frames.len() - 1) as u64) as usize
        };
        let second = if self.mode == PlaybackMode::Interpolate {
            (first + 1).min(self.frames.len() - 1)
        } else {
            first
        };
        let weight = if first != second {
            (symbol % self.interval_symbols) as f64 / self.interval_symbols as f64
        } else {
            0.0
        };
        let power = self.powers[first]
            .iter()
            .zip(&self.powers[second])
            .map(|(left, right)| {
                if weight == 0.0 {
                    return *left;
                }
                Some((1.0 - weight) * left.as_ref()? + weight * right.as_ref()?)
            })
            .collect();
        ResidualSnapshot {
            power,
            first_source: first,
            second_source: second,
            second_weight: weight,
        }
    }
}
