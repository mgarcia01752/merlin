/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Supplied known-reference response compensation; no estimated channel is implied.
use crate::{
    error::invalid, signal_processing::waveform::samples::validate_samples, types::IqPair,
    SimulationError,
};
/// Frozen reference response normalized with the caller's per-subcarrier RMS.
pub struct KnownReferenceEqualizer {
    inverse: Vec<IqPair>,
}
impl KnownReferenceEqualizer {
    /// Builds a per-bin inverse from explicit channel knowledge.
    /// # Arguments
    /// * `response` - N finite nonzero complex gains, including protected bins.
    /// * `reference_rms` - Positive finite TX bin amplitude reference, not absolute RF power.
    /// # Returns
    /// Receiver-owned inverse coefficients; unity response is the noiseless reference.
    /// # Errors
    /// Rejects unsupported FFT size, zero/nonfinite gains or nonrepresentable inverse.
    pub fn new(response: &[IqPair], reference_rms: f64) -> Result<Self, SimulationError> {
        if ![4096, 8192].contains(&response.len())
            || !reference_rms.is_finite()
            || reference_rms <= 0.0
        {
            return Err(invalid("Invalid known channel response or RMS reference."));
        }
        validate_samples(response, response.len())?;
        let inverse: Vec<_> = response
            .iter()
            .map(|gain| {
                let scale = gain[0].abs().max(gain[1].abs());
                let real = gain[0] / scale;
                let imaginary = gain[1] / scale;
                let denominator = real * real + imaginary * imaginary;
                [
                    real / denominator / scale / reference_rms,
                    -imaginary / denominator / scale / reference_rms,
                ]
            })
            .collect();
        validate_samples(&inverse, response.len())?;
        Ok(Self { inverse })
    }
    /// Reports inverse magnitude squared for calibrated raw-bin noise variance.
    /// # Returns
    /// One finite nonnegative power multiplier per physical bin.
    pub fn inverse_gain_power(&self) -> Vec<f64> {
        self.inverse
            .iter()
            .map(|gain| gain[0] * gain[0] + gain[1] * gain[1])
            .collect()
    }
    /// Removes the supplied channel response and bin amplitude scale.
    /// # Arguments
    /// * `bins` - N received physical bins after FFT, lowest frequency first.
    /// # Returns
    /// Normalized bins; pilot amplitude boost remains present for extraction checks.
    /// # Errors
    /// Rejects shape/numeric errors or overflowing products.
    pub fn apply(&self, bins: &[IqPair]) -> Result<Vec<IqPair>, SimulationError> {
        validate_samples(bins, self.inverse.len())?;
        let output: Vec<_> = bins
            .iter()
            .zip(&self.inverse)
            .map(|(sample, gain)| {
                [
                    sample[0] * gain[0] - sample[1] * gain[1],
                    sample[0] * gain[1] + sample[1] * gain[0],
                ]
            })
            .collect();
        validate_samples(&output, self.inverse.len())?;
        Ok(output)
    }
}
