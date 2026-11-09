/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded reference waveform experiment for explicitly supplied shortened words.
use crate::{
    error::invalid,
    simulation::{
        channel::{
            config::MAX_ACQUISITION_SYMBOLS,
            link::{LinkStats, WaveformLink},
        },
        payload::{PayloadReceiver, PayloadTransmitter},
        test_points::{ProbeObserver, RunPhase, TestPointProbe},
    },
    transmission::{layout::CarrierLayoutSettings, packing::NcpModulation, TransmissionSettings},
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
/// Application bounds; these do not describe DOCSIS capacity or MAC framing.
const MAX_WORDS: usize = 16;
const MAX_SOURCE_SYMBOLS: u64 = 4096;
/// One actual receiver outcome and independently measured original-bit mismatch count.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShortenedWordResult {
    pub receive_sequence: u64,
    pub payload_bits: usize,
    pub bit_errors: usize,
    pub first_source_symbol: u64,
    pub last_source_symbol: u64,
    pub ldpc_iterations: usize,
    pub ldpc_success: bool,
    pub bch_success: bool,
    pub bch_corrected_bits: u32,
    pub known_zeros_consistent: bool,
}
/// Bounded reference result; failed codeword decisions are reported, not replaced.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ShortenedWaveformResult {
    pub payload_origin_symbol: u64,
    pub source_symbols: u64,
    pub words: Vec<ShortenedWordResult>,
    pub pending_payload_bits: usize,
    pub link: LinkStats,
}
/// Runs explicitly sized words through live PLC acquisition, NCP, interleavers and samples.
/// # Arguments
/// * `settings` - Fixed validated transmission geometry.
/// * `layout` - Explicit acquisition-compatible physical roles.
/// * `loading` - Physical-frequency data loading; protected-bin values are ignored.
/// * `payloads` - One to16 borrowed binary shortened information words. Wire length
///   must be odd bytes. Caller owns headers/stuffing and independent truth.
/// * `probe` - Bounded optional waveform test-point selection.
/// * `observer` - Caller-owned sink; samples are never used as TX truth by the receiver.
/// # Returns
/// Completed real receiver outcomes, iteration counts, independent error counts and link statistics.
/// This reference experiment adds no noise, does not alter SRC EOF policy and does not build MAC frames.
/// # Errors
/// Rejects invalid setup/payloads, control acquisition failure, terminal receiver errors
/// or failure to complete within4096 source symbols. Decoder non-convergence is a result.
pub fn run_shortened_waveform(
    settings: &TransmissionSettings,
    layout: &CarrierLayoutSettings,
    loading: &[u8],
    payloads: &[Vec<u8>],
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ShortenedWaveformResult, SimulationError> {
    if payloads.is_empty() || payloads.len() > MAX_WORDS {
        return Err(invalid("Shortened waveform requires1..16 original words."));
    }
    for payload in payloads {
        let geometry = fec_codec::shortening::Shortening::new(payload.len()).map_err(invalid)?;
        crate::transmission::packing::shortening::validate_shortened_transport(geometry)?;
        if payload.iter().any(|bit| *bit > 1) {
            return Err(invalid("Shortened waveform payload is not binary."));
        }
    }
    let ncp = NcpModulation::Qpsk;
    let mut link = WaveformLink::new(
        settings,
        layout,
        ncp,
        137,
        &Default::default(),
        0.01,
        Some(loading),
    )?;
    let mut origin = None;
    for index in 0..MAX_ACQUISITION_SYMBOLS {
        let reduced = link.acquisition_cells(index);
        link.transfer(index, &reduced, RunPhase::Warmup, probe, observer)?;
        if (index + 1) % PLC_FRAME_SYMBOLS == 0 && link.state().require_ready().is_ok() {
            origin = Some(index + 1);
            break;
        }
    }
    let origin = origin.ok_or_else(|| {
        SimulationError::new(
            "ReceiverNotReady",
            "Shortened reference PLC acquisition did not finish.",
        )
    })?;
    let mut receiver = PayloadReceiver::with_shortened_words(link.state(), origin)?;
    let mut transmitter = PayloadTransmitter::with_profile(settings, layout, ncp, origin, loading)?;
    for payload in payloads {
        transmitter.enqueue_shortened(payload)?;
    }
    let mut result = ShortenedWaveformResult {
        payload_origin_symbol: origin,
        source_symbols: 0,
        words: Vec::new(),
        pending_payload_bits: 0,
        link: link.stats.clone(),
    };
    for _ in 0..MAX_SOURCE_SYMBOLS {
        let transmitted = transmitter.transmit()?;
        let received = link.transfer(
            transmitted.channel.channel_symbol_index,
            &transmitted.channel.reduced,
            RunPhase::Observation,
            probe,
            observer,
        )?;
        let recovered = receiver.receive(
            link.state(),
            received.symbol_index,
            &received.signals.reduced,
            0.01,
            50,
        )?;
        if let Some(decoded) = recovered.decoded {
            if let Some(error) = decoded.error {
                return Err(error);
            }
            for word in decoded.recovered {
                let original = payloads
                    .get(result.words.len())
                    .ok_or_else(|| invalid("Receiver produced excess words."))?;
                let actual = word.payload()?;
                if actual.len() != original.len() {
                    return Err(invalid("Received and original word lengths disagree."));
                }
                result.words.push(ShortenedWordResult {
                    receive_sequence: word.receive_sequence,
                    payload_bits: actual.len(),
                    bit_errors: actual
                        .iter()
                        .zip(original)
                        .filter(|(actual, original)| actual != original)
                        .count(),
                    first_source_symbol: word.first_source_symbol,
                    last_source_symbol: word.last_source_symbol,
                    ldpc_iterations: word.decoded.ldpc.iterations,
                    ldpc_success: word.decoded.ldpc.success,
                    bch_success: word.decoded.bch.success,
                    bch_corrected_bits: word.decoded.bch.corrected_bits,
                    known_zeros_consistent: word.known_zeros_consistent,
                });
            }
        }
        result.source_symbols += 1;
        if transmitter.queued_codewords() == 0
            && result.words.len() == payloads.len()
            && receiver.pending_bits() == 0
        {
            break;
        }
    }
    if result.words.len() != payloads.len() {
        return Err(invalid("Shortened reference exceeded completion budget."));
    }
    // All words have real received terminal boundaries; drain retains physical filter history.
    while let Some(transmitted) = transmitter.drain()? {
        let received = link.transfer(
            transmitted.channel_symbol_index,
            &transmitted.reduced,
            RunPhase::Drain,
            probe,
            observer,
        )?;
        let recovered = receiver.receive(
            link.state(),
            received.symbol_index,
            &received.signals.reduced,
            0.01,
            50,
        )?;
        if let Some(decoded) = recovered.decoded {
            if let Some(error) = decoded.error {
                return Err(error);
            }
            if !decoded.recovered.is_empty() {
                return Err(invalid("Unexpected extra recovered word during drain."));
            }
        }
    }
    link.finish(probe, observer)?;
    result.pending_payload_bits = receiver.pending_bits();
    result.link = link.stats;
    Ok(result)
}
