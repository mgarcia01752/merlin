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
    SimulationError,
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
) -> Result<Vec<SoftNcpCell>, SimulationError> {
    hypotheses_with_variances(
        points,
        modulation,
        &vec![variance; points.len()],
        randomizer,
    )
}
/// Builds NCP hypotheses with the variance attached to each received data position.
/// # Arguments
/// * `points` - Eligible source-order cells.
/// * `modulation` - Received NCP profile.
/// * `variances` - Positive finite post-equalization complex variances, one per point.
/// * `randomizer` - Synchronized hypothesis state, clocked once per point.
/// # Returns
/// Reverse-order NCP soft cells with original positions retained.
/// # Errors
/// Rejects shape/numeric errors or randomizer failures.
pub fn hypotheses_with_variances(
    points: &[IqPair],
    modulation: NcpModulation,
    variances: &[f64],
    randomizer: &mut CellRandomizer,
) -> Result<Vec<SoftNcpCell>, SimulationError> {
    if points.len() > crate::transmission::SubcarrierSpacing::Khz25.max_interleaver_cells()
        || points.iter().flatten().any(|value| !value.is_finite())
        || variances.len() != points.len()
        || variances
            .iter()
            .any(|variance| !variance.is_finite() || *variance <= 0.0)
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
                &qam_llrs(modulation.order(), *point, variances[data_position])?,
            )?,
        });
    }
    output.reverse();
    Ok(output)
}
