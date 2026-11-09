/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Safe Rust radix-2 butterfly engine; public direction wrappers own PHY conventions.
use super::samples::validate_samples;
use crate::{transmission::SubcarrierSpacing, types::IqPair, LabError};
use std::f64::consts::TAU;
pub(super) struct TransformPlan {
    order: Vec<usize>,
    roots: Vec<IqPair>,
}
impl TransformPlan {
    /// Precomputes positive roots and bit reversal for one supported FFT size.
    /// # Arguments
    /// * `spacing` - Determines bounded 4K/8K geometry.
    /// # Returns
    /// Immutable reusable numerical plan.
    pub(super) fn new(spacing: SubcarrierSpacing) -> Self {
        let length = spacing.fft_samples() as usize;
        let bits = length.trailing_zeros();
        Self {
            order: (0..length)
                .map(|index| index.reverse_bits() >> (usize::BITS - bits))
                .collect(),
            roots: (0..length / 2)
                .map(|index| {
                    let (imaginary, real) = (TAU * index as f64 / length as f64).sin_cos();
                    [real, imaginary]
                })
                .collect(),
        }
    }
    /// Executes the unitary centered transform in the selected direction.
    /// # Arguments
    /// * `input` - Complete finite physical bins or useful time samples.
    /// * `inverse` - True for TX synthesis; false for RX analysis.
    /// # Returns
    /// Owned transform output without mutating the source or plan.
    /// # Errors
    /// Rejects wrong length, nonfinite data and nonfinite numerical output.
    pub(super) fn apply(&self, input: &[IqPair], inverse: bool) -> Result<Vec<IqPair>, LabError> {
        let length = self.order.len();
        validate_samples(input, length)?;
        let mut output: Vec<_> = self
            .order
            .iter()
            .map(|index| {
                let sign = if !inverse && index % 2 == 1 {
                    -1.0
                } else {
                    1.0
                };
                input[*index].map(|value| value * sign)
            })
            .collect();
        let mut width = 2;
        while width <= length {
            for chunk in output.chunks_exact_mut(width) {
                let (left, right) = chunk.split_at_mut(width / 2);
                for (index, (low, high)) in left.iter_mut().zip(right).enumerate() {
                    let [real, mut imaginary] = self.roots[index * length / width];
                    if !inverse {
                        imaginary = -imaginary;
                    }
                    let product = [
                        high[0] * real - high[1] * imaginary,
                        high[0] * imaginary + high[1] * real,
                    ];
                    *high = [low[0] - product[0], low[1] - product[1]];
                    *low = [low[0] + product[0], low[1] + product[1]];
                }
            }
            width *= 2;
        }
        let scale = 1.0 / (length as f64).sqrt();
        for (index, sample) in output.iter_mut().enumerate() {
            let sign = if inverse && index % 2 == 1 { -1.0 } else { 1.0 };
            *sample = sample.map(|value| value * scale * sign);
        }
        validate_samples(&output, length)?;
        Ok(output)
    }
}
