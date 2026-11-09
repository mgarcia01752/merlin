/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Job-owned mother encoder and puncture-aware soft decoder.
use super::{
    matrix::mother_matrix, KNOWN_ZERO_LLR, MOTHER_CODEWORD_BITS, MOTHER_INFORMATION_BITS,
    NCP_CODEWORD_BITS, NCP_INFORMATION_BITS, NCP_TRANSMITTED_COORDINATES,
};
use ldpc_toolbox::{
    decoder::{
        factory::{DecoderFactory, DecoderImplementation},
        LdpcDecoder,
    },
    encoder::Encoder,
    gf2::GF2,
};
use ndarray::Array1;
use num_traits::{One, Zero};

/// NCP control-block outcome; parity success does not establish field CRC success.
pub struct NcpBlockResult {
    pub payload: [u8; 3],
    pub iterations: usize,
    pub success: bool,
}
/// Reusable (48,24) NCP codec built from the pinned (160,80) mother LDPC.
pub struct NcpFecCodec {
    encoder: Encoder,
    decoder: Box<dyn LdpcDecoder>,
}
impl NcpFecCodec {
    /// Constructs an independent NCP encoder and sum-product decoder.
    ///
    /// # Returns
    /// Job-owned setup with the PHY Table19 matrix; no payload BCH is used.
    /// # Errors
    /// Reports a noninvertible parity matrix instead of panicking.
    pub fn new() -> Result<Self, String> {
        let matrix = mother_matrix();
        let encoder = Encoder::from_h(&matrix).map_err(|error| error.to_string())?;
        let decoder = DecoderImplementation::HLPhif64.build_decoder(matrix);
        Ok(Self { encoder, decoder })
    }
    /// Encodes one three-byte NCP message or three-byte CRC.
    ///
    /// # Arguments
    /// * `payload` - Bytes0..2; a0 is byte0's MSB, per Figures71–72.
    /// # Returns
    ///48 binary bits in ascending retained mother-coordinate order. Shortened
    /// a24..79 are zero; parity80..103,112..127,144..159 is punctured.
    pub fn encode(&self, payload: &[u8; 3]) -> [u8; NCP_CODEWORD_BITS] {
        let information = Array1::from_iter((0..MOTHER_INFORMATION_BITS).map(|index| {
            if index < NCP_INFORMATION_BITS && (payload[index / 8] >> (7 - index % 8)) & 1 != 0 {
                GF2::one()
            } else {
                GF2::zero()
            }
        }));
        let mother = self.encoder.encode(&information);
        NCP_TRANSMITTED_COORDINATES.map(|coordinate| u8::from(!mother[coordinate].is_zero()))
    }
    /// Soft-decodes one shortened/punctured NCP block.
    ///
    /// # Arguments
    /// * `llrs` - Exactly48 finite natural-log P(0)/P(1) values in encode order.
    /// * `max_iterations` - Bounded decoder work,1..100.
    /// # Returns
    /// Payload decisions, actual iterations and convergence/shortening status.
    /// Punctures receive zero LLR; known shortened zeros receive +100.
    /// Valid input may return success=false; callers must reject that block.
    /// # Errors
    /// Rejects invalid size, nonfinite LLRs and unsupported iteration bounds.
    pub fn decode(
        &mut self,
        llrs: &[f64],
        max_iterations: usize,
    ) -> Result<NcpBlockResult, String> {
        if llrs.len() != NCP_CODEWORD_BITS
            || llrs.iter().any(|value| !value.is_finite())
            || !(1..=100).contains(&max_iterations)
        {
            log::debug!(
                "ncp-ldpc-input-rejected llrs={} max-iterations={max_iterations}",
                llrs.len()
            );
            return Err("Expected48 finite NCP LLRs and1..100 iterations.".into());
        }
        let mut mother = [0.0; MOTHER_CODEWORD_BITS];
        mother[NCP_INFORMATION_BITS..MOTHER_INFORMATION_BITS].fill(KNOWN_ZERO_LLR);
        for (&coordinate, &llr) in NCP_TRANSMITTED_COORDINATES.iter().zip(llrs) {
            mother[coordinate] = llr.clamp(-KNOWN_ZERO_LLR, KNOWN_ZERO_LLR);
        }
        let (converged, decoded) = match self.decoder.decode(&mother, max_iterations) {
            Ok(result) => (true, result),
            Err(result) => (false, result),
        };
        let mut payload = [0u8; 3];
        for index in 0..NCP_INFORMATION_BITS {
            payload[index / 8] |= decoded.codeword[index] << (7 - index % 8);
        }
        let success = converged
            && decoded.codeword[NCP_INFORMATION_BITS..MOTHER_INFORMATION_BITS]
                .iter()
                .all(|bit| *bit == 0);
        if !success {
            log::debug!(
                "ncp-ldpc-not-converged iterations={} max-iterations={max_iterations}",
                decoded.iterations
            );
        }
        Ok(NcpBlockResult {
            payload,
            iterations: decoded.iterations,
            success,
        })
    }
}
