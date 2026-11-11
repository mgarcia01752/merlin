/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Finite cumulative accounting and per-step completed decisions, never full run history.
use crate::{
    simulation::source::{PrbsComparison, PrbsSeed},
    transmission::DurationTiming,
};
pub use fec_codec::statistics::{FecCounters, FecOutcome};
use serde::Serialize;
/// Distinct processing intervals; observation duration does not include the others.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelPhase {
    Acquisition,
    Observation,
    Completion,
    InterleaverDrain,
    Finished,
    Failed,
}
/// Actual received word and post-decoding source comparison; no TX identifier enters RX.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChannelWordResult {
    pub placement: super::accounting::WordPlacement,
    pub first_source_sample: u64,
    pub end_source_sample: u64,
    pub recovered_during_drain: bool,
    pub receive_sequence: u64,
    pub first_source_symbol: u64,
    pub last_source_symbol: u64,
    pub comparison: PrbsComparison,
    pub ldpc_success: bool,
    pub ldpc_iterations: usize,
    pub bch_success: bool,
    pub bch_corrected_bits: u32,
    /// Actual received hard decisions changed by LDPC.
    pub ldpc_hard_decision_changes: usize,
}
/// Bounded result for one call, allowing consumers to store/export desired details.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChannelStep {
    pub phase: ChannelPhase,
    pub channel_symbol_index: Option<u64>,
    pub source_symbol_index: Option<u64>,
    pub recovered: Vec<ChannelWordResult>,
}
/// Constant-size totals; completed words belong to returned steps, not this history.
#[derive(Debug, Clone, Serialize)]
pub struct ChannelSummary {
    /// All completed payload attempts; observation-only totals live in window.fec.
    pub fec: FecCounters,
    /// Finite file/fill progress; absent for the unchanged continuous PRBS path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_source: Option<crate::simulation::source::file::FileSourceSummary>,
    /// Post-equalization data MER against independent TX diagnostic symbols, not RX input.
    pub measured_mer_db: Option<f64>,
    pub compared_data_carriers: u64,
    pub window: Option<super::accounting::WindowTotals>,
    pub phase: ChannelPhase,
    pub timing: DurationTiming,
    pub prbs_seed: PrbsSeed,
    pub payload_origin_symbol: Option<u64>,
    pub acquisition_symbols: u64,
    pub observation_symbols: u64,
    /// Source symbols used to place the final already-generated queued words.
    pub completion_symbols: u64,
    pub interleaver_drain_symbols: u64,
    pub waveform_tail_samples: usize,
    pub total_transmitted_samples: u64,
    pub rx_warmup_symbols: u64,
    pub generated_payloads: u64,
    pub recovered_payloads: u64,
    pub pending_payload_bits: usize,
    pub comparison: PrbsComparison,
    /// None until a nonempty recovered comparison exists.
    pub payload_ber: Option<f64>,
    pub payload_codeword_errors: u64,
    pub payload_ldpc_failures: u64,
    pub payload_bch_failures: u64,
    pub payload_mean_ldpc_iterations: Option<f64>,
    pub payload_ldpc_iteration_sum: u64,
    /// Fixed101 bins indexed by actual iteration count0..100, not per-run history.
    pub payload_ldpc_iterations: Vec<u64>,
    pub bch_corrected_bits: u64,
    pub accepted_ncp_fields: u64,
    pub ncp_ldpc_iterations: u64,
    pub plc_frames: u64,
    pub plc_bit_errors: u64,
    pub plc_ldpc_iterations: u64,
    pub maximum_bin_error: f64,
}
