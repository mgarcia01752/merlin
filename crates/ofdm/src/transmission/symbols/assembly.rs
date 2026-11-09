/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Converts validated packer plans to actual NCP, randomized data/filler and pilot placeholders.
use super::{AssembledSymbol, TransportCell};
use crate::{
    error::invalid,
    signal_processing::modulation::{map_symbol, randomization::CellRandomizer},
    transmission::{
        ncp::NcpFieldCodec,
        packing::{PackedCell, PackedSymbol},
    },
    SimulationError,
};
/// Encodes control signaling and maps one source symbol without publishing diagnostic IDs.
/// # Arguments
/// * `symbol` - Bounded fixed256-QAM packer plan; labels precede cell randomization.
/// * `control` - Reusable independent NCP encoder.
/// * `randomizer` - Continuous source-symbol state starting at local0, not reset per word.
/// # Returns
/// NCP field and actual mapped cells; pilots stay typed placeholders. Randomizer
/// commits only after successful validation/assembly. TX diagnostic IDs do not enter cells.
/// # Errors
/// Rejects malformed plan/labels, invalid control reservation or randomizer sequence;
/// the supplied stream state is unchanged on rejection.
pub fn assemble_packed(
    symbol: &PackedSymbol,
    control: &NcpFieldCodec,
    randomizer: &mut CellRandomizer,
) -> Result<AssembledSymbol, SimulationError> {
    assemble_packed_with_flags(
        symbol,
        control,
        randomizer,
        crate::transmission::ncp::activation::NcpFieldFlags::default(),
    )
}
/// Assembles actual cells with explicit NCP generation flags and continuous randomization.
/// # Arguments
/// * `symbol` - Uniform-profile source plan.
/// * `control` - Shared NCP codec.
/// * `randomizer` - Caller-owned continuous cell state.
/// * `flags` - Session-validated C/N/U flags.
/// # Returns
/// Actual encoded/mapped cells; commits randomizer only on success.
/// # Errors
/// Rejects malformed reservations, labels or randomizer sequencing.
pub fn assemble_packed_with_flags(
    symbol: &PackedSymbol,
    control: &NcpFieldCodec,
    randomizer: &mut CellRandomizer,
    flags: crate::transmission::ncp::activation::NcpFieldFlags,
) -> Result<AssembledSymbol, SimulationError> {
    let field = control.encode_packed_with_flags(symbol, flags)?;
    if symbol
        .cells
        .iter()
        .any(|cell| matches!(cell,PackedCell::Data {label,..} if *label>=crate::transmission::symbols::PAYLOAD_ORDER))
    {
        return Err(invalid("Payload assembly requires valid256-QAM labels."));
    }
    let mut candidate = randomizer.clone();
    candidate.begin_symbol(symbol.symbol_index)?;
    let mut cells = Vec::with_capacity(symbol.cells.len());
    for cell in &symbol.cells {
        cells.push(Some(match cell {
            PackedCell::BitData { label, order, .. } => {
                if *order == 2 {
                    TransportCell::Sample(map_symbol(2, candidate.zero_loaded_bpsk()?)?)
                } else {
                    TransportCell::Sample(map_symbol(
                        *order,
                        candidate.apply_data(*order, *label)?,
                    )?)
                }
            }
            PackedCell::Data { label, .. } => TransportCell::Sample(map_symbol(
                crate::transmission::symbols::PAYLOAD_ORDER,
                candidate.apply_data(crate::transmission::symbols::PAYLOAD_ORDER, *label)?,
            )?),
            PackedCell::NcpReserved { field_cell } => {
                let order = symbol.ncp_modulation.order();
                TransportCell::Sample(map_symbol(
                    order,
                    candidate.apply_ncp(order, field.labels[*field_cell])?,
                )?)
            }
            PackedCell::ZeroLoadedFiller => {
                TransportCell::Sample(map_symbol(2, candidate.zero_loaded_bpsk()?)?)
            }
            PackedCell::ScatteredPilotPlaceholder => {
                candidate.skip_scattered_pilot()?;
                TransportCell::PilotPlaceholder
            }
        }));
    }
    *randomizer = candidate;
    Ok(AssembledSymbol {
        local_symbol_index: symbol.symbol_index,
        ncp: field,
        cells,
    })
}
