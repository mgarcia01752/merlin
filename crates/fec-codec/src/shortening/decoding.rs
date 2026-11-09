/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Receiver reconstruction: shortened zeros are known information, not puncture erasures.
use super::Shortening;
use crate::{
    binary_bits,
    parameters::{LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
/// Finite receiver arithmetic policy, not a DOCSIS CNR or normative LLR value.
pub const SHORTENED_ZERO_LLR_MAGNITUDE: f64 = 100.0;
impl Shortening {
    /// Restores omitted systematic zeros as positive finite known-bit evidence.
    /// # Arguments
    /// * `wire_llrs` - K+1968 finite ln(P0/P1) values, positive favoring zero.
    /// # Returns
    ///16200 mother-coordinate LLRs; omitted positions are+100 and retained values
    /// saturate to±100. No retained likelihood is converted to a hard decision.
    /// # Errors
    /// Rejects incorrect size, NaN or infinity before reconstruction.
    pub fn restore_ldpc_llrs(self, wire_llrs: &[f64]) -> Result<Vec<f64>, String> {
        if wire_llrs.len() != self.transmitted_bits()
            || wire_llrs.iter().any(|value| !value.is_finite())
        {
            log::debug!(
                "shortened-ldpc-input-rejected expected-llrs={} actual-llrs={}",
                self.transmitted_bits(),
                wire_llrs.len()
            );
            return Err("Expected finite transmitted-coordinate shortened LDPC LLRs.".into());
        }
        let bounded =
            |value: &f64| value.clamp(-SHORTENED_ZERO_LLR_MAGNITUDE, SHORTENED_ZERO_LLR_MAGNITUDE);
        let mut mother: Vec<_> = wire_llrs[..self.bch_bits()].iter().map(bounded).collect();
        mother.resize(LDPC_INPUT_BITS, SHORTENED_ZERO_LLR_MAGNITUDE);
        mother.extend(wire_llrs[self.bch_bits()..].iter().map(bounded));
        debug_assert_eq!(mother.len(), LDPC_CODEWORD_BITS);
        Ok(mother)
    }
    /// Reconstructs the BCH mother-coordinate input after actual LDPC decisions.
    /// # Arguments
    /// * `ldpc_decisions` -16200 binary mother-coordinate hard decisions.
    /// # Returns
    /// S leading zeros plus K+168 recovered BCH bits, exactly14400 bits.
    /// Omitted-position consistency is a separate outcome, not silently inferred here.
    /// # Errors
    /// Rejects malformed dimensions/nonbinary values without replacing recovered bits.
    pub fn restore_bch_input(self, ldpc_decisions: &[u8]) -> Result<Vec<u8>, String> {
        binary_bits(ldpc_decisions, LDPC_CODEWORD_BITS)?;
        let mut bch = vec![0; self.omitted_bits()];
        bch.extend_from_slice(&ldpc_decisions[..self.bch_bits()]);
        Ok(bch)
    }
    /// Checks decoder decisions at the omitted known-zero coordinates.
    /// # Arguments
    /// * `ldpc_decisions` -16200 binary LDPC mother decisions.
    /// * `bch_decisions` -14400 binary BCH mother decisions after correction.
    /// # Returns
    /// True only when both omitted regions remain zero; false includes malformed data.
    pub fn known_zeros_consistent(self, ldpc_decisions: &[u8], bch_decisions: &[u8]) -> bool {
        binary_bits(ldpc_decisions, LDPC_CODEWORD_BITS).is_ok()
            && binary_bits(bch_decisions, LDPC_INPUT_BITS).is_ok()
            && ldpc_decisions[self.bch_bits()..LDPC_INPUT_BITS]
                .iter()
                .all(|bit| *bit == 0)
            && bch_decisions[..self.omitted_bits()]
                .iter()
                .all(|bit| *bit == 0)
    }
}
