/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin composition of separate pilot DSP modules.
use super::PilotStageResult;
use crate::{
    signal_processing::pilots::{
        generate_continuous, generate_scattered, PilotPower, PilotSequence,
    },
    transmission::{layout::CarrierLayout, SubcarrierSpacing},
    SimulationError,
};
/// Reuses the immutable physical-bin sequence; no RNG, codec or mutable pilot state.
pub struct PilotGenerator {
    sequence: PilotSequence,
}
impl PilotGenerator {
    /// Creates reusable pilot sequence setup for one spacing.
    /// # Arguments
    /// * `spacing` - Fixed physical FFT numerology.
    /// # Returns
    /// Owned generator; each stage remains separately callable through DSP APIs.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        Self {
            sequence: PilotSequence::new(spacing),
        }
    }
    /// Borrows the full physical-bin sequence for direct testing or observation.
    /// # Returns
    /// Immutable sequence; no caller mutation or hidden clock advancement is possible.
    pub fn sequence(&self) -> &PilotSequence {
        &self.sequence
    }
    /// Composes independent power, continuous and scattered generation stages.
    /// # Arguments
    /// * `layout` - Validated physical geometry with explicit continuous pilots.
    /// * `symbol_index` - PLC-aligned physical symbol index.
    /// * `reference_rms` - Explicit RMS amplitude of other subcarriers.
    /// # Returns
    /// Owned stage values before any observation or full-bin assembly.
    /// # Errors
    /// Rejects invalid power or a layout of different numerology.
    pub fn generate(
        &self,
        layout: &CarrierLayout,
        symbol_index: u64,
        reference_rms: f64,
    ) -> Result<PilotStageResult, SimulationError> {
        let power = PilotPower::new(reference_rms)?;
        Ok(PilotStageResult {
            symbol_index,
            spacing: self.sequence.spacing(),
            power,
            continuous: generate_continuous(layout, &self.sequence, power)?,
            scattered: generate_scattered(layout, &self.sequence, power, symbol_index)?,
        })
    }
}
