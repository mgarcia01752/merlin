/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.4.2.3, Figures46–47: one mother code, variable original length.
use crate::parameters::{BCH_INPUT_BITS, BCH_PARITY_BITS, PAYLOAD_PARITY_BITS};
/// Checked geometry; never substitutes another rate or a normal64800-bit mother code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Shortening {
    payload_bits: usize,
}
impl Shortening {
    /// Defines a nonempty original payload within the supported mother-code capacity.
    /// # Arguments
    /// * `payload_bits` - Original information length K in1..=14232, before all parity.
    /// # Returns
    /// Immutable geometry; K=14232 selects the full-word compatibility boundary.
    /// # Errors
    /// Rejects zero or oversized K; nonempty is application policy, not a PHY minimum.
    pub fn new(payload_bits: usize) -> Result<Self, String> {
        if !(1..=BCH_INPUT_BITS).contains(&payload_bits) {
            return Err("Shortened payload length must be in1..=14232 bits.".into());
        }
        Ok(Self { payload_bits })
    }
    /// Reports original bits, excluding all parity and omitted zero positions.
    /// # Returns
    /// Validated K.
    pub fn payload_bits(self) -> usize {
        self.payload_bits
    }
    /// Reports zeros prepended at BCH input and appended at LDPC input.
    /// # Returns
    /// S=14232-K, including zero for the existing full word.
    pub fn omitted_bits(self) -> usize {
        BCH_INPUT_BITS - self.payload_bits
    }
    /// Reports transmitted BCH information-plus-parity length.
    /// # Returns
    /// K+168; this is also the start of omitted LDPC systematic zeros.
    pub fn bch_bits(self) -> usize {
        self.payload_bits + BCH_PARITY_BITS
    }
    /// Reports the complete transmitted word before bit interleaving/QAM.
    /// # Returns
    /// N=K+1968; no parity is shortened or punctured.
    pub fn transmitted_bits(self) -> usize {
        self.payload_bits + PAYLOAD_PARITY_BITS
    }
    /// Reports original data divided by transmitted bits.
    /// # Returns
    /// K/(K+1968), including outer parity overhead.
    pub fn payload_rate(self) -> f64 {
        self.payload_bits as f64 / self.transmitted_bits() as f64
    }
    /// Reports the effective inner LDPC rate after omitting systematic zeros.
    /// # Returns
    /// (K+168)/(K+1968), distinct from original-data rate.
    pub fn ldpc_rate(self) -> f64 {
        self.bch_bits() as f64 / self.transmitted_bits() as f64
    }
}
