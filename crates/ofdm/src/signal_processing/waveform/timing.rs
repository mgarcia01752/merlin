/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Window timing and ideal homogeneous-cable delay budget; not receiver immunity.
use super::{windowing::TransmitWindow, WaveformGeometry};
use crate::{error::invalid, transmission::settings::SAMPLE_RATE_HZ, SimulationError};
use serde::Serialize;

const SPEED_OF_LIGHT_METRES_PER_SECOND: f64 = 299_792_458.0;
const METRES_PER_FOOT: f64 = 0.3048;

/// One bounded window-coefficient display point; neither voltage nor measured IQ.
#[derive(Debug, Serialize)]
pub struct WindowPoint {
    /// Fraction across the prefix-start zoom, including some useful FFT samples.
    pub x: f64,
    /// Actual dimensionless transmit-window coefficient.
    pub coefficient: f64,
}

/// Frozen geometry and ideal timing-only echo budget with zero reserved margin.
#[derive(Debug, Serialize)]
pub struct SymbolTiming {
    pub prefix_samples: usize,
    pub roll_off_samples: usize,
    pub fft_samples: usize,
    pub prefix_us: f64,
    pub roll_off_us: f64,
    pub useful_us: f64,
    pub symbol_us: f64,
    pub guard_us: f64,
    pub zoom_us: f64,
    pub prefix_fraction: f64,
    pub roll_off_fraction: f64,
    /// None means cable velocity is unknown; never silently infer a cable type.
    pub extra_path_feet: Option<f64>,
    /// Half the extra path, valid only for an explicitly assumed two-way echo.
    pub reflection_spacing_feet: Option<f64>,
    /// At most 130 display points from the existing half-sample TX window.
    pub points: Vec<WindowPoint>,
}

/// Computes prefix/window timing and an optional ideal uniform-cable path budget.
/// # Arguments
/// * `geometry` - Validated frozen transmitter geometry, independent of any GUI.
/// * `velocity_factor` - Cable speed divided by c in (0,1], or None if unknown.
/// # Returns
/// Bounded window coefficients, times in µs, and optional path/round-trip feet.
/// Assumes zero reserved receiver timing margin; does not predict echo strength,
/// maximum plant reach, equalizer capability or guaranteed decoding performance.
/// # Errors
/// Rejects nonfinite, zero, negative or superluminal velocity factors.
pub fn symbol_timing(
    geometry: &WaveformGeometry,
    velocity_factor: Option<f64>,
) -> Result<SymbolTiming, SimulationError> {
    if velocity_factor.is_some_and(|value| !value.is_finite() || value <= 0.0 || value > 1.0) {
        return Err(invalid(
            "Cable velocity factor must be finite and in (0, 1].",
        ));
    }
    let prefix = geometry.prefix_samples();
    let roll = geometry.roll_off_samples();
    let zoom_samples = prefix + prefix / 4;
    let microseconds = |samples: usize| samples as f64 * 1e6 / f64::from(SAMPLE_RATE_HZ);
    let guard_seconds = (prefix - roll) as f64 / f64::from(SAMPLE_RATE_HZ);
    let extra_path_feet = velocity_factor.map(|velocity| {
        guard_seconds * SPEED_OF_LIGHT_METRES_PER_SECOND * velocity / METRES_PER_FOOT
    });
    let window = TransmitWindow::new(geometry);
    let mut indices: Vec<_> = (0..=128).map(|index| zoom_samples * index / 128).collect();
    if roll > 0 {
        indices.push(roll - 1);
    }
    indices.sort_unstable();
    indices.dedup();
    let points = indices
        .into_iter()
        .map(|index| WindowPoint {
            x: index as f64 / zoom_samples as f64,
            coefficient: window.coefficients()[index],
        })
        .collect();
    Ok(SymbolTiming {
        prefix_samples: prefix,
        roll_off_samples: roll,
        fft_samples: geometry.fft_samples(),
        prefix_us: microseconds(prefix),
        roll_off_us: microseconds(roll),
        useful_us: microseconds(geometry.fft_samples()),
        symbol_us: microseconds(geometry.stride_samples()),
        guard_us: guard_seconds * 1e6,
        zoom_us: microseconds(zoom_samples),
        prefix_fraction: prefix as f64 / zoom_samples as f64,
        roll_off_fraction: roll as f64 / zoom_samples as f64,
        extra_path_feet,
        reflection_spacing_feet: extra_path_feet.map(|distance| distance / 2.0),
        points,
    })
}
