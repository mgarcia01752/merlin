/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared fixed-word validation and permutation application.
use crate::{error::invalid, SimulationError};

/// Applies a trusted stage permutation after validating caller-owned binary input.
/// # Arguments
/// * `input` - Exactly the trusted map length of binary values at the stage input boundary.
/// * `order` - Verified output-to-input bijection from the owning stage table.
/// # Returns
/// Owned permuted bits; inputs stay unchanged.
/// # Errors
/// Rejects malformed length or nonbinary input before indexing the trusted map.
pub(super) fn transmit(input: &[u8], order: &[usize]) -> Result<Vec<u8>, SimulationError> {
    if input.len() != order.len() || input.iter().any(|bit| *bit > 1) {
        return Err(invalid(
            "Expected correctly sized binary bits at interleaver boundary.",
        ));
    }
    Ok(order.iter().map(|position| input[*position]).collect())
}
/// Applies the inverse of a trusted stage map to finite soft values.
/// # Arguments
/// * `input` - Exactly the trusted map length of finite output-order likelihoods.
/// * `order` - Verified output-to-input bijection from the owning stage table.
/// # Returns
/// Owned input-order likelihoods, preserving every sign and magnitude.
/// # Errors
/// Rejects malformed length or nonfinite likelihoods before permutation.
pub(super) fn receive(input: &[f64], order: &[usize]) -> Result<Vec<f64>, SimulationError> {
    if input.len() != order.len() || input.iter().any(|value| !value.is_finite()) {
        return Err(invalid(
            "Expected correctly sized finite LLRs at interleaver boundary.",
        ));
    }
    let mut output = vec![0.0; order.len()];
    for (value, position) in input.iter().zip(order) {
        output[*position] = *value;
    }
    Ok(output)
}
