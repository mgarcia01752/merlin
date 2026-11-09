/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared known-reference orchestration of existing physical/waveform/PLC stages.
use crate::{
    error::invalid,
    signal_processing::{
        channel::reference::NoiselessChannel,
        waveform::{SampleBlock, WaveformGeometry},
    },
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
    LabError,
};
const REFERENCE_VARIANCE: f64 = 0.01;
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct LinkStats {
    pub compared_bins: u64,
    pub maximum_bin_error: f64,
    pub plc_frames: u64,
    pub plc_information_bits: u64,
    pub plc_bit_errors: u64,
    pub plc_ldpc_iterations: Vec<usize>,
    pub preambles_match: bool,
}
pub(crate) struct WaveformLink {
    geometry: WaveformGeometry,
    layout: CarrierLayout,
    tx: WaveformTransmitter,
    rx: WaveformReceiver,
    plc_rx: PlcAcquisitionReceiver,
    scheduler: PlcMessageScheduler,
    frame: Option<PlcFrameSignals>,
    received_plc: Vec<IqPair>,
    block_samples: usize,
    context: ProbeContext,
    pub stats: LinkStats,
}
impl WaveformLink {
    /// Creates reference-only stage owners without passing TX truth into any RX operation.
    /// # Arguments
    /// * `settings` - Fixed TX and explicit RX bootstrap numerology.
    /// * `raw` - Known bootstrap carrier map; payload RX later uses decoded descriptors.
    /// * `ncp` - Fixed positive control constellation advertised through actual DPD.
    /// * `block_samples` - Validated positive bounded transfer chunk length.
    /// # Returns
    /// Fresh waveform/PLC streams with no sample history or configured payload receiver.
    /// # Errors
    /// Propagates configuration and stage initialization failures.
    pub(crate) fn new(
        settings: &TransmissionSettings,
        raw: &CarrierLayoutSettings,
        ncp: NcpModulation,
        block_samples: usize,
    ) -> Result<Self, LabError> {
        let geometry = WaveformGeometry::new(settings)?;
        let layout = CarrierLayout::new(settings, raw)?;
        let descriptors = FixedPlcDescriptors::new(
            settings,
            &layout,
            DescriptorIdentity {
                channel_id: 7,
                change_count: 0,
                source_mac: [0, 0xca, 0, 0, 0, 1],
                primary_capable: true,
                ncp_bits: ncp.order().ilog2() as u8,
            },
        )?;
        let scheduler = PlcMessageScheduler::new(settings, &descriptors)?;
        let tx = WaveformTransmitter::new(&geometry, 0, 0);
        let rx = WaveformReceiver::new(
            &geometry,
            raw,
            ReceiveReference {
                first_input_sample: 0,
                first_symbol_sample: 0,
                first_symbol_index: 0,
                reference_rms: 1.0,
                channel_response: vec![[1.0, 0.0]; geometry.fft_samples()],
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
        Ok(Self {
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
    ) -> Result<ReceivedSymbol, LabError> {
        if index % PLC_FRAME_SYMBOLS == 0 {
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
        for (offset, chunk) in transmitted
            .output
            .samples()
            .chunks(self.block_samples)
            .enumerate()
        {
            let input = SampleBlock::new(
                transmitted.output.start_index() + (offset * self.block_samples) as u64,
                chunk.to_vec(),
            )?;
            let received = NoiselessChannel::transfer(&input);
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
        observation::observe_receive(&received, &self.geometry, &context, probe, observer)?;
        for (sample, expected) in received.bins.iter().zip(&physical.bins) {
            let expected = expected
                .as_ref()
                .ok_or_else(|| invalid("Incomplete TX grid in payload reference."))?;
            self.stats.maximum_bin_error = self
                .stats
                .maximum_bin_error
                .max((sample[0] - expected[0]).hypot(sample[1] - expected[1]));
            self.stats.compared_bins += 1;
        }
        self.received_plc.extend_from_slice(&received.signals.plc);
        if index % PLC_FRAME_SYMBOLS == PLC_FRAME_SYMBOLS - 1 {
            let decision = self
                .plc_rx
                .receive_frame(&self.received_plc, REFERENCE_VARIANCE, 50)?;
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
    ) -> Result<usize, LabError> {
        let tail = self.tx.finish()?;
        let context = self.context(RunPhase::Drain);
        observation::observe_channel(&tail, false, &self.geometry, &context, probe, observer)?;
        let received = NoiselessChannel::transfer(&tail);
        observation::observe_channel(&received, true, &self.geometry, &context, probe, observer)?;
        if !self.rx.receive(&received)?.is_empty() {
            return Err(invalid(
                "Waveform tail completed an unexpected payload period.",
            ));
        }
        Ok(self.rx.pending_samples())
    }
}
