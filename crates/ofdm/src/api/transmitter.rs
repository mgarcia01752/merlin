/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed idle TX snapshots; scientific generation stays with the existing transmitter owner.
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
mod calibrated;
mod control;
mod result;
mod timing;
use super::*;
use crate::common::constants::DEFAULT_CONTINUOUS_PRBS15_SEED;
use crate::simulation::transmitter_preview as engine;
pub use control::*;
pub use result::*;
use std::sync::Arc;
pub use timing::*;

/// Immutable reusable idle-preview admission, independent of live simulation/source state.
#[derive(Clone)]
pub struct TransmitterPreviewRequest {
    request: engine::TransmitterPreviewRequest,
    key: ScenarioKey,
    profile_id: u8,
    sources: Vec<CaptureProvenance>,
    source: SourceInput,
}

impl TransmitterPreviewRequest {
    /// Freezes accepted numerology, profile and passive source through existing TX admission.
    /// # Arguments
    /// * `transmission` - Validated timing/numerology snapshot.
    /// * `profile` - Original/hypothesis Dpd with preserved physical assignments.
    /// * `source` - Passive file/fill or continuous PRBS15 input; no stream advances.
    /// # Returns
    /// Reusable Send+Sync request; generation requires an explicit operation.
    /// # Errors
    /// Preserves existing unsupported profile/loading/geometry/source failures.
    pub fn new(
        transmission: &TransmissionConfiguration,
        profile: &Dpd,
        source: &SourceInput,
    ) -> ApiResult<Self> {
        Self::prepare(
            transmission,
            &profile.profile,
            profile.sources.clone(),
            source,
        )
    }

    /// Admits one encoded source profile without altering existing preview-ID support.
    /// # Arguments
    /// * `transmission` - Validated timing/numerology.
    /// * `capture` - Complete original source catalog.
    /// * `profile_id` - Encoded source identity; the existing TX preview supports 0..15.
    /// * `source` - Passive immutable source, shared without advancing bits.
    /// # Returns
    /// A frozen request that bootstraps the selected supported loading as Profile A.
    /// # Errors
    /// Rejects absent/unsupported source IDs and existing TX compatibility failures.
    pub fn from_capture(
        transmission: &TransmissionConfiguration,
        capture: &ProfileCapture,
        profile_id: u8,
        source: &SourceInput,
    ) -> ApiResult<Self> {
        Self::prepare(
            transmission,
            capture.profile_ref(profile_id)?,
            vec![capture.provenance().clone()],
            source,
        )
    }

    fn prepare(
        transmission: &TransmissionConfiguration,
        profile: &crate::profile::candidate::CapturedProfile,
        sources: Vec<CaptureProvenance>,
        source: &SourceInput,
    ) -> ApiResult<Self> {
        transmission.check_profile_grid(&profile.key.grid)?;
        let request = engine::TransmitterPreviewRequest::new(
            transmission.settings(),
            profile,
            source.configuration.clone(),
            // Dormant for file input; its fill generator retains its independent explicit seed.
            source
                .prbs_seed()
                .map_or(DEFAULT_CONTINUOUS_PRBS15_SEED, |seed| seed.value()),
        )?;
        Ok(Self {
            request,
            key: profile.key.clone(),
            profile_id: profile.profile_id,
            sources,
            source: source.clone(),
        })
    }

    /// Generates a fresh private preview synchronously, without a channel/receiver run.
    /// # Arguments
    /// * `control` - Fresh one-shot observation/cancellation control; callers dispatch work.
    /// # Returns
    /// Owned actual post-overlap/pre-channel samples/PSD; no accepted source is mutated.
    /// # Errors
    /// Busy for a reused/executing control; existing engine errors otherwise. Cancelled
    /// generation returns no new snapshot and reports Cancelled through the control.
    pub fn generate(&self, control: &TransmitterPreviewControl) -> ApiResult<TransmitterPreview> {
        control.begin()?;
        let result = self
            .request
            .generate_with_cancellation(&control.stop(), |done, total| {
                control.progress(done, total)
            });
        control.finish(result.is_ok());
        let result = result?;
        if control.snapshot().state == TransmitterPreviewState::Cancelled {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "TX preview cancelled",
            ));
        }
        Ok(TransmitterPreview {
            inner: Arc::new(result),
            key: self.key.clone(),
            profile_id: self.profile_id,
            sources: self.sources.clone(),
            source: self.source.clone(),
        })
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }
}

impl RetainedPayload for TransmitterPreviewRequest {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            request,
            key,
            profile_id,
            sources,
            source,
        } = self;
        merlin_api::retention_support::sum([
            request.retained_children(),
            key.retained_children(),
            profile_id.retained_children(),
            sources.retained_children(),
            source.retained_children(),
        ])
    }
}
