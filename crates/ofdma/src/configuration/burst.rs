/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit mode admission; unsupported modes never fall back to data.
use crate::{PhyError, PhyResult};
use serde::{Deserialize, Serialize};

/// Recognized upstream burst modes; only scheduled data currently executes.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BurstMode {
    /// Ordinary scheduled-data grant, with no generic preamble.
    ScheduledData,
    /// Dedicated initial ranging; requires its own future waveform.
    InitialRanging,
    /// Dedicated fine ranging; requires its own future waveform.
    FineRanging,
    /// Separate probing waveform and observations.
    Probing,
}

impl BurstMode {
    /// Admits the declared implemented execution mode.
    ///
    /// Returns unit for data; rejects other modes before source or RNG use.
    pub fn validate(self) -> PhyResult<()> {
        if self != Self::ScheduledData {
            return Err(PhyError::Unsupported {
                field: "burst_mode",
            });
        }
        Ok(())
    }
}
