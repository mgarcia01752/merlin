/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 * Adapted from MERLIN crates/ofdm/src/simulation/source/prbs.rs at
 * 096d5a476194d4f2e016863d723b225257bb6037.
 * Modifications: upstream typed chunks, transactional identity/range admission,
 * local errors and no downstream BCH payload helper.
 */
//! PRBS15 payload: s[n+15]=s[n+14] XOR s[n], LSB-first seed and output.
use crate::{
    configuration::MAX_CODED_BITS,
    error::{invalid, limit},
    types::{BitRange, Identity, SourceChunk},
    PhyResult,
};

/// Owned continuous nonzero PRBS15 source, independent of PHY randomization.
#[derive(Debug, Clone)]
pub struct Prbs15 {
    state: u16,
    position: u64,
}

impl Prbs15 {
    /// Loads `seed` bits 0..14 at absolute source position zero.
    ///
    /// Returns independent idle state; rejects zero and values above 32767.
    pub fn new(seed: u16) -> PhyResult<Self> {
        if seed == 0 || seed > 32767 {
            return Err(invalid("source.seed"));
        }
        Ok(Self {
            state: seed,
            position: 0,
        })
    }
    /// Returns the next absolute source bit without clocking the generator.
    ///
    /// The position continues across codewords, frames, grants and periods.
    pub fn position(&self) -> u64 {
        self.position
    }
    /// Generates an owned binary chunk after complete identity/budget admission.
    ///
    /// `identity` associates the request; `count` is meaningful source bits.
    /// Returns exact bits/range. Invalid requests preserve register and position.
    pub fn generate(&mut self, identity: Identity, count: usize) -> PhyResult<SourceChunk> {
        identity.validate()?;
        if count > MAX_CODED_BITS {
            return Err(limit("source.chunk"));
        }
        let range = BitRange::new(self.position, count)?;
        let bits = (0..count)
            .map(|_| {
                let (state, bit) = merlin_dsp::prbs15::step(self.state);
                self.state = state;
                bit
            })
            .collect();
        self.position = range.end();
        Ok(SourceChunk {
            identity,
            range,
            bits,
        })
    }
}
