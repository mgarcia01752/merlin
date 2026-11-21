/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Downstream geometry/error adapter to the shared centered unitary numerical plan.
use crate::{error::invalid, transmission::SubcarrierSpacing, types::IqPair, SimulationError};
use merlin_dsp::transform::{TransformPlan as NumericalPlan, TransformSize};

pub(super) struct TransformPlan {
    numerical: NumericalPlan,
}
impl TransformPlan {
    /// Selects the existing fixed-clock downstream geometry without changing PHY policy.
    /// # Arguments
    /// * `spacing` - Determines the same bounded 4K/8K geometry as before extraction.
    /// # Returns
    /// Immutable reusable numerical plan with independently owned call scratch.
    pub(super) fn new(spacing: SubcarrierSpacing) -> Self {
        let size = match spacing {
            SubcarrierSpacing::Khz25 => TransformSize::Samples8192,
            SubcarrierSpacing::Khz50 => TransformSize::Samples4096,
        };
        Self {
            numerical: NumericalPlan::new(size),
        }
    }
    /// Executes the existing unitary centered transform in the selected direction.
    /// # Arguments
    /// * `input` - Complete finite physical bins or useful time samples.
    /// * `inverse` - True for TX synthesis; false for RX analysis.
    /// # Returns
    /// Owned output without modifying the source, plan, ordering or normalization.
    /// # Errors
    /// Retains the existing downstream invalid-sample error for wrong length,
    /// nonfinite input and nonfinite numerical output.
    pub(super) fn apply(
        &self,
        input: &[IqPair],
        inverse: bool,
    ) -> Result<Vec<IqPair>, SimulationError> {
        let result = if inverse {
            self.numerical.inverse(input)
        } else {
            self.numerical.forward(input)
        };
        result.map_err(|_| invalid("Waveform input has invalid length or nonfinite I/Q."))
    }
}
