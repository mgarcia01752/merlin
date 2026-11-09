/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact trial comparisons, decoder counters and confidence reporting.
use super::config::{SimulationSettings, CODING_MODE};
use crate::scenario::SourceProvenance;
use fec_codec::{
    concatenated::{DecodedBlock, EncodedBlock, PayloadComparison},
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
use serde::{Deserialize, Serialize};
// Exported interpretation is part of the stable result contract.
const SIMULATION_ASSUMPTIONS:[&str;6]=[
"Cell-word randomization is bypassed in this coding experiment; MAC/NCP/PLC framing and waveform generation are outside this boundary.",
            "Unit-average symbol energy; complex residual noise power=10^(-MER/10), real-component variance is half that power.",
            "Max-log LLR=ln(P0/P1), positive favors0; clip magnitude100; horizontal layered sum-product decoding.",
            "Sequential cycling through selected observed data carriers; no claimed DOCSIS frequency/time-interleaver placement.",
            "Ideal equalized receiver; channel-estimate coefficients are contextual observations, not applied again to measured RxMER.",
            "No inferred impulse noise, temporal correlation, clipping, LDPC implementation equivalence or modem behavior.",
];
const VALIDATION_STATE: &str =
    "experimental_coding_chain_independent_interoperability_vectors_pending";
/// Trial progress, small enough for bounded application polling/notifications.
#[derive(Debug, Clone, Serialize)]
pub struct SimulationProgress {
    pub completed_codewords: usize,
    pub requested_codewords: usize,
    pub elapsed_ms: u64,
}
/// Simulated outcomes with truth errors separate from decoder-reported failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationResult {
    pub result_kind: String,
    pub mode: String,
    pub status: String,
    pub settings: SimulationSettings,
    pub completed_codewords: usize,
    pub pre_ldpc_bit_errors: u64,
    pub pre_ldpc_bits_tested: u64,
    pub pre_ldpc_ber: Option<f64>,
    pub post_ldpc_bit_errors: u64,
    pub post_ldpc_bits_tested: u64,
    pub post_ldpc_ber: Option<f64>,
    pub post_bch_payload_bit_errors: u64,
    pub payload_bits_tested: u64,
    pub post_bch_payload_ber: Option<f64>,
    pub ldpc_detected_failures: usize,
    pub bch_detected_failures: usize,
    pub bch_corrected_bits: u64,
    pub ldpc_undetected_codeword_errors: usize,
    pub bch_undetected_payload_errors: usize,
    pub post_bch_payload_codeword_errors: usize,
    pub post_bch_payload_cwer: Option<f64>,
    pub zero_error_one_sided_upper_95: Option<f64>,
    pub cwer_wilson_95: Option<[f64; 2]>,
    pub total_decoder_iterations: usize,
    pub maximum_decoder_iterations_used: usize,
    pub mean_decoder_iterations: Option<f64>,
    pub source: Option<SourceProvenance>,
    pub selected_carrier_offsets: Vec<usize>,
    pub elapsed_ms: u64,
    pub assumptions: Vec<String>,
    pub validation_state: String,
}

