/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Concatenated codec path, independently usable without modulation or a channel.
use crate::{
    bch::{decode_bch, encode_bch, BchResult},
    binary_bits,
    ldpc::{LdpcCodec, LdpcResult},
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};

/// Finite reliable-bit evidence for a noiseless codec-loop test; not a noise model.
const NOISELESS_LLR_MAGNITUDE: f64 = 20.0;
/// Transmit stages retained for explicit post-LDPC and coded-bit comparisons.
pub struct EncodedBlock {
    pub bch_codeword: Vec<u8>,
    pub ldpc_codeword: Vec<u8>,
}
/// Receive stages retain both decoder outcomes, including failed hard decisions.
pub struct DecodedBlock {
    pub ldpc: LdpcResult,
    pub bch: BchResult,
}
/// Original-versus-recovered payload comparison, independent of decoder flags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayloadComparison {
    pub bits_tested: usize,
    pub bit_errors: usize,
}
/// One reusable downstream BCH→LDPC encoder and LDPC→BCH decoder.
pub struct ConcatenatedCodec {
    ldpc: LdpcCodec,
}
impl ConcatenatedCodec {
    /// Creates job-owned coding state for the supported short8/9 block.
    /// # Returns
    /// A reusable concatenated codec with no QAM or channel assumptions.
    /// # Errors
    /// Propagates invalid LDPC matrix/setup failures.
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            ldpc: LdpcCodec::new()?,
        })
    }
    /// Encodes payload bits through BCH first and LDPC second.
    /// # Arguments
    /// * `payload` - Exactly14232 binary source bits in MSB-first order.
    /// # Returns
    /// BCH14400-bit and LDPC16200-bit words without changing the input.
    /// # Errors
    /// Rejects invalid bit counts/values or codec setup failures.
    pub fn encode(&self, payload: &[u8]) -> Result<EncodedBlock, String> {
        let bch_codeword = encode_bch(payload)?;
        let ldpc_codeword = self.ldpc.encode(&bch_codeword)?;
        Ok(EncodedBlock {
            bch_codeword,
            ldpc_codeword,
        })
    }
    /// Runs reverse decoding, preserving failures at both stages.
    /// # Arguments
    /// * `llrs` - Exactly16200 finite ln(P0/P1) values; positive favors zero.
    /// * `max_iterations` - LDPC iteration limit in1..=100.
    /// # Returns
    /// LDPC and BCH outcomes; parity success alone does not establish truth BER.
    /// # Errors
    /// Rejects invalid receiver input or invalid codec output dimensions.
    pub fn decode(&mut self, llrs: &[f64], max_iterations: usize) -> Result<DecodedBlock, String> {
        let ldpc = self.ldpc.decode(llrs, max_iterations)?;
        let bch_input = ldpc
            .codeword
            .get(..LDPC_INPUT_BITS)
            .ok_or("LDPC output is shorter than the BCH block.")?;
        let bch = decode_bch(bch_input)?;
        Ok(DecodedBlock { ldpc, bch })
    }
    /// Decodes a binary word using fixed finite noiseless bit likelihoods.
    /// # Arguments
    /// * `codeword` - Exactly16200 binary encoded bits; no channel is injected.
    /// * `max_iterations` - Positive LDPC iteration cap, at most100.
    /// # Returns
    /// Reverse codec outcomes for an isolated coding-loop test.
    /// # Errors
    /// Rejects invalid encoded bits or receiver limits.
    pub fn decode_noiseless(
        &mut self,
        codeword: &[u8],
        max_iterations: usize,
    ) -> Result<DecodedBlock, String> {
        binary_bits(codeword, LDPC_CODEWORD_BITS)?;
        let llrs: Vec<_> = codeword
            .iter()
            .map(|bit| {
                if *bit == 0 {
                    NOISELESS_LLR_MAGNITUDE
                } else {
                    -NOISELESS_LLR_MAGNITUDE
                }
            })
            .collect();
        self.decode(&llrs, max_iterations)
    }
}

/// Compares recovered BCH payload bits with the original source bits.
/// # Arguments
/// * `original` - Exactly14232 binary bits before BCH encoding.
/// * `decoded` - Reverse codec outcomes; decoder status is not used as truth.
/// # Returns
/// Tested payload length and exact mismatched-bit count.
/// # Errors
/// Rejects malformed original or decoded BCH data, including caller-built results.
pub fn compare_payload(
    original: &[u8],
    decoded: &DecodedBlock,
) -> Result<PayloadComparison, String> {
    binary_bits(original, BCH_INPUT_BITS)?;
    binary_bits(&decoded.bch.codeword, LDPC_INPUT_BITS)?;
    let bit_errors = original
        .iter()
        .zip(&decoded.bch.codeword[..BCH_INPUT_BITS])
        .filter(|(expected, actual)| expected != actual)
        .count();
    Ok(PayloadComparison {
        bits_tested: original.len(),
        bit_errors,
    })
}
