/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Received-only temporal pilot differences for conservative scalar LLR reliability.
use super::ChannelEstimate;
use crate::{types::IqPair, SimulationError};
use serde::{Deserialize, Serialize};

/// Numerical receiver policies, independent of the applied channel and noise powers.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MceSettings {
    /// Positive raw complex FFT-bin variance floor; not a measured RF noise floor.
    pub minimum_variance: f64,
    /// Exponential averaging time in symbols, bounded to 1..128.
    pub averaging_symbols: u16,
}
impl Default for MceSettings {
    fn default() -> Self {
        Self {
            minimum_variance: 1e-8,
            averaging_symbols: 16,
        }
    }
}
impl MceSettings {
    /// Checks numerical floors and bounded receiver work before state is allocated.
    /// # Returns
    /// Unit for a supported receiver policy.
    /// # Errors
    /// Returns MceInvalidSettings for nonfinite/out-of-range values.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if !self.minimum_variance.is_finite()
            || !(1e-12..=1.0).contains(&self.minimum_variance)
            || !(1..=128).contains(&self.averaging_symbols)
        {
            return Err(SimulationError::new("MceInvalidSettings", "MCE requires a variance floor from 1e-12 to 1 and averaging from 1 to 128 symbols."));
        }
        Ok(())
    }
}
/// Bounded receiver-derived status; channel truth is never an input.
#[derive(Debug, Clone, Serialize)]
pub struct ReceiverReliability {
    pub channel_symbol_index: u64,
    /// Full pilot history is available; this is not an accuracy certificate.
    pub tracking: bool,
    pub compared_pilots: usize,
    /// Estimated raw complex FFT-bin variance before correction.
    pub raw_noise_power: f64,
}
/// Receiver-owned, one-entry-per-bin history and constant-size noise summary.
pub struct ReliabilityEstimator {
    settings: MceSettings,
    previous: Vec<Option<(u64, IqPair)>>,
    power: Option<f64>,
    next: Option<u64>,
    status: Option<ReceiverReliability>,
}
impl ReliabilityEstimator {
    /// Creates independent state without any supplied H or simulated variance.
    /// # Arguments
    /// * `fft_samples` - Exactly 4096 or 8192 physical bins.
    /// * `settings` - Validated smoothing and numerical variance floor.
    /// # Returns
    /// Empty bounded pilot history.
    /// # Errors
    /// Rejects invalid FFT size or receiver policies.
    pub fn new(fft_samples: usize, settings: MceSettings) -> Result<Self, SimulationError> {
        settings.validate()?;
        if ![4096, 8192].contains(&fft_samples) {
            return Err(SimulationError::new(
                "MceInvalidSettings",
                "Unsupported MCE FFT size.",
            ));
        }
        Ok(Self {
            settings,
            previous: vec![None; fft_samples],
            power: None,
            next: None,
            status: None,
        })
    }
    /// Updates from consecutive continuous pilots before publishing a noise proxy.
    /// # Arguments
    /// * `estimate` - Actual received pilots and receiver estimates, never truth.
    /// # Returns
    /// Smoothed raw variance. Half the squared same-pilot difference estimates
    /// white-noise power for a static channel; channel changes inflate it.
    /// # Errors
    /// Rejects malformed/duplicate pilots, sequence violations and nonfinite powers
    /// before committing history. Missing repeated pilots is an explicit failure.
    pub fn observe(&mut self, estimate: &ChannelEstimate) -> Result<f64, SimulationError> {
        let symbol = estimate.channel_symbol_index;
        if self.next.is_some_and(|next| next != symbol)
            || estimate.response.len() != self.previous.len()
        {
            return Err(SimulationError::new(
                "MceEstimationFailed",
                "MCE pilot sequence or FFT size changed.",
            ));
        }
        let next = symbol.checked_add(1).ok_or_else(|| {
            SimulationError::new("MceEstimationFailed", "MCE symbol count overflow.")
        })?;
        let mut sum = 0.0;
        let mut count = 0;
        let mut seen = std::collections::BTreeSet::new();
        for pilot in estimate.pilots.iter().filter(|pilot| !pilot.scattered) {
            let bin = usize::from(pilot.bin);
            if bin >= self.previous.len()
                || !seen.insert(bin)
                || pilot
                    .response
                    .iter()
                    .chain(pilot.reference.iter())
                    .any(|value| !value.is_finite())
            {
                return Err(SimulationError::new(
                    "MceEstimationFailed",
                    "MCE continuous pilot input is invalid.",
                ));
            }
            if let Some((previous_symbol, previous)) = self.previous[bin] {
                if previous_symbol.checked_add(1) == Some(symbol) {
                    let difference = [
                        pilot.response[0] - previous[0],
                        pilot.response[1] - previous[1],
                    ];
                    let reference_power = pilot.reference[0].powi(2) + pilot.reference[1].powi(2);
                    sum += 0.5 * reference_power * (difference[0].powi(2) + difference[1].powi(2));
                    count += 1;
                }
            }
        }
        if !sum.is_finite()
            || sum > 1000.0 * count.max(1) as f64
            || seen.len() < 2
            || (self.next.is_some() && count < 2)
        {
            return Err(SimulationError::new("MceEstimationFailed", "MCE needs at least two consecutive continuous pilots with finite bounded noise power."));
        }
        let power = if count > 0 {
            let observed = sum / count as f64;
            self.power.map_or(observed, |previous| {
                previous + (observed - previous) / f64::from(self.settings.averaging_symbols)
            })
        } else {
            self.settings.minimum_variance
        };
        for pilot in estimate.pilots.iter().filter(|pilot| !pilot.scattered) {
            self.previous[usize::from(pilot.bin)] = Some((symbol, pilot.response));
        }
        self.power = Some(power);
        self.next = Some(next);
        let power = power.max(self.settings.minimum_variance);
        self.status = Some(ReceiverReliability {
            channel_symbol_index: symbol,
            tracking: estimate.ready,
            compared_pilots: count,
            raw_noise_power: power,
        });
        Ok(power)
    }
    /// Borrows the latest receiver-derived status without advancing estimation.
    /// # Returns
    /// None before the first accepted pilot observation.
    pub fn status(&self) -> Option<&ReceiverReliability> {
        self.status.as_ref()
    }
}
