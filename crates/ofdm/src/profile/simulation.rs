/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual concatenated codec experiment, not an RF receiver or modem emulator.
use super::{invalid, model::*, LabError};
use crate::{math::qam::reference_for_order, types::modulation::CarrierType};
use fec_codec::{
    decode_bch, encode_bch, LdpcCodec, BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS,
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rand_distr::{Distribution, StandardNormal};
use serde::{Deserialize, Serialize};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Instant,
};

/// Reproducible noncryptographic simulation seed.
pub type SimulationSeed = u64;
/// The verified coding tables define this bounded experiment; full PHY placement is outside its scope.
pub const CODING_MODE: &str = "ds_short_8_9_16qam_coding_experiment";
const BITS_PER_SYMBOL: usize = 4;
const INTERLEAVER_COLUMNS: usize = 8;
const INTERLEAVER_ROWS: usize = 2025;
// PHY I20 Tables24/25; ETSI EN302769 V1.2.1 Table7(a), short16-QAM.
const COLUMN_TWISTS: [usize; 8] = [0, 0, 0, 1, 7, 20, 20, 21];
// ETSI EN302769 V1.2.1 Table10(a), rate8/9, 16-QAM input -> substream.
const DEMULTIPLEXER: [usize; 8] = [7, 1, 4, 2, 5, 3, 6, 0];
const LLR_LIMIT: f64 = 100.0;

/// Finite work policy. Runtime exhaustion/cancellation preserve completed trials.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSettings {
    pub seed: SimulationSeed,
    pub codewords: usize,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub awgn_mer_db: f64,
    pub noiseless: bool,
}
impl Default for SimulationSettings {
    /// Provides a small interactive AWGN coding experiment.
    /// # Returns
    /// Seed1, ten words, 30iterations and a10second runtime budget at25dB MER.
    fn default() -> Self {
        Self {
            seed: 1,
            codewords: 10,
            max_iterations: 30,
            max_runtime_ms: 10_000,
            awgn_mer_db: 25.0,
            noiseless: false,
        }
    }
}
impl SimulationSettings {
    /// Validates numeric inputs before allocating a codec or generating trials.
    /// # Returns
    /// Unit for a finite, bounded experiment.
    /// # Errors
    /// Rejects nonfinite RF inputs and excessive trial/runtime/iteration requests.
    pub fn validate(&self) -> Result<(), LabError> {
        if !(1..=512).contains(&self.codewords)
            || !(1..=100).contains(&self.max_iterations)
            || !(1..=300_000).contains(&self.max_runtime_ms)
            || !self.awgn_mer_db.is_finite()
            || !(-20.0..=80.0).contains(&self.awgn_mer_db)
        {
            return Err(invalid(
                "Invalid FEC trial count, iteration/runtime limit or MER.",
            ));
        }
        Ok(())
    }
}
/// Measured-noise selection. Unknown/missing data readings are never imputed.
pub struct MeasuredNoise<'capture> {
    pub frame: &'capture RxMerFrame,
    pub candidate: &'capture CandidateProfile,
    pub first: usize,
    pub count: usize,
}
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

/// Returns output-bit -> original-codeword-bit order for this coding experiment.
/// # Returns
/// A full16200-element bijection: parity interleave, twist, then16-QAM demux.
pub fn transmit_bit_order() -> Vec<usize> {
    let mut parity: Vec<usize> = (0..LDPC_CODEWORD_BITS).collect();
    for group in 0..5 {
        for offset in 0..360 {
            parity[LDPC_INPUT_BITS + 360 * group + offset] = LDPC_INPUT_BITS + 5 * offset + group;
        }
    }
    let mut twisted = vec![0; LDPC_CODEWORD_BITS];
    for (input, original) in parity.into_iter().enumerate() {
        let column = input / INTERLEAVER_ROWS;
        let row = (input % INTERLEAVER_ROWS + COLUMN_TWISTS[column]) % INTERLEAVER_ROWS;
        twisted[row * INTERLEAVER_COLUMNS + column] = original;
    }
    let mut order = vec![0; LDPC_CODEWORD_BITS];
    for (input, original) in twisted.into_iter().enumerate() {
        let output = input / INTERLEAVER_COLUMNS * INTERLEAVER_COLUMNS
            + DEMULTIPLEXER[input % INTERLEAVER_COLUMNS];
        order[output] = original;
    }
    order
}

