/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Magnitude and group delay of a uniformly sampled complex channel response.
use crate::pnm::data_type::{FrequencyHz, IqPair};
use std::f64::consts::{PI, TAU};

/// Channel-response magnitude in decibels relative to unit amplitude.
pub type MagnitudeDb = f64;
/// Group delay in seconds; negative values are meaningful and retained.
pub type GroupDelaySeconds = f64;
const AMPLITUDE_DB_FACTOR: f64 = 20.0;

/// Converts a complex response to magnitude without normalization or smoothing.
/// # Arguments
/// * `sample` - Decoded in-phase and quadrature components, or an excluded carrier.
/// # Returns
/// Finite dB magnitude; zero, nonfinite and excluded samples return None.
pub fn magnitude_db(sample: Option<IqPair>) -> Option<MagnitudeDb> {
    let [in_phase, quadrature] = sample?;
    if !in_phase.is_finite() || !quadrature.is_finite() {
        return None;
    }
    let magnitude = in_phase.hypot(quadrature);
    let result = AMPLITUDE_DB_FACTOR * magnitude.log10();
    result.is_finite().then_some(result)
}

/// Differentiates unwrapped phase independently within each valid carrier run.
/// # Arguments
/// * `samples` - Uniformly spaced I/Q pairs; missing/zero/nonfinite pairs split runs.
/// * `spacing_hz` - Positive frequency step in Hz.
/// # Returns
/// Seconds per carrier, using -d(phase)/d(frequency)/(2*pi). Interior differences
/// are central; runs of at least three use second-order one-sided edges. Two
/// samples use their slope; isolated samples and zero spacing yield None.
/// Phase steps exceeding pi are ambiguous: no absolute delay recovery is implied.
pub fn group_delay_seconds(
    samples: &[Option<IqPair>],
    spacing_hz: FrequencyHz,
) -> Vec<Option<GroupDelaySeconds>> {
    let mut delays = vec![None; samples.len()];
    if spacing_hz == 0 {
        return delays;
    }
    let mut phases: Vec<Option<f64>> = samples
        .iter()
        .map(|sample| {
            magnitude_db(*sample)?;
            let [in_phase, quadrature] = (*sample)?;
            Some(quadrature.atan2(in_phase))
        })
        .collect();
    let mut start = 0;
    while start < phases.len() {
        if phases[start].is_none() {
            start += 1;
            continue;
        }
        let mut end = start + 1;
        while end < phases.len() && phases[end].is_some() {
            let previous = phases[end - 1].unwrap();
            let current = phases[end].unwrap();
            let difference = current - previous;
            let mut wrapped = (difference + PI).rem_euclid(TAU) - PI;
            if wrapped == -PI && difference > 0.0 {
                wrapped = PI;
            }
            phases[end] = Some(previous + wrapped);
            end += 1;
        }
        let length = end - start;
        for index in start..end {
            let slope = if length < 2 {
                continue;
            } else if length == 2 {
                phases[start + 1].unwrap() - phases[start].unwrap()
            } else if index == start {
                (-3.0 * phases[index].unwrap() + 4.0 * phases[index + 1].unwrap()
                    - phases[index + 2].unwrap())
                    / 2.0
            } else if index == end - 1 {
                (3.0 * phases[index].unwrap() - 4.0 * phases[index - 1].unwrap()
                    + phases[index - 2].unwrap())
                    / 2.0
            } else {
                (phases[index + 1].unwrap() - phases[index - 1].unwrap()) / 2.0
            };
            delays[index] = Some(-slope / (TAU * f64::from(spacing_hz)));
        }
        start = end;
    }
    delays
}
