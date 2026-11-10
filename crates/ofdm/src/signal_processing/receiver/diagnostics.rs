/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded numeric receiver diagnostics; none are production decoder inputs.
use crate::{error::invalid, transmission::SubcarrierSpacing, types::IqPair, SimulationError};
use serde::Serialize;

/// Response curves and centered band-limited impulse power on the fixed sample clock.
#[derive(Debug, Serialize)]
pub struct ResponseDiagnostics {
    pub gain_db: Vec<Option<f64>>,
    pub phase_degrees: Vec<Option<f64>>,
    pub real: Vec<Option<f64>>,
    pub imaginary: Vec<Option<f64>>,
    pub delay_microseconds: Vec<f64>,
    pub impulse_db_relative_peak: Vec<f64>,
    pub peak_delay_microseconds: f64,
    pub peak_extra_path_feet: f64,
    pub peak_two_way_spacing_feet: f64,
    pub available_bins: usize,
}
/// Derives response curves and a zero-filled, band-limited impulse diagnostic.
/// # Arguments
/// * `spacing` - Existing 4K/8K transform geometry at 204.8 MS/s.
/// * `response` - Exactly N lowest-frequency-first coefficients; None masks unknown/excluded bins.
/// * `velocity_factor` - Explicit uniform-cable propagation factor in (0,1].
/// # Returns
/// Wrapped phase/gain and centered signed delays, including a peak-relative impulse curve.
/// Path lengths assume uniform cable and are not reach or guaranteed echo protection.
/// # Errors
/// Rejects wrong shape, missing/zero response, nonfinite coefficients or invalid velocity.
pub fn response_diagnostics(
    spacing: SubcarrierSpacing,
    response: &[Option<IqPair>],
    velocity_factor: f64,
) -> Result<ResponseDiagnostics, SimulationError> {
    if response.len() != spacing.fft_samples() as usize
        || !velocity_factor.is_finite()
        || !(0.0..=1.0).contains(&velocity_factor)
        || velocity_factor == 0.0
    {
        return Err(invalid("Invalid response geometry or velocity factor."));
    }
    if response.iter().flatten().flatten().any(|v| !v.is_finite())
        || !response
            .iter()
            .flatten()
            .any(|p| p[0] != 0.0 || p[1] != 0.0)
    {
        return Err(invalid(
            "Response requires finite nonzero observed coefficients.",
        ));
    }
    let bins: Vec<_> = response.iter().map(|p| p.unwrap_or([0.0, 0.0])).collect();
    let impulse =
        crate::signal_processing::waveform::ifft::InverseTransform::new(spacing).apply(&bins)?;
    let count = impulse.len();
    let powers: Vec<_> = (0..count)
        .map(|i| {
            let p = impulse[(i + count / 2) % count];
            (p[0] * p[0] + p[1] * p[1]) / count as f64
        })
        .collect();
    if powers.iter().any(|power| !power.is_finite()) {
        return Err(invalid("Impulse power exceeds finite diagnostic range."));
    }
    let (peak_index, peak_power) = powers
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(b.1))
        .ok_or_else(|| invalid("Empty impulse response."))?;
    let delay_microseconds: Vec<_> = (0..count)
        .map(|i| (i as f64 - count as f64 / 2.0) / 204.8)
        .collect();
    let peak_delay_microseconds = delay_microseconds[peak_index];
    let peak_extra_path_feet =
        peak_delay_microseconds * 1e-6 * 299_792_458.0 * velocity_factor / 0.3048;
    Ok(ResponseDiagnostics {
        gain_db: response
            .iter()
            .map(|p| {
                p.and_then(|p| {
                    crate::signal_processing::waveform::spectrum::density_db(
                        p[0] * p[0] + p[1] * p[1],
                    )
                })
            })
            .collect(),
        phase_degrees: response
            .iter()
            .map(|p| p.map(|p| p[1].atan2(p[0]).to_degrees()))
            .collect(),
        real: response.iter().map(|p| p.map(|p| p[0])).collect(),
        imaginary: response.iter().map(|p| p.map(|p| p[1])).collect(),
        impulse_db_relative_peak: powers
            .iter()
            .map(|p| 10.0 * (p / peak_power).max(1e-16).log10())
            .collect(),
        delay_microseconds,
        peak_delay_microseconds,
        peak_extra_path_feet,
        peak_two_way_spacing_feet: peak_extra_path_feet / 2.0,
        available_bins: response.iter().filter(|p| p.is_some()).count(),
    })
}
/// Exact bounded likelihood statistics; sign convention remains the producer's log P(0)/P(1).
#[derive(Debug, Serialize)]
pub struct LlrStatistics {
    pub count: usize,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
    pub mean: Option<f64>,
    pub histogram: Vec<u64>,
    pub histogram_limit: f64,
}
/// Summarizes actual retained LLRs without slicing, rescaling or replacing decoder inputs.
/// # Arguments
/// * `llrs` - At most 32768 finite receiver soft values.
/// # Returns
/// Counts and a symmetric 41-bin histogram; empty input has unavailable statistics.
/// # Errors
/// Rejects oversized/nonfinite observations.
pub fn llr_statistics(llrs: &[f64]) -> Result<LlrStatistics, SimulationError> {
    if llrs.len() > 32768 || llrs.iter().any(|v| !v.is_finite()) {
        return Err(invalid("Invalid bounded LLR observation."));
    }
    let maximum = llrs.iter().copied().reduce(f64::max);
    let minimum = llrs.iter().copied().reduce(f64::min);
    let limit = llrs.iter().map(|v| v.abs()).fold(1.0_f64, f64::max);
    let mut histogram = vec![0; 41];
    for value in llrs {
        let index = (((value / limit + 1.0) * 0.5 * 41.0).floor() as usize).min(40);
        histogram[index] += 1;
    }
    Ok(LlrStatistics {
        count: llrs.len(),
        minimum,
        maximum,
        mean: (!llrs.is_empty()).then(|| llrs.iter().map(|v| v / llrs.len() as f64).sum()),
        histogram,
        histogram_limit: limit,
    })
}
