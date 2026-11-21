/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Checked absolute positions and source ownership, outside receiver truth.
use crate::{
    error::{invalid, limit},
    PhyResult,
};
use serde::Serialize;

/// Immutable association of one run, accepted generation and grant.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct Identity {
    /// Nonzero simulation run identity.
    pub run: u64,
    /// Nonzero accepted configuration generation.
    pub generation: u64,
    /// Synthetic grant identity, unique within the run.
    pub grant: u64,
}

impl Identity {
    /// Validates run and configuration identity before accepting stage input.
    ///
    /// Returns unit; zero identities reject without changing caller state.
    pub fn validate(self) -> PhyResult<()> {
        if self.run == 0 || self.generation == 0 {
            return Err(invalid("identity"));
        }
        Ok(())
    }
}

/// Checked half-open range of absolute source bits, never a circular PRBS index.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub struct BitRange {
    start: u64,
    length: usize,
    end: u64,
}

impl BitRange {
    /// Checks `start + length` before accepting a range.
    ///
    /// Returns an immutable range; arithmetic overflow is a resource error.
    pub fn new(start: u64, length: usize) -> PhyResult<Self> {
        let end = start
            .checked_add(u64::try_from(length).map_err(|_| limit("bit_range"))?)
            .ok_or_else(|| limit("bit_range"))?;
        Ok(Self { start, length, end })
    }
    /// Returns the absolute first source-bit position without mutation.
    ///
    /// The value is not reduced modulo a source period.
    pub fn start(self) -> u64 {
        self.start
    }
    /// Returns the count of meaningful bits in this range.
    ///
    /// Zero denotes an empty range, not unavailable coverage.
    pub fn length(self) -> usize {
        self.length
    }
    /// Returns the checked absolute end, exclusive.
    ///
    /// This lookup consumes no source state.
    pub fn end(self) -> u64 {
        self.end
    }
}

/// Owned binary source chunk with its run and absolute range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SourceChunk {
    /// Validated run/configuration/grant association.
    pub identity: Identity,
    /// Original meaningful source range.
    pub range: BitRange,
    /// Binary bits in emission order, bounded by the source operation.
    pub bits: Vec<u8>,
}
