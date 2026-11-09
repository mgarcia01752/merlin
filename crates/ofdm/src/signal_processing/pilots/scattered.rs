/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Scattered-pilot signals generated after interleaving at physical pattern locations.
use super::{model::select_values, PilotPower, PilotSequence, PilotValue};
use crate::{
    error::invalid,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    SimulationError,
};
/// Generates phase-aligned scattered-pilot BPSK values on the physical grid.
/// # Arguments
/// * `layout` - Existing validated physical roles, with CP/PLC/exclusions taking priority.
/// * `sequence` - Full physical-bin sequence; clocking does not skip protected bins.
/// * `power` - Validated pilot boost, not a data/NCP randomizer setting.
/// * `symbol_index` -0 is first post-PLC-preamble symbol; locations repeat128.
/// # Returns
/// Owned increasing-bin values for actual scattered-pilot positions only.
/// This does not apply physical frame insertion or alter interleaved payload cells.
/// # Errors
/// Rejects mismatched numerology or invalid sequence/scaling operations.
pub fn generate_scattered(
    layout: &CarrierLayout,
    sequence: &PilotSequence,
    power: PilotPower,
    symbol_index: u64,
) -> Result<Vec<PilotValue>, SimulationError> {
    if layout.subcarrier_spacing() != sequence.spacing() {
        return Err(invalid(
            "Scattered pilot layout/sequence numerology mismatch.",
        ));
    }
    select_values(
        &layout.symbol_roles(symbol_index),
        PhysicalCarrierRole::ScatteredPilot,
        sequence,
        power,
    )
}
