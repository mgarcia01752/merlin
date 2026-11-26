/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual transport cells and retained TX packing provenance, independent of I/Q geometry.
use super::*;
use crate::simulation::payload::role_projection::first_symbol_roles;
use serde::Serialize;

/// Original typed transport value; unknown role and unavailable delay history remain distinct.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopTransmitCell {
    pub cell_index: usize,
    pub value: Option<FullLoopTransportCell>,
    pub role: Option<FullLoopCellRole>,
}
/// Bounded original transmitted-cell window; role projection never changes producer state.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopTransmitCells {
    pub stage: FullLoopStage,
    pub retained_index: usize,
    pub context: FullLoopStageContext,
    pub element_start: usize,
    pub total_elements: usize,
    pub retained_elements: usize,
    pub role_source: &'static str,
    pub cells: Vec<FullLoopTransmitCell>,
}

impl FullLoopResult {
    /// Copies actual mapped/interleaved transport cells with proven optional TX packing roles.
    /// # Arguments
    /// * `stage` - PayloadMapped or PayloadInterleaved, distinct from physical FFT bins.
    /// * `snapshot` - Actual retained event, never regenerated later history.
    /// * `start` - Offset into retained cells; absolute cell indices remain in the result.
    /// * `count` - Positive count, or zero for remaining cells, at most 8192.
    /// # Returns
    /// Typed original cells with actual mapped roles or first-aligned-symbol role transport.
    /// Later or insufficient provenance remains unknown; no role is inferred from I/Q.
    /// # Errors
    /// Rejects wrong stages, absent observations, invalid windows and inconsistent metadata.
    pub fn transmit_cells(
        &self,
        stage: FullLoopStage,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> ApiResult<FullLoopTransmitCells> {
        if !matches!(
            stage,
            FullLoopStage::PayloadMapped | FullLoopStage::PayloadInterleaved
        ) {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Transmit cells require mapped or interleaved cells",
            ));
        }
        super::super::physical::check_count(count)?;
        let event = self.stages.events(stage)?.get(snapshot).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Retained transmit event is unavailable",
            )
        })?;
        let FullLoopStageData::TransportCells(values) = &event.data else {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Transmit event has incompatible data kind",
            ));
        };
        let remaining = values
            .len()
            .checked_sub(start)
            .filter(|n| *n > 0)
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidInput,
                    "Transmit cell start is outside retained values",
                )
            })?;
        let count = if count == 0 { remaining } else { count };
        super::super::physical::check_count(count)?;
        if count > remaining {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Transmit cell window is outside retained values",
            ));
        }
        let first = event.element_start.checked_add(start).ok_or_else(|| {
            ApiError::new(ErrorCode::InvalidInput, "Transmit cell origin overflows")
        })?;
        let (roles, role_source) = if stage == FullLoopStage::PayloadMapped {
            (
                event
                    .context
                    .transmit_cell_roles
                    .as_ref()
                    .map(|items| items.iter().copied().map(Some).collect()),
                "actual_mapped_packing",
            )
        } else {
            let roles = self
                .stages
                .events(FullLoopStage::PayloadMapped)
                .ok()
                .and_then(|events| events.first())
                .map(|source| {
                    first_symbol_roles(&source.context, source.element_start, &event.context)
                })
                .transpose()?
                .flatten();
            let origin = if roles.is_some() {
                "first_aligned_source_metadata"
            } else {
                "unavailable"
            };
            (roles, origin)
        };
        let role_source = if roles.is_some() {
            role_source
        } else {
            "unavailable"
        };
        let cells = values[start..start + count]
            .iter()
            .enumerate()
            .map(|(offset, value)| {
                let cell_index = first + offset;
                FullLoopTransmitCell {
                    cell_index,
                    value: value.clone(),
                    role: roles
                        .as_ref()
                        .and_then(|items| items.get(cell_index))
                        .copied()
                        .flatten(),
                }
            })
            .collect();
        Ok(FullLoopTransmitCells {
            stage,
            retained_index: snapshot,
            context: event.context.clone(),
            element_start: first,
            total_elements: event.total_elements,
            retained_elements: values.len(),
            role_source,
            cells,
        })
    }
}
