/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Small waveform verification experiment, distinct from the future one-second payload runner.
use super::{observation, receiver::ReceiveReference, WaveformReceiver, WaveformTransmitter};
use crate::{
    error::invalid,
    signal_processing::{
        channel::reference::NoiselessChannel,
        modulation::map_symbol,
        waveform::{samples::MAX_BLOCK_SAMPLES, SampleBlock, WaveformGeometry},
    },
    simulation::{
        plc::{generate_frame, PlcAcquisitionReceiver},
        test_points::{
            PhysicalProbeGrid, ProbeContext, ProbeObserver, RunPhase, SignalReference,
            TestPointProbe,
        },
    },
    transmission::{
        frame::assemble,
        layout::{CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole},
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            receiver_configuration::ReceiverConfigurationStatus,
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        TransmissionSettings,
    },
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
use serde::Serialize;
/// Caller-supplied bounded reference experiment, with known timing and unity channel.
pub struct ReferenceExperiment {
    pub settings: TransmissionSettings,
    pub layout: CarrierLayoutSettings,
    /// One or two complete PLC frames; this is not the one-second payload runner.
    pub frames: u8,
    /// Maximum RX chunk size; streams can split within any prefix or useful symbol.
    pub receive_block_samples: usize,
    pub first_sample: u64,
    pub reference_rms: f64,
}
/// Actual PLC decode comparisons and bounded waveform statistics; no full stream retained.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ReferenceWaveformResult {
    pub spacing_hz: u32,
    pub fft_samples: usize,
    pub prefix_samples: usize,
    pub roll_off_samples: usize,
    pub transmitted_symbols: u64,
    pub received_symbols: u64,
    pub compared_bins: u64,
    pub compared_pilots: u64,
    pub maximum_bin_error: f64,
    pub compared_information_bits: u64,
    pub information_bit_errors: u64,
    pub plc_frames_decoded: u64,
    pub preambles_match: bool,
    pub ldpc_iterations: Vec<usize>,
    pub receiver_configuration: ReceiverConfigurationStatus,
    pub drain_samples: usize,
    pub pending_rx_samples: usize,
}
/// Streams real pilot/PLC signals through TX, a unity sample channel and matched RX.
/// # Arguments
/// * `experiment` - Fixed geometry/layout,1..2 frames, positive scale and bounded RX chunks.
/// * `probe` - Optional finite selector; disabled probes emit no callbacks.
/// * `observer` - Read-only stage sink; an error stops orchestration, not completed stage math.
/// # Returns
/// Compact real comparisons, actual LDPC iterations and received-configuration readiness.
/// Data bins carry deterministic256-QAM reference cells, not coded payload or real NCP.
/// # Errors
/// Rejects invalid configuration/work bounds or stage/observer errors. This short
/// synchronous verifier has no cancellation API and does not replace the future runner.
pub fn run_reference_waveform(
    experiment: &ReferenceExperiment,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ReferenceWaveformResult, SimulationError> {
    if !(1..=2).contains(&experiment.frames)
        || !(1..=MAX_BLOCK_SAMPLES).contains(&experiment.receive_block_samples)
    {
        return Err(invalid(
            "Reference waveform needs1..2 frames and bounded positive RX chunks.",
        ));
    }
    let geometry = WaveformGeometry::new(&experiment.settings)?;
    let symbols = u64::from(experiment.frames) * PLC_FRAME_SYMBOLS;
    experiment
        .first_sample
        .checked_add(
            symbols * geometry.stride_samples() as u64 + geometry.roll_off_samples() as u64,
        )
        .ok_or_else(|| invalid("Reference waveform sample clock overflow."))?;
    let layout = CarrierLayout::new(&experiment.settings, &experiment.layout)?;
    let descriptors = FixedPlcDescriptors::new(
        &experiment.settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 0,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: 4,
        },
    )?;
    let mut scheduler = PlcMessageScheduler::new(&experiment.settings, &descriptors)?;
    let mut tx = WaveformTransmitter::new(&geometry, 0, experiment.first_sample);
    let mut rx = WaveformReceiver::new(
        &geometry,
        &experiment.layout,
        ReceiveReference {
            first_input_sample: experiment.first_sample,
            first_symbol_sample: experiment.first_sample,
            first_symbol_index: 0,
            reference_rms: experiment.reference_rms,
            channel_response: vec![[1.0, 0.0]; geometry.fft_samples()],
        },
    )?;
    let mut plc_rx = PlcAcquisitionReceiver::new(experiment.settings.subcarrier_spacing)?;
    let mut context = ProbeContext::new(
        "reference-waveform",
        "fixed-known-reference",
        RunPhase::Observation,
    )?;
    context.physical_grid = Some(PhysicalProbeGrid {
        spacing: experiment.settings.subcarrier_spacing,
        zero_frequency_hz: experiment.layout.zero_frequency_hz,
    });
    context.reference = SignalReference::OtherSubcarrierRms {
        rms: experiment.reference_rms,
    };
    let mut result = ReferenceWaveformResult {
        spacing_hz: experiment.settings.subcarrier_spacing.frequency_hz(),
        fft_samples: geometry.fft_samples(),
        prefix_samples: geometry.prefix_samples(),
        roll_off_samples: geometry.roll_off_samples(),
        transmitted_symbols: symbols,
        received_symbols: 0,
        compared_bins: 0,
        compared_pilots: 0,
        maximum_bin_error: 0.0,
        compared_information_bits: 0,
        information_bit_errors: 0,
        plc_frames_decoded: 0,
        preambles_match: true,
        ldpc_iterations: Vec::new(),
        receiver_configuration: ReceiverConfigurationStatus::Waiting,
        drain_samples: 0,
        pending_rx_samples: 0,
    };
    for frame_index in 0..u64::from(experiment.frames) {
        let frame_start = frame_index * PLC_FRAME_SYMBOLS;
        let message = PlcMessageFrame::new(
            experiment.settings.subcarrier_spacing,
            ExtendedTimestamp::from_sample_ticks(
                frame_start * geometry.stride_samples() as u64,
                0,
            )?,
            scheduler.next_block()?.block,
        )?;
        let frame = generate_frame(message, frame_index)?;
        let mut received_plc = Vec::new();
        for symbol_index in frame_start..frame_start + PLC_FRAME_SYMBOLS {
            context.channel_symbol_index = Some(symbol_index);
            context.plc_phase = Some((symbol_index % PLC_FRAME_SYMBOLS) as u8);
            let reduced = layout
                .symbol_roles(symbol_index)
                .iter()
                .enumerate()
                .filter_map(|(bin, role)| match role {
                    PhysicalCarrierRole::Data => {
                        Some(map_symbol(256, ((symbol_index + bin as u64) % 256) as u16).map(Some))
                    }
                    PhysicalCarrierRole::ScatteredPilot => Some(Ok(None)),
                    _ => None,
                })
                .collect::<Result<Vec<_>, _>>()?;
            let physical = assemble(
                &layout,
                symbol_index,
                &reduced,
                frame
                    .symbol_iq(symbol_index)
                    .ok_or_else(|| invalid("Missing PLC symbol in generated frame."))?,
                experiment.reference_rms,
            )?;
            let transmitted = tx.transmit(&physical)?;
            observation::observe_transmit(&transmitted, &geometry, &context, probe, observer)?;
            observation::observe_channel(
                &transmitted.output,
                false,
                &geometry,
                &context,
                probe,
                observer,
            )?;
            for (chunk_index, chunk) in transmitted
                .output
                .samples()
                .chunks(experiment.receive_block_samples)
                .enumerate()
            {
                let block = SampleBlock::new(
                    transmitted.output.start_index()
                        + (chunk_index * experiment.receive_block_samples) as u64,
                    chunk.to_vec(),
                )?;
                let received = NoiselessChannel::transfer(&block);
                observation::observe_channel(
                    &received, true, &geometry, &context, probe, observer,
                )?;
                for recovered in rx.receive(&received)? {
                    observation::observe_receive(&recovered, &geometry, &context, probe, observer)?;
                    result.received_symbols += 1;
                    for (actual, expected) in recovered.bins.iter().zip(&physical.bins) {
                        let expected = expected.as_ref().ok_or_else(|| {
                            invalid("Reference experiment cannot compare missing bins.")
                        })?;
                        let distance = (actual[0] - expected[0]).hypot(actual[1] - expected[1]);
                        result.maximum_bin_error = result.maximum_bin_error.max(distance);
                        result.compared_bins += 1;
                    }
                    result.compared_pilots += recovered.signals.pilots.len() as u64;
                    received_plc.extend_from_slice(&recovered.signals.plc);
                }
            }
        }
        let decoded = plc_rx.receive_frame(&received_plc, 0.01, 100)?;
        if decoded.transport_error.is_some()
            || decoded
                .admissions
                .iter()
                .any(|admission| admission.error.is_some())
        {
            return Err(invalid(
                "Reference waveform PLC transport/admission failed.",
            ));
        }
        result.preambles_match &= decoded.frame.preamble_signs_match;
        result
            .ldpc_iterations
            .extend(decoded.frame.blocks.iter().map(|word| word.iterations));
        let received_message = decoded
            .frame
            .message
            .as_ref()
            .ok_or_else(|| invalid("Reference waveform PLC FEC/message decode failed."))?;
        let truth = frame.message.to_bytes();
        let recovered = received_message.to_bytes();
        if truth.len() != recovered.len() {
            return Err(invalid("Reference PLC information length mismatch."));
        }
        result.information_bit_errors += truth
            .iter()
            .zip(recovered)
            .map(|(left, right)| (left ^ right).count_ones() as u64)
            .sum::<u64>();
        result.compared_information_bits += truth.len() as u64 * 8;
        result.plc_frames_decoded += 1;
    }
    let tail = tx.finish()?;
    result.drain_samples = tail.samples().len();
    context.phase = RunPhase::Drain;
    context.channel_symbol_index = None;
    context.plc_phase = None;
    observation::observe_channel(&tail, false, &geometry, &context, probe, observer)?;
    let received_tail = NoiselessChannel::transfer(&tail);
    observation::observe_channel(&received_tail, true, &geometry, &context, probe, observer)?;
    if !rx.receive(&received_tail)?.is_empty() {
        return Err(invalid(
            "Window drain unexpectedly completed an extra symbol.",
        ));
    }
    result.pending_rx_samples = rx.pending_samples();
    result.receiver_configuration = plc_rx.configuration().status();
    Ok(result)
}
