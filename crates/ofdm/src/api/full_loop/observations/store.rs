/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive bounded copies; existing dispatcher owns scientific stage selection and validation.
use super::*;
use crate::simulation::test_points::{ProbeEvent, ProbeObserver};
use serde::Serialize;
use std::collections::BTreeMap;

/// Actual retained event, or an owned bounded contiguous window of that event.
#[derive(Clone, Serialize)]
#[non_exhaustive]
pub struct FullLoopStageSnapshot {
    pub stage: FullLoopStage,
    pub retained_index: usize,
    pub direction: SignalDirection,
    pub context: FullLoopStageContext,
    pub domain: SignalDomain,
    pub units: ProbeUnits,
    pub element_start: usize,
    pub total_elements: usize,
    pub data: FullLoopStageData,
}
/// Actual retained availability; absence is never filled with ideal/generated placeholders.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopStageInfo {
    pub stage: FullLoopStage,
    pub snapshots: usize,
    pub retained_elements: usize,
}
/// Owner-accounted payload/context bytes, excluding allocator/engine buffers.
#[derive(Clone, Default, Serialize)]
#[non_exhaustive]
pub struct FullLoopObservationStats {
    pub retained_events: usize,
    pub retained_elements: usize,
    pub retained_bytes: usize,
    pub dropped_events: usize,
    pub byte_budget_exhausted: bool,
}
/// Contiguous selected event-element viewport; zero count copies up to 4096 remaining values.
#[derive(Clone, Copy, Default)]
pub struct FullLoopStageViewport {
    pub start: usize,
    pub count: usize,
}

#[derive(Default)]
pub(in crate::api::full_loop) struct RetainedStages {
    events: BTreeMap<FullLoopStage, Vec<FullLoopStageSnapshot>>,
    pub(in crate::api::full_loop) stats: FullLoopObservationStats,
    budget: usize,
}
impl RetainedStages {
    pub(in crate::api::full_loop) fn events(
        &self,
        stage: FullLoopStage,
    ) -> ApiResult<&[FullLoopStageSnapshot]> {
        self.events.get(&stage).map(Vec::as_slice).ok_or_else(|| {
            ApiError::new(ErrorCode::MissingCoverage, "No actual stage data retained")
        })
    }

