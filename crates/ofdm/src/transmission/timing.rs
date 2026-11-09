/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Integer sample-clock schedule; roll-off overlaps instead of extending cadence.
use super::{settings::SAMPLE_RATE_HZ, TransmissionSettings};
use crate::{error::invalid, LabError};
use serde::Serialize;
use std::time::Duration;

/// Nanoseconds per second, for integer duration-to-sample conversion.
const NANOS_PER_SECOND: u128 = 1_000_000_000;

/// Complete source-symbol schedule within a caller-selected observation interval.
/// Acquisition, queue completion, cell drain and window tails are separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct DurationTiming {
    pub sample_rate_hz: u32,
    pub fft_samples: u32,
    pub symbol_period_samples: u32,
    pub windowed_symbol_samples: u32,
    pub observation_samples: u64,
    pub complete_symbols: u64,
    /// Sample ticks left after the last complete source symbol; not transmitted.
    pub unused_samples: u64,
}

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
    /// Plans a duration using integer sample ticks and complete OFDM periods.
    ///
    /// # Arguments
    /// * `duration` - Positive observation time; fractions below one tick are floored.
    /// # Returns
    /// Checked64-bit sample/symbol accounting. Roll-off overlaps and adds no cadence.
    /// # Errors
    /// Rejects invalid numerology, less than one complete symbol or sample overflow.
    pub fn duration_timing(&self, duration: Duration) -> Result<DurationTiming, LabError> {
        let ticks = duration
            .as_nanos()
            .checked_mul(u128::from(SAMPLE_RATE_HZ))
            .ok_or_else(|| invalid("Observation duration multiplication overflow."))?
            / NANOS_PER_SECOND;
        let ticks = u64::try_from(ticks)
            .map_err(|_| invalid("Observation duration exceeds64-bit sample accounting."))?;
        self.sample_timing(ticks)
    }

    /// Plans an exact sample-clock observation budget without floating-point time.
    ///
    /// # Arguments
    /// * `observation_samples` - Available ticks at204.8Msamples/s, at least one period.
    /// # Returns
    /// Whole source symbols and unused trailing ticks, without warm-up/drain.
    /// # Errors
    /// Rejects invalid settings or an interval shorter than one complete symbol.
    pub fn sample_timing(&self, observation_samples: u64) -> Result<DurationTiming, LabError> {
        self.validate()?;
        let fft_samples = self.subcarrier_spacing.fft_samples();
        let symbol_period_samples = fft_samples + u32::from(self.cyclic_prefix_samples);
        if observation_samples < u64::from(symbol_period_samples) {
            return Err(invalid(
                "Observation must contain at least one complete OFDM symbol.",
            ));
        }
        Ok(DurationTiming {
            sample_rate_hz: SAMPLE_RATE_HZ,
            fft_samples,
            symbol_period_samples,
            windowed_symbol_samples: symbol_period_samples + u32::from(self.roll_off_samples),
            observation_samples,
            complete_symbols: observation_samples / u64::from(symbol_period_samples),
            unused_samples: observation_samples % u64::from(symbol_period_samples),
        })
    }

    /// Calculates the one-second noiseless experiment's sample-clock schedule.
    /// # Returns
    /// Whole symbols wholly inside one second, plus unused sample-clock ticks.
    /// # Errors
    /// Rejects invalid transmission settings before calculating timing.
    pub fn one_second_timing(&self) -> Result<TransmissionTiming, LabError> {
        let timing = self.duration_timing(Duration::from_secs(1))?;
        Ok(TransmissionTiming {
            sample_rate_hz: timing.sample_rate_hz,
            fft_samples: timing.fft_samples,
            symbol_period_samples: timing.symbol_period_samples,
            windowed_symbol_samples: timing.windowed_symbol_samples,
            observation_samples: timing.observation_samples as u32,
            complete_symbols: timing.complete_symbols as u32,
            unused_samples: timing.unused_samples as u32,
        })
    }
}
