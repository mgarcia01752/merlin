/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit measurement selection and supported fixed-profile admission.
use super::ChannelRunSettings;
use crate::{
    error::invalid,
    scenario::document::ScenarioDocument,
    signal_processing::channel::impairments::{offset, ChannelImpairments, NoiseModel},
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    types::modulation::CarrierType,
    SimulationError,
};
use serde::Deserialize;

/// Derives the exact captured static layout without changing frequency or assignments.
/// # Arguments
/// * `profile` - Expanded captured assignments, including PLC and continuous pilots.
/// * `transmission` - Caller-selected fixed numerology, CP/RP and bandwidth.
/// # Returns
/// Validated static layout for the captured grid; no modulation downgrade is performed.
/// # Errors
/// Rejects inconsistent spacing, invalid PLC/pilot geometry or unsupported static roles.
pub fn captured_layout(
    profile: &crate::profile::candidate::CapturedProfile,
    transmission: &crate::transmission::TransmissionSettings,
) -> Result<crate::transmission::layout::CarrierLayoutSettings, SimulationError> {
    let grid = &profile.key.grid;
    grid.validate()?;
    if grid.spacing_hz != transmission.subcarrier_spacing.frequency_hz()
        || profile.assignments.len() != grid.sample_count
    {
        return Err(invalid("Profile/numerology grid mismatch."));
    }
    let physical = |offset: usize| {
        u16::try_from(usize::from(grid.first_subcarrier_index) + offset)
            .map_err(|_| invalid("Captured bin index overflow."))
    };
    let plc: Vec<_> = profile
        .assignments
        .iter()
        .enumerate()
        .filter(|(_, assignment)| assignment.carrier_type == CarrierType::Plc)
        .map(|(offset, _)| physical(offset))
        .collect::<Result<_, _>>()?;
    let width = if grid.spacing_hz == 25000 { 16 } else { 8 };
    if plc.len() != width || plc.windows(2).any(|bins| bins[1] != bins[0] + 1) {
        return Err(invalid(
            "Captured PLC must be one contiguous complete block.",
        ));
    }
    let predefined = crate::transmission::layout::physical::predefined_pilots(
        transmission.subcarrier_spacing,
        plc[0],
    )?;
    let mut layout = crate::transmission::layout::CarrierLayoutSettings {
        zero_frequency_hz: grid.zero_frequency_hz,
        first_active_bin: grid.first_subcarrier_index,
        active_end_exclusive: physical(grid.sample_count)?,
        plc_first_bin: plc[0],
        excluded_bins: Vec::new(),
        additional_continuous_pilots: Vec::new(),
    };
    for (offset, assignment) in profile.assignments.iter().enumerate() {
        let bin = physical(offset)?;
        match assignment.carrier_type {
            CarrierType::ContinuousPilot => {
                if !predefined.contains(&bin) {
                    layout.additional_continuous_pilots.push(bin);
                }
            }
            CarrierType::Exclusion => layout.excluded_bins.push(bin),
            CarrierType::Data | CarrierType::ZeroBitLoaded | CarrierType::Plc => {}
            _ => {
                return Err(invalid(
                    "Captured layout includes unsupported reserved/unknown roles.",
                ))
            }
        }
    }
    CarrierLayout::new(transmission, &layout)?;
    Ok(layout)
}

