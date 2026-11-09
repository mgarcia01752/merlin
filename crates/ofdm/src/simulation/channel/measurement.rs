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
    LabError,
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
) -> Result<crate::transmission::layout::CarrierLayoutSettings, LabError> {
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
            CarrierType::Data | CarrierType::Plc => {}
            _ => {
                return Err(invalid(
                    "Captured layout includes unsupported zero/reserved/unknown roles.",
                ))
            }
        }
    }
    CarrierLayout::new(transmission, &layout)?;
    Ok(layout)
}

/// Caller-selected sources; tolerance never rewrites capture epochs.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeasurementSelection {
    pub frame_index: usize,
    pub profile_id: u8,
    pub channel_index: Option<usize>,
    pub tolerance_seconds: u32,
    pub noise_seed: u64,
}
impl MeasurementSelection {
    /// Admits an imported scenario into the current uniform256-QAM waveform experiment.
    /// # Arguments
    /// * `scenario` - Immutable original decoded observations.
    /// * `settings` - Caller-owned geometry; only the explicit impairment setting is filled.
    /// # Returns
    /// Unit after exact source association and protected-role/profile coverage checks.
    /// # Errors
    /// Rejects unsupported/mixed loading, mismatched roles, ambiguous profiles,
    /// out-of-tolerance epochs, missing observations or additional injected noise.
    pub fn apply(
        &self,
        scenario: &ScenarioDocument,
        settings: &mut ChannelRunSettings,
    ) -> Result<(), LabError> {
        scenario.validate()?;
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
                PhysicalCarrierRole::Data => (CarrierType::Data, Some(8)),
                PhysicalCarrierRole::ContinuousPilot => (CarrierType::ContinuousPilot, Some(1)),
                PhysicalCarrierRole::Plc => (CarrierType::Plc, Some(4)),
                _ => unreachable!(),
            };
            if (assignment.carrier_type, assignment.bit_loading) != expected {
                return Err(LabError::new("UnsupportedMeasuredProfile","Captured assignments differ from the configured uniform256-QAM/pilot/PLC layout."));
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
            seed: self.noise_seed,
            noise: NoiseModel::Rxmer {
                frame: frame.clone(),
            },
            response,
        };
        impairments.plan(&settings.transmission, &layout, settings.llr_variance)?;
        settings.impairments = impairments;
        Ok(())
    }
}
