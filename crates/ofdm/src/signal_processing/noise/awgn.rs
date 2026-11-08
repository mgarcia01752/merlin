/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Gaussian perturbation at an equalized symbol reference point.
use crate::types::IqPair;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
/// Adds independent Gaussian I/Q components to one symbol in place.
/// # Arguments
/// * `symbol` - Unit-energy reference symbol, mutated with additive noise.
/// * `power` - Validated E[|noise|²]; each real variance is half this value.
/// * `rng` - Job stream shared with source generation, preserving draw order.
/// # Returns
/// Unit after drawing the two real Gaussian components.
pub(crate) fn add_awgn(symbol: &mut IqPair, power: f64, rng: &mut impl Rng) {
    let standard_deviation = (power / 2.0).sqrt();
    for component in symbol {
        let gaussian: f64 = StandardNormal.sample(rng);
        *component += standard_deviation * gaussian;
    }
}
