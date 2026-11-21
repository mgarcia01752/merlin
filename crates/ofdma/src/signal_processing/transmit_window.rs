/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! PHY §7.4.10.1 symmetric half-sample raised-cosine transmit taper.
use crate::configuration::Waveform;
use std::f64::consts::PI;

/// Computes immutable real coefficients for accepted `geometry`.
///
/// Returns N+C+R coefficients; R=0 is all ones. The first/last nonzero-R
/// weights are positive and complementary overlap weights sum to one.
pub fn transmit_window(geometry: &Waveform) -> Vec<f64> {
    let stride = geometry.stride();
    let r = geometry.rolloff();
    (0..stride + r)
        .map(|t| {
            if r == 0 {
                1.0
            } else if t < r {
                0.5 * (1.0 - (PI * (t as f64 + 0.5) / r as f64).cos())
            } else if t >= stride {
                0.5 * (1.0 + (PI * ((t - stride) as f64 + 0.5) / r as f64).cos())
            } else {
                1.0
            }
        })
        .collect()
}
