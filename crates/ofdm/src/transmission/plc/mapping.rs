/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §7.5.13.9: shared AnnexA16-QAM mapping, unit ensemble mean energy.
use super::{geometry::PLC_CODEWORD_NIBBLES, RandomizedPlcBlock};
use crate::{
    error::invalid,
    signal_processing::modulation::{map_symbol, qam16_llrs},
    types::IqPair,
    LabError,
};
/// Maps an immutable randomized PLC codeword using the shared normalized table.
/// # Arguments
/// * `block` -96 time-major randomized labels; no second randomization is applied.
/// # Returns
///96 unit-reference I/Q cells; integer16-QAM coordinates are scaled by1/sqrt(10).
/// # Errors
/// Propagates unsupported reference or invalid label errors.
pub fn map(block: &RandomizedPlcBlock) -> Result<Vec<IqPair>, LabError> {
    block
        .values()
        .iter()
        .map(|label| map_symbol(16, u16::from(*label)))
        .collect()
}
/// Soft-demaps a received PLC codeword without transmitter labels or masks.
/// # Arguments
/// * `iq` - Exactly96 time-major, equalized unit-reference I/Q samples.
/// * `noise_power` - Positive finite complex noise variance, used even in reference tests.
/// # Returns
///384 MSB-first LLRs, positive for bit0; shared demapper clips to±100.
/// # Errors
/// Rejects sample count, nonfinite samples or invalid noise variance.
pub fn demap(iq: &[IqPair], noise_power: f64) -> Result<Vec<f64>, LabError> {
    if iq.len() != PLC_CODEWORD_NIBBLES {
        return Err(invalid("PLC demapper requires exactly96 I/Q cells."));
    }
    let mut llrs = Vec::with_capacity(iq.len() * 4);
    for sample in iq {
        llrs.extend_from_slice(&qam16_llrs(*sample, noise_power)?);
    }
    Ok(llrs)
}
