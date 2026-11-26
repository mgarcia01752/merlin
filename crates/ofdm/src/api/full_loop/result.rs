/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned actual job reports; retaining a result never reruns science or reads global state.
use super::*;
pub use crate::simulation::channel::results::PayloadRecovery as FullLoopFecRecovery;
pub use crate::simulation::channel::ChannelJobResult as FullLoopReport;
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
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
    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Counts requested buffer capacity, including spare capacity and private
    /// caches. Shared payloads are counted on every owned Arc edge; ordered
    /// entries and shared roots are reported separately from payload bytes.
    /// Allocator metadata and container/control-block implementation overhead
    /// are excluded. This passive read does not copy payloads or run science.
    ///
    /// # Returns
    /// Current-root metadata including the fixed root exactly once. This is
    /// neither a future-output reservation nor a process-memory bound.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }

    /// Borrows the actual bounded scientific report, including all retained completed trials.
    /// # Returns
    /// Read-only report valid for this owned result's lifetime, independent of later runs.
    pub fn report(&self) -> &FullLoopReport {
        &self.job
    }

    /// Reads whole-run source-verified recovery separately from decoder outcome categories.
    /// # Returns
    /// Owned constant-size counts for this actual complete, cancelled or failed result.
    /// Rejected words and decoder-accepted source mismatches never count as recovered;
    /// unfinished generated words remain unrecovered. Observation windows and word-detail
    /// retention do not change this scope. No source, receiver or simulation state advances.
    pub fn fec_recovery(&self) -> FullLoopFecRecovery {
        FullLoopFecRecovery::from_counters(
            self.job.summary.generated_payloads,
            &self.job.summary.fec,
            self.job.unrecovered_generated_payloads,
        )
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

impl RetainedPayload for FullLoopResult {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            job,
            inputs,
            stages,
            observation_policy,
        } = self;
        merlin_api::retention_support::sum([
            job.retained_children(),
            inputs.retained_children(),
            stages.retained_children(),
            observation_policy.retained_children(),
        ])
    }
}
