/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen per-run scientific geometry and bounded duration policy.
use crate::{
    error::invalid,
    signal_processing::waveform::samples::MAX_BLOCK_SAMPLES,
    simulation::source::{Prbs15, PrbsSeed},
    transmission::{
        layout::{CarrierLayout, CarrierLayoutSettings},
        packing::NcpModulation,
        DurationTiming, TransmissionSettings,
    },
    LabError,
};
use std::time::Duration;
/// Application work bound, not a DOCSIS transmission-duration limit.
pub const MAX_CHANNEL_DURATION_SECONDS: u64 = 3600;
/// Payload/control receiver work limit, matching the current codec admission policy.
pub const MAX_CHANNEL_DECODER_ITERATIONS: usize = 100;
/// Supply enough queued words for the maximum fixed256-QAM physical domain.
pub(crate) const QUEUED_PAYLOAD_TARGET: usize = 8;
pub(crate) const MAX_ACQUISITION_SYMBOLS: u64 = 8 * crate::types::PLC_FRAME_SYMBOLS;
pub(crate) const MAX_COMPLETION_SYMBOLS: u64 = 4096;
/// Independent settings snapshot; callers may edit it before creating a session.
#[derive(Clone, serde::Serialize)]
pub struct ChannelRunSettings {
    /// Exclusive injected residual source and optional supplied relative response H.
    pub impairments: crate::signal_processing::channel::impairments::ChannelImpairments,
    pub transmission: TransmissionSettings,
    pub layout: CarrierLayoutSettings,
    /// Source observation interval; acquisition, completion and drain are additional.
    pub duration: Duration,
    pub ncp_modulation: NcpModulation,
    pub prbs_seed: PrbsSeed,
    /// Positive finite noiseless demapper reference variance. Actual noise modes
    /// supply their own per-bin residual variances without changing this fallback.
    pub llr_variance: f64,
    pub max_iterations: usize,
    /// Maximum samples passed to one RX call; independent of observation duration.
    pub receive_block_samples: usize,
}
impl ChannelRunSettings {
    /// Selects the initial one-second PRBS15/QPSK-control experiment policy.
    ///
    /// # Arguments
    /// * `transmission` - Fixed numerology, frozen by session construction.
    /// * `layout` - Explicit protected roles/pilot positions; no implicit pilot synthesis.
    /// # Returns
    /// Editable settings; validation occurs before any session processing.
    pub fn new(transmission: TransmissionSettings, layout: CarrierLayoutSettings) -> Self {
        Self {
            impairments: Default::default(),
            transmission,
            layout,
            duration: Duration::from_secs(1),
            ncp_modulation: NcpModulation::Qpsk,
            prbs_seed: 0x7fff,
            llr_variance: 0.01,
            max_iterations: 50,
            receive_block_samples: 4096,
        }
    }
    /// Validates duration, geometry, source and receiver work limits.
    ///
    /// # Returns
    /// Integer source-observation schedule, with unused ticks explicit.
    /// # Errors
    /// Rejects invalid settings, less than one symbol, duration above one hour,
    /// invalid PRBS seed/LLRs/iterations or unsupported sample block sizes.
    pub fn validate(&self) -> Result<DurationTiming, LabError> {
        if self.duration > Duration::from_secs(MAX_CHANNEL_DURATION_SECONDS)
            || !self.llr_variance.is_finite()
            || self.llr_variance <= 0.0
            || !(1..=MAX_CHANNEL_DECODER_ITERATIONS).contains(&self.max_iterations)
            || !(1..=MAX_BLOCK_SAMPLES).contains(&self.receive_block_samples)
        {
            return Err(invalid("Invalid channel duration or receiver work limits."));
        }
        let timing = self.transmission.duration_timing(self.duration)?;
        Prbs15::new(self.prbs_seed)?;
        let layout = CarrierLayout::new(&self.transmission, &self.layout)?;
        self.impairments
            .plan(&self.transmission, &layout, self.llr_variance)?;
        // Reserve representable absolute identities for bounded acquisition/completion/drain.
        timing
            .complete_symbols
            .checked_add(
                MAX_ACQUISITION_SYMBOLS
                    + MAX_COMPLETION_SYMBOLS
                    + u64::from(
                        self.transmission
                            .subcarrier_spacing
                            .max_time_interleaver_depth(),
                    ),
            )
            .and_then(|symbols| symbols.checked_mul(u64::from(timing.symbol_period_samples)))
            .and_then(|samples| samples.checked_add(u64::from(self.transmission.roll_off_samples)))
            .ok_or_else(|| invalid("Channel sample plan exceeds64-bit accounting."))?;
        Ok(timing)
    }
}
