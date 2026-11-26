/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed source/window and owned original-cell/result metadata.
use super::*;
use serde::{Deserialize, Serialize};

/// Actual received role, independent of transport profile and display stage.
#[derive(Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FullLoopCellStream {
    #[default]
    Payload,
    Ncp,
    PlcData,
    PlcPreamble,
    ContinuousPilots,
    ScatteredPilots,
}
/// Immutable-result query; coordinate end bounds are inclusive and None means unbounded.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopCellQuery {
    pub stream: FullLoopCellStream,
    pub profile_id: Option<u8>,
    /// None selects the first actually available order after the profile filter.
    pub modulation_order: Option<u16>,
    pub first_bin: Option<u16>,
    pub last_bin: Option<u16>,
    pub first_symbol: Option<u64>,
    pub last_symbol: Option<u64>,
    /// Offset inside the filtered original population, never a physical-bin number.
    pub start: usize,
    /// Zero copies up to the operation's cap; qam permits 2048 and cell_decisions permits 128.
    pub count: usize,
}
/// One actual matched physical receiver cell; all I/Q stays in original producer coordinates.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCell {
    pub physical_bin: u16,
    pub frequency_hz: u32,
    pub channel_symbol_index: u64,
    pub source_symbol_index: Option<u64>,
    pub profile_id: Option<u8>,
    pub modulation_order: u16,
    pub reference_amplitude: f64,
    pub before_equalization: [f64; 2],
    pub after_equalization: [f64; 2],
    pub reference_iq: Option<[f64; 2]>,
    /// Actual post-descrambling receiver values; protected streams have none.
    pub llrs: Vec<f64>,
}
/// Original available-coordinate bounds, before profile/order/window filtering.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCellBounds {
    pub first_bin: u16,
    pub last_bin: u16,
    pub first_symbol: u64,
    pub last_symbol: u64,
}
/// Original retained population, validated filters and actual display window accounting.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCellSelection {
    pub query: FullLoopCellQuery,
    pub retained_cells: usize,
    pub selected_cells: usize,
    pub available_profiles: Vec<u8>,
    pub available_orders: Vec<u16>,
    pub selected_order: Option<u16>,
    pub bounds: Option<FullLoopCellBounds>,
    pub display_start: usize,
    pub displayed_cells: usize,
}
/// Scientific metrics over every selected retained cell, separate from display pagination.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopQamView {
    pub selection: FullLoopCellSelection,
    pub receiver_mode: ReceiverMode,
    pub raw_reference_plane: &'static str,
    pub equalized_reference_plane: &'static str,
    pub cells: Vec<FullLoopCell>,
    pub raw_nearest_mer: FullLoopCellMer,
    pub equalized_nearest_mer: FullLoopCellMer,
    pub raw_tx_reference_mer: Option<FullLoopCellMer>,
    pub equalized_tx_reference_mer: Option<FullLoopCellMer>,
}
/// A diagnostic physical decision; production FEC continues to use its original soft values.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCellDecisionRow {
    pub cell: FullLoopCell,
    pub decision: FullLoopCellDecision,
}
/// Bounded physical decisions and statistics over explicit actual selected-cell soft data.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopCellDecisions {
    pub selection: FullLoopCellSelection,
    pub rows: Vec<FullLoopCellDecisionRow>,
    pub llr_first_selected_cell: usize,
    pub llr_selected_cells: usize,
    pub llr_statistics: FullLoopLlrStatistics,
}
