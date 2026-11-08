/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Replaceable codec primitives. No Qt, transport, captures or receiver policy.
use ldpc_toolbox::{
    codes::dvbs2::Code,
    decoder::{
        factory::{DecoderFactory, DecoderImplementation},
        LdpcDecoder,
    },
    encoder::Encoder,
    gf2::GF2,
    sparse::SparseMatrix,
};
use ndarray::Array1;
use num_traits::{One, Zero};

/// DOCSIS PHY I20 Table 23 / ETSI EN302769 V1.2.1 Table 3(b).
pub const BCH_INPUT_BITS: usize = 14_232;
/// BCH output and LDPC input length for the selected unshortened coding mode.
pub const LDPC_INPUT_BITS: usize = 14_400;
/// LDPC transmitted codeword length for the selected coding mode.
pub const LDPC_CODEWORD_BITS: usize = 16_200;

#[cxx::bridge(namespace = "pnm_fec")]
mod ffi {
    struct BchDecoded {
        codeword: Vec<u8>,
        corrected: u32,
        success: bool,
    }
    // Native codec boundary only; reusable PNM core continues forbidding unsafe.
    unsafe extern "C++" {
        include!("fec-codec/native/bch.hpp");
        fn bch_encode(message: &[u8]) -> Result<Vec<u8>>;
        fn bch_decode(codeword: &[u8]) -> Result<BchDecoded>;
    }
}

/// Complete corrected BCH word, with decoder status separate from truth BER.
pub struct BchResult {
    pub codeword: Vec<u8>,
    pub corrected_bits: u32,
    pub success: bool,
}

fn binary_bits(bits: &[u8], expected: usize) -> Result<(), String> {
    if bits.len() != expected || bits.iter().any(|bit| *bit > 1) {
        return Err(format!("Expected exactly {expected} binary bits."));
    }
    Ok(())
}

/// Encodes MSB-first payload into MSB-first systematic BCH data followed by parity.
/// # Arguments
/// * `message` - Exactly14232 binary bits, matching the selected PHY coding mode.
/// # Returns
/// 14400 bits; input bytes are never modified.
/// # Errors
/// Rejects invalid lengths/bits or native codec failures.
pub fn encode_bch(message: &[u8]) -> Result<Vec<u8>, String> {
    binary_bits(message, BCH_INPUT_BITS)?;
    ffi::bch_encode(message).map_err(|error| error.to_string())
}

/// Corrects at most12 errors in the selected shortened BCH word.
/// # Arguments
/// * `codeword` - Exactly14400 binary bits in transmitted MSB-first order.
/// # Returns
/// Complete word and bounded-distance status; success is not a truth oracle.
/// # Errors
/// Rejects invalid inputs or native codec failures. Decoder failure is a result.
pub fn decode_bch(codeword: &[u8]) -> Result<BchResult, String> {
    binary_bits(codeword, LDPC_INPUT_BITS)?;
    let decoded = ffi::bch_decode(codeword).map_err(|error| error.to_string())?;
    Ok(BchResult {
        codeword: decoded.codeword,
        corrected_bits: decoded.corrected,
        success: decoded.success,
    })
}

/// Reused LDPC encoder/decoder; construction occurs once per simulation job.
pub struct LdpcCodec {
    matrix: SparseMatrix,
    encoder: Encoder,
    decoder: Box<dyn LdpcDecoder>,
}
/// Soft-decoder outcome, preserving failure decisions for BER analysis.
pub struct LdpcResult {
    pub codeword: Vec<u8>,
    pub iterations: usize,
    pub success: bool,
}
impl LdpcCodec {
    /// Constructs the rate8/9 short matrix verified against DVB-C2 AnnexB.6.
    /// # Returns
    /// Independent job-owned codec with horizontal layered sum-product decoding.
    /// # Errors
    /// Reports an invalid parity matrix instead of panicking.
    pub fn new() -> Result<Self, String> {
        let matrix = Code::R8_9short.h();
        let encoder = Encoder::from_h(&matrix).map_err(|error| error.to_string())?;
        let decoder = DecoderImplementation::HLPhif64.build_decoder(matrix.clone());
        Ok(Self {
            matrix,
            encoder,
            decoder,
        })
    }
    /// Encodes a complete systematic LDPC information word.
    /// # Arguments
    /// * `message` - Exactly14400 binary bits (BCH data followed by BCH parity).
    /// # Returns
    /// 16200 binary bits in systematic transmitted order.
    /// # Errors
    /// Rejects invalid lengths or nonbinary values before invoking the library.
    pub fn encode(&self, message: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(message, LDPC_INPUT_BITS)?;
        let data =
            Array1::from_iter(message.iter().map(
                |bit| {
                    if *bit == 0 {
                        GF2::zero()
                    } else {
                        GF2::one()
                    }
                },
            ));
        Ok(self
            .encoder
            .encode(&data)
            .iter()
            .map(|value| u8::from(!value.is_zero()))
            .collect())
    }
    /// Decodes finite natural-log P(bit0)/P(bit1) likelihood ratios.
    /// # Arguments
    /// * `llrs` - Exactly16200 finite LLRs; positive favors zero.
    /// * `max_iterations` - Positive bounded work limit, at most100.
    /// # Returns
    /// Final hard decisions, iterations and parity-check success.
    /// # Errors
    /// Rejects invalid size, nonfinite values and an unbounded iteration request.
    pub fn decode(&mut self, llrs: &[f64], max_iterations: usize) -> Result<LdpcResult, String> {
        if llrs.len() != LDPC_CODEWORD_BITS
            || llrs.iter().any(|value| !value.is_finite())
            || !(1..=100).contains(&max_iterations)
        {
            return Err("Invalid LDPC LLRs or iteration limit.".into());
        }
        let (success, output) = match self.decoder.decode(llrs, max_iterations) {
            Ok(output) => (true, output),
            Err(output) => (false, output),
        };
        Ok(LdpcResult {
            codeword: output.codeword,
            iterations: output.iterations,
            success,
        })
    }
    /// Checks every parity equation independently of decoder success.
    /// # Arguments
    /// * `codeword` - Complete binary LDPC word.
    /// # Returns
    /// True only for the right length, binary values and zero syndrome.
    pub fn valid_syndrome(&self, codeword: &[u8]) -> bool {
        binary_bits(codeword, LDPC_CODEWORD_BITS).is_ok()
            && (0..self.matrix.num_rows()).all(|row| {
                self.matrix
                    .iter_row(row)
                    .fold(0, |parity, index| parity ^ codeword[*index])
                    == 0
            })
    }
}
