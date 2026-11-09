/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Tables38–40 and section7.5.6.1; fixed per-session numerology.
use crate::{error::invalid, types::FrequencyHz, SimulationError};
use serde::{Deserialize, Serialize};

/// Fixed downstream sample clock, in samples per second (PHY I20 section7.5.7).
pub const SAMPLE_RATE_HZ: FrequencyHz = 204_800_000;
/// Application scope: at most one192MHz downstream channel.
pub const MAX_CHANNEL_BANDWIDTH_HZ: FrequencyHz = 192_000_000;
/// Minimum occupied bandwidth, PHY I20 section7.5.9.
pub const MIN_CHANNEL_BANDWIDTH_HZ: FrequencyHz = 24_000_000;
/// Occupied bandwidth is an integer multiple of6MHz (PHY I20 section7.5.9).
pub const CHANNEL_BANDWIDTH_STEP_HZ: FrequencyHz = 6_000_000;
/// Initial data modulation:256-QAM, eight coded bits per data cell.
pub const DATA_BITS_PER_CELL: u8 = 8;
/// Permitted downstream cyclic-prefix lengths in sample-clock ticks.
pub const CYCLIC_PREFIX_SAMPLES: [u16; 5] = [192, 256, 512, 768, 1024];
/// Permitted downstream roll-off lengths; zero is evaluation-only.
pub const ROLL_OFF_SAMPLES: [u16; 5] = [0, 64, 128, 192, 256];

/// Supported downstream spacing; determines FFT size rather than duplicating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SubcarrierSpacing {
    #[serde(rename = "25000")]
    Khz25,
    #[serde(rename = "50000")]
    Khz50,
}
impl SubcarrierSpacing {
    /// Returns subcarrier separation in Hz.
    pub fn frequency_hz(self) -> FrequencyHz {
        match self {
            Self::Khz25 => 25_000,
            Self::Khz50 => 50_000,
        }
    }
    /// Returns the specified FFT size at the fixed204.8MHz sample clock.
    pub fn fft_samples(self) -> u32 {
        SAMPLE_RATE_HZ / self.frequency_hz()
    }
    /// Returns the mandatory excluded carrier count at each physical FFT edge.
    /// # Returns
    ///296for8K or148for4K, MULPI I25 section6.4.40/PDF199.
    pub fn mandatory_excluded_edge_bins(self) -> usize {
        match self {
            Self::Khz25 => 296,
            Self::Khz50 => 148,
        }
    }
    /// Returns the maximum time-interleaver depth for this spacing.
    pub fn max_time_interleaver_depth(self) -> u8 {
        match self {
            Self::Khz25 => 16,
            Self::Khz50 => 32,
        }
    }

    /// Returns the maximum reduced data/scattered-pilot domain size.
    /// # Returns
    ///7537for8K or3745for4K, from PHY I20 section7.5.6, PDF129.
    /// PLC, continuous pilots and excluded bins are outside this domain.
    pub fn max_interleaver_cells(self) -> usize {
        match self {
            Self::Khz25 => 7537,
            Self::Khz50 => 3745,
        }
    }
}

/// Fixed setup for one downstream channel; consumers validate and freeze per run.
///
/// This defines numerology, not a complete carrier map or operational CMTS.
/// Data modulation is currently fixed to256-QAM; pilot/control roles are separate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransmissionSettings {
    /// Occupied bandwidth in Hz, distinct from active/data-carrier bandwidth.
    pub occupied_bandwidth_hz: FrequencyHz,
    /// Selects both subcarrier separation and FFT length at the fixed clock.
    pub subcarrier_spacing: SubcarrierSpacing,
    /// Cyclic prefix length in204.8MHz sample-clock ticks.
    pub cyclic_prefix_samples: u16,
    /// Overlapped window taper length in sample-clock ticks.
    pub roll_off_samples: u16,
    /// Time-interleaver branch count; frequency interleaving has no user depth.
    pub time_interleaver_depth: u8,
}
impl Default for TransmissionSettings {
    /// Provides a selected192MHz,25kHz baseline; these are application defaults.
    fn default() -> Self {
        Self {
            occupied_bandwidth_hz: MAX_CHANNEL_BANDWIDTH_HZ,
            subcarrier_spacing: SubcarrierSpacing::Khz25,
            cyclic_prefix_samples: 512,
            roll_off_samples: 128,
            time_interleaver_depth: 16,
        }
    }
}
impl TransmissionSettings {
    /// Checks PHY numerology and the application's single-channel bandwidth cap.
    /// # Returns
    /// Unit for a supported configuration; zero roll-off is evaluation-only.
    /// # Errors
    /// Rejects unsupported bandwidth, prefix, roll-off or interleaver depth.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if !(MIN_CHANNEL_BANDWIDTH_HZ..=MAX_CHANNEL_BANDWIDTH_HZ)
            .contains(&self.occupied_bandwidth_hz)
            || self.occupied_bandwidth_hz % CHANNEL_BANDWIDTH_STEP_HZ != 0
        {
            return Err(invalid(
                "Occupied bandwidth must be24..192MHz in6MHz steps.",
            ));
        }
        if !CYCLIC_PREFIX_SAMPLES.contains(&self.cyclic_prefix_samples)
            || !ROLL_OFF_SAMPLES.contains(&self.roll_off_samples)
            || self.roll_off_samples >= self.cyclic_prefix_samples
        {
            return Err(invalid(
                "Unsupported CP/RP or roll-off is not shorter than the prefix.",
            ));
        }
        if !(1..=self.subcarrier_spacing.max_time_interleaver_depth())
            .contains(&self.time_interleaver_depth)
        {
            return Err(invalid(
                "Time-interleaver depth exceeds the spacing-specific range.",
            ));
        }
        Ok(())
    }
}
