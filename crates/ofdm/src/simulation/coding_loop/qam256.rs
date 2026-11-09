/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! BCH→LDPC→bit processing→randomization→256-QAM and the noiseless inverse.
use crate::{
    error::invalid,
    signal_processing::modulation::{
        interleaving::BitInterleaver, map_symbol, qam_llrs, randomization::CellRandomizer,
    },
    types::ofdm_modulation::ConstellationOrder,
    SimulationError,
};
use fec_codec::{
    concatenated::{compare_payload, ConcatenatedCodec},
    parameters::{LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
use serde::Serialize;

const CONSTELLATION_ORDER: ConstellationOrder = 256;
// Positive finite LLR scaling for the reference demapper; no noise is injected.
const LLR_REFERENCE_NOISE_POWER: f64 = 0.01;
const CODING_MODE: &str = "ds_short_8_9_256qam_noiseless_cell_loop";

/// Per-codeword truth measurements, including actual LDPC iteration count.
#[derive(Debug, Clone, Serialize)]
pub struct Qam256BlockResult {
    pub coding_mode: &'static str,
    pub data_cells: usize,
    pub coded_bits_tested: usize,
    /// Hard errors after soft inverse randomization and bit processing, before FEC.
    pub pre_ldpc_bit_errors: usize,
    pub post_ldpc_bits_tested: usize,
    /// Errors in the14400 systematic bits after LDPC, before BCH.
    pub post_ldpc_bit_errors: usize,
    pub payload_bits_tested: usize,
    pub payload_bit_errors: usize,
    /// Iterations reported by the LDPC decoder, regardless of success/failure.
    pub ldpc_iterations: usize,
    pub ldpc_success: bool,
    pub bch_success: bool,
    pub bch_corrected_bits: u32,
    /// Finite scaling for LLR generation; does not describe an injected noise source.
    pub llr_reference_noise_power: f64,
    pub assumptions: Vec<String>,
}

/// Reusable codec/permutation setup for independent bounded256-QAM block trials.
///
/// Each call starts fresh phase0 randomizers, so successive trials are not a
/// continuous OFDM transmission. No pilots, NCP messages or waveform are generated.
pub struct NoiselessQam256Codec {
    codec: ConcatenatedCodec,
    interleaver: BitInterleaver,
}
impl NoiselessQam256Codec {
    /// Constructs the supported short8/9 coding path once for repeated trials.
    /// # Returns
    /// Job-owned codec state and immutable256-QAM bit permutation.
    /// # Errors
    /// Reports codec setup failures or an unavailable bit-processing mode.
    pub fn new() -> Result<Self, SimulationError> {
        Ok(Self {
            codec: ConcatenatedCodec::new().map_err(invalid)?,
            interleaver: BitInterleaver::new(CONSTELLATION_ORDER)?,
        })
    }

    /// Runs one independent noiseless block through all supported coding stages.
    /// # Arguments
    /// * `payload` - Exactly14232 binary bits; retained unchanged for truth comparison.
    /// * `max_iterations` - LDPC iteration cap1..=100, independent of trial timing.
    /// # Returns
    /// Per-block truth errors, decoder status and iterations. Valid requests may
    /// return a failed decoder outcome; inspect flags and truth counts separately.
    /// # Errors
    /// Rejects invalid payload/work limits and propagates stage failures. Work is
    /// bounded to one16200-bit block; no filesystem, UI or callbacks are involved.
    pub fn run_block(
        &mut self,
        payload: &[u8],
        max_iterations: usize,
    ) -> Result<Qam256BlockResult, SimulationError> {
        if !(1..=100).contains(&max_iterations) {
            return Err(invalid("LDPC iteration cap must be1..100."));
        }
        let encoded = self.codec.encode(payload).map_err(invalid)?;
        let labels = self.interleaver.encode_labels(&encoded.ldpc_codeword)?;
        let mut transmit = CellRandomizer::new();
        let mut receive = CellRandomizer::new();
        transmit.begin_symbol(0)?;
        receive.begin_symbol(0)?;
        let mut cell_llrs = Vec::with_capacity(LDPC_CODEWORD_BITS);
        for &label in &labels {
            let randomized = transmit.apply_data(CONSTELLATION_ORDER, label)?;
            let point = map_symbol(CONSTELLATION_ORDER, randomized)?;
            let soft = qam_llrs(CONSTELLATION_ORDER, point, LLR_REFERENCE_NOISE_POWER)?;
            cell_llrs.extend(receive.descramble_data_llrs(CONSTELLATION_ORDER, &soft)?);
        }
        let llrs = self.interleaver.decode_llrs(&cell_llrs)?;
        let pre_ldpc_bit_errors = llrs
            .iter()
            .zip(&encoded.ldpc_codeword)
            .filter(|(llr, bit)| u8::from(**llr < 0.0) != **bit)
            .count();
        let decoded = self.codec.decode(&llrs, max_iterations).map_err(invalid)?;
        let comparison = compare_payload(payload, &decoded).map_err(invalid)?;
        let post_ldpc_bit_errors = decoded.ldpc.codeword[..LDPC_INPUT_BITS]
            .iter()
            .zip(&encoded.bch_codeword)
            .filter(|(actual, expected)| actual != expected)
            .count();
        Ok(Qam256BlockResult {
            coding_mode: CODING_MODE, data_cells: labels.len(),
            coded_bits_tested: LDPC_CODEWORD_BITS, pre_ldpc_bit_errors,
            post_ldpc_bits_tested: LDPC_INPUT_BITS, post_ldpc_bit_errors,
            payload_bits_tested: comparison.bits_tested, payload_bit_errors: comparison.bit_errors,
            ldpc_iterations: decoded.ldpc.iterations, ldpc_success: decoded.ldpc.success,
            bch_success: decoded.bch.success, bch_corrected_bits: decoded.bch.corrected_bits,
            llr_reference_noise_power: LLR_REFERENCE_NOISE_POWER,
            assumptions: [
                "Independent noiseless codeword trial; randomizers begin at phase0 for each trial.",
                "Finite positive demapper reference variance scales LLRs; no channel noise is injected.",
                "No physical OFDM packing, pilot/control insertion, time/frequency interleaving or waveform.",
            ].into_iter().map(str::to_owned).collect(),
        })
    }
}
