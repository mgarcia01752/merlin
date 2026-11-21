/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent matched ideal-data MER, never a normal receiver input or target.
use crate::{
    error::{invalid, limit},
    types::Iq,
    PhyResult,
};
use serde::Serialize;

/// Honest finite/zero-error/unavailable metric state; JSON never needs infinity.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum MerValue {
    /// No complete matched nonzero-reference observations.
    Unavailable,
    /// Matched error energy is exactly zero, so no finite dB value exists.
    ZeroError,
    /// Ten times log10(sum ideal energy / sum error energy).
    FiniteDb(f64),
}
/// Bounded aggregate of actually matched deboosted data/complementary symbols.
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
pub struct MatchedMer {
    /// Number of actual complete ideal/received pairs, excluding missing cells.
    pub pairs: usize,
    /// Sum of actual ideal reference energies, in normalized QAM units.
    pub reference_energy: f64,
    /// Sum of received-minus-ideal squared errors in the same reference plane.
    pub error_energy: f64,
}
impl MatchedMer {
    /// Adds one finite independently matched `ideal` and `received` observation.
    ///
    /// Returns unit; nonfinite/overflow rejects before changing any accumulated sum.
    pub fn observe(&mut self, ideal: Iq, received: Iq) -> PhyResult<()> {
        let reference = self.reference_energy + ideal.power();
        let error = self.error_energy + (received - ideal).power();
        let pairs = self
            .pairs
            .checked_add(1)
            .ok_or_else(|| limit("matched_mer.pairs"))?;
        if !ideal.is_finite()
            || !received.is_finite()
            || !reference.is_finite()
            || !error.is_finite()
        {
            return Err(invalid("matched_mer.energy"));
        }
        self.pairs = pairs;
        self.reference_energy = reference;
        self.error_energy = error;
        Ok(())
    }
    /// Returns the explicit metric state without injecting an arbitrary noise floor.
    ///
    /// Missing/zero reference is unavailable; exactly zero error stays unbounded.
    pub fn value(&self) -> MerValue {
        if self.pairs == 0 || self.reference_energy == 0.0 {
            MerValue::Unavailable
        } else if self.error_energy == 0.0 {
            MerValue::ZeroError
        } else {
            MerValue::FiniteDb(10.0 * (self.reference_energy.log10() - self.error_energy.log10()))
        }
    }
}
