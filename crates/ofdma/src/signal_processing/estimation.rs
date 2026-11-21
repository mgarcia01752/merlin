/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Engineering frame LS/interpolation over actual received known pilots only.
use crate::{
    bit_processing::pilot_reference,
    configuration::{Estimator, UsConfiguration},
    error::invalid,
    resources::{CellRole, ResourcePlan},
    types::Iq,
    PhyResult,
};

/// Estimates effective C×H at each physical bin from received `symbols`.
///
/// Missing symbols stay None. `resources` and `config` supply legitimate known
/// pilot positions/signs/gains; `policy` selects frame LS or bracketed linear LS.
/// Returns N optional response estimates. Rejects malformed/nonfinite input.
pub fn estimate(
    config: &UsConfiguration,
    resources: &ResourcePlan,
    symbols: &[Option<Vec<Iq>>],
    policy: &Estimator,
) -> PhyResult<Vec<Option<Iq>>> {
    let n = config.waveform().numerology().fft_len();
    if symbols.len() != config.waveform().frame_symbols()
        || symbols
            .iter()
            .flatten()
            .any(|grid| grid.len() != n || grid.iter().any(|z| !z.is_finite()))
    {
        return Err(invalid("estimation.symbols"));
    }
    let references = pilot_reference(config.waveform().numerology());
    let mut numerators = vec![Iq::default(); n];
    let mut denominators = vec![0.0; n];
    let mut domain = vec![false; n];
    for cell in &resources.visits {
        domain[cell.carrier] = true;
        if cell.role != CellRole::KnownPilot {
            continue;
        }
        if let Some(grid) = &symbols[cell.symbol] {
            let reference = references[cell.carrier] * cell.gain;
            numerators[cell.carrier] =
                numerators[cell.carrier] + reference.conjugate() * grid[cell.carrier];
            denominators[cell.carrier] += reference.power();
        }
    }
    if numerators.iter().any(|z| !z.is_finite()) || denominators.iter().any(|d| !d.is_finite()) {
        return Err(invalid("estimation.arithmetic"));
    }
    match policy {
        Estimator::FrameFlatLs => {
            let numerator = numerators
                .iter()
                .copied()
                .fold(Iq::default(), |sum, z| sum + z);
            let denominator: f64 = denominators.iter().sum();
            let value = if denominator > 0.0 {
                Some(numerator * denominator.recip())
            } else {
                None
            };
            if value.is_some_and(|z| !z.is_finite()) {
                return Err(invalid("estimation.flat"));
            }
            Ok(domain
                .into_iter()
                .map(|active| if active { value } else { None })
                .collect())
        }
        Estimator::FramePilotLinear { maximum_gap } => {
            let mut estimates: Vec<_> = numerators
                .into_iter()
                .zip(denominators)
                .map(|(num, den)| {
                    if den > 0.0 {
                        Some(num * den.recip())
                    } else {
                        None
                    }
                })
                .collect();
            let mut left = None;
            for k in 0..n {
                if !domain[k] {
                    left = None;
                    continue;
                }
                if let Some(right_value) = estimates[k] {
                    if let Some((previous, left_value)) = left {
                        if k - previous <= *maximum_gap {
                            for (bin, value) in
                                estimates.iter_mut().enumerate().take(k).skip(previous + 1)
                            {
                                let weight = (bin - previous) as f64 / (k - previous) as f64;
                                *value = Some(left_value * (1.0 - weight) + right_value * weight);
                            }
                        }
                    }
                    left = Some((k, right_value));
                }
            }
            if estimates.iter().flatten().any(|z| !z.is_finite()) {
                return Err(invalid("estimation.linear"));
            }
            Ok(estimates)
        }
    }
}

/// Divides finite complex `value` by a finite response above `power_floor`.
///
/// Returns None for missing/deep-null response; otherwise returns owned ZF I/Q.
/// Stable Cartesian division rejects nonrepresentable output without clipping.
pub fn equalize(value: Iq, response: Option<Iq>, power_floor: f64) -> PhyResult<Option<Iq>> {
    if !value.is_finite() || !power_floor.is_finite() || power_floor <= 0.0 {
        return Err(invalid("equalization.input"));
    }
    let Some(response) = response else {
        return Ok(None);
    };
    if !response.is_finite() {
        return Err(invalid("equalization.response"));
    }
    let magnitude = response.re.hypot(response.im);
    if !magnitude.is_finite() {
        return Err(invalid("equalization.magnitude"));
    }
    if magnitude <= power_floor.sqrt() {
        return Ok(None);
    }
    let unit = Iq {
        re: response.re / magnitude,
        im: -response.im / magnitude,
    };
    let output = Iq {
        re: value.re / magnitude,
        im: value.im / magnitude,
    } * unit;
    if !output.is_finite() {
        return Err(invalid("equalization.output"));
    }
    Ok(Some(output))
}
