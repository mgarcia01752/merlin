/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen frequency-diagonal response/noise plans at explicit equalized reference planes.
use crate::{
    error::invalid,
    scenario::{ChannelFrame, RxMerFrame},
    signal_processing::receiver::equalization::KnownReferenceEqualizer,
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        TransmissionSettings,
    },
    types::IqPair,
    SimulationError,
};
use serde::{Deserialize, Serialize};

/// Exclusive residual source: no implicit thermal floor and no AWGN+RxMER sum.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum NoiseModel {
    #[default]
    Noiseless,
    /// Absolute normalized complex time-sample power; unitary per-bin variance is equal.
    Awgn { sample_power: f64 },
    /// Total post-equalization residual error at original carrier centers.
    Rxmer { frame: RxMerFrame },
}
/// Scientific settings without files, bindings or runtime state.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelImpairments {
    #[serde(default)]
    pub seed: u64,
    #[serde(default)]
    pub noise: NoiseModel,
    /// Supplied relative channel response H, not equalizer1/H or calibrated path loss.
    pub response: Option<ChannelFrame>,
    /// Explicit synthetic magnitude response composed without rewriting captures.
    #[serde(default)]
    pub additional_loss: super::additional_loss::AdditionalLoss,
    /// Scalar conditioning and optional explicitly calibrated independent RX noise.
    #[serde(default)]
    pub power: super::power::PowerConditioning,
}
/// Direct numerical inspection boundary for the complete frozen physical FFT grid.
pub struct ImpairmentPlan {
    pub response: Vec<IqPair>,
    pub residual_power: Vec<f64>,
    pub receiver_noise_power: f64,
    pub llr_variance: Vec<f64>,
}
impl ImpairmentPlan {
    /// Applies H times signal-plus-residual at the public frequency-domain boundary.
    /// # Arguments
    /// * `bins` - Complete physical bins; original input remains unchanged.
    /// * `roles` - Carrier roles; RxMER excludes nulled bins, independent RX noise does not.
    /// * `rng` - Dedicated caller-owned Gaussian stream, advanced only for nonzero powers.
    /// # Returns
    /// Owned altered bins before IFFT/CP/window synthesis. Pilot boost is not
    /// included in residual variance because RxMER references ideal data power.
    /// # Errors
    /// Rejects wrong shape or unavailable/nonnumeric input before drawing. Numeric
    /// output overflow returns an error after draws; caller must close that stream.
    pub fn apply_bins(
        &self,
        bins: &[Option<IqPair>],
        roles: &[PhysicalCarrierRole],
        rng: &mut impl rand::Rng,
    ) -> Result<Vec<Option<IqPair>>, SimulationError> {
        let count = self.response.len();
        if !self.receiver_noise_power.is_finite()
            || self.receiver_noise_power < 0.0
            || bins.len() != count
            || roles.len() != count
            || self.residual_power.len() != count
            || self.llr_variance.len() != count
            || ![4096, 8192].contains(&count)
            || bins.iter().any(|point| {
                point.is_none_or(|point| point.iter().any(|component| !component.is_finite()))
            })
            || self
                .response
                .iter()
                .flatten()
                .any(|component| !component.is_finite())
            || self.residual_power.iter().any(|power| {
                !power.is_finite() || !(0.0..=super::MAX_NORMALIZED_NOISE_POWER).contains(power)
            })
        {
            return Err(invalid("Invalid physical-bin channel input/plan."));
        }
        let mut altered = bins.to_vec();
        for (bin, point) in altered.iter_mut().enumerate() {
            let point = point
                .as_mut()
                .ok_or_else(|| invalid("Unavailable physical bin."))?;
            if roles[bin] != PhysicalCarrierRole::Excluded {
                // PHY I20 section9.3.6 uses ideal DATA constellation power,
                // excluding pilot boost, in the RxMER numerator.
                let power = self.residual_power[bin];
                if power > 0.0 {
                    crate::signal_processing::noise::awgn::add_awgn(point, power, rng);
                }
                let gain = self.response[bin];
                *point = [
                    point[0] * gain[0] - point[1] * gain[1],
                    point[0] * gain[1] + point[1] * gain[0],
                ];
                if point.iter().any(|component| !component.is_finite()) {
                    return Err(invalid("Channel response multiplication overflow."));
                }
            }
            if self.receiver_noise_power > 0.0 {
                crate::signal_processing::noise::awgn::add_awgn(
                    point,
                    self.receiver_noise_power,
                    rng,
                );
            }
        }
        Ok(altered)
    }
}
impl ChannelImpairments {
    /// Maps observations by physical frequency without interpolation or missing-data substitution.
    /// # Arguments
    /// * `settings` - Fixed FFT numerology.
    /// * `layout` - Static data/control/pilot roles; excluded bins need no measurements.
    /// * `reference_variance` - Positive noiseless LLR scale for unused/zero-noise bins.
    /// # Returns
    /// Owned H, normalized residual powers and per-bin post-equalization LLR variances.
    /// # Errors
    /// Rejects malformed observations, incompatible grids or missing/zero active gains.
    /// Time association belongs to the explicit measurement-selection layer.
    pub fn plan(
        &self,
        settings: &TransmissionSettings,
        layout: &CarrierLayout,
        reference_variance: f64,
    ) -> Result<ImpairmentPlan, SimulationError> {
        if !reference_variance.is_finite() || reference_variance <= 0.0 {
            return Err(invalid("Invalid LLR reference variance."));
        }
        self.additional_loss.validate()?;
        self.power.validate()?;
        let active: Vec<_> = layout
            .static_roles()
            .iter()
            .enumerate()
            .filter(|(_, role)| **role != PhysicalCarrierRole::Excluded)
            .filter_map(|(bin, _)| layout.frequency_hz(bin as u16))
            .collect();
        let low = *active
            .first()
            .ok_or_else(|| invalid("No active channel carriers."))?;
        let high = *active
            .last()
            .ok_or_else(|| invalid("No active channel carriers."))?;
        let count = settings.subcarrier_spacing.fft_samples() as usize;
        let mut plan = ImpairmentPlan {
            response: vec![[1.0, 0.0]; count],
            receiver_noise_power: self.power.receiver_noise_power(),
            residual_power: vec![0.0; count],
            llr_variance: vec![reference_variance; count],
        };
        if let NoiseModel::Awgn { sample_power } = self.noise {
            super::awgn::SampleAwgn::new(sample_power, self.seed, 0)?;
            if self.power != super::power::PowerConditioning::default()
                || self.response.is_some()
                || self.additional_loss != super::additional_loss::AdditionalLoss::Off
            {
                return Err(invalid("Sample AWGN with supplied response is not supported; use explicit RxMER reference plane."));
            }
            plan.residual_power.fill(sample_power);
            if sample_power > 0.0 {
                plan.llr_variance.fill(sample_power);
            }
        }
        if let NoiseModel::Rxmer { frame } = &self.noise {
            frame.validate()?;
        }
        if let Some(response) = &self.response {
            response.key.grid.validate()?;
            response.source.validate()?;
            if response.iq.len() != response.key.grid.sample_count
                || response.source.capture_time != Some(response.capture_time)
            {
                return Err(invalid("Malformed channel response observation."));
            }
            if let NoiseModel::Rxmer { frame } = &self.noise {
                if frame.key != response.key {
                    return Err(invalid("RxMER/response identity or grid mismatch."));
                }
            }
        }
        for (bin, role) in layout.static_roles().iter().enumerate() {
            if *role == PhysicalCarrierRole::Excluded {
                continue;
            }
            let frequency = layout
                .frequency_hz(bin as u16)
                .ok_or_else(|| invalid("Missing physical frequency."))?;
            if let NoiseModel::Rxmer { frame } = &self.noise {
                let offset = offset(
                    &frame.key.grid,
                    frequency,
                    settings.subcarrier_spacing.frequency_hz(),
                )?;
                let mer = frame.mer_db[offset]
                    .ok_or_else(|| invalid("Missing RxMER for an active simulation carrier."))?;
                let power = 10.0_f64.powf(-mer / 10.0);
                if !power.is_finite() || power <= 0.0 || power > super::MAX_NORMALIZED_NOISE_POWER {
                    return Err(invalid(
                        "RxMER residual power is outside the numeric simulation range.",
                    ));
                }
                plan.residual_power[bin] = power;
                plan.llr_variance[bin] = power;
            }
            if let Some(response) = &self.response {
                let offset = offset(
                    &response.key.grid,
                    frequency,
                    settings.subcarrier_spacing.frequency_hz(),
                )?;
                plan.response[bin] = response.iq[offset]
                    .ok_or_else(|| invalid("Missing response for an active simulation carrier."))?;
            }
        }
        if self.additional_loss != super::additional_loss::AdditionalLoss::Off {
            for (bin, role) in layout.static_roles().iter().enumerate() {
                if *role == PhysicalCarrierRole::Excluded {
                    continue;
                }
                let frequency = layout
                    .frequency_hz(bin as u16)
                    .ok_or_else(|| invalid("Missing physical frequency."))?;
                let gain =
                    10.0_f64.powf(self.additional_loss.gain_db(frequency, low, high)? / 20.0);
                for component in &mut plan.response[bin] {
                    *component *= gain;
                }
            }
        }
        let scalar = self.power.net_amplitude();
        for (bin, role) in layout.static_roles().iter().enumerate() {
            if *role == PhysicalCarrierRole::Excluded {
                continue;
            }
            for component in &mut plan.response[bin] {
                *component *= scalar;
            }
            let h2 = plan.response[bin][0].powi(2) + plan.response[bin][1].powi(2);
            if plan.receiver_noise_power > 0.0 {
                plan.llr_variance[bin] = plan.residual_power[bin] + plan.receiver_noise_power / h2;
            }
        }
        if !plan.receiver_noise_power.is_finite()
            || plan.receiver_noise_power > super::MAX_NORMALIZED_NOISE_POWER
        {
            return Err(invalid(
                "Independent receiver noise exceeds normalized numerical limits.",
            ));
        }
        KnownReferenceEqualizer::new(&plan.response, 1.0)?;
        Ok(plan)
    }
}
/// Locates an exact physical carrier within an unmodified observation grid.
/// # Arguments
/// * `grid` - Original measurement grid.
/// * `frequency` - Simulation carrier center in Hz.
/// * `spacing` - Required simulation/measurement spacing in Hz.
/// # Returns
/// Original array offset; subband selection never rewrites source headers.
/// # Errors
/// Rejects spacing mismatch, nonaligned centers or uncovered frequencies.
pub(crate) fn offset(
    grid: &crate::scenario::FrequencyGrid,
    frequency: u32,
    spacing: u32,
) -> Result<usize, SimulationError> {
    let start = grid.frequency(0)?;
    let delta = frequency
        .checked_sub(start)
        .ok_or_else(|| invalid("Simulation frequency is below measured coverage."))?;
    let offset = (delta / spacing.max(1)) as usize;
    if grid.spacing_hz != spacing
        || spacing == 0
        || delta % spacing != 0
        || offset >= grid.sample_count
    {
        return Err(invalid(
            "Simulation frequency grid is not covered exactly by the capture.",
        ));
    }
    Ok(offset)
}
