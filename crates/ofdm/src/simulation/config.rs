/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated scientific work and receiver settings.
use super::source::SimulationSeed;
use crate::error::{invalid, LabError};
use serde::{Deserialize, Serialize};
/// The verified coding tables define this bounded experiment; full PHY placement is outside its scope.
pub const CODING_MODE: &str = "ds_short_8_9_16qam_coding_experiment";
/// Finite work policy. Runtime exhaustion/cancellation preserve completed trials.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SimulationSettings {
    pub seed: SimulationSeed,
    pub codewords: usize,
    pub max_iterations: usize,
    pub max_runtime_ms: u64,
    pub awgn_mer_db: f64,
    pub noiseless: bool,
}
impl Default for SimulationSettings {
    /// Provides a small interactive AWGN coding experiment.
    /// # Returns
    /// Seed1, ten words, 30iterations and a10second runtime budget at25dB MER.
    fn default() -> Self {
        Self {
            seed: 1,
            codewords: 10,
            max_iterations: 30,
            max_runtime_ms: 10_000,
            awgn_mer_db: 25.0,
            noiseless: false,
        }
    }
}
impl SimulationSettings {
    /// Validates numeric inputs before allocating a codec or generating trials.
    /// # Returns
    /// Unit for a finite, bounded experiment.
    /// # Errors
    /// Rejects nonfinite RF inputs and excessive trial/runtime/iteration requests.
    pub fn validate(&self) -> Result<(), LabError> {
        if !(1..=512).contains(&self.codewords)
            || !(1..=100).contains(&self.max_iterations)
            || !(1..=300_000).contains(&self.max_runtime_ms)
            || !self.awgn_mer_db.is_finite()
            || !(-20.0..=80.0).contains(&self.awgn_mer_db)
        {
            return Err(invalid(
                "Invalid FEC trial count, iteration/runtime limit or MER.",
            ));
        }
        Ok(())
    }
}