impl SimulationResult {
    /// Initializes a result with no fabricated trials or probabilities.
    /// # Arguments
    /// * `settings` - Validated reproducible receiver/work policy.
    /// * `source` - Optional measured frame provenance.
    /// * `selected_carrier_offsets` - Exact data-carrier positions used by the model.
    /// # Returns
    /// Empty stage counters with unchanged exported assumptions.
    pub(crate) fn new(
        settings: &SimulationSettings,
        source: Option<SourceProvenance>,
        selected_carrier_offsets: Vec<usize>,
    ) -> Self {
        Self {
            result_kind: "simulated_fec_outcome".into(),
            mode: CODING_MODE.into(),
            status: "completed".into(),
            settings: settings.clone(),
            completed_codewords: 0,
            pre_ldpc_bit_errors: 0,
            pre_ldpc_bits_tested: 0,
            pre_ldpc_ber: None,
            post_ldpc_bit_errors: 0,
            post_ldpc_bits_tested: 0,
            post_ldpc_ber: None,
            post_bch_payload_bit_errors: 0,
            payload_bits_tested: 0,
            post_bch_payload_ber: None,
            ldpc_detected_failures: 0,
            bch_detected_failures: 0,
            bch_corrected_bits: 0,
            ldpc_undetected_codeword_errors: 0,
            bch_undetected_payload_errors: 0,
            post_bch_payload_codeword_errors: 0,
            post_bch_payload_cwer: None,
            zero_error_one_sided_upper_95: None,
            cwer_wilson_95: None,
            total_decoder_iterations: 0,
            maximum_decoder_iterations_used: 0,
            mean_decoder_iterations: None,
            source,
            selected_carrier_offsets,
            elapsed_ms: 0,
            assumptions: SIMULATION_ASSUMPTIONS
                .iter()
                .map(|value| (*value).into())
                .collect(),
            validation_state: VALIDATION_STATE.into(),
        }
    }
    /// Records truth errors and decoder outcomes for one completed codeword.
    /// # Arguments
    /// * `encoded` - Original systematic transmit stages.
    /// * `llrs` - Exactly16200 current-word soft bits.
    /// * `decoded` - Reverse codec stages, including failed decisions.
    /// * `comparison` - Independently computed original-versus-payload comparison.
    /// # Returns
    /// Unit with one additional completed trial.
    pub(crate) fn record(
        &mut self,
        encoded: &EncodedBlock,
        llrs: &[f64],
        decoded: &DecodedBlock,
        comparison: PayloadComparison,
    ) {
        self.pre_ldpc_bit_errors += llrs
            .iter()
            .zip(&encoded.ldpc_codeword)
            .filter(|(llr, bit)| u8::from(**llr < 0.0) != **bit)
            .count() as u64;
        self.pre_ldpc_bits_tested += LDPC_CODEWORD_BITS as u64;
        self.ldpc_detected_failures += usize::from(!decoded.ldpc.success);
        self.ldpc_undetected_codeword_errors +=
            usize::from(decoded.ldpc.success && decoded.ldpc.codeword != encoded.ldpc_codeword);
        self.total_decoder_iterations += decoded.ldpc.iterations;
        self.maximum_decoder_iterations_used = self
            .maximum_decoder_iterations_used
            .max(decoded.ldpc.iterations);
        let decoded_bch = &decoded.ldpc.codeword[..LDPC_INPUT_BITS];
        self.post_ldpc_bit_errors += decoded_bch
            .iter()
            .zip(&encoded.bch_codeword)
            .filter(|(decoded, expected)| decoded != expected)
            .count() as u64;
        self.post_ldpc_bits_tested += LDPC_INPUT_BITS as u64;
        self.bch_detected_failures += usize::from(!decoded.bch.success);
        self.bch_corrected_bits += u64::from(decoded.bch.corrected_bits);
        let payload_errors = comparison.bit_errors;
        self.post_bch_payload_bit_errors += payload_errors as u64;
        self.payload_bits_tested += BCH_INPUT_BITS as u64;
        self.post_bch_payload_codeword_errors += usize::from(payload_errors > 0);
        self.bch_undetected_payload_errors +=
            usize::from(decoded.bch.success && payload_errors > 0);
        self.completed_codewords += 1;
    }
    /// Finishes rates and confidence intervals using completed trials only.
    /// # Arguments
    /// * `elapsed_ms` - Trial-loop elapsed time, excluding codec setup.
    /// # Returns
    /// Unit; empty/cancelled runs retain None probabilities when no word completed.
    pub(crate) fn finish(&mut self, elapsed_ms: u64) {
        let ratio = |errors: u64, count: u64| (count > 0).then(|| errors as f64 / count as f64);
        self.pre_ldpc_ber = ratio(self.pre_ldpc_bit_errors, self.pre_ldpc_bits_tested);
        self.post_ldpc_ber = ratio(self.post_ldpc_bit_errors, self.post_ldpc_bits_tested);
        self.post_bch_payload_ber =
            ratio(self.post_bch_payload_bit_errors, self.payload_bits_tested);
        self.post_bch_payload_cwer = ratio(
            self.post_bch_payload_codeword_errors as u64,
            self.completed_codewords as u64,
        );
        if let Some(rate) = self.post_bch_payload_cwer {
            let trials = self.completed_codewords as f64;
            let z_squared = 1.959_963_984_540_054_f64.powi(2);
            let denominator = 1.0 + z_squared / trials;
            let center = (rate + z_squared / (2.0 * trials)) / denominator;
            let half_width = (z_squared
                * (rate * (1.0 - rate) / trials + z_squared / (4.0 * trials.powi(2))))
            .sqrt()
                / denominator;
            self.cwer_wilson_95 = Some([
                (center - half_width).max(0.0),
                (center + half_width).min(1.0),
            ]);
            if self.post_bch_payload_codeword_errors == 0 {
                self.zero_error_one_sided_upper_95 = Some(1.0 - 0.05_f64.powf(1.0 / trials));
            }
            self.mean_decoder_iterations = Some(self.total_decoder_iterations as f64 / trials);
        }
        self.elapsed_ms = elapsed_ms;
    }
}
