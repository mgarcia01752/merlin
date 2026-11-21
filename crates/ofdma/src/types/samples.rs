/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Indexed contiguous complex samples at the upstream sampling rate.
use super::{Identity, Iq};
use crate::{
    error::{invalid, limit},
    PhyResult,
};
use serde::Serialize;

/// Owned contiguous sample block; partial coverage is represented by its length.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SampleBlock {
    /// Run/configuration/grant associated with this emission.
    pub identity: Identity,
    /// Absolute first sample tick at 102.4 MHz.
    pub start: u64,
    /// Finite complex samples, including explicitly emitted zeros.
    pub samples: Vec<Iq>,
}

impl SampleBlock {
    /// Validates a borrowed block before any downstream stage changes state.
    ///
    /// `maximum` bounds sample count; returns the checked exclusive end tick.
    /// Rejects stale zero identities, excessive size, overflow and nonfinite data.
    pub fn validate(&self, maximum: usize) -> PhyResult<u64> {
        self.identity.validate()?;
        if self.samples.len() > maximum {
            return Err(limit("sample_block"));
        }
        if self.samples.iter().any(|value| !value.is_finite()) {
            return Err(invalid("sample_block.samples"));
        }
        self.start
            .checked_add(self.samples.len() as u64)
            .ok_or_else(|| limit("sample_block.end"))
    }
}
