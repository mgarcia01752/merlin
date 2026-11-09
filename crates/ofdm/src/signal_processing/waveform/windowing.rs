/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.4.10.1 cyclic suffix and half-sample Tukey taper, delegated by7.5.8.1.
use super::{samples::validate_samples, WaveformGeometry};
use crate::{types::IqPair, SimulationError};
use std::f64::consts::PI;
/// Frozen TX extension/window coefficients, independently inspectable for verification.
pub struct TransmitWindow {
    geometry: WaveformGeometry,
    coefficients: Vec<f64>,
}
impl TransmitWindow {
    /// Precomputes the symmetric taper for a validated per-run configuration.
    /// # Arguments
    /// * `geometry` - Selected FFT/prefix/roll-off snapshot copied into this stage.
    /// # Returns
    /// Independent window owner; zero roll-off gives a rectangular evaluation window.
    pub fn new(geometry: &WaveformGeometry) -> Self {
        let mut coefficients = vec![1.0; geometry.extended_samples()];
        let roll = geometry.roll_off_samples();
        for index in 0..roll {
            let rising = 0.5 * (1.0 - (PI * (index as f64 + 0.5) / roll as f64).cos());
            coefficients[index] = rising;
            coefficients[geometry.extended_samples() - 1 - index] = rising;
        }
        Self {
            geometry: geometry.clone(),
            coefficients,
        }
    }
    /// Borrows the complete window in increasing extended-symbol sample order.
    /// # Returns
    /// Dimensionless symmetric coefficients, with explicit half-sample taper centers.
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }
    /// Appends a cyclic suffix then applies the frozen transmit window.
    /// # Arguments
    /// * `prefixed` - N+CP finite samples from TX prefix insertion.
    /// # Returns
    /// N+CP+RP windowed samples, before overlap-add; original input remains unchanged.
    /// # Errors
    /// Rejects wrong size or nonfinite samples before extension.
    pub fn apply(&self, prefixed: &[IqPair]) -> Result<Vec<IqPair>, SimulationError> {
        validate_samples(prefixed, self.geometry.stride_samples())?;
        let mut extended = prefixed.to_vec();
        extended.extend_from_slice(
            &prefixed[self.geometry.prefix_samples()
                ..self.geometry.prefix_samples() + self.geometry.roll_off_samples()],
        );
        for (sample, coefficient) in extended.iter_mut().zip(&self.coefficients) {
            *sample = sample.map(|value| value * coefficient);
        }
        Ok(extended)
    }
}
