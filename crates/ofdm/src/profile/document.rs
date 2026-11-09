/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Internal candidate documents and source-bound role verification.
use super::{
    candidate::*,
    evaluation::{EvaluationReport, EvaluationSettings},
};
use crate::error::{invalid, SimulationError};
use serde::{Deserialize, Serialize};
// Preserve the established document import without duplicating the scenario model.
pub use crate::scenario::document::ScenarioDocument;
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
    pub fn validate(&self) -> Result<(), SimulationError> {
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
) -> Result<(), SimulationError> {
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
