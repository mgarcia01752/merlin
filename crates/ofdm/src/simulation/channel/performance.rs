/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Per-carrier post-equalization MER against an independent diagnostic reference.
use serde::Serialize;
/// Constant-space power sums for one physical FFT bin during source observation.
/// The reference is diagnostic TX truth, never a decoder input or slicer decision.
#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub struct CarrierMer {
    samples: u64,
    signal_power: f64,
    error_power: f64,
}
impl CarrierMer {
    /// Records one matched reference/received cell after equalization.
    /// # Arguments
    /// * `reference`, `received` - Finite complex I/Q values on the same physical bin.
    /// # Returns
    /// Unit; invalid pairs are ignored, leaving their gap unmeasured.
    pub fn observe(&mut self, reference: [f64; 2], received: [f64; 2]) {
        if reference.iter().chain(&received).any(|v| !v.is_finite()) {
            return;
        }
        let signal = reference[0].powi(2) + reference[1].powi(2);
        let error = (reference[0] - received[0]).powi(2) + (reference[1] - received[1]).powi(2);
        if !signal.is_finite()
            || !error.is_finite()
            || !(self.signal_power + signal).is_finite()
            || !(self.error_power + error).is_finite()
            || self.samples == u64::MAX
        {
            return;
        }
        self.signal_power += signal;
        self.error_power += error;
        self.samples += 1;
    }
    /// Reads the number of valid same-bin reference/received comparisons.
    /// # Returns
    /// Zero denotes an unmeasured bin, never zero MER.
    pub fn samples(&self) -> u64 {
        self.samples
    }
    /// Calculates the power-ratio MER without averaging logarithmic dB values.
    /// # Returns
    /// Finite dB, or None for missing/zero-signal or exact zero-error observations.
    /// Zero-error bins remain distinguishable via samples and error_power getters.
    pub fn mer_db(&self) -> Option<f64> {
        if self.signal_power > 0.0 && self.error_power > 0.0 {
            let value = 10.0 * (self.signal_power.log10() - self.error_power.log10());
            value.is_finite().then_some(value)
        } else {
            None
        }
    }
    /// Reads accumulated reference power in the diagnostic normalization.
    /// # Returns
    /// Linear power sum over samples, not a calibrated RF measurement.
    pub fn signal_power(&self) -> f64 {
        self.signal_power
    }
    /// Reads accumulated squared complex-error power.
    /// # Returns
    /// Linear error-power sum over samples; zero with samples denotes exact agreement.
    pub fn error_power(&self) -> f64 {
        self.error_power
    }
}
