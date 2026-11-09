/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Integer sample-clock schedule; roll-off overlaps instead of extending cadence.
use super::{settings::SAMPLE_RATE_HZ, TransmissionSettings};
use crate::LabError;
use serde::Serialize;

/// Complete-symbol accounting within a one-second observation interval.
///
/// Does not include interleaver warm-up/drain or window tails. Those require the
/// actual transmitter/receiver state and must be reported separately by a runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct TransmissionTiming {
    pub sample_rate_hz: u32,
    pub fft_samples: u32,
    pub symbol_period_samples: u32,
    pub windowed_symbol_samples: u32,
    pub observation_samples: u32,
    pub complete_symbols: u32,
    pub unused_samples: u32,
}
impl TransmissionSettings {
    /// Calculates the one-second noiseless experiment's sample-clock schedule.
    /// # Returns
    /// Whole symbols wholly inside one second, plus unused sample-clock ticks.
    /// # Errors
    /// Rejects invalid transmission settings before calculating timing.
    pub fn one_second_timing(&self) -> Result<TransmissionTiming, LabError> {
        self.validate()?;
        let fft_samples = self.subcarrier_spacing.fft_samples();
        let symbol_period_samples = fft_samples + u32::from(self.cyclic_prefix_samples);
        Ok(TransmissionTiming {
            sample_rate_hz: SAMPLE_RATE_HZ,
            fft_samples,
            symbol_period_samples,
            windowed_symbol_samples: symbol_period_samples + u32::from(self.roll_off_samples),
            observation_samples: SAMPLE_RATE_HZ,
            complete_symbols: SAMPLE_RATE_HZ / symbol_period_samples,
            unused_samples: SAMPLE_RATE_HZ % symbol_period_samples,
        })
    }
}
