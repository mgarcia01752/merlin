/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded contiguous sample blocks with an absolute sample-clock origin.
use crate::{error::invalid, types::IqPair, SimulationError};
/// Application bound per public sample block; long streams use repeated blocks.
pub const MAX_BLOCK_SAMPLES: usize = 65_536;
/// Complex envelope samples at the fixed 204.8 MHz sample clock, without RF units.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct SampleBlock {
    start_index: u64,
    samples: Vec<IqPair>,
}
impl SampleBlock {
    /// Validates and owns one contiguous finite sample window.
    /// # Arguments
    /// * `start_index` - Absolute first sample tick; an empty block has no samples.
    /// * `samples` - Complex I/Q at 204.8 Msamples/s, at most MAX_BLOCK_SAMPLES.
    /// # Returns
    /// Owned data with its original sample identity, usable across block boundaries.
    /// # Errors
    /// Rejects nonfinite data, excess length or an overflowing exclusive sample end.
    pub fn new(start_index: u64, samples: Vec<IqPair>) -> Result<Self, SimulationError> {
        validate_samples(&samples, samples.len())?;
        if samples.len() > MAX_BLOCK_SAMPLES
            || start_index.checked_add(samples.len() as u64).is_none()
        {
            return Err(invalid(
                "Sample block exceeds length or sample-index bounds.",
            ));
        }
        Ok(Self {
            start_index,
            samples,
        })
    }
    /// Reports the block's first sample tick.
    /// # Returns
    /// Absolute stream identity, not a wall-clock timestamp.
    pub fn start_index(&self) -> u64 {
        self.start_index
    }
    /// Borrows sample pairs without copying the block.
    /// # Returns
    /// Finite complex samples in increasing time order.
    pub fn samples(&self) -> &[IqPair] {
        &self.samples
    }
    /// Reports the exclusive end tick.
    /// # Returns
    /// Checked during construction; empty blocks end at their start.
    pub fn end_index(&self) -> u64 {
        self.start_index + self.samples.len() as u64
    }
}
/// Checks one exact-size DSP input before allocating or mutating state.
/// # Arguments
/// * `samples` - Borrowed complex values.
/// * `expected` - Required sample/bin count from validated geometry.
/// # Returns
/// Unit when length and all components are finite.
/// # Errors
/// Rejects unexpected size or nonfinite I/Q.
pub(crate) fn validate_samples(samples: &[IqPair], expected: usize) -> Result<(), SimulationError> {
    if samples.len() != expected || samples.iter().flatten().any(|value| !value.is_finite()) {
        return Err(invalid(
            "Waveform input has invalid length or nonfinite I/Q.",
        ));
    }
    Ok(())
}
