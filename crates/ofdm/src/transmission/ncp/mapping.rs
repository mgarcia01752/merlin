/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.14.3, PDF162–163: reverse each retained-coordinate group.
use crate::{
    error::invalid, signal_processing::modulation::SymbolLabel,
    transmission::packing::NcpModulation, LabError,
};
use fec_codec::ncp::NCP_CODEWORD_BITS;

/// Groups encoded NCP coordinates into labels in high-to-low field-cell order.
/// # Arguments
/// * `bits` - Exactly48 binary values in ascending retained-coordinate order.
/// * `modulation` - Uniform NCP modulation; no zero-loaded NCP profile.
/// # Returns
/// Labels with y0={a1,a0} for QPSK, {a3..a0} for16QAM or {a5..a0} for64QAM.
/// # Errors
/// Rejects incorrect length or nonbinary input without changing it.
pub fn labels_from_bits(
    bits: &[u8],
    modulation: NcpModulation,
) -> Result<Vec<SymbolLabel>, LabError> {
    if bits.len() != NCP_CODEWORD_BITS || bits.iter().any(|bit| *bit > 1) {
        return Err(invalid("Expected48 binary NCP bits."));
    }
    let width = modulation.order().ilog2() as usize;
    Ok(bits
        .chunks_exact(width)
        .map(|chunk| {
            chunk
                .iter()
                .enumerate()
                .fold(0, |label, (index, bit)| label | (u16::from(*bit) << index))
        })
        .collect())
}
/// Restores retained-coordinate LLR order from MSB-first QAM cell LLRs.
/// # Arguments
/// * `cells` - Exactly one block of cells, each containing2,4or6 finite LLRs.
/// * `modulation` - Uniform NCP modulation used by the demapper.
/// # Returns
///48 finite LLRs for the NCP FEC decoder; inputs remain unchanged.
/// # Errors
/// Rejects incorrect shapes or nonfinite values.
pub fn llrs_from_cells(
    cells: &[Vec<f64>],
    modulation: NcpModulation,
) -> Result<Vec<f64>, LabError> {
    let width = modulation.order().ilog2() as usize;
    if cells.len() != modulation.cells_per_block()
        || cells
            .iter()
            .any(|cell| cell.len() != width || cell.iter().any(|value| !value.is_finite()))
    {
        return Err(invalid("Invalid NCP cell LLR shape or nonfinite value."));
    }
    Ok(cells
        .iter()
        .flat_map(|cell| cell.iter().rev().copied())
        .collect())
}
