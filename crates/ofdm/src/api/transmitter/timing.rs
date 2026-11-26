/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Authoritative timing/window coefficients, separate from presentation coordinates.
use super::*;
pub use crate::signal_processing::waveform::timing::{SymbolTiming, WindowPoint};
use crate::signal_processing::waveform::{windowing::TransmitWindow, WaveformGeometry};
use serde::Serialize;

/// Complete finite window coefficient snapshot on the existing transmitter sample clock.
#[derive(Serialize)]
pub struct SymbolWindow {
    pub sample_rate_hz: u32,
    pub fft_samples: usize,
    pub prefix_samples: usize,
    pub roll_off_samples: usize,
    pub stride_samples: usize,
    pub coefficients: Vec<f64>,
}
impl TransmissionConfiguration {
    /// Queries existing symbol/window timing and optional ideal uniform-cable delay budget.
    /// # Arguments
    /// * `velocity_factor` - Explicit finite speed/c in (0,1], or None for unknown cable.
    /// # Returns
    /// Actual bounded coefficients/times and optional ideal path feet, not echo immunity.
    /// # Errors
    /// Rejects invalid velocity through the existing numerical owner; starts no source/run.
    pub fn symbol_timing(&self, velocity_factor: Option<f64>) -> ApiResult<SymbolTiming> {
        Ok(crate::signal_processing::waveform::timing::symbol_timing(
            &WaveformGeometry::new(self.settings())?,
            velocity_factor,
        )?)
    }

    /// Reads original sample-clock window coefficients without creating a waveform stream.
    /// # Returns
    /// Owned complete weights and geometry for presentation of adjacent windows.
    /// # Errors
    /// Preserves existing geometry admission failures; no cable behavior is inferred.
    pub fn window(&self) -> ApiResult<SymbolWindow> {
        let geometry = WaveformGeometry::new(self.settings())?;
        let window = TransmitWindow::new(&geometry);
        Ok(SymbolWindow {
            sample_rate_hz: crate::transmission::settings::SAMPLE_RATE_HZ,
            fft_samples: geometry.fft_samples(),
            prefix_samples: geometry.prefix_samples(),
            roll_off_samples: geometry.roll_off_samples(),
            stride_samples: geometry.stride_samples(),
            coefficients: window.coefficients().to_vec(),
        })
    }
}
