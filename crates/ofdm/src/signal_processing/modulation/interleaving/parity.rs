/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Short8/9 parity interleaving, independent of twists and QAM demultiplexing.
use crate::SimulationError;
use fec_codec::parameters::{LDPC_CODEWORD_BITS, LDPC_INPUT_BITS};
const PARITY_GROUP_BITS: usize = 360;
const PARITY_GROUPS: usize = (LDPC_CODEWORD_BITS - LDPC_INPUT_BITS) / PARITY_GROUP_BITS;
/// Returns output-position to input-position indices for the entire coded word.
/// # Returns
/// A16200-element bijection; systematic bits are unchanged.
pub fn permutation() -> Vec<usize> {
    with_systematic_bits(LDPC_INPUT_BITS)
}
/// Applies this stage alone without changing the supplied bits.
/// # Arguments
/// * `bits` - Exactly16200 binary values at this stage's input boundary.
/// # Returns
/// Owned output bits, available as a direct intermediate test point.
/// # Errors
/// Rejects unsupported mode, wrong length or nonbinary data before processing.
pub fn transmit(bits: &[u8]) -> Result<Vec<u8>, SimulationError> {
    super::boundary::transmit(bits, &permutation())
}
/// Reverses this stage alone on soft likelihoods, preserving sign and magnitude.
/// # Arguments
/// * `llrs` - Exactly16200 finite values at this stage's output boundary.
/// # Returns
/// Owned input-order likelihoods; invoke inverse stages in reverse TX order.
/// # Errors
/// Rejects unsupported mode, wrong length or nonfinite data.
pub fn receive_llrs(llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
    super::boundary::receive(llrs, &permutation())
}

/// Builds parity coordinates with a validated full or shortened systematic prefix.
/// # Arguments
/// * `systematic_bits` - Validated information-plus-BCH length from the owning codec.
/// # Returns
/// Systematic identity followed by the shared1800-bit parity permutation.
pub(super) fn with_systematic_bits(systematic_bits: usize) -> Vec<usize> {
    let mut order: Vec<_> = (0..systematic_bits + PARITY_GROUP_BITS * PARITY_GROUPS).collect();
    for group in 0..PARITY_GROUPS {
        for offset in 0..PARITY_GROUP_BITS {
            order[systematic_bits + PARITY_GROUP_BITS * group + offset] =
                systematic_bits + PARITY_GROUPS * offset + group;
        }
    }
    order
}
