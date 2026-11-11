/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded Welch PSD of actual complex samples, independent of GUI and transmitters.
use super::{fft::ForwardTransform, samples::validate_samples};
use crate::{
    error::invalid,
    transmission::{settings::SAMPLE_RATE_HZ, SubcarrierSpacing},
    types::IqPair,
    SimulationError,
};
use std::f64::consts::TAU;

/// Maximum overlapping analysis windows retained in one bounded estimate.
pub const MAX_SPECTRUM_AVERAGES: usize = 32;
/// Maximum min/max buckets returned for a presentation viewport.
pub const MAX_SPECTRUM_POINTS: usize = 2048;

/// Full-resolution centered PSD; powers are normalized sample units squared/Hz.
#[derive(Clone)]
pub struct PowerSpectrum {
    pub zero_frequency_hz: u32,
    pub spacing_hz: u32,
    pub fft_samples: usize,
    pub averages: usize,
    pub equivalent_noise_bandwidth_hz: f64,
    pub density: Vec<f64>,
}

/// One contiguous bin bucket, preserving both extrema without inventing power.
#[derive(Debug)]
pub struct SpectrumBucket {
    pub first_bin: usize,
    pub last_bin: usize,
    pub minimum_db: Option<f64>,
    pub maximum_db: Option<f64>,
}

/// Converts normalized power density to dB without assigning a finite zero level.
/// # Arguments
/// * `density` - Nonnegative finite power per Hz.
/// # Returns
/// dB re one normalized sample unit squared/Hz, or None for zero/unavailable input.
pub fn density_db(density: f64) -> Option<f64> {
    if density > 0.0 && density.is_finite() {
        return Some(10.0 * density.log10());
    }
    None
}

/// Estimates centered PSD using periodic Hann windows and 50% overlap.
/// # Arguments
/// * `spacing` - Existing OFDM FFT size at the fixed 204.8 MHz sample clock.
/// * `zero_frequency_hz` - Absolute physical frequency of the lowest FFT bin.
/// * `samples` - Finite contiguous TX/RX samples; N through N+(31*N/2).
/// # Returns
/// Linear-power average with density normalization N*|unitary FFT|²/(Fs*sum(w²)).
/// Partial trailing windows are excluded and reflected by the average count.
/// # Errors
/// Rejects invalid lengths, nonfinite samples, frequency overflow or FFT failures.
pub fn estimate(
    spacing: SubcarrierSpacing,
    zero_frequency_hz: u32,
    samples: &[IqPair],
) -> Result<PowerSpectrum, SimulationError> {
    let length = spacing.fft_samples() as usize;
    let hop = length / 2;
    if samples.len() < length || samples.len() > length + (MAX_SPECTRUM_AVERAGES - 1) * hop {
        return Err(invalid(
            "Spectrum needs 1..32 complete overlapping FFT windows.",
        ));
    }
    validate_samples(samples, samples.len())?;
    zero_frequency_hz
        .checked_add((length as u32 - 1) * spacing.frequency_hz())
        .ok_or_else(|| invalid("Spectrum frequency grid overflows Hz."))?;
    let window: Vec<_> = (0..length)
        .map(|index| 0.5 - 0.5 * (TAU * index as f64 / length as f64).cos())
        .collect();
    let energy: f64 = window.iter().map(|weight| weight * weight).sum();
    let window_sum: f64 = window.iter().sum();
    let scale = length as f64 / (f64::from(SAMPLE_RATE_HZ) * energy);
    let plan = ForwardTransform::new(spacing);
    let mut density = vec![0.0; length];
    let averages = 1 + (samples.len() - length) / hop;
    for frame in 0..averages {
        let input: Vec<_> = samples[frame * hop..frame * hop + length]
            .iter()
            .zip(&window)
            .map(|(point, weight)| point.map(|value| value * weight))
            .collect();
        for (power, point) in density.iter_mut().zip(plan.apply(&input)?) {
            *power += (point[0] * point[0] + point[1] * point[1]) * scale / averages as f64;
        }
    }
    if density.iter().any(|power| !power.is_finite()) {
        return Err(invalid("Spectrum power overflows normalized units."));
    }
    Ok(PowerSpectrum {
        zero_frequency_hz,
        spacing_hz: spacing.frequency_hz(),
        fft_samples: length,
        averages,
        equivalent_noise_bandwidth_hz: f64::from(SAMPLE_RATE_HZ) * energy
            / (window_sum * window_sum),
        density,
    })
}

impl PowerSpectrum {
    /// Queries a bounded min/max viewport without altering retained full resolution.
    /// # Arguments
    /// * `start` - First bin, inside the retained spectrum.
    /// * `count` - Positive bin count fully inside the retained spectrum.
    /// # Returns
    /// At most 2048 contiguous buckets with explicit original bin spans and extrema.
    /// # Errors
    /// Rejects empty or out-of-range viewports; no interpolation or alignment occurs.
    pub fn buckets(
        &self,
        start: usize,
        count: usize,
    ) -> Result<Vec<SpectrumBucket>, SimulationError> {
        if count == 0
            || start
                .checked_add(count)
                .is_none_or(|end| end > self.density.len())
        {
            return Err(invalid("Spectrum viewport is outside retained FFT bins."));
        }
        let width = count.div_ceil(MAX_SPECTRUM_POINTS);
        Ok(self.density[start..start + count]
            .chunks(width)
            .enumerate()
            .map(|(index, powers)| {
                let minimum = powers.iter().copied().fold(f64::INFINITY, f64::min);
                let maximum = powers.iter().copied().fold(0.0, f64::max);
                let first_bin = start + index * width;
                SpectrumBucket {
                    first_bin,
                    last_bin: first_bin + powers.len() - 1,
                    minimum_db: density_db(minimum),
                    maximum_db: density_db(maximum),
                }
            })
            .collect())
    }
}
