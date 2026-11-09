/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed pilot outputs and common selection/scaling, never frame assembly.
use super::{PilotBit, PilotPower, PilotSequence};
use crate::{
    error::invalid,
    signal_processing::modulation::map_symbol,
    transmission::layout::PhysicalCarrierRole,
    types::{IqPair, SubcarrierIdx},
    SimulationError,
};
use serde::Serialize;
/// One actual pilot signal at a physical FFT bin, in increasing-bin output order.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PilotValue {
    pub bin: SubcarrierIdx,
    pub sequence_bit: PilotBit,
    pub iq: IqPair,
}

/// Selects one physical pilot role and applies the shared sequence/map/scale stages.
/// # Arguments
/// * `roles` - Full physical FFT roles for one symbol phase.
/// * `kind` - Continuous or scattered role chosen by the public producer.
/// * `sequence` - Matching physical FFT sequence including protected bins.
/// * `power` - Validated amplitude reference.
/// # Returns
/// Owned increasing-bin pilot records without changing any input.
/// # Errors
/// Rejects invalid bin representation, sequence coverage or scaling.
pub(super) fn select_values(
    roles: &[PhysicalCarrierRole],
    kind: PhysicalCarrierRole,
    sequence: &PilotSequence,
    power: PilotPower,
) -> Result<Vec<PilotValue>, SimulationError> {
    let mut output = Vec::new();
    for (index, role) in roles.iter().enumerate() {
        if *role == kind {
            let bin = SubcarrierIdx::try_from(index)
                .map_err(|_| invalid("Pilot bin exceeds index representation."))?;
            let bit = sequence
                .bit_at(bin)
                .ok_or_else(|| invalid("Pilot sequence does not cover the physical grid."))?;
            let iq = power.scale(map_symbol(2, u16::from(bit))?)?;
            output.push(PilotValue {
                bin,
                sequence_bit: bit,
                iq,
            });
        }
    }
    Ok(output)
}
