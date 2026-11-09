/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Versioned decoded scenario input and bounded observation validation.
use crate::{
    error::{invalid, SimulationError},
    profile::candidate::{CandidateProfile, CapturedProfile},
    scenario::{ChannelFrame, RxMerFrame},
    types::ProtocolScope,
};
use serde::{Deserialize, Serialize};
/// Versioned decoded inputs, with independent source frames and optional context.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioDocument {
    pub schema_version: u32,
    pub protocol: ProtocolScope,
    pub frames: Vec<RxMerFrame>,
    #[serde(default)]
    pub profiles: Vec<CapturedProfile>,
    #[serde(default)]
    pub channels: Vec<ChannelFrame>,
}
impl ScenarioDocument {
    /// Validates limits and every original frame; grouping remains explicit.
    /// # Returns
    /// Unit when all arrays are finite and bounded.
    /// # Errors
    /// Rejects schema drift, excessive work and malformed grids or timestamps.
    pub fn validate(&self) -> Result<(), SimulationError> {
        let samples = self
            .frames
            .iter()
            .map(|frame| frame.mer_db.len())
            .chain(
                self.profiles
                    .iter()
                    .map(|profile| profile.assignments.len()),
            )
            .chain(self.channels.iter().map(|channel| channel.iq.len()))
            .try_fold(0usize, |sum, count| sum.checked_add(count));
        if self.schema_version != 1
            || self.frames.is_empty()
            || self.frames.len() + self.profiles.len() + self.channels.len() > 4096
            || samples.is_none_or(|count| count > 4_000_000)
        {
            return Err(invalid("Unsupported or oversized decoded scenario."));
        }
        for frame in &self.frames {
            frame.validate()?;
        }
        for profile in &self.profiles {
            let candidate = CandidateProfile {
                schema_version: 1,
                name: "Captured role validation".into(),
                protocol: self.protocol,
                key: profile.key.clone(),
                profile_id: Some(profile.profile_id),
                assignments: profile.assignments.clone(),
                sources: vec![profile.source.clone()],
                roles_verified: false,
            };
            candidate.validate()?;
            if profile.source.capture_time != Some(profile.capture_time) {
                return Err(invalid(
                    "Captured profile timestamp differs from its provenance.",
                ));
            }
        }
        for channel in &self.channels {
            channel.source.validate()?;
            channel.key.grid.validate()?;
            if channel.iq.len() != channel.key.grid.sample_count
                || channel.source.capture_time != Some(channel.capture_time)
                || channel
                    .iq
                    .iter()
                    .flatten()
                    .flatten()
                    .any(|value| !value.is_finite())
            {
                return Err(invalid("Malformed channel observation."));
            }
        }
        Ok(())
    }
}
