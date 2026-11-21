/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted from MERLIN crates/ofdm/src/signal_processing/waveform/transform.rs
 * at 096d5a476194d4f2e016863d723b225257bb6037.
 * Modifications: upstream 2K/4K geometry, Iq values and local typed validation.
 */
//! Frozen test-only source implementation. Safe immutable radix-2 plan; positive TX roots and unitary centered wrappers.
use docsis_ofdma::{configuration::Numerology, types::Iq, PhyError, PhyResult};
fn invalid(field: &'static str) -> PhyError {
    PhyError::InvalidInput { field }
}
use std::f64::consts::TAU;

/// Reusable transform with no shared scratch or external numerical dependency.
pub struct Transform {
    order: Vec<usize>,
    roots: Vec<Iq>,
}

impl Transform {
    /// Precomputes bit reversal and roots for accepted upstream `mode`.
    ///
    /// Returns immutable O(N) plan storage; subsequent calls own their scratch.
    pub fn new(mode: Numerology) -> Self {
        let n = mode.fft_len();
        let width = n.trailing_zeros();
        Self {
            order: (0..n)
                .map(|i| i.reverse_bits() >> (usize::BITS - width))
                .collect(),
            roots: (0..n / 2)
                .map(|i| {
                    let (im, re) = (TAU * i as f64 / n as f64).sin_cos();
                    Iq { re, im }
                })
                .collect(),
        }
    }
    /// Synthesizes physical ascending-frequency `grid` with positive exponent.
    ///
    /// Returns N useful samples scaled by 1/sqrt(N), without CP or taper.
    /// Rejects wrong length/nonfinite input/output; no caller state changes.
    pub fn inverse(&self, grid: &[Iq]) -> PhyResult<Vec<Iq>> {
        self.apply(grid, true)
    }
    /// Analyzes an independently selected useful receive `samples` window.
    ///
    /// Returns N centered physical bins scaled by 1/sqrt(N), negative exponent.
    /// Rejects wrong length/nonfinite input/output; no TX truth is accepted.
    pub fn forward(&self, samples: &[Iq]) -> PhyResult<Vec<Iq>> {
        self.apply(samples, false)
    }
    fn apply(&self, input: &[Iq], inverse: bool) -> PhyResult<Vec<Iq>> {
        let n = self.order.len();
        if input.len() != n || input.iter().any(|v| !v.is_finite()) {
            return Err(invalid("transform.input"));
        }
        let mut output: Vec<_> = self
            .order
            .iter()
            .map(|&i| input[i] * if !inverse && i % 2 == 1 { -1.0 } else { 1.0 })
            .collect();
        let mut width = 2;
        while width <= n {
            for chunk in output.chunks_exact_mut(width) {
                let (left, right) = chunk.split_at_mut(width / 2);
                for (i, (low, high)) in left.iter_mut().zip(right).enumerate() {
                    let mut root = self.roots[i * n / width];
                    if !inverse {
                        root.im = -root.im;
                    }
                    let product = *high * root;
                    *high = *low - product;
                    *low = *low + product;
                }
            }
            width *= 2;
        }
        let scale = (n as f64).sqrt().recip();
        for (i, value) in output.iter_mut().enumerate() {
            *value = *value * (scale * if inverse && i % 2 == 1 { -1.0 } else { 1.0 });
        }
        if output.iter().any(|v| !v.is_finite()) {
            return Err(invalid("transform.output"));
        }
        Ok(output)
    }
}
