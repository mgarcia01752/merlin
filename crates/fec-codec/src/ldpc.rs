/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Job-owned LDPC encoder and soft decoder.
use crate::{
    binary_bits,
    parameters::{LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
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
            log::debug!(
                "payload-ldpc-input-rejected llrs={} max-iterations={max_iterations}",
                llrs.len()
            );
            return Err("Invalid LDPC LLRs or iteration limit.".into());
        }
        let (success, output) = match self.decoder.decode(llrs, max_iterations) {
            Ok(output) => (true, output),
            Err(output) => (false, output),
        };
        if !success {
            log::debug!(
                "payload-ldpc-not-converged iterations={} max-iterations={max_iterations}",
                output.iterations
            );
        }
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
