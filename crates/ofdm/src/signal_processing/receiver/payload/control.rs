/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! NCP hypotheses are formed independently of TX reservation counts and diagnostics.
use super::model::SoftNcpCell;
use crate::{
    signal_processing::modulation::{qam_llrs, randomization::CellRandomizer},
    transmission::packing::NcpModulation,
    types::IqPair,
    LabError,
};
/// Soft-demaps all eligible cells under the known NCP profile and reverses field order.
/// # Arguments
/// * `points` - Ascending received eligible data positions, no pilots or TX field lengths.
/// * `modulation` - Uniform NCP selection from the acquired configuration.
/// * `variance` - Positive finite complex reference variance for LLR scaling.
/// * `randomizer` - Already synchronized NCP-hypothesis stream, clocks once per point.
/// # Returns
/// High-to-low cells with original positions retained for probes and independent field discovery.
/// # Errors
/// Propagates invalid numeric data or randomizer state. Caller discards candidate state on failure.
pub fn hypotheses(
    points: &[IqPair],
    modulation: NcpModulation,
    variance: f64,
    randomizer: &mut CellRandomizer,
) -> Result<Vec<SoftNcpCell>, LabError> {
    if points.len() > crate::transmission::SubcarrierSpacing::Khz25.max_interleaver_cells()
        || points.iter().flatten().any(|value| !value.is_finite())
        || !variance.is_finite()
        || variance <= 0.0
    {
        return Err(crate::error::invalid(
            "Invalid bounded NCP hypothesis input.",
        ));
    }
    let mut output = Vec::with_capacity(points.len());
    for (data_position, point) in points.iter().enumerate() {
        output.push(SoftNcpCell {
            data_position,
            llrs: randomizer.descramble_ncp_llrs(
                modulation.order(),
                &qam_llrs(modulation.order(), *point, variance)?,
            )?,
        });
    }
    output.reverse();
    Ok(output)
}