/// Explicit later captured frame/response; simulated timing follows original epochs.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementTransition {
    pub frame_index: usize,
    pub channel_index: Option<usize>,
}
/// Caller-selected sources; tolerance never rewrites capture epochs.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementSelection {
    #[serde(default = "default_rxmer_noise")]
    pub use_rxmer_noise: bool,
    #[serde(default)]
    pub transitions: Vec<MeasurementTransition>,
    pub frame_index: usize,
    pub profile_id: u8,
    pub channel_index: Option<usize>,
    pub tolerance_seconds: u32,
    pub noise_seed: u64,
}
fn default_rxmer_noise() -> bool {
    true
}
impl MeasurementSelection {
    /// Admits an imported ProfileA scenario and explicit later measurement epochs.
    /// # Arguments
    /// * `scenario` - Immutable original decoded observations.
    /// * `settings` - Caller-owned geometry; loading, impairments and timeline commit atomically.
    /// # Returns
    /// Unit after exact source association and protected-role/profile coverage checks.
    /// # Errors
    /// Rejects unsupported coding modes, mismatched roles, ambiguous profiles,
    /// out-of-tolerance epochs, missing observations or additional injected noise.
    pub fn apply(
        &self,
        scenario: &ScenarioDocument,
        settings: &mut ChannelRunSettings,
    ) -> Result<(), SimulationError> {
        scenario.validate()?;
        if self.transitions.len() > 4096 {
            return Err(invalid("Too many imported measurement transitions."));
        }
        self.apply_validated(scenario, settings)
    }
    fn apply_validated(
        &self,
        scenario: &ScenarioDocument,
        settings: &mut ChannelRunSettings,
    ) -> Result<(), SimulationError> {
        // Each independent captured-profile experiment advertises these assignments
        // as bootstrap Profile A (wire ID0). The capture ID remains selection provenance.
        if self.profile_id > 15 {
            return Err(SimulationError::new(
                "UnsupportedMeasuredProfile",
                "Captured data profile ID must be in 0..15.",
            ));
        }

        if !matches!(settings.impairments.noise, NoiseModel::Noiseless)
            || settings.impairments.response.is_some()
        {
            return Err(invalid(
                "Measurement selection cannot be combined with another impairment source.",
            ));
        }
        let frame = scenario
            .frames
            .get(self.frame_index)
            .ok_or_else(|| invalid("RxMER frame index is out of range."))?;
        let candidates: Vec<_> = scenario
            .profiles
            .iter()
            .filter(|profile| profile.profile_id == self.profile_id && profile.key == frame.key)
            .collect();
        if candidates.len() != 1 {
            return Err(invalid(
                "Select one unambiguous captured profile ID matching the RxMER grid.",
            ));
        }
        let profile = candidates[0];
        if profile.capture_time.abs_diff(frame.capture_time) > self.tolerance_seconds {
            return Err(invalid("Profile epoch exceeds measurement tolerance."));
        }
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout)?;
        let mut physical_loading = vec![0; layout.static_roles().len()];
        for (bin, role) in layout.static_roles().iter().enumerate() {
            if *role == PhysicalCarrierRole::Excluded {
                continue;
            }
            let position = offset(
                &frame.key.grid,
                layout
                    .frequency_hz(bin as u16)
                    .ok_or_else(|| invalid("Missing frequency."))?,
                settings.transmission.subcarrier_spacing.frequency_hz(),
            )?;
            let assignment = &profile.assignments[position];
            let expected = match role {
                PhysicalCarrierRole::Data => {
                    let supported = match assignment.carrier_type {
                        CarrierType::Data => assignment
                            .bit_loading
                            .is_some_and(|bits| matches!(bits, 4 | 6..=14)),
                        CarrierType::ZeroBitLoaded => assignment.bit_loading == Some(0),
                        _ => false,
                    };
                    if !supported {
                        return Err(invalid("Unsupported captured data loading."));
                    }
                    physical_loading[bin] = assignment.bit_loading.unwrap_or(0);
                    (assignment.carrier_type, assignment.bit_loading)
                }
                PhysicalCarrierRole::ContinuousPilot => (CarrierType::ContinuousPilot, Some(1)),
                PhysicalCarrierRole::Plc => (CarrierType::Plc, Some(4)),
                _ => unreachable!(),
            };
            if (assignment.carrier_type, assignment.bit_loading) != expected {
                return Err(SimulationError::new("UnsupportedMeasuredProfile","Captured protected assignments differ from the configured data/pilot/PLC layout."));
            }
        }
        let response = if let Some(index) = self.channel_index {
            let response = scenario
                .channels
                .get(index)
                .ok_or_else(|| invalid("Channel response index is out of range."))?;
            if response.key != frame.key
                || response.capture_time.abs_diff(frame.capture_time) > self.tolerance_seconds
            {
                return Err(invalid(
                    "Response identity/grid or epoch differs from the selected scenario.",
                ));
            }
            Some(response.clone())
        } else {
            None
        };
        let impairments = ChannelImpairments {
            additional_loss: settings.impairments.additional_loss.clone(),
            power: settings.impairments.power.clone(),
            seed: self.noise_seed,
            noise: if self.use_rxmer_noise {
                NoiseModel::Rxmer {
                    frame: frame.clone(),
                }
            } else {
                NoiseModel::Noiseless
            },
            response,
        };
        impairments.plan(&settings.transmission, &layout, settings.llr_variance)?;
        let mut candidate = settings.clone();
        candidate.data_bit_loading = Some(physical_loading);
        candidate.impairments = impairments;
        if !self.transitions.is_empty() && !candidate.impairment_timeline.is_empty() {
            return Err(invalid(
                "Select imported transitions or an explicit impairment timeline, not both.",
            ));
        }
        let mut previous = frame.capture_time;
        let mut retained_samples = 0usize;
        for transition in &self.transitions {
            let next_frame = scenario
                .frames
                .get(transition.frame_index)
                .ok_or_else(|| invalid("Transition RxMER frame index is out of range."))?;
            if next_frame.key != frame.key || next_frame.capture_time <= previous {
                return Err(invalid(
                    "Measurement frames must share an exact identity/grid and increase in epoch.",
                ));
            }
            let selection = MeasurementSelection {
                use_rxmer_noise: self.use_rxmer_noise,
                transitions: Vec::new(),
                frame_index: transition.frame_index,
                profile_id: self.profile_id,
                channel_index: transition.channel_index,
                tolerance_seconds: self.tolerance_seconds,
                noise_seed: self.noise_seed,
            };
            let mut next_settings = settings.clone();
            next_settings.impairments = ChannelImpairments {
                additional_loss: settings.impairments.additional_loss.clone(),
                power: settings.impairments.power.clone(),
                ..Default::default()
            };
            next_settings.impairment_timeline.clear();
            selection.apply_validated(scenario, &mut next_settings)?;
            retained_samples = retained_samples
                .checked_add(next_frame.mer_db.len())
                .and_then(|count| {
                    count.checked_add(
                        next_settings
                            .impairments
                            .response
                            .as_ref()
                            .map_or(0, |response| response.iq.len()),
                    )
                })
                .filter(|count| *count <= 4_000_000)
                .ok_or_else(|| invalid("Imported timeline exceeds retained-sample budget."))?;
            let ticks = u64::from(next_frame.capture_time - frame.capture_time)
                * u64::from(crate::transmission::settings::SAMPLE_RATE_HZ);
            let period = u64::from(settings.transmission.subcarrier_spacing.fft_samples())
                + u64::from(settings.transmission.cyclic_prefix_samples);
            candidate
                .impairment_timeline
                .push(super::timeline::ImpairmentEpoch {
                    first_observation_symbol: ticks.div_ceil(period),
                    capture_time: next_frame.capture_time,
                    impairments: next_settings.impairments,
                });
            previous = next_frame.capture_time;
        }
        candidate.validate()?;
        *settings = candidate;
        Ok(())
    }
}
