/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent mother encoding and composed transmitted PLC coding operation.
use super::{
    parameters::{PLC_CODEWORD_BITS, PLC_INFORMATION_BITS, PLC_MOTHER_BITS},
    puncturing::puncture,
    PlcFecCodec,
};
use crate::binary_bits;
use ldpc_toolbox::gf2::GF2;
use ndarray::Array1;
use num_traits::{One, Zero};
impl PlcFecCodec {
    /// Encodes information into the complete systematic mother word before puncturing.
    /// # Arguments
    /// * `information` -288 binary bits in MSB-first message-byte order.
    /// # Returns
    ///480 bits: a0..287 followed by b288..479. No shortening or data randomization.
    /// # Errors
    /// Rejects wrong length or nonbinary input before invoking the encoder.
    pub fn encode_mother(&self, information: &[u8]) -> Result<[u8; PLC_MOTHER_BITS], String> {
        binary_bits(information, PLC_INFORMATION_BITS)?;
        let data = Array1::from_iter(information.iter().map(|bit| {
            if *bit == 0 {
                GF2::zero()
            } else {
                GF2::one()
            }
        }));
        let mother = self.encoder.encode(&data);
        Ok(std::array::from_fn(|index| {
            u8::from(!mother[index].is_zero())
        }))
    }
    /// Composes mother encoding and the separate puncturing stage.
    /// # Arguments
    /// * `information` -288 binary information bits; no implicit byte reversal.
    /// # Returns
    ///384 transmitted bits in ascending retained coordinate order.
    /// # Errors
    /// Rejects invalid information or propagates puncture validation failures.
    pub fn encode(&self, information: &[u8]) -> Result<[u8; PLC_CODEWORD_BITS], String> {
        puncture(&self.encode_mother(information)?)
    }
}
