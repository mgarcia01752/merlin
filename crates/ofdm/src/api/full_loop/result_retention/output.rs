/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Capacity-only moves after the unchanged engine; no scientific branch or value changes.
use super::normalize::{text, vector};
use crate::simulation::channel::ChannelJobResult;

/// Moves all owned final outputs to exact capacities without changing values.
/// # Arguments
/// * `value` - Actual complete, failed or cancelled engine report.
/// # Returns
/// Unit; this is capacity normalization after preflight, never admission.
pub(super) fn normalize(value: &mut ChannelJobResult) {
    let ChannelJobResult {
        channel_filter: _,
        receiver_reliability: _,
        carrier_mer,
        recovered_file,
        channel_estimate,
        protected_constellation,
        constellation,
        schema_version: _,
        status: _,
        elapsed_ms: _,
        settings: _,
        limits: _,
        summary,
        retained_words,
        omitted_word_details: _,
        unrecovered_generated_payloads: _,
        probe_stats: _,
        error,
    } = value;
    vector(carrier_mer);
    if let Some(file) = recovered_file {
        file.normalize_result_storage();
    }
    if let Some(estimate) = channel_estimate {
        let crate::signal_processing::receiver::channel_estimation::ChannelEstimate {
            channel_symbol_index: _,
            ready: _,
            observed_bins: _,
            maximum_age_symbols: _,
            phase_slope_radians_per_bin: _,
            pilots,
            response,
        } = estimate;
        vector(pilots);
        vector(response);
    }
    if let Some(protected) = protected_constellation {
        protected.normalize_result_storage();
    }
    if let Some(capture) = constellation {
        use crate::simulation::channel::constellation::{
            ConstellationPair, ConstellationResult, NcpConstellationPair,
        };
        let ConstellationResult {
            schema_version: _,
            selection: _,
            status: _,
            collected_pairs: _,
            reference: _,
            channel_coefficients: _,
            pairs,
            ncp_pairs,
        } = capture;
        for pair in pairs.iter_mut() {
            let ConstellationPair {
                channel_symbol_index: _,
                source_symbol_index: _,
                physical_bin: _,
                frequency_hz: _,
                data_position: _,
                profile_id: _,
                modulation_order: _,
                before_equalization: _,
                after_equalization: _,
                reference_iq: _,
                llrs,
            } = pair;
            vector(llrs);
        }
        for pair in ncp_pairs.iter_mut() {
            let NcpConstellationPair {
                channel_symbol_index: _,
                source_symbol_index: _,
                physical_bin: _,
                frequency_hz: _,
                data_position: _,
                modulation_order: _,
                before_equalization: _,
                after_equalization: _,
                reference_iq: _,
                llrs,
            } = pair;
            vector(llrs);
        }
        vector(pairs);
        vector(ncp_pairs);
    }
    let crate::simulation::channel::ChannelSummary {
        fec: _,
        file_source: _,
        measured_mer_db: _,
        compared_data_carriers: _,
        window: _,
        phase: _,
        timing: _,
        prbs_seed: _,
        payload_origin_symbol: _,
        acquisition_symbols: _,
        observation_symbols: _,
        completion_symbols: _,
        interleaver_drain_symbols: _,
        waveform_tail_samples: _,
        channel_filter_tail_samples: _,
        total_transmitted_samples: _,
        rx_warmup_symbols: _,
        generated_payloads: _,
        recovered_payloads: _,
        pending_payload_bits: _,
        comparison: _,
        payload_ber: _,
        payload_codeword_errors: _,
        payload_ldpc_failures: _,
        payload_bch_failures: _,
        payload_mean_ldpc_iterations: _,
        payload_ldpc_iteration_sum: _,
        payload_ldpc_iterations,
        bch_corrected_bits: _,
        accepted_ncp_fields: _,
        ncp_ldpc_iterations: _,
        plc_frames: _,
        plc_bit_errors: _,
        plc_ldpc_iterations: _,
        maximum_bin_error: _,
    } = summary;
    vector(payload_ldpc_iterations);
    vector(retained_words);
    if let Some(crate::SimulationError { code: _, message }) = error {
        text(message);
    }
}
