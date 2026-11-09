/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Continuous-pilot signals at existing static physical layout positions.
use super::{model::select_values, PilotPower, PilotSequence, PilotValue};
use crate::{
    error::invalid,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    LabError,
};
/// Generates continuous BPSK pilot values without moving or generating locations.
/// # Arguments
/// * `layout` - Existing validated continuous/PLC/excluded geometry.
/// * `sequence` - Full physical-bin sequence of the same numerology.
/// * `power` - Validated pilot boost relative to other-subcarrier RMS.
/// # Returns
/// Owned increasing-bin signals, including predefined and caller-supplied pilots.
/// Inputs remain unchanged; values do not vary with OFDM-symbol phase.
/// # Errors
/// Rejects mismatched numerology or invalid sequence/scaling operations.
pub fn generate_continuous(
    layout: &CarrierLayout,
    sequence: &PilotSequence,
    power: PilotPower,
) -> Result<Vec<PilotValue>, LabError> {
    if layout.subcarrier_spacing() != sequence.spacing() {
        return Err(invalid(
            "Continuous pilot layout/sequence numerology mismatch.",
        ));
    }
    select_values(
        &layout.symbol_roles(0),
        PhysicalCarrierRole::ContinuousPilot,
        sequence,
        power,
    )
}
