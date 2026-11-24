/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Checked final owner quantities; execution buffers and allocator overhead are separate.
use super::{observations::RetainedStages, Frozen, FullLoopRequest, FullLoopResult};
use crate::{
    signal_processing::receiver::channel_estimation::PilotEstimate,
    simulation::{
        channel::{
            config::MAX_CHANNEL_DECODER_ITERATIONS,
            constellation::{ConstellationPair, NcpConstellationPair},
            performance::CarrierMer,
            protected_constellation::ProtectedPair,
            ChannelJobResult, ChannelWordResult,
        },
        source::PayloadSourceConfig,
    },
    types::ofdm_modulation::MAX_OFDM_BITS_PER_SYMBOL,
};
use merlin_api::{
    retention_support::{capacity_bytes, payload_bytes, sum},
    RetainedFootprint,
};
use std::mem::size_of;

// Four independent finite streams: continuous/scattered pilots, PLC data/preamble.
const PROTECTED_STREAMS: usize = 4;
// Audited static diagnostics across simulation/transmission/receiver/FEC owners
// are no longer than this existing composition diagnostic (135 UTF-8 bytes).
// The retained run path has no user text, I/O errors or JSON parser diagnostics.
const STATIC_ENGINE_ERROR_BYTES: usize = "Synthetic echoes require Flat / unity input; imported H is a separate response source. Tilt and cable loss may be combined with echoes.".len();
// Shortened codecs can construct the pinned ldpc-toolbox 0.10.1 encoder during
// stepping. Its sole Encoder::Error Display variant has this fixed text.
const LDPC_SETUP_ERROR_BYTES: usize =
    "the square matrix formed by the last columns of the parity check is not invertible".len();

/// Computes every final owner dimension using validated existing policies.
/// # Arguments
/// * `request` - Unstarted immutable source request.
/// * `frozen` - Exact final input/settings/policy destination snapshot.
/// # Returns
/// Complete conservative metadata, or None on checked arithmetic overflow.
pub(super) fn calculate(request: &FullLoopRequest, frozen: &Frozen) -> Option<RetainedFootprint> {
    let Frozen {
        settings,
        inputs: _,
        policy,
    } = frozen;
    let fft = usize::try_from(settings.transmission.subcarrier_spacing.fft_samples()).ok()?;
    let mut quantities = vec![
        Some(payload_bytes(size_of::<FullLoopResult>())),
        Some(payload_bytes(size_of::<ChannelJobResult>())),
        Some(payload_bytes(size_of::<RetainedStages>())),
        frozen.children(),
        capacity_bytes::<ChannelWordResult>(request.options.max_retained_words),
        capacity_bytes::<CarrierMer>(fft),
        capacity_bytes::<u64>(MAX_CHANNEL_DECODER_ITERATIONS.checked_add(1)?),
        // A pilot appears at most once per physical bin in the latest estimate.
        capacity_bytes::<PilotEstimate>(fft),
        capacity_bytes::<[f64; 2]>(fft),
        Some(payload_bytes(error_text_bytes()?)),
        RetainedStages::result_retained_children(policy.as_ref()),
    ];
    match &settings.source {
        PayloadSourceConfig::Prbs15 => {}
        PayloadSourceConfig::File { payload } => {
            quantities.push(capacity_bytes::<u8>(payload.bytes().len()))
        }
    }
    if let Some(selection) = &settings.constellation {
        let crate::simulation::channel::constellation::ConstellationSelection {
            requested_pairs,
            profile_id: _,
            start_hz: _,
            stop_hz: _,
            capture_ncp,
        } = selection;
        quantities.push(cells::<ConstellationPair>(*requested_pairs));
        if *capture_ncp {
            // The collector's additional NCP cap can only reduce this bound.
            quantities.push(cells::<NcpConstellationPair>(*requested_pairs));
        }
    }
    if let Some(selection) = &settings.protected_constellation {
        let crate::simulation::channel::protected_constellation::ProtectedCaptureSelection {
            pairs_per_stream,
            first_channel_symbol: _,
            end_channel_symbol_exclusive: _,
        } = selection;
        quantities.push(capacity_bytes::<ProtectedPair>(
            pairs_per_stream.checked_mul(PROTECTED_STREAMS)?,
        ));
        // PilotSequence::new clocks once per FFT bin; None only reduces storage.
        quantities.push(capacity_bytes::<u8>(fft));
    }
    let mut result = sum(quantities)?;
    // FullLoopResult always owns one immutable job and one stage-store Arc.
    result.shared_roots = result.shared_roots.checked_add(2)?;
    Some(result)
}

fn cells<T>(count: usize) -> Option<RetainedFootprint> {
    sum([
        capacity_bytes::<T>(count),
        capacity_bytes::<f64>(count.checked_mul(usize::from(MAX_OFDM_BITS_PER_SYMBOL))?),
    ])
}

fn error_text_bytes() -> Option<usize> {
    // fec_codec::binary_bits is the sole formatted stepping diagnostic: its
    // expected length is a usize. Decimal width is finite on the target layout.
    let digits = usize::try_from(usize::MAX.ilog10()).ok()?.checked_add(1)?;
    let binary_bits = "Expected exactly "
        .len()
        .checked_add(digits)?
        .checked_add(" binary bits.".len())?;
    Some(
        STATIC_ENGINE_ERROR_BYTES
            .max(LDPC_SETUP_ERROR_BYTES)
            .max(binary_bits),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_cell_and_histogram_quantities_reject_overflow() {
        assert!(cells::<ConstellationPair>(usize::MAX).is_none());
        assert!(capacity_bytes::<u64>(usize::MAX).is_none());
        assert!(
            error_text_bytes().unwrap()
                >= format!("Expected exactly {} binary bits.", usize::MAX).len()
        );
        assert!(error_text_bytes().unwrap() >= LDPC_SETUP_ERROR_BYTES);
    }
}
