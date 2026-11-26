/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact owned query storage; no observed value or source allocation is changed.
use super::{FullLoopStageContext, FullLoopStageData, FullLoopStageSnapshot};
use crate::api::full_loop::result_retention::normalize::{text, vector};

/// Moves one unchanged snapshot into exact final owned capacities.
///
/// # Arguments
/// * `snapshot` - Current query or final retained event; original coordinates survive.
///
/// # Returns
/// Unit with text/vector capacity equal to length. Temporary builders remain
/// governed by existing finite query/retention limits, separately from the final
/// returned owner. A query copies a subset of one source event, so its exact
/// final child storage cannot exceed that source event's charged capacities.
pub(super) fn normalize_snapshot(snapshot: &mut FullLoopStageSnapshot) {
    let FullLoopStageSnapshot {
        stage: _,
        retained_index: _,
        direction: _,
        context,
        domain: _,
        units: _,
        element_start: _,
        total_elements: _,
        data,
    } = snapshot;
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
    } = context;
    text(run_id);
    text(configuration_id);
    if let Some(roles) = transmit_cell_roles {
        vector(roles);
    }
    match data {
        FullLoopStageData::Bits(values) | FullLoopStageData::Bytes(values) => vector(values),
        FullLoopStageData::Iq(values) => vector(values),
        FullLoopStageData::OptionalIq(values) => vector(values),
        FullLoopStageData::QamLabels(values) => vector(values),
        FullLoopStageData::TransportCells(values) => vector(values),
        FullLoopStageData::Pilots(values) => vector(values),
        FullLoopStageData::Llrs(values) => vector(values),
    }
}
