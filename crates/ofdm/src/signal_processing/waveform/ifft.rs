/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! TX: physical frequency bins to useful time samples, centered unitary PHY convention.
use super::transform::TransformPlan;
use crate::{transmission::SubcarrierSpacing, types::IqPair, LabError};
/// Reusable fixed-size IFFT plan, with one private safe Rust butterfly implementation.
pub struct InverseTransform {
    plan: TransformPlan,
}
impl InverseTransform {
    /// Precomputes roots and ordering for one configured spacing.
    /// # Arguments
    /// * `spacing` - 25/50 kHz at the fixed 204.8 MHz sample clock.
    /// # Returns
    /// Immutable reusable 8K/4K plan; no runtime or native FFT dependency.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        Self {
            plan: TransformPlan::new(spacing),
        }
    }
    /// Applies the centered unitary IFFT without changing the input.
    /// # Arguments
    /// * `input` - Exactly N finite complex bins, lowest frequency first.
    /// # Returns
    /// Owned time samples with 1/sqrt(N) scaling; discrete energy is preserved.
    /// # Errors
    /// Rejects wrong length, nonfinite input or numerical overflow in the transform.
    pub fn apply(&self, input: &[IqPair]) -> Result<Vec<IqPair>, LabError> {
        self.plan.apply(input, true)
    }
}
