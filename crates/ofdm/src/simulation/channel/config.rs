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
    SimulationError,
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
    pub source: crate::simulation::source::PayloadSourceConfig,
    pub data_bit_loading: Option<Vec<u8>>,
    pub impairment_timeline: Vec<super::timeline::ImpairmentEpoch>,
    pub protected_constellation: Option<super::protected_constellation::ProtectedCaptureSelection>,
    pub receiver_mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    /// Optional bounded payload-only raw/corrected constellation observations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellation: Option<super::constellation::ConstellationSelection>,
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
            source: Default::default(),
            data_bit_loading: None,
            impairment_timeline: Vec::new(),
            receiver_mode: Default::default(),
            protected_constellation: None,
            constellation: None,
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
    pub fn validate(&self) -> Result<DurationTiming, SimulationError> {
        if self.duration > Duration::from_secs(MAX_CHANNEL_DURATION_SECONDS)
            || !self.llr_variance.is_finite()
            || self.llr_variance <= 0.0
            || !(1..=MAX_CHANNEL_DECODER_ITERATIONS).contains(&self.max_iterations)
            || !(1..=MAX_BLOCK_SAMPLES).contains(&self.receive_block_samples)
        {
            return Err(invalid("Invalid channel duration or receiver work limits."));
        }
        let timing = self.transmission.duration_timing(self.duration)?;
        if matches!(
            self.source,
            crate::simulation::source::PayloadSourceConfig::Prbs15
        ) {
            Prbs15::new(self.prbs_seed)?;
        }
        if let Some(selection) = &self.protected_constellation {
            selection.validate()?;
        }
        let layout = CarrierLayout::new(&self.transmission, &self.layout)?;
        if let Some(physical) = &self.data_bit_loading {
            let loading =
                crate::transmission::layout::bit_loading::source_bit_loading(&layout, physical)?;
            crate::signal_processing::modulation::interleaving::mixed::PayloadBitOrder::new(
                &loading,
            )?;
        }
        if let Some(selection) = &self.constellation {
            selection.validate(&layout)?;
        }
        self.impairments
            .plan(&self.transmission, &layout, self.llr_variance)?;
        let retained_samples = self
            .impairment_timeline
            .iter()
            .try_fold(0usize, |total, epoch| {
                let count = match &epoch.impairments.noise {
                    crate::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } => {
                        frame.mer_db.len()
                    }
                    _ => 0,
                };
                total.checked_add(count)?.checked_add(
                    epoch
                        .impairments
                        .response
                        .as_ref()
                        .map_or(0, |frame| frame.iq.len()),
                )
            });
        if retained_samples.is_none_or(|count| count > 4_000_000) {
            return Err(invalid(
                "Measurement timeline exceeds the retained-sample budget.",
            ));
        }
        if self.impairment_timeline.len() > 4096 {
            return Err(invalid("Too many channel measurement epochs."));
        }
        let mut previous = 0;
        let mut previous_capture = None;
        for epoch in &self.impairment_timeline {
            if epoch.first_observation_symbol <= previous
                || epoch.first_observation_symbol >= timing.complete_symbols
            {
                return Err(invalid(
                    "Measurement transitions must increase within the observation window.",
                ));
            }
            if !matches!(
                self.impairments.noise,
                crate::signal_processing::channel::impairments::NoiseModel::Rxmer { .. }
            ) || !matches!(
                epoch.impairments.noise,
                crate::signal_processing::channel::impairments::NoiseModel::Rxmer { .. }
            ) || epoch.impairments.seed != self.impairments.seed
            {
                return Err(invalid(
                    "Measurement timeline requires one continuous seeded RxMER noise stream.",
                ));
            }
            let crate::signal_processing::channel::impairments::NoiseModel::Rxmer { frame: base } =
                &self.impairments.noise
            else {
                unreachable!()
            };
            let crate::signal_processing::channel::impairments::NoiseModel::Rxmer { frame: next } =
                &epoch.impairments.noise
            else {
                unreachable!()
            };
            if base.key != next.key
                || next.capture_time != epoch.capture_time
                || next.capture_time <= previous_capture.unwrap_or(base.capture_time)
            {
                return Err(invalid("Timeline identity/grid/epoch mismatch."));
            }
            epoch
                .impairments
                .plan(&self.transmission, &layout, self.llr_variance)?;
            previous = epoch.first_observation_symbol;
            previous_capture = Some(epoch.capture_time);
        }
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
