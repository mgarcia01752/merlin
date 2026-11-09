/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded payload waveform proof; duration-independent stage APIs remain separate from this verifier.
pub use super::link::LinkStats;
use super::{
    link::WaveformLink, observation, PayloadReceiveSymbol, PayloadReceiver, PayloadTransmitter,
};
use crate::{
    error::invalid,
    signal_processing::waveform::samples::MAX_BLOCK_SAMPLES,
    simulation::{
        source::SeededSource,
        test_points::{CellProbeGrid, ProbeObserver, RunPhase, TestPointProbe},
    },
    transmission::{layout::CarrierLayoutSettings, packing::NcpModulation, TransmissionSettings},
    types::PLC_FRAME_SYMBOLS,
    LabError,
};
use fec_codec::concatenated::compare_payload;
use serde::Serialize;
const MAX_SOURCE_SYMBOLS: u16 = 512;
const MAX_REFERENCE_PAYLOADS: usize = 16;
const MAX_ACQUISITION_FRAMES: u64 = 8;
/// Fixed bounded reference workload; source-symbol count is not a duration reset.
pub struct PayloadWaveformExperiment {
    pub settings: TransmissionSettings,
    pub layout: CarrierLayoutSettings,
    pub ncp_modulation: NcpModulation,
    pub source_symbols: u16,
    /// Strictly increasing local enqueue times within the source interval, at most16.
    pub payload_at_symbols: Vec<u16>,
    pub seed: crate::simulation::source::SimulationSeed,
    pub receive_block_samples: usize,
}
/// Actual recovered payload truth and independent decoder flags/iterations.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PayloadWordComparison {
    pub receive_sequence: u64,
    pub first_source_symbol: u64,
    pub last_source_symbol: u64,
    pub payload_bits: usize,
    pub bit_errors: usize,
    pub ldpc_success: bool,
    pub ldpc_iterations: usize,
    pub bch_success: bool,
    pub bch_corrected_bits: u32,
}
/// Compact measurements from real control/payload decoding through waveform samples.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PayloadWaveformResult {
    pub spacing_hz: u32,
    pub ncp_order: u16,
    pub payload_origin_symbol: u64,
    pub source_symbols: u16,
    pub source_payloads: usize,
    pub received_payloads: Vec<PayloadWordComparison>,
    pub payload_bit_errors: usize,
    pub pending_payload_bits: usize,
    pub warmup_symbols: usize,
    pub drain_symbols: usize,
    pub unavailable_tx_data_cells: usize,
    pub accepted_ncp_fields: usize,
    pub ncp_ldpc_iterations: Vec<usize>,
    pub waveform_remainder_samples: usize,
    pub link: LinkStats,
}
/// Proves actual payload/NCP waveform recovery after real PLC configuration acquisition.
/// # Arguments
/// * `experiment` - Up to512 source symbols and16 scheduled payloads, with bounded RX chunks.
/// * `probe` - Optional finite stage/time/element selector; science is computed before callbacks.
/// * `observer` - Read-only sink; callback failure stops this short synchronous verifier.
/// # Returns
/// Real payload comparisons and actual NCP/LDPC/BCH outcomes, including words
/// completed during drain. No TX IDs, offsets or original payloads enter RX.
/// # Errors
/// Rejects unsupported settings/workload, insufficient source interval, control/session
/// failure or observer error. This is not the later cancellable duration-based runner.
pub fn run_payload_waveform(
    experiment: &PayloadWaveformExperiment,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<PayloadWaveformResult, LabError> {
    if !(1..=MAX_SOURCE_SYMBOLS).contains(&experiment.source_symbols)
        || experiment.payload_at_symbols.is_empty()
        || experiment.payload_at_symbols.len() > MAX_REFERENCE_PAYLOADS
        || experiment
            .payload_at_symbols
            .iter()
            .any(|time| *time >= experiment.source_symbols)
        || experiment
            .payload_at_symbols
            .windows(2)
            .any(|times| times[0] >= times[1])
        || !(1..=MAX_BLOCK_SAMPLES).contains(&experiment.receive_block_samples)
    {
        return Err(invalid(
            "Invalid bounded payload waveform schedule or receive chunk size.",
        ));
    }
    let mut link = WaveformLink::new(
        &experiment.settings,
        &experiment.layout,
        experiment.ncp_modulation,
        experiment.receive_block_samples,
    )?;
    let mut origin = 0;
    for frame in 0..MAX_ACQUISITION_FRAMES {
        for phase in 0..PLC_FRAME_SYMBOLS {
            let index = frame * PLC_FRAME_SYMBOLS + phase;
            let reduced = link.acquisition_cells(index);
            link.transfer(index, &reduced, RunPhase::Warmup, probe, observer)?;
        }
        if link.state().require_ready().is_ok() {
            origin = (frame + 1) * PLC_FRAME_SYMBOLS;
            break;
        }
    }
    if origin == 0 {
        return Err(LabError::new(
            "ReceiverNotReady",
            "PLC acquisition did not finish within the reference frame budget.",
        ));
    }
    let configuration = link.state().require_ready()?;
    let grid = CellProbeGrid {
        settings: configuration.interleaver_settings()?,
        origin_symbol: origin,
        ncp_modulation: configuration.ncp_modulation(),
    };
    let mut receiver = PayloadReceiver::new(link.state(), origin)?;
    let mut transmitter = PayloadTransmitter::new(
        &experiment.settings,
        &experiment.layout,
        experiment.ncp_modulation,
        origin,
    )?;
    let mut source = SeededSource::new(experiment.seed);
    let payloads: Vec<_> = experiment
        .payload_at_symbols
        .iter()
        .map(|_| source.next_payload())
        .collect();
    let mut result = PayloadWaveformResult {
        spacing_hz: experiment.settings.subcarrier_spacing.frequency_hz(),
        ncp_order: experiment.ncp_modulation.order(),
        payload_origin_symbol: origin,
        source_symbols: experiment.source_symbols,
        source_payloads: payloads.len(),
        received_payloads: Vec::new(),
        payload_bit_errors: 0,
        pending_payload_bits: 0,
        warmup_symbols: 0,
        drain_symbols: 0,
        unavailable_tx_data_cells: 0,
        accepted_ncp_fields: 0,
        ncp_ldpc_iterations: Vec::new(),
        waveform_remainder_samples: 0,
        link: link.stats.clone(),
    };
    for local in 0..experiment.source_symbols {
        let context = link.context(RunPhase::Observation);
        if let Some(payload) = experiment
            .payload_at_symbols
            .iter()
            .position(|time| *time == local)
        {
            let encoded = transmitter.enqueue(&payloads[payload])?;
            observation::observe_encoded(&encoded, grid, &context, probe, observer)?;
        }
        let transmitted = transmitter.transmit()?;
        result.unavailable_tx_data_cells += transmitted.channel.unavailable_data_ranks.len();
        observation::observe_transmit(&transmitted, grid, &context, probe, observer)?;
        let received = link.transfer(
            transmitted.channel.channel_symbol_index,
            &transmitted.channel.reduced,
            RunPhase::Observation,
            probe,
            observer,
        )?;
        let decoded = receiver.receive(
            link.state(),
            received.symbol_index,
            &received.signals.reduced,
            0.01,
            50,
        )?;
        observation::observe_receive(&decoded, grid, &context, probe, observer)?;
        record(&mut result, &decoded, &payloads)?;
    }
    while let Some(transmitted) = transmitter.drain()? {
        result.drain_symbols += 1;
        result.unavailable_tx_data_cells += transmitted.unavailable_data_ranks.len();
        let context = link.context(RunPhase::Drain);
        let received = link.transfer(
            transmitted.channel_symbol_index,
            &transmitted.reduced,
            RunPhase::Drain,
            probe,
            observer,
        )?;
        let decoded = receiver.receive(
            link.state(),
            received.symbol_index,
            &received.signals.reduced,
            0.01,
            50,
        )?;
        observation::observe_receive(&decoded, grid, &context, probe, observer)?;
        record(&mut result, &decoded, &payloads)?;
    }
    result.pending_payload_bits = receiver.pending_bits();
    result.waveform_remainder_samples = link.finish(probe, observer)?;
    result.link = link.stats;
    if result.received_payloads.len() != result.source_payloads || result.pending_payload_bits != 0
    {
        return Err(invalid(
            "Payload reference ended with incomplete source recovery.",
        ));
    }
    Ok(result)
}
fn record(
    result: &mut PayloadWaveformResult,
    received: &PayloadReceiveSymbol,
    payloads: &[Vec<u8>],
) -> Result<(), LabError> {
    let Some(decoded) = &received.decoded else {
        result.warmup_symbols += 1;
        return Ok(());
    };
    if let Some(error) = &decoded.error {
        return Err(error.clone());
    }
    if let Some(field) = &decoded.ncp {
        result.accepted_ncp_fields += 1;
        result
            .ncp_ldpc_iterations
            .extend_from_slice(&field.block_iterations);
    }
    for word in &decoded.recovered {
        let original = payloads
            .get(result.received_payloads.len())
            .ok_or_else(|| invalid("Receiver produced more words than source truth."))?;
        let comparison = compare_payload(original, &word.decoded).map_err(invalid)?;
        result.payload_bit_errors += comparison.bit_errors;
        result.received_payloads.push(PayloadWordComparison {
            receive_sequence: word.receive_sequence,
            first_source_symbol: word.first_source_symbol,
            last_source_symbol: word.last_source_symbol,
            payload_bits: comparison.bits_tested,
            bit_errors: comparison.bit_errors,
            ldpc_success: word.decoded.ldpc.success,
            ldpc_iterations: word.decoded.ldpc.iterations,
            bch_success: word.decoded.bch.success,
            bch_corrected_bits: word.decoded.bch.corrected_bits,
        });
    }
    Ok(())
}
