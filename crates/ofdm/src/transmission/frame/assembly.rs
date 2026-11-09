/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Physical protected-role insertion; no frequency reordering, IFFT or invented history.
use crate::{
    error::invalid,
    signal_processing::pilots::{
        generate_continuous, generate_scattered, PilotPower, PilotSequence,
    },
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    types::IqPair,
    SimulationError,
};
use serde::Serialize;
/// Full increasing-frequency PHY grid. None remains an unavailable history cell.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PhysicalFrequencySymbol {
    pub symbol_index: u64,
    pub spacing: crate::transmission::SubcarrierSpacing,
    pub zero_frequency_hz: crate::types::FrequencyHz,
    pub reference_rms: f64,
    pub roles: Vec<PhysicalCarrierRole>,
    pub bins: Vec<Option<IqPair>>,
}
/// Combines post-cell-interleaving data/NCP, PLC and pilots into protected FFT bins.
/// # Arguments
/// * `layout` - Validated static physical map.
/// * `symbol_index` - PLC-aligned output channel index, including preamble phases.
/// * `reduced` - Ascending data/scattered-bin rank. Scattered placeholders must
///   be None; data None preserves unavailable startup/drain history.
/// * `plc` - One ascending PLC column, normalized data16-QAM or preamble BPSK.
/// * `reference_rms` - Positive finite amplitude applied to all normalized cells.
/// # Returns
/// Owned FFT-sized grid: excluded bins are zero, pilots boosted2x; inputs unchanged.
/// # Errors
/// Rejects counts, occupied scattered placeholders, nonfinite/overflowing cells
/// or invalid reference; never moves cells to fill a gap.
pub fn assemble(
    layout: &CarrierLayout,
    symbol_index: u64,
    reduced: &[Option<IqPair>],
    plc: &[IqPair],
    reference_rms: f64,
) -> Result<PhysicalFrequencySymbol, SimulationError> {
    let roles = layout.symbol_roles(symbol_index);
    let reduced_count = roles
        .iter()
        .filter(|role| {
            matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            )
        })
        .count();
    let plc_count = roles
        .iter()
        .filter(|role| **role == PhysicalCarrierRole::Plc)
        .count();
    if reduced.len() != reduced_count || plc.len() != plc_count {
        return Err(invalid(
            "Physical assembly input counts do not match protected layout.",
        ));
    }
    let power = PilotPower::new(reference_rms)?;
    let scale = |sample: IqPair| -> Result<IqPair, SimulationError> {
        let scaled = sample.map(|value| value * reference_rms);
        if sample
            .iter()
            .chain(scaled.iter())
            .any(|value| !value.is_finite())
        {
            return Err(invalid(
                "Physical assembly requires finite, nonoverflowing I/Q.",
            ));
        }
        Ok(scaled)
    };
    let mut bins = vec![Some([0.0, 0.0]); roles.len()];
    let mut rank = 0;
    let mut plc_rank = 0;
    for (bin, role) in roles.iter().enumerate() {
        match role {
            PhysicalCarrierRole::Data => {
                bins[bin] = reduced[rank].map(scale).transpose()?;
                rank += 1;
            }
            PhysicalCarrierRole::ScatteredPilot => {
                if reduced[rank].is_some() {
                    return Err(invalid(
                        "Scattered pilot requires an empty post-interleaver placeholder.",
                    ));
                }
                rank += 1;
            }
            PhysicalCarrierRole::Plc => {
                bins[bin] = Some(scale(plc[plc_rank])?);
                plc_rank += 1;
            }
            _ => {}
        }
    }
    let sequence = PilotSequence::new(layout.subcarrier_spacing());
    for pilot in generate_continuous(layout, &sequence, power)?
        .into_iter()
        .chain(generate_scattered(layout, &sequence, power, symbol_index)?)
    {
        bins[usize::from(pilot.bin)] = Some(pilot.iq);
    }
    Ok(PhysicalFrequencySymbol {
        symbol_index,
        spacing: layout.subcarrier_spacing(),
        zero_frequency_hz: layout
            .frequency_hz(0)
            .ok_or_else(|| invalid("Missing physical FFT origin."))?,
        reference_rms,
        roles,
        bins,
    })
}
