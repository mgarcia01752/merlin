/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared fixed-word validation and permutation application.
use crate::{error::invalid, SimulationError};
use fec_codec::parameters::LDPC_CODEWORD_BITS;
/// Applies a trusted stage permutation after validating caller-owned binary input.
/// # Arguments
/// * `input` - Exactly16200 binary values at the stage input boundary.
/// * `order` - Verified output-to-input bijection from the owning stage table.
/// # Returns
/// Owned permuted bits; inputs stay unchanged.
/// # Errors
/// Rejects malformed length or nonbinary input before indexing the trusted map.
pub(super) fn transmit(input: &[u8], order: &[usize]) -> Result<Vec<u8>, SimulationError> {
    if input.len() != LDPC_CODEWORD_BITS || input.iter().any(|bit| *bit > 1) {
        return Err(invalid(
            "Expected16200 binary bits at interleaver boundary.",
        ));
    }
    Ok(order.iter().map(|position| input[*position]).collect())
}
/// Applies the inverse of a trusted stage map to finite soft values.
/// # Arguments
/// * `input` - Exactly16200 finite output-order likelihoods.
/// * `order` - Verified output-to-input bijection from the owning stage table.
/// # Returns
/// Owned input-order likelihoods, preserving every sign and magnitude.
/// # Errors
/// Rejects malformed length or nonfinite likelihoods before permutation.
pub(super) fn receive(input: &[f64], order: &[usize]) -> Result<Vec<f64>, SimulationError> {
    if input.len() != LDPC_CODEWORD_BITS || input.iter().any(|value| !value.is_finite()) {
        return Err(invalid(
            "Expected16200 finite LLRs at interleaver boundary.",
        ));
    }
    let mut output = vec![0.0; LDPC_CODEWORD_BITS];
    for (value, position) in input.iter().zip(order) {
        output[*position] = *value;
    }
    Ok(output)
}
