/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Max-log soft demapping using the verified16-QAM reference.
use super::{qam::map_qam16, QAM16_BITS_PER_SYMBOL};
use crate::error::{invalid, LabError};
const LLR_LIMIT: f64 = 100.0;
/// Computes max-log LLRs with positive values favoring bit0.
/// # Arguments
/// * `received` - Equalized unit-energy complex symbol [I,Q].
/// * `noise_power` - E[|n|²]; each real component has variance noise_power/2.
/// # Returns
/// Four natural-log max-log likelihoods, clipped to the documented decoder limit.
/// # Errors
/// Rejects invalid symbols and nonpositive/nonfinite complex noise power.
pub fn qam16_llrs(received: [f64; 2], noise_power: f64) -> Result<[f64; 4], LabError> {
    if received.iter().any(|value| !value.is_finite())
        || !noise_power.is_finite()
        || noise_power <= 0.0
    {
        return Err(invalid(
            "Finite I/Q and positive complex noise variance are required.",
        ));
    }
    let mut distances = [[f64::INFINITY; 2]; QAM16_BITS_PER_SYMBOL];
    for label in 0..16 {
        let point = map_qam16(label).ok_or_else(|| invalid("Missing16-QAM reference table."))?;
        let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
        for (bit, hypotheses) in distances.iter_mut().enumerate() {
            let value = (label >> (3 - bit)) & 1;
            hypotheses[value] = hypotheses[value].min(distance);
        }
    }
    Ok(distances.map(|hypotheses| {
        ((hypotheses[1] - hypotheses[0]) / noise_power).clamp(-LLR_LIMIT, LLR_LIMIT)
    }))
}
