/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit finite retention policy; existing probe dispatch remains the scientific observation owner.
use super::*;
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};

/// Maximum copied actual elements in one public stage query.
pub const MAX_STAGE_QUERY_ELEMENTS: usize = 4096;
/// Maximum actual original elements selected per event, matching the legacy workbench.
pub const MAX_STAGE_EVENT_ELEMENTS: usize = 16384;
/// Maximum payload/context retention accounted by this owner, excluding engine/allocator overhead.
pub const MAX_STAGE_RETENTION_BYTES: usize = 96 * 1024 * 1024;

/// One finite stage and actual-phase selection; no source or reconstructed values are requested.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FullLoopStageRetention {
    pub stage: FullLoopStage,
    pub max_events: usize,
    /// True keeps observation phase only; false retains the actual original phase tags.
    pub observation_only: bool,
}

/// Editable bounded policy, frozen by FullLoopObservationPolicy before attaching to a request.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopObservationInput {
    pub stages: Vec<FullLoopStageRetention>,
    pub max_events: usize,
    pub max_elements_per_event: usize,
    pub max_total_elements: usize,
    pub max_retained_bytes: usize,
}

pub(super) const SUPPORTED: [FullLoopStage; 22] = [
    FullLoopStage::SourcePayload,
    FullLoopStage::PayloadCoded,
    FullLoopStage::PayloadInterleavedBits,
    FullLoopStage::PayloadLabels,
    FullLoopStage::PayloadMapped,
    FullLoopStage::PayloadInterleaved,
    FullLoopStage::PhysicalBins,
    FullLoopStage::PlcMessage,
    FullLoopStage::PlcIq,
    FullLoopStage::PlcPreamble,
    FullLoopStage::ContinuousPilots,
    FullLoopStage::ScatteredPilots,
    FullLoopStage::TxIfft,
    FullLoopStage::TxPrefix,
    FullLoopStage::TxWindow,
    FullLoopStage::TxOverlap,
    FullLoopStage::ChannelOutput,
    FullLoopStage::RxFramed,
    FullLoopStage::RxUseful,
    FullLoopStage::RxFft,
    FullLoopStage::RxChannelEstimate,
    FullLoopStage::PayloadLdpcInput,
];
impl Default for FullLoopObservationInput {
    /// Selects the existing finite workbench observation policy without enabling it on any request.
    /// # Returns
    /// Explicit TX/RX stages, original phase distinctions and fixed event/element/byte budgets.
    fn default() -> Self {
        Self {
            stages: SUPPORTED
                .into_iter()
                .map(|stage| FullLoopStageRetention {
                    stage,
                    max_events: match stage {
                        FullLoopStage::PlcMessage => 64,
                        FullLoopStage::ChannelOutput => 12,
                        FullLoopStage::TxOverlap
                        | FullLoopStage::RxFramed
                        | FullLoopStage::RxChannelEstimate => 4,
                        _ => 1,
                    },
                    observation_only: !matches!(
                        stage,
                        FullLoopStage::PlcMessage
                            | FullLoopStage::PlcIq
                            | FullLoopStage::PlcPreamble
                    ),
                })
                .collect(),
            max_events: 10000,
            max_elements_per_event: 16384,
            max_total_elements: 1048576,
            max_retained_bytes: MAX_STAGE_RETENTION_BYTES,
        }
    }
}
/// Immutable validated policy; attaching it creates a fresh request without executing science.
#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct FullLoopObservationPolicy {
    pub(super) input: FullLoopObservationInput,
}
impl FullLoopObservationPolicy {
    /// Validates finite supported stages and budgets before any event/data copies.
    /// # Arguments
    /// * `input` - Owned explicit selections; original scientific producers remain unchanged.
    /// # Returns
    /// Immutable policy; no probe, worker or source is started.
    /// # Errors
    /// Rejects empty/duplicate/unsupported stage IDs or finite work/byte budget violations.
    pub fn new(input: FullLoopObservationInput) -> ApiResult<Self> {
        if input.stages.is_empty()
            || input.stages.len() > 32
            || !(1..=10000).contains(&input.max_events)
            || !(1..=MAX_STAGE_EVENT_ELEMENTS).contains(&input.max_elements_per_event)
            || !(1..=1048576).contains(&input.max_total_elements)
            || !(1..=MAX_STAGE_RETENTION_BYTES).contains(&input.max_retained_bytes)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Stage retention policy exceeds finite budgets",
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for selection in &input.stages {
            if !SUPPORTED.contains(&selection.stage) {
                return Err(ApiError::new(
                    ErrorCode::UnsupportedCapability,
                    "Stage is outside supported full-loop retention",
                ));
            }
            if !seen.insert(selection.stage) || !(1..=64).contains(&selection.max_events) {
                return Err(ApiError::new(
                    ErrorCode::InvalidInput,
                    "Duplicate stage or invalid per-stage event limit",
                ));
            }
        }
        Ok(Self { input })
    }
    /// Admits at most 64 KiB strict JSON through the same typed stage-policy validator.
    /// # Arguments
    /// * `bytes` - Original bounded transport; registered stage IDs use their existing serde names.
    /// # Returns
    /// Immutable policy; recognized unsupported stages retain an explicit error.
    /// # Errors
    /// Rejects malformed/unknown fields and transport/domain/resource violations.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > 65536 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Stage policy exceeds 64 KiB",
            ));
        }
        Self::new(
            serde_json::from_slice(bytes)
                .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?,
        )
    }
    /// Borrows exact accepted selections and budgets without changing a request or run.
    /// # Returns
    /// Immutable policy input; no scientific data is materialized by inspection.
    pub fn settings(&self) -> &FullLoopObservationInput {
        &self.input
    }

    pub(in crate::api::full_loop) fn probe(
        &self,
    ) -> ApiResult<crate::simulation::test_points::TestPointProbe> {
        use crate::simulation::test_points::{
            ProbeConfig, ProbeSelection, RunPhase, TestPointProbe,
        };
        let mut probe = TestPointProbe::new(ProbeConfig {
            selections: self
                .input
                .stages
                .iter()
                .map(|selection| ProbeSelection {
                    stage: selection.stage,
                    elements: 0..self.input.max_elements_per_event,
                    source_symbols: None,
                    channel_symbols: None,
                })
                .collect(),
            max_events: self.input.max_events,
            max_elements_per_event: self.input.max_elements_per_event,
            max_total_elements: self.input.max_total_elements,
        })?;
        for selection in &self.input.stages {
            probe.limit_stage_events(selection.stage, selection.max_events)?;
            probe.set_stage_phase_filter(
                selection.stage,
                selection.observation_only.then_some(RunPhase::Observation),
            )?;
        }
        Ok(probe)
    }
}

