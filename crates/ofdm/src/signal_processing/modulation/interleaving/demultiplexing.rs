/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Column-major to row-major cell-bit demultiplexing and its soft inverse.
use crate::{types::ofdm_modulation::ConstellationOrder, SimulationError};
/// Returns output cell-bit positions to input column-major positions.
/// # Arguments
/// * `order` - Uniform16or256-QAM with verified short8/9 demultiplexing.
/// # Returns
/// A16200-element permutation, preserving MSB-first cell-bit grouping.
/// # Errors
/// Rejects unverified coding orders.
pub fn permutation(order: ConstellationOrder) -> Result<Vec<usize>, SimulationError> {
    Ok(for_parameters(super::parameters::require(order)?))
}

/// Builds the stage mapping from the constructor's shared verified table.
/// # Arguments
/// * `parameters` - Valid16/256-QAM short8/9 table, not caller-built geometry.
/// # Returns
/// Output-to-input permutation reused by the composed facade.
pub(super) fn for_parameters(parameters: &super::parameters::Parameters) -> Vec<usize> {
    let columns = parameters.twists.len();
    let mut indices = vec![0; fec_codec::parameters::LDPC_CODEWORD_BITS];
    for column in 0..columns {
        for row in 0..parameters.rows {
            indices[row * columns + parameters.demultiplexer[column]] =
                column * parameters.rows + row;
        }
    }
    indices
}
/// Applies this stage alone without changing the supplied bits.
/// # Arguments
/// * `order` - Supported uniform16or256-QAM coding order.
/// * `bits` - Exactly16200 binary values at this stage's input boundary.
/// # Returns
/// Owned output bits, available as a direct intermediate test point.
/// # Errors
/// Rejects unsupported mode, wrong length or nonbinary data before processing.
pub fn transmit(order: ConstellationOrder, bits: &[u8]) -> Result<Vec<u8>, SimulationError> {
    super::boundary::transmit(bits, &permutation(order)?)
}
/// Reverses this stage alone on soft likelihoods, preserving sign and magnitude.
/// # Arguments
/// * `order` - Supported uniform16or256-QAM coding order.
/// * `llrs` - Exactly16200 finite values at this stage's output boundary.
/// # Returns
/// Owned input-order likelihoods; invoke inverse stages in reverse TX order.
/// # Errors
/// Rejects unsupported mode, wrong length or nonfinite data.
pub fn receive_llrs(order: ConstellationOrder, llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
    super::boundary::receive(llrs, &permutation(order)?)
}
