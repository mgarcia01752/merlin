/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Captured assignments and editable profile hypotheses with protected roles.
use crate::{
    error::{invalid, LabError},
    scenario::{ScenarioKey, SourceProvenance},
    types::{modulation::CarrierType, BitsPerSymbol, CaptureTime, ProfileId, ProtocolScope},
};
use serde::{Deserialize, Serialize};
/// Version of the internal candidate file; never a vendor deployment format.
pub const CANDIDATE_SCHEMA_VERSION: u32 = 1;
/// One decoded captured profile, retaining source and exact frequency grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedProfile {
    pub key: ScenarioKey,
    pub source: SourceProvenance,
    pub capture_time: CaptureTime,
    pub profile_id: ProfileId,
    pub assignments: Vec<CarrierAssignment>,
}
/// Declared source of an assignment, distinct from whether its threshold passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentOrigin {
    Captured,
    Proposed,
}
/// One source-position assignment; unknown roles and missing bit loading persist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierAssignment {
    pub carrier_type: CarrierType,
    pub bit_loading: Option<BitsPerSymbol>,
    pub origin: AssignmentOrigin,
}
/// Editable hypothesis. Protected captured roles are immutable through range edits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateProfile {
    pub schema_version: u32,
    pub name: String,
    pub protocol: ProtocolScope,
    pub key: ScenarioKey,
    pub profile_id: Option<ProfileId>,
    pub assignments: Vec<CarrierAssignment>,
    pub sources: Vec<SourceProvenance>,
    pub roles_verified: bool,
}
impl CandidateProfile {
    /// Validates imported candidate structure before evaluation or mutation.
    /// # Returns
    /// Unit for a supported, bounded candidate with valid data modulations.
    /// # Errors
    /// Rejects schema drift, malformed assignments/provenance or resource limits.
    pub fn validate(&self) -> Result<(), LabError> {
        self.key.grid.validate()?;
        if self.schema_version != CANDIDATE_SCHEMA_VERSION
            || self.name.is_empty()
            || self.name.len() > 256
            || self.assignments.len() != self.key.grid.sample_count
            || self.sources.len() > 4096
        {
            return Err(invalid("Unsupported or malformed candidate profile."));
        }
        for source in &self.sources {
            source.validate()?;
        }
        for assignment in &self.assignments {
            let valid = match assignment.carrier_type {
                CarrierType::Data => assignment
                    .bit_loading
                    .is_none_or(crate::math::bit_loading::is_data_bit_loading),
                CarrierType::Plc => assignment.bit_loading == Some(4),
                CarrierType::ContinuousPilot => assignment.bit_loading == Some(1),
                CarrierType::ZeroBitLoaded => assignment.bit_loading == Some(0),
                CarrierType::Exclusion | CarrierType::Unknown | CarrierType::Reserved => {
                    assignment.bit_loading.is_none()
                }
            };
            if !valid {
                return Err(invalid(
                    "Candidate contains an unsupported role or modulation.",
                ));
            }
        }
        Ok(())
    }
    /// Edits a range of existing data carriers, preserving all special/unknown roles.
    /// # Arguments
    /// * `first`, `count` - Source-offset range, not physical subcarrier numbers.
    /// * `bits` - Verified registry data modulation; None leaves loading unresolved.
    /// # Returns
    /// Number of data carriers edited; captured special roles remain intact.
    /// # Errors
    /// Rejects out-of-bounds ranges or invalid modulation before changing anything.
    pub fn edit_range(
        &mut self,
        first: usize,
        count: usize,
        bits: Option<BitsPerSymbol>,
    ) -> Result<usize, LabError> {
        self.validate()?;
        let end = first
            .checked_add(count)
            .filter(|end| count > 0 && *end <= self.assignments.len())
            .ok_or_else(|| invalid("Edit range is outside the candidate grid."))?;
        if bits.is_some_and(|bits| !crate::math::bit_loading::is_data_bit_loading(bits)) {
            return Err(invalid("Unsupported candidate data modulation."));
        }
        let mut edited = 0;
        for assignment in &mut self.assignments[first..end] {
            if assignment.carrier_type == CarrierType::Data {
                assignment.bit_loading = bits;
                assignment.origin = AssignmentOrigin::Proposed;
                edited += 1;
            }
        }
        Ok(edited)
    }
}
