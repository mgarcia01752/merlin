/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Puncture-aware PLC soft decoding, actual iterations and full mother syndrome.
use super::{
    parameters::{MAX_PLC_ITERATIONS, PLC_MOTHER_BITS},
    puncturing::restore_llrs,
    PlcBlockResult, PlcFecCodec,
};
use crate::binary_bits;
impl PlcFecCodec {
    /// Decodes transmitted soft values after restoring unknown puncture coordinates.
    /// # Arguments
    /// * `llrs` -384 finite natural-log P(0)/P(1) values in encode output order.
    /// * `max_iterations` - Positive application work limit, at most100.
    /// # Returns
    /// All288 information decisions, complete mother word, actual iterations and
    /// convergence with independently checked parity. Failure decisions are retained.
    /// No original information bits or transmitter IDs are supplied to this method.
    /// # Errors
    /// Rejects invalid LLRs/work limits or unexpected decoder output shape.
    pub fn decode(
        &mut self,
        llrs: &[f64],
        max_iterations: usize,
    ) -> Result<PlcBlockResult, String> {
        if !(1..=MAX_PLC_ITERATIONS).contains(&max_iterations) {
            return Err("PLC decoder iterations must be1..100.".into());
        }
        let mother = restore_llrs(llrs)?;
        let (converged, decoded) = match self.decoder.decode(&mother, max_iterations) {
            Ok(result) => (true, result),
            Err(result) => (false, result),
        };
        let mother_codeword: [u8; PLC_MOTHER_BITS] = decoded
            .codeword
            .try_into()
            .map_err(|_| "Unexpected PLC mother decoder output size.".to_string())?;
        let success = converged && self.valid_syndrome(&mother_codeword);
        Ok(PlcBlockResult {
            information: std::array::from_fn(|index| mother_codeword[index]),
            mother_codeword,
            iterations: decoded.iterations,
            success,
        })
    }
    /// Checks every mother parity equation independently of decoder convergence.
    /// # Arguments
    /// * `mother` - Complete480-bit word, not the punctured384-bit transmission.
    /// # Returns
    /// True for binary values of exact length with every parity equation satisfied.
    pub fn valid_syndrome(&self, mother: &[u8]) -> bool {
        binary_bits(mother, PLC_MOTHER_BITS).is_ok()
            && (0..self.matrix.num_rows()).all(|row| {
                self.matrix
                    .iter_row(row)
                    .fold(0, |parity, index| parity ^ mother[*index])
                    == 0
            })
    }
}
