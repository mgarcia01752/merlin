/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit finite ReferenceVariance MaxLog; not measured noise or field RxMER.
use super::{map_symbol, reference_energy};
use crate::{error::invalid, types::Iq, PhyResult};

/// Existing max-log confidence clipping policy, shared with actual soft-view metadata.
pub(crate) const MAX_LOG_LLR_MAGNITUDE: f64 = 50.0;

/// Computes ordered finite natural-log P(0)/P(1) metrics for one received cell.
///
/// `value=None` is an unavailable cell and returns exactly `bits` erasures.
/// `variance` is complex reference-plane error power after role gain removal.
/// Returns MSB-first metrics clamped to ±50; rejects invalid/nonfinite arithmetic.
/// Work is bounded by 4096*(12+1) distance/bit operations per call.
pub fn max_log(bits: u8, value: Option<Iq>, variance: f64) -> PhyResult<Vec<f64>> {
    reference_energy(bits)?;
    if !variance.is_finite() || variance <= 0.0 || !variance.recip().is_finite() {
        return Err(invalid("demapping.variance"));
    }
    let Some(value) = value else {
        return Ok(vec![0.0; bits as usize]);
    };
    if !value.is_finite() {
        return Err(invalid("demapping.iq"));
    }
    let mut distances = vec![[f64::INFINITY; 2]; bits as usize];
    for label in 0..(1_u16 << bits) {
        let distance = (value - map_symbol(bits, label)?).power();
        if !distance.is_finite() {
            return Err(invalid("demapping.distance"));
        }
        for (bit, minimum) in distances.iter_mut().enumerate() {
            let digit = ((label >> (bits as usize - 1 - bit)) & 1) as usize;
            minimum[digit] = minimum[digit].min(distance);
        }
    }
    distances
        .into_iter()
        .map(|[zero, one]| {
            let metric = (one - zero) / variance;
            if !metric.is_finite() {
                return Err(invalid("demapping.metric"));
            }
            Ok(metric.clamp(-MAX_LOG_LLR_MAGNITUDE, MAX_LOG_LLR_MAGNITUDE))
        })
        .collect()
}