/// Maps a four-bit MSB-first label using the embedded normalized reference table.
/// # Arguments
/// * `label` - Integer label in0..16; table lineage is recorded in the resources.
/// # Returns
/// Unit-average-power I/Q, or None for an invalid label.
pub fn map_qam16(label: usize) -> Option<[f64; 2]> {
    reference_for_order(16)?.iq.get(label).copied()
}

/// Computes max-log LLRs with positive values favoring bit0.
/// # Arguments
/// * `received` - Equalized unit-energy complex symbol [I,Q].
/// * `noise_power` - E[|n|²]; each real component has variance noise_power/2.
/// # Returns
/// Four natural-log max-log likelihoods, clipped to the documented decoder limit.
/// # Errors
/// Rejects invalid symbols and nonpositive/nonfinite complex noise power.
pub fn qam16_llrs(received: [f64; 2], noise_power: f64) -> Result<[f64; 4], LabError> {
    if received.iter().any(|value| !value.is_finite())
        || !noise_power.is_finite()
        || noise_power <= 0.0
    {
        return Err(invalid(
            "Finite I/Q and positive complex noise variance are required.",
        ));
    }
    let mut distances = [[f64::INFINITY; 2]; BITS_PER_SYMBOL];
    for label in 0..16 {
        let point = map_qam16(label).ok_or_else(|| invalid("Missing16-QAM reference table."))?;
        let distance = (received[0] - point[0]).powi(2) + (received[1] - point[1]).powi(2);
        for (bit, hypotheses) in distances.iter_mut().enumerate() {
            let value = (label >> (3 - bit)) & 1;
            hypotheses[value] = hypotheses[value].min(distance);
        }
    }
    Ok(distances.map(|hypotheses| {
        ((hypotheses[1] - hypotheses[0]) / noise_power).clamp(-LLR_LIMIT, LLR_LIMIT)
    }))
}

