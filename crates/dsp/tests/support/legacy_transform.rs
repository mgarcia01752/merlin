/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen former MERLIN kernel, test-only equivalence oracle.
//! Base 37be29ec2d536a96a504211fd49b20152e5555cb; source SHA-256 e63b74521c7a0b0c659f4d77f2c34426e4d13d7fddc2fe6eab9f1bede6363770.
//! Adaptations: raw bounded test length and local error/validation, math unchanged.
type IqPair = [f64; 2];
type SimulationError = ();
fn validate_samples(input: &[IqPair], length: usize) -> Result<(), SimulationError> {
    if input.len() != length || input.iter().flatten().any(|value| !value.is_finite()) {
        return Err(());
    }
    Ok(())
}
use std::f64::consts::TAU;
pub(super) struct TransformPlan {
    order: Vec<usize>,
    roots: Vec<IqPair>,
}
impl TransformPlan {
    /// Precomputes the frozen numerical plan for one bounded test length.
    /// # Arguments
    /// * `length` - 2048, 4096 or 8192; typed PHY admission is outside this test fixture.
    /// # Returns
    /// Immutable reusable numerical plan.
    pub(super) fn new(length: usize) -> Self {
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
    pub(super) fn apply(
        &self,
        input: &[IqPair],
        inverse: bool,
    ) -> Result<Vec<IqPair>, SimulationError> {
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
