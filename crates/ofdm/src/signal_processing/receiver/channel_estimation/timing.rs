/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded cyclic-prefix timing search with known numerology; not carrier/PLC-phase acquisition.
use crate::{
    error::invalid,
    signal_processing::waveform::{SampleBlock, WaveformGeometry},
    SimulationError,
};
use serde::Serialize;
/// Observed symbol boundary and normalized prefix/tail correlation.
#[derive(Debug, Clone, Serialize)]
pub struct TimingEstimate {
    pub first_symbol_sample: u64,
    pub normalized_correlation: f64,
}
/// Finds a symbol boundary using received cyclic-prefix repetition alone.
/// # Arguments
/// * `geometry` - Known FFT, prefix and taper settings.
/// * `input` - Contiguous received samples, including the entire search window.
/// * `max_offset_samples` - Inclusive search extent, at most one symbol period.
/// * `minimum_correlation` - Required normalized squared correlation in (0,1].
/// # Returns
/// Highest-scoring unique boundary; leading taper samples are excluded from correlation.
/// Known PLC phase must still be supplied separately; no frequency-offset correction.
/// # Errors
/// Rejects insufficient samples, invalid policy, silent/ambiguous signals or weak correlation.
pub fn estimate_symbol_boundary(
    geometry: &WaveformGeometry,
    input: &SampleBlock,
    max_offset_samples: usize,
    minimum_correlation: f64,
) -> Result<TimingEstimate, SimulationError> {
    if max_offset_samples > geometry.stride_samples()
        || !minimum_correlation.is_finite()
        || !(0.0..=1.0).contains(&minimum_correlation)
        || minimum_correlation == 0.0
        || input.samples().len() < geometry.stride_samples() + max_offset_samples
    {
        return Err(invalid(
            "Invalid CH-EST-RX timing search window or threshold.",
        ));
    }
    let samples = input.samples();
    let first = geometry.roll_off_samples();
    let count = geometry.prefix_samples() - first;
    let fft = geometry.fft_samples();
    let mut best: Option<(usize, f64)> = None;
    let mut tied = false;
    for offset in 0..=max_offset_samples {
        let mut correlation = [0.0, 0.0];
        let mut left_power = 0.0;
        let mut right_power = 0.0;
        for index in offset + first..offset + first + count {
            let left = samples[index];
            let right = samples[index + fft];
            correlation[0] += left[0] * right[0] + left[1] * right[1];
            correlation[1] += left[0] * right[1] - left[1] * right[0];
            left_power += left[0] * left[0] + left[1] * left[1];
            right_power += right[0] * right[0] + right[1] * right[1];
        }
        let denominator = left_power * right_power;
        let score = if denominator > 0.0 {
            (correlation[0] * correlation[0] + correlation[1] * correlation[1]) / denominator
        } else {
            0.0
        };
        if !score.is_finite() {
            return Err(invalid("Timing correlation overflow."));
        }
        match best {
            Some((_, value)) if (score - value).abs() < 8.0 * f64::EPSILON => {
                tied = true;
            }
            Some((_, value)) if score <= value => {}
            _ => {
                best = Some((offset, score));
                tied = false;
            }
        }
    }
    let (offset, score) = best.ok_or_else(|| invalid("No timing candidates."))?;
    if tied || score < minimum_correlation {
        return Err(invalid(
            "Ambiguous or insufficient cyclic-prefix correlation.",
        ));
    }
    Ok(TimingEstimate {
        first_symbol_sample: input.start_index() + offset as u64,
        normalized_correlation: score.min(1.0),
    })
}