fn noise_model(
    settings: &SimulationSettings,
    measured: Option<&MeasuredNoise<'_>>,
) -> Result<(Vec<f64>, Vec<usize>), LabError> {
    let Some(measured) = measured else {
        return Ok((vec![10.0_f64.powf(-settings.awgn_mer_db / 10.0)], vec![]));
    };
    if measured.candidate.protocol != ProtocolScope::Docsis31 {
        return Err(LabError::new("UnsupportedCodingMode", "The first coding experiment is scoped to DOCSIS3.1; DOCSIS4.0 placement is not verified."));
    }
    measured.frame.validate()?;
    measured.candidate.validate()?;
    if measured.frame.key != measured.candidate.key {
        return Err(invalid("Candidate and measured RF grids differ."));
    }
    let end = measured
        .first
        .checked_add(measured.count)
        .filter(|end| measured.count > 0 && *end <= measured.frame.mer_db.len())
        .ok_or_else(|| invalid("Simulation selection is outside the source grid."))?;
    let mut powers = Vec::new();
    let mut offsets = Vec::new();
    for offset in measured.first..end {
        let assignment = &measured.candidate.assignments[offset];
        match assignment.carrier_type {
            CarrierType::Data => {
                if assignment.bit_loading != Some(4) {
                    return Err(LabError::new(
                        "UnsupportedCodingMode",
                        "This coding slice supports uniform16-QAM data assignments only.",
                    ));
                }
                let mer = measured.frame.mer_db[offset].ok_or_else(|| {
                    invalid("A selected data carrier has no measured residual-noise model.")
                })?;
                let power = 10.0_f64.powf(-mer / 10.0);
                if !power.is_finite() || power <= 0.0 {
                    return Err(invalid(
                        "Measured MER is outside the supported numeric noise range.",
                    ));
                }
                powers.push(power);
                offsets.push(offset);
            }
            CarrierType::Unknown | CarrierType::Reserved => {
                return Err(invalid(
                    "Resolve unknown/reserved roles before detailed simulation.",
                ));
            }
            _ => {}
        }
    }
    if powers.is_empty() {
        return Err(invalid("No data carriers are selected for simulation."));
    }
    Ok((powers, offsets))
}

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
    settings.validate()?;
    let (noise_powers, selected_carrier_offsets) = noise_model(settings, measured)?;
    let order = transmit_bit_order();
    let mut codec = LdpcCodec::new().map_err(invalid)?;
    let mut rng = ChaCha8Rng::seed_from_u64(settings.seed);
    let started = Instant::now();
    let mut result = SimulationResult {
        result_kind: "simulated_fec_outcome".into(), mode: CODING_MODE.into(), status: "completed".into(),
        settings: settings.clone(), completed_codewords: 0, pre_ldpc_bit_errors: 0, pre_ldpc_bits_tested: 0,
        pre_ldpc_ber: None, post_ldpc_bit_errors: 0, post_ldpc_bits_tested: 0, post_ldpc_ber: None,
        post_bch_payload_bit_errors: 0, payload_bits_tested: 0, post_bch_payload_ber: None,
        ldpc_detected_failures: 0, bch_detected_failures: 0, bch_corrected_bits: 0,
        ldpc_undetected_codeword_errors: 0, bch_undetected_payload_errors: 0,
        post_bch_payload_codeword_errors: 0, post_bch_payload_cwer: None,
        zero_error_one_sided_upper_95: None, cwer_wilson_95: None,
        total_decoder_iterations: 0, maximum_decoder_iterations_used: 0, mean_decoder_iterations: None,
        source: measured.map(|input| input.frame.source.clone()), selected_carrier_offsets, elapsed_ms: 0,
        assumptions: vec![
            "Coding-block input is post-randomizer; MAC/NCP/PLC framing and waveform generation are outside this boundary.".into(),
            "Unit-average symbol energy; complex residual noise power=10^(-MER/10), real-component variance is half that power.".into(),
            "Max-log LLR=ln(P0/P1), positive favors0; clip magnitude100; horizontal layered sum-product decoding.".into(),
            "Sequential cycling through selected observed data carriers; no claimed DOCSIS frequency/time-interleaver placement.".into(),
            "Ideal equalized receiver; channel-estimate coefficients are contextual observations, not applied again to measured RxMER.".into(),
            "No inferred impulse noise, temporal correlation, clipping, LDPC implementation equivalence or modem behavior.".into(),
        ],
        validation_state: "experimental_coding_chain_independent_interoperability_vectors_pending".into(),
    };
    for _ in 0..settings.codewords {
        if cancel.load(Ordering::Relaxed) {
            result.status = "cancelled".into();
            break;
        }
        if started.elapsed().as_millis() >= u128::from(settings.max_runtime_ms) {
            result.status = "runtime_limit".into();
            break;
        }
        let message: Vec<u8> = (0..BCH_INPUT_BITS)
            .map(|_| u8::from(rng.random::<bool>()))
            .collect();
        let bch = encode_bch(&message).map_err(invalid)?;
        let word = codec.encode(&bch).map_err(invalid)?;
        let mut llrs = vec![0.0; LDPC_CODEWORD_BITS];
        for (symbol_index, symbol_order) in order.chunks_exact(BITS_PER_SYMBOL).enumerate() {
            let label = symbol_order
                .iter()
                .fold(0, |label, offset| (label << 1) | usize::from(word[*offset]));
            let mut symbol = map_qam16(label).ok_or_else(|| invalid("Missing QAM reference."))?;
            let power = noise_powers[symbol_index % noise_powers.len()];
            if !settings.noiseless {
                let standard_deviation = (power / 2.0).sqrt();
                for component in &mut symbol {
                    let gaussian: f64 = StandardNormal.sample(&mut rng);
                    *component += standard_deviation * gaussian;
                }
            }
            let likelihoods = qam16_llrs(symbol, power)?;
            for (bit, source_offset) in symbol_order.iter().enumerate() {
                llrs[*source_offset] = likelihoods[bit];
            }
        }
        result.pre_ldpc_bit_errors += llrs
            .iter()
            .zip(&word)
            .filter(|(llr, bit)| u8::from(**llr < 0.0) != **bit)
            .count() as u64;
        result.pre_ldpc_bits_tested += LDPC_CODEWORD_BITS as u64;
        let decoded = codec
            .decode(&llrs, settings.max_iterations)
            .map_err(invalid)?;
        result.ldpc_detected_failures += usize::from(!decoded.success);
        result.ldpc_undetected_codeword_errors +=
            usize::from(decoded.success && decoded.codeword != word);
        result.total_decoder_iterations += decoded.iterations;
        result.maximum_decoder_iterations_used = result
            .maximum_decoder_iterations_used
            .max(decoded.iterations);
        let decoded_bch = &decoded.codeword[..LDPC_INPUT_BITS];
        result.post_ldpc_bit_errors += decoded_bch
            .iter()
            .zip(&bch)
            .filter(|(decoded, expected)| decoded != expected)
            .count() as u64;
        result.post_ldpc_bits_tested += LDPC_INPUT_BITS as u64;
        let corrected = decode_bch(decoded_bch).map_err(invalid)?;
        result.bch_detected_failures += usize::from(!corrected.success);
        result.bch_corrected_bits += u64::from(corrected.corrected_bits);
        let payload_errors = corrected.codeword[..BCH_INPUT_BITS]
            .iter()
            .zip(&message)
            .filter(|(decoded, expected)| decoded != expected)
            .count();
        result.post_bch_payload_bit_errors += payload_errors as u64;
        result.payload_bits_tested += BCH_INPUT_BITS as u64;
        result.post_bch_payload_codeword_errors += usize::from(payload_errors > 0);
        result.bch_undetected_payload_errors +=
            usize::from(corrected.success && payload_errors > 0);
        result.completed_codewords += 1;
        progress(SimulationProgress {
            completed_codewords: result.completed_codewords,
            requested_codewords: settings.codewords,
            elapsed_ms: started.elapsed().as_millis() as u64,
        });
    }
    let ratio = |errors: u64, count: u64| (count > 0).then(|| errors as f64 / count as f64);
    result.pre_ldpc_ber = ratio(result.pre_ldpc_bit_errors, result.pre_ldpc_bits_tested);
    result.post_ldpc_ber = ratio(result.post_ldpc_bit_errors, result.post_ldpc_bits_tested);
    result.post_bch_payload_ber = ratio(
        result.post_bch_payload_bit_errors,
        result.payload_bits_tested,
    );
    result.post_bch_payload_cwer = ratio(
        result.post_bch_payload_codeword_errors as u64,
        result.completed_codewords as u64,
    );
    if let Some(rate) = result.post_bch_payload_cwer {
        let trials = result.completed_codewords as f64;
        let z_squared = 1.959_963_984_540_054_f64.powi(2);
        let denominator = 1.0 + z_squared / trials;
        let center = (rate + z_squared / (2.0 * trials)) / denominator;
        let half_width = (z_squared
            * (rate * (1.0 - rate) / trials + z_squared / (4.0 * trials.powi(2))))
        .sqrt()
            / denominator;
        result.cwer_wilson_95 = Some([
            (center - half_width).max(0.0),
            (center + half_width).min(1.0),
        ]);
        if result.post_bch_payload_codeword_errors == 0 {
            result.zero_error_one_sided_upper_95 = Some(1.0 - 0.05_f64.powf(1.0 / trials));
        }
        result.mean_decoder_iterations = Some(result.total_decoder_iterations as f64 / trials);
    }
    result.elapsed_ms = started.elapsed().as_millis() as u64;
    Ok(result)
}
