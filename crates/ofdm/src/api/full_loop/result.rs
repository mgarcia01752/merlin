/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned actual job reports; retaining a result never reruns science or reads global state.
use super::*;
pub use crate::simulation::channel::ChannelJobResult as FullLoopReport;
use serde::Serialize;

/// Immutable shareable actual complete/partial report with accepted original inputs retained.
#[derive(Clone)]
pub struct FullLoopResult {
    pub(super) job: Arc<FullLoopReport>,
    pub(super) inputs: FullLoopInputs,
    pub(super) stages: Arc<observations::RetainedStages>,
    pub(super) observation_policy: Option<FullLoopObservationPolicy>,
}
impl FullLoopResult {
    /// Borrows the actual bounded scientific report, including all retained completed trials.
    /// # Returns
    /// Read-only report valid for this owned result's lifetime, independent of later runs.
    pub fn report(&self) -> &FullLoopReport {
        &self.job
    }
    /// Borrows accepted original source/profile/measurements/channel snapshots.
    /// # Returns
    /// Immutable original ownership/provenance; no source or simulation state advances.
    pub fn inputs(&self) -> &FullLoopInputs {
        &self.inputs
    }
}

/// Bounded original input identity summary; measured provenance stays separate from outcomes.
#[derive(Serialize)]
pub struct FullLoopInputSummary {
    pub profile_id: ProfileId,
    pub key: ScenarioKey,
    pub profile_sources: Vec<CaptureProvenance>,
    pub measurement_sources: Vec<CaptureProvenance>,
    pub channel_source: Option<CaptureProvenance>,
    pub source_kind: &'static str,
    pub source_sha256: Option<String>,
}
impl FullLoopInputs {
    /// Copies bounded original input identities without serializing source/capture bytes.
    /// # Returns
    /// Owned profile/measurement/H provenance and explicit source family/hash.
    /// # Errors
    /// Preserves an unavailable immutable capture lookup as a structured invariant failure.
    pub fn summary(&self) -> ApiResult<FullLoopInputSummary> {
        Ok(FullLoopInputSummary {
            profile_id: self.profile.profile_id(),
            key: self.profile.key().clone(),
            profile_sources: self.profile.sources().to_vec(),
            measurement_sources: (0..self.measurements.info().source_count)
                .map(|index| {
                    self.measurements
                        .capture(index)
                        .map(|value| value.provenance().clone())
                })
                .collect::<ApiResult<Vec<_>>>()?,
            channel_source: self
                .response
                .as_ref()
                .map(|value| value.provenance().clone()),
            source_kind: self.source.kind(),
            source_sha256: self.source.original_sha256().map(str::to_owned),
        })
    }
}
impl FullLoopResult {
    /// Borrows actual recovered original-file bytes without fill/padding or filesystem I/O.
    /// # Returns
    /// None for PRBS; actual whole decoded file bytes for finite input, including partial/errors.
    pub fn recovered_bytes(&self) -> Option<&[u8]> {
        self.job.recovered_file.as_ref().map(|file| file.bytes())
    }
}
