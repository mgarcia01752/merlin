/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Individually admitted immutable public queries and same-event exact concatenation.
use super::{exact, text, Data, ObservationError};
use crate::shell::{Retained, RetainedResources, RetentionBudget};
use merlin::downstream::{
    ApiError, ErrorCode, FullLoopResult, FullLoopStage as Stage, FullLoopStageContext,
    FullLoopStageInfo, FullLoopStageSnapshot, FullLoopStageViewport, MAX_STAGE_QUERY_ELEMENTS,
};
use std::mem::size_of;

pub(super) struct Queries<'result> {
    result: &'result FullLoopResult,
    inventory: Retained<Vec<FullLoopStageInfo>>,

    // Distinct live owners during replacement: accumulator, page, new concatenation.
    // Each slot reserves a full source envelope; this is not a growth multiplier.
    _accumulator: Retained<()>,
    _page: Retained<()>,
    _replacement: Retained<()>,
}

impl<'result> Queries<'result> {
    /// Admits inventory and every potentially overlapping owned snapshot before querying.
    ///
    /// # Arguments
    /// * `result` - Immutable source whose public query envelope covers one exact return.
    /// * `budget` - Same aggregate counter as the already charged source and final target.
    ///
    /// # Returns
    /// Fixed-slot query workspace retained through both projection passes.
    ///
    /// # Errors
    /// Metadata/checked-layout/policy failures leave the source untouched and release slots.
    pub(super) fn new(
        result: &'result FullLoopResult,
        budget: &RetentionBudget,
    ) -> Result<Self, ObservationError> {
        let source = result
            .retained_footprint()
            .map_err(ObservationError::Backend)?;
        let inventory = budget
            .reserve_resources(inventory_resources(source.payload_bytes)?)
            .map_err(ObservationError::Retention)?;
        let snapshot = snapshot_resources(source.payload_bytes)?;
        let accumulator = budget
            .reserve_resources(snapshot)
            .map_err(ObservationError::Retention)?;
        let page = budget
            .reserve_resources(snapshot)
            .map_err(ObservationError::Retention)?;
        let replacement = budget
            .reserve_resources(snapshot)
            .map_err(ObservationError::Retention)?;
        Ok(Self {
            result,
            inventory: inventory.retain(result.stage_inventory()),
            _accumulator: accumulator.retain(()),
            _page: page.retain(()),
            _replacement: replacement.retain(()),
        })
    }

    /// Visits scoped events in original order; no query payload can escape its slots.
    ///
    /// # Arguments
    /// * `observe` - Borrow-only planner or builder; original query errors become warnings.
    ///
    /// # Returns
    /// Unit after every original retained index was visited, including unavailable events.
    ///
    /// # Errors
    /// Propagates planner failures without changing the scientific source.
    pub(super) fn visit(
        &self,
        mut observe: impl FnMut(
            Result<&FullLoopStageSnapshot, &ApiError>,
        ) -> Result<(), ObservationError>,
    ) -> Result<(), ObservationError> {
        for info in self.inventory.value() {
            for index in 0..info.snapshots {
                match self.snapshot(info.stage, index) {
                    Ok(event) => observe(Ok(&event))?,
                    Err(error) => observe(Err(&error))?,
                }
            }
        }
        Ok(())
    }