    pub(in crate::api::full_loop) fn new(budget: usize) -> Self {
        Self {
            budget,
            ..Default::default()
        }
    }
    pub(super) fn inventory(&self) -> Vec<FullLoopStageInfo> {
        self.events
            .iter()
            .map(|(stage, events)| FullLoopStageInfo {
                stage: *stage,
                snapshots: events.len(),
                retained_elements: events.iter().map(|event| event.data.len()).sum(),
            })
            .collect()
    }
    pub(super) fn query(
        &self,
        stage: FullLoopStage,
        snapshot: usize,
        viewport: FullLoopStageViewport,
    ) -> ApiResult<FullLoopStageSnapshot> {
        let events = self.events.get(&stage).ok_or_else(|| {
            ApiError::new(ErrorCode::MissingCoverage, "No actual stage data retained")
        })?;
        let event = events.get(snapshot).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Retained stage event is unavailable",
            )
        })?;
        let remaining = event
            .data
            .len()
            .checked_sub(viewport.start)
            .filter(|n| *n > 0)
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidInput,
                    "Stage start is outside retained values",
                )
            })?;
        let count = if viewport.count == 0 {
            remaining.min(MAX_STAGE_QUERY_ELEMENTS)
        } else {
            viewport.count
        };
        if count > MAX_STAGE_QUERY_ELEMENTS {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Stage query exceeds 4096 elements",
            ));
        }
        if count > remaining {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Stage viewport is outside retained values",
            ));
        }
        Ok(FullLoopStageSnapshot {
            stage,
            retained_index: event.retained_index,
            direction: event.direction,
            context: event.context.clone(),
            domain: event.domain,
            units: event.units,
            element_start: event.element_start + viewport.start,
            total_elements: event.total_elements,
            data: event.data.window(viewport.start, viewport.start + count),
        })
    }
}
impl ProbeObserver for RetainedStages {
    /// Copies one already validated selected event only if owner-accounted bytes fit.
    /// # Arguments
    /// * `event` - Borrowed actual core values/metadata, valid for this callback only.
    /// # Returns
    /// Unit; byte exhaustion drops copies explicitly without failing scientific work.
    /// # Errors
    /// Rejects an unexpected producer representation as an implementation mismatch.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), crate::SimulationError> {
        let bytes = FullLoopStageData::bytes(event.data).ok_or_else(|| {
            crate::SimulationError::new(
                "UnsupportedStageData",
                "Unexpected selected producer representation",
            )
        })?;
        let role_bytes = event
            .context
            .transmit_cell_roles
            .as_ref()
            .map_or(0, |roles| std::mem::size_of_val(roles.as_slice()));
        let bytes = bytes
            .saturating_add(std::mem::size_of::<FullLoopStageSnapshot>())
            .saturating_add(role_bytes)
            .saturating_add(event.context.run_id.len())
            .saturating_add(event.context.configuration_id.len())
            .saturating_add(256);
        if self
            .stats
            .retained_bytes
            .checked_add(bytes)
            .is_none_or(|size| size > self.budget)
        {
            self.stats.dropped_events += 1;
            self.stats.byte_budget_exhausted = true;
            return Ok(());
        }
        let data = FullLoopStageData::copy(event.data).ok_or_else(|| {
            crate::SimulationError::new(
                "UnsupportedStageData",
                "Unexpected selected producer representation",
            )
        })?;
        let values = data.len();
        let entries = self.events.entry(event.stage).or_default();
        entries.push(FullLoopStageSnapshot {
            stage: event.stage,
            retained_index: entries.len(),
            direction: event.direction,
            context: event.context.clone(),
            domain: event.domain,
            units: event.units,
            element_start: event.element_start,
            total_elements: event.total_elements,
            data,
        });
        self.stats.retained_events += 1;
        self.stats.retained_elements += values;
        self.stats.retained_bytes += bytes;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::test_points::{
        PhysicalProbeGrid, ProbeContext, ProbeData, RunPhase, SignalReference,
    };

    #[test]
    fn stage_retention_preserves_missing_explicit_zero_and_original_offset_as_owned_data() {
        let mut values = [None, Some([0.0, 0.0]), Some([1.0, -1.0])];
        let mut context = ProbeContext::new(
            "synthetic-retention-owner",
            "independent-values",
            RunPhase::Observation,
        )
        .unwrap();
        context.physical_grid = Some(PhysicalProbeGrid {
            spacing: SubcarrierSpacing::Khz25,
            zero_frequency_hz: 631100000,
        });
        context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
        let event = ProbeEvent {
            stage: FullLoopStage::PhysicalBins,
            direction: SignalDirection::Transmit,
            context: &context,
            domain: SignalDomain::PhysicalFftBin,
            units: ProbeUnits::NormalizedIq,
            element_start: 12,
            total_elements: 99,
            shape: [3],
            data: ProbeData::OptionalIq(&values),
        };
        let mut retained = RetainedStages::new(65536);
        retained.observe(&event).unwrap();
        values[1] = Some([9.0, 9.0]);
        let view = retained
            .query(
                FullLoopStage::PhysicalBins,
                0,
                FullLoopStageViewport::default(),
            )
            .unwrap();
        assert_eq!(view.element_start, 12);
        assert_eq!(view.total_elements, 99);
        if let FullLoopStageData::OptionalIq(data) = view.data {
            assert_eq!(data, [None, Some([0.0, 0.0]), Some([1.0, -1.0])]);
        } else {
            panic!("Typed optional I/Q must be preserved");
        }
        let view = retained
            .query(
                FullLoopStage::PhysicalBins,
                0,
                FullLoopStageViewport { start: 1, count: 2 },
            )
            .unwrap();
        assert_eq!(view.element_start, 13);
    }
}
