/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 * Extracted from MERLIN waveform/transform.rs at
 * 37be29ec2d536a96a504211fd49b20152e5555cb.
 * Changes: bounded direction-neutral sizes/errors; butterfly operation order retained.
 */
//! Centered unitary radix-2 transforms; clocks, prefixes, windows and pilots are PHY-owned.
use crate::{ComplexSample, DspError};
use std::f64::consts::TAU;

/// Explicit allocation bound for current upstream/downstream geometries.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformSize {
    /// 2048 complex values; upstream 50 kHz geometry.
    Samples2048,
    /// 4096 complex values; upstream 25 kHz or downstream 50 kHz geometry.
    Samples4096,
    /// 8192 complex values; downstream 25 kHz geometry.
    Samples8192,
}
impl TransformSize {
    /// Reads the exact bounded plan length without implying a sample clock.
    ///
    /// # Returns
    /// 2048, 4096 or 8192 complex values, not bytes or frequency units.
    #[inline]
    pub fn samples(self) -> usize {
        match self {
            Self::Samples2048 => 2048,
            Self::Samples4096 => 4096,
            Self::Samples8192 => 8192,
        }
    }
}

/// Immutable plan with owned per-call scratch and no native or shared mutable state.
pub struct TransformPlan {
    order: Vec<usize>,
    roots: Vec<ComplexSample>,
}
impl TransformPlan {
    /// Precomputes positive roots and bit reversal for one bounded geometry.
    ///
    /// # Arguments
    /// * `size` - A representable supported numerical length, independent of PHY mode.
    /// # Returns
    /// Reusable O(N) plan; each execution owns its O(N) result allocation.
    #[inline]
    pub fn new(size: TransformSize) -> Self {
        let length = size.samples();
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

    /// Analyzes useful time samples into ascending physical-frequency bins.
    ///
    /// # Arguments
    /// * `samples` - Exactly N finite Cartesian complex samples in the caller's units.
    /// # Returns
    /// Owned centered bins using negative-exponent roots and 1/sqrt(N) scaling.
    /// # Errors
    /// Rejects wrong length/nonfinite input before allocation and nonfinite output.
    #[inline]
    pub fn forward(&self, samples: &[ComplexSample]) -> Result<Vec<ComplexSample>, DspError> {
        self.apply(samples, false)
    }

    /// Synthesizes ascending physical-frequency bins into useful time samples.
    ///
    /// # Arguments
    /// * `bins` - Exactly N finite Cartesian bins; index zero is lowest frequency.
    /// # Returns
    /// Owned samples using positive-exponent roots and 1/sqrt(N) scaling, no prefix.
    /// # Errors
    /// Rejects wrong length/nonfinite input before allocation and nonfinite output.
    #[inline]
    pub fn inverse(&self, bins: &[ComplexSample]) -> Result<Vec<ComplexSample>, DspError> {
        self.apply(bins, true)
    }

    // Preserve direction specialization across crate boundaries, measured
    // before replacing the former same-crate downstream kernel.
    #[inline]
    fn apply(
        &self,
        input: &[ComplexSample],
        inverse: bool,
    ) -> Result<Vec<ComplexSample>, DspError> {
        let length = self.order.len();
        if input.len() != length || input.iter().flatten().any(|value| !value.is_finite()) {
            return Err(DspError::InvalidSamples);
        }
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
        if output.iter().flatten().any(|value| !value.is_finite()) {
            return Err(DspError::NonfiniteOutput);
        }
        Ok(output)
    }
}
