/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone PLC puncture and soft-erasure restoration boundaries.
use super::parameters::{
    MAX_PLC_LLR_MAGNITUDE, PLC_CODEWORD_BITS, PLC_MOTHER_BITS, PLC_TRANSMITTED_COORDINATES,
};
use crate::binary_bits;
/// Selects transmitted coordinates without changing the complete mother word.
/// # Arguments
/// * `mother` - Exactly480 binary values in systematic coordinate order.
/// # Returns
///384 retained bits; positions48..95 and384..431 are absent. This does not encode
/// or verify syndrome; it is a separately testable coordinate operation.
/// # Errors
/// Rejects wrong size or nonbinary input before indexing.
pub fn puncture(mother: &[u8]) -> Result<[u8; PLC_CODEWORD_BITS], String> {
    binary_bits(mother, PLC_MOTHER_BITS)?;
    Ok(PLC_TRANSMITTED_COORDINATES.map(|coordinate| mother[coordinate]))
}
/// Restores zero LLR erasures at the deleted mother-code coordinates.
/// # Arguments
/// * `llrs` -384 finite natural-log P(0)/P(1) values, positive favoring zero.
/// # Returns
///480 soft values; punctures are exactly0, never forced information zeros.
/// Retained values saturate at +/-100 for decoder arithmetic safety.
/// # Errors
/// Rejects wrong length or nonfinite inputs before restoration.
pub fn restore_llrs(llrs: &[f64]) -> Result<[f64; PLC_MOTHER_BITS], String> {
    if llrs.len() != PLC_CODEWORD_BITS || llrs.iter().any(|value| !value.is_finite()) {
        log::debug!(
            "plc-llrs-rejected expected-llrs={PLC_CODEWORD_BITS} actual-llrs={}",
            llrs.len()
        );
        return Err("Expected384 finite PLC LLR values.".into());
    }
    let mut mother = [0.0; PLC_MOTHER_BITS];
    for (&coordinate, &llr) in PLC_TRANSMITTED_COORDINATES.iter().zip(llrs) {
        mother[coordinate] = llr.clamp(-MAX_PLC_LLR_MAGNITUDE, MAX_PLC_LLR_MAGNITUDE);
    }
    Ok(mother)
}
