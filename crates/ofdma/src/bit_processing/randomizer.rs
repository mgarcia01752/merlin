/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Figure 7 D1=bit0, D18=bit17, D23=bit22; XOR then advance toward D23.
use crate::{
    configuration::MAX_CODED_BITS,
    error::{invalid, limit},
    PhyResult,
};

/// Independently owned per-grant mask state, shared algorithm for TX and soft RX.
#[derive(Debug, Clone)]
pub struct Randomizer23 {
    state: u32,
    position: usize,
}

impl Randomizer23 {
    /// Loads raw numeric `seed` without warmup or PRBS15 restrictions.
    ///
    /// Returns a new grant state; rejects bits outside the 23-bit register.
    pub fn new(seed: u32) -> PhyResult<Self> {
        if seed > 0x7f_ffff {
            return Err(invalid("randomizer.seed"));
        }
        Ok(Self {
            state: seed,
            position: 0,
        })
    }
    /// XORs finite binary `bits` in transmission order with candidate mask state.
    ///
    /// Returns owned bits; rejects invalid/excessive input before advancing state.
    pub fn apply_bits(&mut self, bits: &[u8]) -> PhyResult<Vec<u8>> {
        self.admit(bits.len())?;
        if bits.iter().any(|bit| *bit > 1) {
            return Err(invalid("randomizer.bits"));
        }
        Ok(bits.iter().map(|bit| bit ^ self.next()).collect())
    }
    /// Reverses mask-one LLR signs before FEC, preserving finite confidence.
    ///
    /// `llrs` are ordered transmitted positions, including pad. Returns owned
    /// values; rejects nonfinite/excessive input without changing mask state.
    pub fn apply_llrs(&mut self, llrs: &[f64]) -> PhyResult<Vec<f64>> {
        self.admit(llrs.len())?;
        if llrs.iter().any(|value| !value.is_finite()) {
            return Err(invalid("randomizer.llrs"));
        }
        Ok(llrs
            .iter()
            .map(|value| if self.next() == 1 { -*value } else { *value })
            .collect())
    }
    fn admit(&self, count: usize) -> PhyResult<()> {
        if self
            .position
            .checked_add(count)
            .is_none_or(|end| end > MAX_CODED_BITS)
        {
            return Err(limit("randomizer.positions"));
        }
        Ok(())
    }
    fn next(&mut self) -> u8 {
        let bit = (((self.state >> 17) ^ (self.state >> 22)) & 1) as u8;
        self.state = ((self.state << 1) & 0x7f_ffff) | u32::from(bit);
        self.position += 1;
        bit
    }
}
