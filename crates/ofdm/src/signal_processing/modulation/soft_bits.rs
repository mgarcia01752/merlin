/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared max-log soft demapping for embedded label-ordered constellations.
use super::{
    qam::{reference_for_order, QamReference},
    QAM16_BITS_PER_SYMBOL,
};
use crate::{
    error::invalid,
    types::{ofdm_modulation::ConstellationOrder, IqPair},
    LabError,
};
const LLR_LIMIT: f64 = 100.0;
const MAX_REFERENCE_BITS: usize = crate::types::ofdm_modulation::MAX_OFDM_BITS_PER_SYMBOL as usize;

/// Computes max-log likelihoods for any supported reference through16K-QAM.
/// # Arguments
/// * `order` - Physical constellation size, not an OSSI code.
/// * `received` - Equalized unit-energy [I,Q]; no rescaling is applied.
/// * `noise_power` - Positive finite E[|n|²], not a per-axis variance or dB value.
/// # Returns
/// Owned MSB-first natural-log LLRs; positive favors0, clipped to±100.
/// # Errors
/// Rejects unsupported orders, invalid samples/variance and overflowing metrics.
pub fn qam_llrs(
    order: ConstellationOrder,
    received: IqPair,
    noise_power: f64,
) -> Result<Vec<f64>, LabError> {
    let reference = reference_for_order(usize::from(order))
        .ok_or_else(|| LabError::new("UnsupportedModulation", "Unsupported QAM constellation."))?;
    let mut output = vec![0.0; usize::from(reference.bit_loading)];
    demap_into(reference, received, noise_power, &mut output)?;
    Ok(output)
}

/// Computes max-log LLRs with the established16-QAM array interface.
/// # Arguments
/// * `received` - Equalized unit-energy complex symbol [I,Q].
/// * `noise_power` - E[|n|²]; each real component has variance noise_power/2.
/// # Returns
/// Four MSB-first natural-log likelihoods; positive favors0, clipped to±100.
/// # Errors
/// Rejects invalid symbols, complex noise power or overflowing distance metrics.
pub fn qam16_llrs(received: IqPair, noise_power: f64) -> Result<[f64; 4], LabError> {
    let reference = reference_for_order(16).ok_or_else(|| invalid("Missing16-QAM reference."))?;
    let mut output = [0.0; QAM16_BITS_PER_SYMBOL];
    demap_into(reference, received, noise_power, &mut output)?;
    Ok(output)
}

/// Fills caller-owned output through one shared brute-force max-log calculation.
fn demap_into(
    reference: &QamReference,
    received: IqPair,
    noise_power: f64,
    output: &mut [f64],
) -> Result<(), LabError> {
    if received.iter().any(|value| !value.is_finite())
        || !noise_power.is_finite()
        || noise_power <= 0.0
    {
        return Err(invalid(
            "Finite I/Q and positive complex noise variance are required.",
        ));
    }
    let bits = usize::from(reference.bit_loading);
    let mut distances = [[f64::INFINITY; 2]; MAX_REFERENCE_BITS];
    for (label, point) in reference.iq.iter().enumerate() {
        let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
        if !distance.is_finite() {
            return Err(invalid("I/Q distance exceeds finite demapping range."));
        }
        for (bit, hypotheses) in distances[..bits].iter_mut().enumerate() {
            let value = (label >> (bits - 1 - bit)) & 1;
            hypotheses[value] = hypotheses[value].min(distance);
        }
    }
    for (value, hypotheses) in output.iter_mut().zip(&distances[..bits]) {
        *value = ((hypotheses[1] - hypotheses[0]) / noise_power).clamp(-LLR_LIMIT, LLR_LIMIT);
    }
    Ok(())
}
