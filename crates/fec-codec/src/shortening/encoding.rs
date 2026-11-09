/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separately inspectable six-step transmit shortening boundaries.
use super::Shortening;
use crate::{
    binary_bits,
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
impl Shortening {
    /// Prepends known zeros before the existing BCH encoder.
    /// # Arguments
    /// * `payload` - Exactly K original binary bits, in declared source order.
    /// # Returns
    ///14232 information bits with S leading zeros; caller input remains unchanged.
    /// # Errors
    /// Rejects incorrect size or nonbinary input before indexing/allocation.
    pub fn prepend_bch_zeros(self, payload: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(payload, self.payload_bits())?;
        let mut mother = vec![0; self.omitted_bits()];
        mother.extend_from_slice(payload);
        debug_assert_eq!(mother.len(), BCH_INPUT_BITS);
        Ok(mother)
    }
    /// Removes the BCH mother word's prepended zero prefix after encoding.
    /// # Arguments
    /// * `mother` -14400 binary BCH output bits with S leading zeros.
    /// # Returns
    /// K original bits followed by all168 BCH parity bits.
    /// # Errors
    /// Rejects bad dimensions/values or a nonzero omitted prefix; does not verify parity.
    pub fn remove_bch_zeros(self, mother: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(mother, LDPC_INPUT_BITS)?;
        if mother[..self.omitted_bits()].iter().any(|bit| *bit != 0) {
            return Err("BCH shortening prefix is not zero.".into());
        }
        Ok(mother[self.omitted_bits()..].to_vec())
    }
    /// Appends known zeros after the shortened BCH word before LDPC encoding.
    /// # Arguments
    /// * `bch` - K+168 binary information/parity bits, without the BCH zero prefix.
    /// # Returns
    ///14400 systematic LDPC input bits, with S trailing zeros.
    /// # Errors
    /// Rejects bad dimensions or nonbinary values; does not alter caller bits.
    pub fn append_ldpc_zeros(self, bch: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(bch, self.bch_bits())?;
        let mut mother = bch.to_vec();
        mother.resize(LDPC_INPUT_BITS, 0);
        Ok(mother)
    }
    /// Omits only the appended LDPC systematic zero positions after encoding.
    /// # Arguments
    /// * `mother` -16200 binary systematic LDPC output bits.
    /// # Returns
    /// K+168 BCH bits followed by all1800 LDPC parity bits.
    /// # Errors
    /// Rejects malformed words/nonzero omitted positions; does not verify syndrome.
    pub fn remove_ldpc_zeros(self, mother: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(mother, LDPC_CODEWORD_BITS)?;
        if mother[self.bch_bits()..LDPC_INPUT_BITS]
            .iter()
            .any(|bit| *bit != 0)
        {
            return Err("LDPC shortening positions are not zero.".into());
        }
        let mut wire = mother[..self.bch_bits()].to_vec();
        wire.extend_from_slice(&mother[LDPC_INPUT_BITS..]);
        Ok(wire)
    }
}