impl FullLoopObservationPolicy {
    /// Freezes exact final selection storage without changing accepted budgets.
    /// # Returns
    /// Unit; event/stage/phase selection and every finite limit remain unchanged.
    pub(in crate::api::full_loop) fn normalize_result_storage(&mut self) {
        let Self { input } = self;
        let FullLoopObservationInput {
            stages,
            max_events: _,
            max_elements_per_event: _,
            max_total_elements: _,
            max_retained_bytes: _,
        } = input;
        super::super::result_retention::normalize::vector(stages);
    }
}

impl RetainedPayload for FullLoopStageRetention {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            stage,
            max_events,
            observation_only,
        } = self;
        merlin_api::retention_support::sum([
            stage.retained_children(),
            max_events.retained_children(),
            observation_only.retained_children(),
        ])
    }
}

impl RetainedPayload for FullLoopObservationInput {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            stages,
            max_events,
            max_elements_per_event,
            max_total_elements,
            max_retained_bytes,
        } = self;
        merlin_api::retention_support::sum([
            stages.retained_children(),
            max_events.retained_children(),
            max_elements_per_event.retained_children(),
            max_total_elements.retained_children(),
            max_retained_bytes.retained_children(),
        ])
    }
}

impl RetainedPayload for FullLoopObservationPolicy {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { input } = self;
        merlin_api::retention_support::sum([input.retained_children()])
    }
}