    fn snapshot(&self, stage: Stage, index: usize) -> Result<FullLoopStageSnapshot, ApiError> {
        let mut event = self
            .result
            .stage(stage, index, FullLoopStageViewport::default())?;
        let cap = self
            .result
            .observation_policy()
            .map(|policy| policy.settings().max_elements_per_event)
            .unwrap_or(event.data.len());
        let mut page_len = event.data.len();
        while page_len == MAX_STAGE_QUERY_ELEMENTS && event.data.len() < cap {
            let page = self.result.stage(
                stage,
                index,
                FullLoopStageViewport {
                    start: event.data.len(),
                    count: 0,
                },
            );
            match page {
                Ok(page) => {
                    page_len = page.data.len();

                    // The new final snapshot is a prefix of this same immutable source
                    // event, with its identical first context. All eight leaf types are
                    // fixed size. Exact concatenation therefore fits one source envelope.
                    // Old accumulator + page + replacement remain separately reserved.
                    let mut replacement = event.clone();
                    append(&mut replacement.data, page.data)?;
                    normalize(&mut replacement)?;
                    event = replacement;
                }
                Err(error) if error.code() == ErrorCode::InvalidInput => break,
                Err(error) => return Err(error),
            }
        }
        Ok(event)
    }
}

fn unsupported() -> ApiError {
    ApiError::new(
        ErrorCode::UnsupportedCapability,
        "Unsupported public stage representation during presentation paging",
    )
}

fn append(target: &mut Data, source: Data) -> Result<(), ApiError> {
    match (target, source) {
        (Data::Bits(a), Data::Bits(b)) | (Data::Bytes(a), Data::Bytes(b)) => a.extend(b),
        (Data::Iq(a), Data::Iq(b)) => a.extend(b),
        (Data::OptionalIq(a), Data::OptionalIq(b)) => a.extend(b),
        (Data::QamLabels(a), Data::QamLabels(b)) => a.extend(b),
        (Data::TransportCells(a), Data::TransportCells(b)) => a.extend(b),
        (Data::Pilots(a), Data::Pilots(b)) => a.extend(b),
        (Data::Llrs(a), Data::Llrs(b)) => a.extend(b),
        _ => return Err(unsupported()),
    }
    Ok(())
}

// Snapshot is externally non_exhaustive. This audit covers its current public
// fields: context and data own all children; other current fields are fixed.
// A future owned field requires revisiting this consumer's concatenation proof.
fn normalize(snapshot: &mut FullLoopStageSnapshot) -> Result<(), ApiError> {
    let FullLoopStageContext {
        run_id,
        configuration_id,
        phase: _,
        source_symbol_index: _,
        channel_symbol_index: _,
        sample_start_index: _,
        codeword_index: _,
        profile_id: _,
        plc_phase: _,
        reference: _,
        plc_grid: _,
        physical_grid: _,
        waveform_grid: _,
        cell_grid: _,
        transmit_cell_roles,
    } = &mut snapshot.context;
    text(run_id);
    text(configuration_id);
    if let Some(roles) = transmit_cell_roles {
        exact(roles);
    }
    match &mut snapshot.data {
        Data::Bits(v) | Data::Bytes(v) => exact(v),
        Data::Iq(v) => exact(v),
        Data::OptionalIq(v) => exact(v),
        Data::QamLabels(v) => exact(v),
        Data::TransportCells(v) => exact(v),
        Data::Pilots(v) => exact(v),
        Data::Llrs(v) => exact(v),
        _ => return Err(unsupported()),
    }
    Ok(())
}

fn inventory_resources(source: usize) -> Result<RetainedResources, ObservationError> {
    let wrapper = size_of::<Retained<Vec<FullLoopStageInfo>>>()
        .checked_sub(size_of::<Vec<FullLoopStageInfo>>())
        .ok_or(ObservationError::Retention(
            crate::shell::RetentionError::ArithmeticOverflow,
        ))?;
    Ok(RetainedResources {
        payload_bytes: source
            .checked_add(wrapper)
            .ok_or(ObservationError::Retention(
                crate::shell::RetentionError::ArithmeticOverflow,
            ))?,
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    })
}

fn snapshot_resources(source: usize) -> Result<RetainedResources, ObservationError> {
    Ok(RetainedResources {
        payload_bytes: source.checked_add(size_of::<Retained<()>>()).ok_or(
            ObservationError::Retention(crate::shell::RetentionError::ArithmeticOverflow),
        )?,
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests;
