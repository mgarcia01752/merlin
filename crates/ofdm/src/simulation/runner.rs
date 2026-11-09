/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Connects source, codec and channel operations with bounded job lifecycle.
use super::{
    config::SimulationSettings,
    results::{SimulationProgress, SimulationResult},
    source::SeededSource,
};
use crate::{
    error::{invalid, LabError},
    signal_processing::{
        modulation::{
            interleaving::transmit_bit_order, map_qam16, qam16_llrs, QAM16_BITS_PER_SYMBOL,
        },
        noise::{
            awgn::add_awgn,
            measured::{noise_model, MeasuredNoise},
        },
    },
};
use fec_codec::{
    concatenated::{compare_payload, ConcatenatedCodec},
    parameters::LDPC_CODEWORD_BITS,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};
/// Runs real BCH+LDPC coding/decoding over controlled residual AWGN.
/// # Arguments
/// * `settings` - Deterministic seed and finite work/receiver settings.
/// * `measured` - Optional exact-frame measured MER; no channel gain is added again.
/// * `cancel` - Checked before every codeword; at most one word completes afterward.
/// * `progress` - Small notification after each completed word; no capture arrays.
/// # Returns
/// Truth-based stage metrics for completed words, including partial/cancelled runs.
/// # Errors
/// Rejects invalid/unsupported mode inputs or codec setup failures.
pub fn simulate(
    settings: &SimulationSettings,
    measured: Option<&MeasuredNoise<'_>>,
    cancel: &AtomicBool,
    mut progress: impl FnMut(SimulationProgress),
) -> Result<SimulationResult, LabError> {
    log::debug!(
        "simulation-start requested-codewords={} max-iterations={} max-runtime-ms={}",
        settings.codewords,
        settings.max_iterations,
        settings.max_runtime_ms
    );
    settings.validate()?;
    let (noise_powers, selected_carrier_offsets) = noise_model(settings.awgn_mer_db, measured)?;
    let order = transmit_bit_order();
    let mut codec = ConcatenatedCodec::new().map_err(invalid)?;
    let mut source = SeededSource::new(settings.seed);
    let started = Instant::now();
    let mut result = SimulationResult::new(
        settings,
        measured.map(|input| input.frame.source.clone()),
        selected_carrier_offsets,
    );
    for _ in 0..settings.codewords {
        if cancel.load(Ordering::Relaxed) {
            log::info!(
                "simulation-cancelled completed-codewords={}",
                result.completed_codewords
            );
            result.status = "cancelled".into();
            break;
        }
        if started.elapsed().as_millis() >= u128::from(settings.max_runtime_ms) {
            log::warn!(
                "simulation-runtime-limit completed-codewords={} max-runtime-ms={}",
                result.completed_codewords,
                settings.max_runtime_ms
            );
            result.status = "runtime_limit".into();
            break;
        }
        let message = source.next_payload();
        let encoded = codec.encode(&message).map_err(invalid)?;
        let mut llrs = vec![0.0; LDPC_CODEWORD_BITS];
        for (symbol_index, symbol_order) in order.chunks_exact(QAM16_BITS_PER_SYMBOL).enumerate() {
            let label = symbol_order.iter().fold(0, |label, offset| {
                (label << 1) | usize::from(encoded.ldpc_codeword[*offset])
            });
            let mut symbol = map_qam16(label).ok_or_else(|| invalid("Missing QAM reference."))?;
            let power = noise_powers[symbol_index % noise_powers.len()];
            if !settings.noiseless {
                add_awgn(&mut symbol, power, source.rng_mut());
            }
            let likelihoods = qam16_llrs(symbol, power)?;
            for (bit, source_offset) in symbol_order.iter().enumerate() {
                llrs[*source_offset] = likelihoods[bit];
            }
        }
        let decoded = codec
            .decode(&llrs, settings.max_iterations)
            .map_err(invalid)?;
        let comparison = compare_payload(&message, &decoded).map_err(invalid)?;
        result.record(&encoded, &llrs, &decoded, comparison);
        progress(SimulationProgress {
            completed_codewords: result.completed_codewords,
            requested_codewords: settings.codewords,
            elapsed_ms: started.elapsed().as_millis() as u64,
        });
    }
    result.finish(started.elapsed().as_millis() as u64);
    log::debug!(
        "simulation-finished status={} completed-codewords={} payload-bit-errors={}",
        result.status,
        result.completed_codewords,
        result.post_bch_payload_bit_errors
    );
    Ok(result)
}
