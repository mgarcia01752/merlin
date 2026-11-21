/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit synthetic observation silence, not a normative OFDMA guard-time TLV.
use crate::{
    error::{invalid, limit},
    PhyResult,
};
use serde::{Deserialize, Serialize};

/// Declared exact zero-valued samples outside the complete isolated burst support.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GuardIntervals {
    /// Leading silence ends at configured prefix origin B; application maximum 4096.
    pub leading: usize,
    /// Trailing silence starts after the final rolloff tail; maximum 4096.
    pub trailing: usize,
}
impl GuardIntervals {
    /// Validates finite guard budgets and checked leading-origin subtraction.
    ///
    /// Returns total extra support; never changes B, useful windows or RNG state.
    pub fn validate(&self, prefix_origin: u64) -> PhyResult<usize> {
        if self.leading > 4096 || self.trailing > 4096 {
            return Err(limit("guards.samples"));
        }
        if prefix_origin < self.leading as u64 {
            return Err(invalid("guards.origin"));
        }
        Ok(self.leading + self.trailing)
    }
}
