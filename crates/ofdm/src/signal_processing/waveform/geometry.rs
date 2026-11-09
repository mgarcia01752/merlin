/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated immutable numerology shared by independently owned TX and RX stages.
use crate::{transmission::TransmissionSettings, SimulationError};
/// Per-run snapshot; changing prefix or roll-off requires constructing new stages.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct WaveformGeometry {
    settings: TransmissionSettings,
}
impl WaveformGeometry {
    /// Validates and copies a user-selected configuration before processing.
    /// # Arguments
    /// * `settings` - PHY-permitted spacing, prefix, roll-off and interleaving values.
    /// # Returns
    /// Owned immutable snapshot; later caller edits cannot change active stage geometry.
    /// # Errors
    /// Rejects settings unsupported by the existing transmission validator.
    pub fn new(settings: &TransmissionSettings) -> Result<Self, SimulationError> {
        settings.validate()?;
        Ok(Self {
            settings: settings.clone(),
        })
    }
    /// Borrows the validated snapshot.
    /// # Returns
    /// Read-only settings; occupied bandwidth does not determine FFT length.
    pub fn settings(&self) -> &TransmissionSettings {
        &self.settings
    }
    /// Reports useful symbol length in complex samples.
    /// # Returns
    /// 4096 or 8192, determined solely by spacing at 204.8 Msamples/s.
    pub fn fft_samples(&self) -> usize {
        self.settings.subcarrier_spacing.fft_samples() as usize
    }
    /// Reports the selected prefix length.
    /// # Returns
    /// Sample count held constant throughout this stage's session.
    pub fn prefix_samples(&self) -> usize {
        usize::from(self.settings.cyclic_prefix_samples)
    }
    /// Reports the selected taper/overlap length.
    /// # Returns
    /// Sample count strictly smaller than the prefix; zero is evaluation-only.
    pub fn roll_off_samples(&self) -> usize {
        usize::from(self.settings.roll_off_samples)
    }
    /// Reports spacing between successive symbol starts.
    /// # Returns
    /// FFT plus prefix samples; roll-off does not extend cadence.
    pub fn stride_samples(&self) -> usize {
        self.fft_samples() + self.prefix_samples()
    }
    /// Reports the independently windowed symbol's full length.
    /// # Returns
    /// Stride plus roll-off, before overlap-add with adjacent symbols.
    pub fn extended_samples(&self) -> usize {
        self.stride_samples() + self.roll_off_samples()
    }
}
