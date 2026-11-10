/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared known-reference orchestration of existing physical/waveform/PLC stages.
use crate::{
    error::invalid,
    signal_processing::waveform::{SampleBlock, WaveformGeometry},
    simulation::{
        plc::{generate_frame, PlcAcquisitionReceiver, PlcFrameSignals},
        test_points::{
            PhysicalProbeGrid, ProbeContext, ProbeObserver, RunPhase, SignalReference,
            TestPointProbe,
        },
        waveform::{
            observation, receiver::ReceiveReference, ReceivedSymbol, WaveformReceiver,
            WaveformTransmitter,
        },
    },
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        packing::NcpModulation,
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            receiver_configuration::ReceiverConfigurationState,
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        TransmissionSettings,
    },
    types::{IqPair, PLC_FRAME_SYMBOLS},
    SimulationError,
};
const REFERENCE_VARIANCE: f64 = 0.01;
const REFERENCE_CHANNEL_ID: u8 = 7;
const REFERENCE_SOURCE_MAC: crate::types::MacAddress = [0, 0xca, 0, 0, 0, 1];
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LinkStats {
    pub data_signal_power: f64,
    pub data_error_power: f64,
    pub compared_data_carriers: u64,
    pub compared_bins: u64,
    pub maximum_bin_error: f64,
    pub plc_frames: u64,
    pub plc_information_bits: u64,
    pub plc_bit_errors: u64,
    pub plc_ldpc_iterations: Vec<usize>,
    pub preambles_match: bool,
}
pub(crate) struct WaveformLink {
    channel: super::impairment::ImpairedChannel,
    geometry: WaveformGeometry,
    layout: CarrierLayout,
    tx: WaveformTransmitter,
    rx: WaveformReceiver,
    plc_rx: PlcAcquisitionReceiver,
    scheduler: PlcMessageScheduler,
    frame: Option<PlcFrameSignals>,
    received_plc: Vec<IqPair>,
    block_samples: usize,
    calibrated_variances: Vec<f64>,
    raw_plc_frame: Vec<(usize, IqPair)>,
    pub channel_estimate:
        Option<crate::signal_processing::receiver::channel_estimation::ChannelEstimate>,
    context: ProbeContext,
    pub stats: LinkStats,
    pub protected_constellation: Option<super::protected_constellation::ProtectedConstellation>,
}
impl WaveformLink {
    /// Creates reference-only stage owners without passing TX truth into any RX operation.
    /// # Arguments
    /// * `settings` - Fixed TX and explicit RX bootstrap numerology.
    /// * `raw` - Known bootstrap carrier map; payload RX later uses decoded descriptors.
    /// * `ncp` - Fixed positive control constellation advertised through actual DPD.
    /// * `block_samples` - Validated positive bounded transfer chunk length.
    /// * `impairments` - Explicit residual source and optional relative response H.
    /// * `variance` - Noiseless fallback LLR variance, overridden by noise calibration.
    /// # Returns
    /// Fresh waveform/PLC streams with no sample history or configured payload receiver.
    /// # Errors
    /// Propagates configuration and stage initialization failures.
    pub(crate) fn new(
        settings: &TransmissionSettings,
        raw: &CarrierLayoutSettings,
        ncp: NcpModulation,
        block_samples: usize,
        impairments: &crate::signal_processing::channel::impairments::ChannelImpairments,
        variance: f64,
        data_loading: Option<&[u8]>,
    ) -> Result<Self, SimulationError> {
        let geometry = WaveformGeometry::new(settings)?;
        let layout = CarrierLayout::new(settings, raw)?;
        let identity = match &impairments.noise {
            crate::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } => {
                Some(&frame.key)
            }
            _ => impairments.response.as_ref().map(|response| &response.key),
        };
        let descriptors = FixedPlcDescriptors::with_data_loading(
            settings,
            &layout,
            DescriptorIdentity {
                channel_id: identity.map_or(REFERENCE_CHANNEL_ID, |key| key.channel_id),
                change_count: 0,
                source_mac: identity.map_or(REFERENCE_SOURCE_MAC, |key| key.mac_address),
                primary_capable: true,
                ncp_bits: ncp.order().ilog2() as u8,
            },
            data_loading,
        )?;
        let scheduler = PlcMessageScheduler::new(settings, &descriptors)?;
        let tx = WaveformTransmitter::new(&geometry, 0, 0);
        let channel =
            super::impairment::ImpairedChannel::new(&geometry, &layout, impairments, variance)?;
        let rx = WaveformReceiver::new(
            &geometry,
            raw,
            ReceiveReference {
                first_input_sample: 0,
                first_symbol_sample: 0,
                first_symbol_index: 0,
                reference_rms: 1.0,
                channel_response: channel.plan.response.clone(),
            },
        )?;
        let mut context = ProbeContext::new(
            "payload-waveform",
            "fixed-payload-reference",
            RunPhase::Warmup,
        )?;
        context.physical_grid = Some(PhysicalProbeGrid {
            spacing: settings.subcarrier_spacing,
            zero_frequency_hz: raw.zero_frequency_hz,
        });
        context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
        let calibrated_variances = channel.plan.llr_variance.clone();
        Ok(Self {
            protected_constellation: None,
            calibrated_variances,
            raw_plc_frame: Vec::new(),
            channel_estimate: None,
            channel,
            geometry,
            layout,
            tx,
            rx,
            plc_rx: PlcAcquisitionReceiver::new(settings.subcarrier_spacing)?,
            scheduler,
            frame: None,
            received_plc: Vec::new(),
            block_samples,
            context,
            stats: LinkStats {
                data_signal_power: 0.0,
                data_error_power: 0.0,
                compared_data_carriers: 0,
                compared_bins: 0,
                maximum_bin_error: 0.0,
                plc_frames: 0,
                plc_information_bits: 0,
                plc_bit_errors: 0,
                plc_ldpc_iterations: Vec::new(),
                preambles_match: true,
            },
        })
    }
    /// Selects receiver-owned pilot estimation before any sample is transferred.
    /// # Arguments
    /// * `raw` - Independent receiver bootstrap carrier assignments.
    /// # Returns
    /// Unit after replacing the unstarted reference receiver.
    /// # Errors
    /// Rejects receiver bootstrap settings; no channel response is supplied to RX.
    pub(crate) fn use_pilot_estimation(
        &mut self,
        raw: &CarrierLayoutSettings,
    ) -> Result<(), SimulationError> {
        self.rx = WaveformReceiver::from_pilots(&self.geometry, raw, 0, 0, 0, 1.0)?;
        Ok(())
    }
    /// Changes one physical measurement epoch without resetting waveform or RNG state.
    /// # Arguments
    /// * `impairments` - Validated same-grid measurement response/residual.
    /// * `variance` - Positive noiseless demapper reference fallback.
    /// * `mode` - Correction source; supplied H is never sent to estimated RX.
    /// # Returns
    /// Unit after replacing the channel plan and calibrated residuals.
    /// # Errors
    /// Rejects malformed measurement coverage or reference gains.
    pub(crate) fn transition(
        &mut self,
        impairments: &crate::signal_processing::channel::impairments::ChannelImpairments,
        variance: f64,
        mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    ) -> Result<(), SimulationError> {
        let plan = impairments.plan(self.geometry.settings(), &self.layout, variance)?;
        if mode
            == crate::signal_processing::receiver::channel_estimation::ReceiverMode::SuppliedChannel
        {
            self.rx.set_reference_response(&plan.response)?;
        }
        self.calibrated_variances = plan.llr_variance.clone();
        self.channel.transition(plan);
        Ok(())
    }
    /// Replaces residual variances without resetting source, receiver or RNG state.
    /// # Arguments
    /// * `snapshot` - Rust playback powers at the current observation symbol.
    /// * `grid` - Original exact measurement coordinates.
    /// # Returns
    /// Unit after atomically updating channel powers and calibrated LLR variances.
    /// # Errors
    /// Rejects missing/invalid active powers or incompatible physical coverage.
    pub(crate) fn residual_transition(
        &mut self,
        snapshot: &super::measurement_playback::ResidualSnapshot,
        grid: &crate::scenario::FrequencyGrid,
    ) -> Result<(), SimulationError> {
        let mut powers = self.channel.plan.residual_power.clone();
        let mut variances = self.calibrated_variances.clone();
        for (bin, role) in self.layout.static_roles().iter().enumerate() {
            if *role == PhysicalCarrierRole::Excluded {
                continue;
            }
            let frequency = self
                .layout
                .frequency_hz(bin as u16)
                .ok_or_else(|| invalid("Missing playback frequency."))?;
            let offset = crate::signal_processing::channel::impairments::offset(
                grid,
                frequency,
                self.geometry.settings().subcarrier_spacing.frequency_hz(),
            )?;
            let power = snapshot
                .power
                .get(offset)
                .copied()
                .flatten()
                .ok_or_else(|| invalid("Missing playback residual."))?;
            if !power.is_finite()
                || power <= 0.0
                || power > crate::signal_processing::channel::MAX_NORMALIZED_NOISE_POWER
            {
                return Err(invalid("Invalid playback residual power."));
            }
            powers[bin] = power;
            variances[bin] = power;
        }
        self.channel.plan.residual_power = powers;
        self.channel.plan.llr_variance = variances.clone();
        self.calibrated_variances = variances;
        Ok(())
    }
    /// Borrows the actual PLC acquisition gate after received frame processing.
    /// # Returns
    /// Immutable live state; ready snapshots originate only in decoded management packets.
    pub(crate) fn state(&self) -> &ReceiverConfigurationState {
        self.plc_rx.configuration()
    }
    /// Copies run metadata for post-computation observations.
    /// # Arguments
    /// * `phase` - Explicit acquisition/warm-up, observation or drain interval.
    /// # Returns
    /// Caller-owned context without stage-specific source/word identities.
    pub(crate) fn context(&self, phase: RunPhase) -> ProbeContext {
        let mut context = self.context.clone();
        context.phase = phase;
        context
    }
    /// Builds an explicit reference filler plane while payload acquisition is closed.
    /// # Arguments
    /// * `index` - Absolute PLC-aligned channel symbol phase.
    /// # Returns
    /// Unity BPSK data-plane filler and empty scattered positions; never payload truth.
    pub(crate) fn acquisition_cells(&self, index: u64) -> Vec<Option<IqPair>> {
        self.layout
            .symbol_roles(index)
            .iter()
            .filter_map(|role| match role {
                PhysicalCarrierRole::Data => Some(Some([1.0, 0.0])),
                PhysicalCarrierRole::ScatteredPilot => Some(None),
                _ => None,
            })
            .collect()
    }
    /// Streams one complete physical symbol through actual waveform and independent PLC RX.
    /// # Arguments
    /// * `index` - Consecutive absolute channel symbol starting at zero.
    /// * `reduced` - Complete data/scattered input from explicit acquisition or payload TX.
    /// * `phase` - Scientific run interval for probe metadata.
    /// * `probe` - Shared finite optional observation budget.
    /// * `observer` - Read-only sink receiving completed stage data.
    /// # Returns
    /// One received symbol; PLC frame completion also updates the independent gate/statistics.
    /// # Errors
    /// Propagates stage, control transport/admission or observer failure; verifier stops on error.
    pub(crate) fn transfer(
        &mut self,
        index: u64,
        reduced: &[Option<IqPair>],
        phase: RunPhase,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<ReceivedSymbol, SimulationError> {
        if index % PLC_FRAME_SYMBOLS == 0 {
            self.raw_plc_frame.clear();
            let timestamp = ExtendedTimestamp::from_sample_ticks(
                index
                    .checked_mul(self.geometry.stride_samples() as u64)
                    .ok_or_else(|| invalid("PLC sample timestamp overflow."))?,
                0,
            )?;
            self.frame = Some(generate_frame(
                PlcMessageFrame::new(
                    self.geometry.settings().subcarrier_spacing,
                    timestamp,
                    self.scheduler.next_block()?.block,
                )?,
                index / PLC_FRAME_SYMBOLS,
            )?);
        }
        let frame = self
            .frame
            .as_ref()
            .ok_or_else(|| invalid("Missing reference PLC frame."))?;
        let plc = frame
            .symbol_iq(index)
            .ok_or_else(|| invalid("Missing reference PLC column."))?;
        let physical = assemble(&self.layout, index, reduced, plc, 1.0)?;
        let transmitted = self.tx.transmit(&physical)?;
        let mut context = self.context(phase);
        context.channel_symbol_index = Some(index);
        context.plc_phase = Some((index % PLC_FRAME_SYMBOLS) as u8);
        observation::observe_transmit(&transmitted, &self.geometry, &context, probe, observer)?;
        observation::observe_channel(
            &transmitted.output,
            false,
            &self.geometry,
            &context,
            probe,
            observer,
        )?;
        let mut completed = Vec::new();
        let channel_output = self.channel.transfer(&physical, &transmitted.output)?;
        for (offset, chunk) in channel_output
            .samples()
            .chunks(self.block_samples)
            .enumerate()
        {
            let input = SampleBlock::new(
                transmitted.output.start_index() + (offset * self.block_samples) as u64,
                chunk.to_vec(),
            )?;
            let received = input;
            observation::observe_channel(
                &received,
                true,
                &self.geometry,
                &context,
                probe,
                observer,
            )?;
            completed.extend(self.rx.receive(&received)?);
        }
        if completed.len() != 1 {
            return Err(invalid(
                "Reference transfer did not produce exactly one received symbol.",
            ));
        }
        let received = completed
            .pop()
            .ok_or_else(|| invalid("Missing completed RX symbol."))?;
        self.channel_estimate = received.channel_estimate.clone();
        if received.channel_estimate.is_some() {
            for (bin, role) in self.layout.static_roles().iter().enumerate() {
                if *role == PhysicalCarrierRole::Plc {
                    self.raw_plc_frame.push((bin, received.bins[bin]));
                }
            }
            for (bin, calibrated) in self.calibrated_variances.iter_mut().enumerate() {
                let gain = self.channel.plan.response[bin];
                // Raw variance is a simulation calibration, not an H input to RX.
                let raw =
                    self.channel.plan.llr_variance[bin] * (gain[0] * gain[0] + gain[1] * gain[1]);
                let effective = raw * received.inverse_gain_power[bin];
                if !effective.is_finite() || effective <= 0.0 {
                    return Err(invalid(
                        "Estimated receiver noise calibration is not finite positive power.",
                    ));
                }
                *calibrated = effective;
            }
        }
        if let Some(capture) = &mut self.protected_constellation {
            capture.observe(&received, &self.layout)?;
        }
        observation::observe_receive(&received, &self.geometry, &context, probe, observer)?;
        for (bin, (sample, expected)) in received.equalized.iter().zip(&physical.bins).enumerate() {
            let expected = expected
                .as_ref()
                .ok_or_else(|| invalid("Incomplete TX grid in payload reference."))?;
            self.stats.maximum_bin_error = self
                .stats
                .maximum_bin_error
                .max((sample[0] - expected[0]).hypot(sample[1] - expected[1]));
            self.stats.compared_bins += 1;
            if phase == RunPhase::Observation && physical.roles[bin] == PhysicalCarrierRole::Data {
                self.stats.data_signal_power +=
                    expected[0] * expected[0] + expected[1] * expected[1];
                self.stats.data_error_power +=
                    (sample[0] - expected[0]).powi(2) + (sample[1] - expected[1]).powi(2);
                self.stats.compared_data_carriers += 1;
            }
        }
        self.received_plc.extend_from_slice(&received.signals.plc);
        if index % PLC_FRAME_SYMBOLS == PLC_FRAME_SYMBOLS - 1 {
            if let Some(estimate) = &received.channel_estimate {
                if !estimate.ready {
                    self.received_plc.clear();
                    return Ok(received);
                }
                // Bounded block acquisition: use the completed pilot pattern to
                // correct this frame's retained raw PLC samples, without TX truth.
                self.received_plc = self
                    .raw_plc_frame
                    .iter()
                    .map(|(bin, point)| {
                        let gain = estimate.response[*bin];
                        let power = gain[0] * gain[0] + gain[1] * gain[1];
                        [
                            (point[0] * gain[0] + point[1] * gain[1]) / power,
                            (point[1] * gain[0] - point[0] * gain[1]) / power,
                        ]
                    })
                    .collect();
            }
            let decision =
                self.plc_rx
                    .receive_frame(&self.received_plc, self.plc_variance(), 50)?;
            self.received_plc.clear();
            if decision.transport_error.is_some()
                || decision
                    .admissions
                    .iter()
                    .any(|admission| admission.error.is_some())
            {
                return Err(invalid(
                    "PLC transport/configuration failed in payload reference.",
                ));
            }
            let message = decision
                .frame
                .message
                .as_ref()
                .ok_or_else(|| invalid("PLC decode failed in payload reference."))?;
            let truth = self
                .frame
                .as_ref()
                .ok_or_else(|| invalid("Missing independent PLC truth."))?
                .message
                .to_bytes();
            let decoded = message.to_bytes();
            if truth.len() != decoded.len() {
                return Err(invalid("PLC truth comparison length mismatch."));
            }
            self.stats.plc_information_bits += truth.len() as u64 * 8;
            self.stats.plc_bit_errors += truth
                .iter()
                .zip(decoded)
                .map(|(left, right)| (left ^ right).count_ones() as u64)
                .sum::<u64>();
            self.stats.plc_frames += 1;
            self.stats.preambles_match &= decision.frame.preamble_signs_match;
            self.stats
                .plc_ldpc_iterations
                .extend(decision.frame.blocks.iter().map(|block| block.iterations));
        }
        Ok(received)
    }
    /// Transfers the final TX window tail without fabricating another complete symbol.
    /// # Arguments
    /// * `probe` - Shared optional bounded selector.
    /// * `observer` - Read-only channel-plane observation sink.
    /// # Returns
    /// Explicit incomplete RX sample remainder, normally the selected roll-off length.
    /// # Errors
    /// Propagates stage/callback failure or an unexpected extra recovered period.
    pub(crate) fn finish(
        &mut self,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<usize, SimulationError> {
        let tail = self.tx.finish()?;
        let context = self.context(RunPhase::Drain);
        observation::observe_channel(&tail, false, &self.geometry, &context, probe, observer)?;
        let received = self.channel.finish(&tail)?;
        observation::observe_channel(&received, true, &self.geometry, &context, probe, observer)?;
        if !self.rx.receive(&received)?.is_empty() {
            return Err(invalid(
                "Waveform tail completed an unexpected payload period.",
            ));
        }
        Ok(self.rx.pending_samples())
    }

    /// Borrows frozen per-physical-bin post-equalization LLR calibration.
    /// # Returns
    /// Positive finite variances; receiver transports them with the received cells.
    pub(crate) fn variances(&self) -> &[f64] {
        &self.calibrated_variances
    }
    fn plc_variance(&self) -> f64 {
        let mut sum = 0.0;
        let mut count = 0;
        for (role, variance) in self.layout.static_roles().iter().zip(self.variances()) {
            if *role == PhysicalCarrierRole::Plc {
                sum += variance;
                count += 1;
            }
        }
        if count == 0 {
            REFERENCE_VARIANCE
        } else {
            sum / f64::from(count)
        }
    }
}
