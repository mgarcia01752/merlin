/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.7/Figures66–67: block-local permutation, separate from data/NCP.
use super::{
    geometry::{PLC_BITS_PER_CELL, PLC_CODEWORD_BITS, PLC_CODEWORD_NIBBLES},
    nibbles::{PlcCodewordNibbles, PlcNibble},
    PlcFrameGeometry,
};
use crate::{error::invalid, transmission::SubcarrierSpacing, LabError};
use serde::Serialize;
///96 labels in time-major order, lowest local PLC carrier first within each symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct InterleavedPlcBlock {
    spacing: SubcarrierSpacing,
    values: Vec<PlcNibble>,
}
impl InterleavedPlcBlock {
    /// Copies a received block before inverse permutation or later signal processing.
    /// # Arguments
    /// * `spacing` - Known fixed spacing; establishes6x16 or12x8 output geometry.
    /// * `values` -96 time-major labels, each0..15.
    /// # Returns
    /// Owned validated block; no bits are inferred for missing symbols.
    /// # Errors
    /// Rejects invalid count or labels before copying.
    pub fn from_values(spacing: SubcarrierSpacing, values: &[PlcNibble]) -> Result<Self, LabError> {
        let checked = PlcCodewordNibbles::from_values(values)?;
        Ok(Self {
            spacing,
            values: checked.values().to_vec(),
        })
    }
    /// Returns this block's fixed channel numerology.
    /// # Returns
    ///25or50kHz spacing, without filename or capture inference.
    pub fn spacing(&self) -> SubcarrierSpacing {
        self.spacing
    }
    /// Borrows the complete time-major label stream.
    /// # Returns
    ///96 labels; element offset is time*carriers_per_symbol+local_carrier.
    pub fn values(&self) -> &[PlcNibble] {
        &self.values
    }
    /// Borrows one relative OFDM symbol's PLC labels.
    /// # Arguments
    /// * `symbol` - Relative symbol index within this codeword, starting at0.
    /// # Returns
    ///8or16 labels in increasing local carrier order, or None outside this block.
    pub fn symbol(&self, symbol: usize) -> Option<&[PlcNibble]> {
        let geometry = PlcFrameGeometry::new(self.spacing);
        if symbol >= geometry.data_symbols_per_codeword {
            return None;
        }
        let start = symbol * geometry.plc_subcarriers;
        Some(&self.values[start..start + geometry.plc_subcarriers])
    }
}
/// Exposes the exact source-nibble coordinate for every time-major output label.
/// # Arguments
/// * `spacing` - Fixed25/50kHz setting, not data time-interleaver depth.
/// # Returns
///96 source offsets: v[t,f]=u[t+S*f], S=6for8K or12for4K.
pub fn source_indices(spacing: SubcarrierSpacing) -> [usize; PLC_CODEWORD_NIBBLES] {
    let geometry = PlcFrameGeometry::new(spacing);
    std::array::from_fn(|output| {
        output / geometry.plc_subcarriers
            + geometry.data_symbols_per_codeword * (output % geometry.plc_subcarriers)
    })
}
/// Permutes a validated codeword's labels independently of any codec instance.
/// # Arguments
/// * `spacing` - Known fixed PLC geometry.
/// * `coded` -96 unrandomized codeword labels.
/// # Returns
/// Owned time-major PLC labels, before randomization and constellation mapping.
pub fn interleave(spacing: SubcarrierSpacing, coded: &PlcCodewordNibbles) -> InterleavedPlcBlock {
    let values = source_indices(spacing)
        .map(|source| coded.values()[source])
        .to_vec();
    InterleavedPlcBlock { spacing, values }
}
/// Restores codeword label order from an unrandomized hard-label block.
/// # Arguments
/// * `interleaved` - Validated96-label block with known spacing.
/// # Returns
/// Owned original labels; no signal scaling or decoder acceptance is implied.
/// # Errors
/// Propagates unexpected restored label-validation failures.
pub fn deinterleave(interleaved: &InterleavedPlcBlock) -> Result<PlcCodewordNibbles, LabError> {
    let mut values = [0; PLC_CODEWORD_NIBBLES];
    for (output, source) in source_indices(interleaved.spacing).iter().enumerate() {
        values[*source] = interleaved.values[output];
    }
    // All values came from an immutable validated container with exactly96 labels.
    PlcCodewordNibbles::from_values(&values)
}
/// Restores codeword bit likelihood order while preserving every numeric value.
/// # Arguments
/// * `spacing` - Known PLC geometry.
/// * `llrs` -384 finite values, time-major labels with four MSB-first LLRs each.
///   Receiver must undo PLC randomization before passing these values.
/// # Returns
///384 LLRs in punctured-codeword bit order; no hard slicing, clipping or priors.
/// # Errors
/// Rejects wrong length or nonfinite values before constructing output.
pub fn deinterleave_llrs(
    spacing: SubcarrierSpacing,
    llrs: &[f64],
) -> Result<[f64; PLC_CODEWORD_BITS], LabError> {
    if llrs.len() != PLC_CODEWORD_BITS || llrs.iter().any(|value| !value.is_finite()) {
        log::debug!("plc-interleaver-llrs-rejected actual-llrs={}", llrs.len());
        return Err(invalid("PLC inverse requires384 finite bit likelihoods."));
    }
    let mut output = [0.0; PLC_CODEWORD_BITS];
    for (time_label, codeword_label) in source_indices(spacing).iter().enumerate() {
        output[codeword_label * PLC_BITS_PER_CELL..(codeword_label + 1) * PLC_BITS_PER_CELL]
            .copy_from_slice(
                &llrs[time_label * PLC_BITS_PER_CELL..(time_label + 1) * PLC_BITS_PER_CELL],
            );
    }
    Ok(output)
}
