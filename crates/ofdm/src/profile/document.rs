/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stable consumer boundary. File I/O and PNM decoding belong to callers.
use super::{
    evaluation::{EvaluationReport, EvaluationSettings},
    invalid,
    model::*,
    LabError,
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
    pub fn validate(&self) -> Result<(), LabError> {
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
/// Portable hypothesis and analytical result; export never deploys an assignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateDocument {
    pub document_kind: String,
    pub schema_version: u32,
    pub candidate: CandidateProfile,
    pub settings: EvaluationSettings,
    pub evaluation: Option<EvaluationReport>,
}
impl CandidateDocument {
    /// Validates an internal candidate without trusting saved result claims.
    /// # Returns
    /// Unit for the supported schema and numeric settings.
    /// # Errors
    /// Rejects incompatible schemas and malformed profile content.
    pub fn validate(&self) -> Result<(), LabError> {
        if self.schema_version != 1 || self.document_kind != "docsis_ofdm_fec_candidate" {
            return Err(invalid("Unsupported candidate document schema."));
        }
        self.candidate.validate()?;
        self.settings.validate()?;
        Ok(())
    }
}

/// Rechecks a proposed candidate against supplied captured role assignments.
/// # Arguments
/// * `candidate` - Candidate hypothesis; verified status is recalculated.
/// * `scenario` - Source-bound decoded profiles owned by the consumer.
/// # Returns
/// Unit; special captured roles remain unchanged and unverifiable status is false.
/// # Errors
/// Rejects an alteration to a matching captured non-data assignment.
pub fn verify_captured_roles(
    candidate: &mut CandidateProfile,
    scenario: &ScenarioDocument,
) -> Result<(), LabError> {
    candidate.validate()?;
    scenario.validate()?;
    candidate.roles_verified = false;
    for profile in scenario.profiles.iter().filter(|profile| {
        profile.key == candidate.key
            && Some(profile.profile_id) == candidate.profile_id
            && candidate
                .sources
                .iter()
                .any(|source| source.sha256 == profile.source.sha256)
    }) {
        for (captured, proposed) in profile.assignments.iter().zip(&candidate.assignments) {
            if captured.carrier_type != crate::types::modulation::CarrierType::Data
                && captured != proposed
            {
                return Err(invalid("Candidate alters a protected captured role."));
            }
        }
        candidate.roles_verified = profile.assignments.iter().all(|assignment| {
            !matches!(
                assignment.carrier_type,
                crate::types::modulation::CarrierType::Unknown
                    | crate::types::modulation::CarrierType::Reserved
            )
        });
    }
    Ok(())
}
