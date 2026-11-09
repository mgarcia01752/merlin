/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Column-major circular twists, preserving an explicit intermediate boundary.
use crate::{types::ofdm_modulation::ConstellationOrder, SimulationError};
/// Returns the column-major output to input permutation for supported short8/9.
/// # Arguments
/// * `order` - Supported full-word uniform data order; no shortening or mixed loading.
/// # Returns
/// A possibly zero-extended permutation after independently applied column rotations.
/// # Errors
/// Rejects unverified coding orders rather than substituting a twist table.
pub fn permutation(order: ConstellationOrder) -> Result<Vec<usize>, SimulationError> {
    Ok(for_parameters(super::parameters::require(order)?))
}

/// Builds the stage mapping from the constructor's shared verified table.
/// # Arguments
/// * `parameters` - Valid downstream short8/9 table, not caller-built geometry.
/// # Returns
/// Output-to-input permutation reused by the composed facade.
pub(super) fn for_parameters(parameters: &super::parameters::Parameters) -> Vec<usize> {
    let mut indices = vec![0; parameters.rows * parameters.twists.len()];
    for (column, shift) in parameters.twists.iter().enumerate() {
        for row in 0..parameters.rows {
            indices[column * parameters.rows + (row + shift) % parameters.rows] =
                column * parameters.rows + row;
        }
    }
    indices
}
/// Applies this stage alone without changing the supplied bits.
/// # Arguments
/// * `order` - Supported uniform data coding order.
/// * `bits` - Exactly the padded stage count of binary values at this stage's input boundary.
/// # Returns
/// Owned output bits, available as a direct intermediate test point.
/// # Errors
/// Rejects unsupported mode, wrong length or nonbinary data before processing.
pub fn transmit(order: ConstellationOrder, bits: &[u8]) -> Result<Vec<u8>, SimulationError> {
    super::boundary::transmit(bits, &permutation(order)?)
}
/// Reverses this stage alone on soft likelihoods, preserving sign and magnitude.
/// # Arguments
/// * `order` - Supported uniform data coding order.
/// * `llrs` - Exactly the padded stage count of finite values at this stage's output boundary.
/// # Returns
/// Owned input-order likelihoods; invoke inverse stages in reverse TX order.
/// # Errors
/// Rejects unsupported mode, wrong length or nonfinite data.
pub fn receive_llrs(order: ConstellationOrder, llrs: &[f64]) -> Result<Vec<f64>, SimulationError> {
    super::boundary::receive(llrs, &permutation(order)?)
}
