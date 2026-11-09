/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! MULPI I25 §7.1.5:55 coarse bits, modulo20 sample counter and4 phase bits.
use crate::{error::invalid, SimulationError};
use serde::Serialize;
const SAMPLE_TICKS_PER_COARSE_TICK: u64 = 20;
const FRACTIONAL_PHASE_BITS: u32 = 4;
const SAMPLE_PHASE_BITS: u32 = 5;
const COARSE_SHIFT: u32 = FRACTIONAL_PHASE_BITS + SAMPLE_PHASE_BITS;
const MAX_COARSE_TICKS: u64 = (1 << (u64::BITS - COARSE_SHIFT)) - 1;
/// Validated extended timestamp in its DOCSIS reference, not Unix seconds or ns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ExtendedTimestamp {
    ///10.24MHz ticks since the TAI epoch;55 bits.
    coarse_ticks: u64,
    ///204.8MHz ticks within a coarse tick:0..19, not a binary modulo32 count.
    sample_phase: u8,
    /// Fractional204.8MHz phase:0..15; zero is permitted for standalone use.
    fractional_phase: u8,
}
impl ExtendedTimestamp {
    /// Builds a validated timestamp from the three wire fields.
    /// # Arguments
    /// * `coarse_ticks` -55-bit10.24MHz counter in the caller's TAI reference.
    /// * `sample_phase` - Sample offset within that tick,0..19.
    /// * `fractional_phase` - Fractional phase,0..15.
    /// # Returns
    /// Owned timestamp; no wall-clock conversion or host clock lookup.
    /// # Errors
    /// Rejects unrepresentable coarse count or reserved sample/phase values.
    pub fn new(
        coarse_ticks: u64,
        sample_phase: u8,
        fractional_phase: u8,
    ) -> Result<Self, SimulationError> {
        if coarse_ticks > MAX_COARSE_TICKS
            || u64::from(sample_phase) >= SAMPLE_TICKS_PER_COARSE_TICK
            || fractional_phase >= (1 << FRACTIONAL_PHASE_BITS)
        {
            return Err(invalid("Invalid DOCSIS extended timestamp fields."));
        }
        Ok(Self {
            coarse_ticks,
            sample_phase,
            fractional_phase,
        })
    }
    /// Builds a timestamp from integer204.8MHz ticks and a fractional phase.
    /// # Arguments
    /// * `sample_ticks` - Integer ticks since the caller's DOCSIS TAI epoch.
    /// * `fractional_phase` - Lower four phase bits; no implicit rounding.
    /// # Returns
    /// Validated timestamp after modulo20 division.
    /// # Errors
    /// Rejects values outside the64-bit wire representation or phase0..15.
    pub fn from_sample_ticks(
        sample_ticks: u64,
        fractional_phase: u8,
    ) -> Result<Self, SimulationError> {
        Self::new(
            sample_ticks / SAMPLE_TICKS_PER_COARSE_TICK,
            (sample_ticks % SAMPLE_TICKS_PER_COARSE_TICK) as u8,
            fractional_phase,
        )
    }
    /// Returns integer204.8MHz ticks, excluding the fractional phase field.
    /// # Returns
    /// Exact tick count; construction guarantees arithmetic fits u64.
    pub fn sample_ticks(self) -> u64 {
        self.coarse_ticks * SAMPLE_TICKS_PER_COARSE_TICK + u64::from(self.sample_phase)
    }
    /// Advances a copied timestamp without changing the original or its phase.
    /// # Arguments
    /// * `samples` - Integer204.8MHz ticks to add, not nanoseconds.
    /// # Returns
    /// New timestamp with proper modulo20 carry.
    /// # Errors
    /// Rejects arithmetic or wire-range overflow; never silently wraps the epoch.
    pub fn advance_samples(self, samples: u64) -> Result<Self, SimulationError> {
        let ticks = self
            .sample_ticks()
            .checked_add(samples)
            .ok_or_else(|| invalid("DOCSIS timestamp sample addition overflow."))?;
        Self::from_sample_ticks(ticks, self.fractional_phase)
    }
    /// Serializes all fields in network byte order.
    /// # Returns
    /// Eight timestamp octets; no message-block header or CRC.
    pub fn to_bytes(self) -> [u8; 8] {
        ((self.coarse_ticks << COARSE_SHIFT)
            | (u64::from(self.sample_phase) << FRACTIONAL_PHASE_BITS)
            | u64::from(self.fractional_phase))
        .to_be_bytes()
    }
    /// Parses timestamp octets without assuming Unix or nanosecond semantics.
    /// # Arguments
    /// * `bytes` - Exactly eight network-order timestamp octets.
    /// # Returns
    /// Owned validated fields; input stays unchanged.
    /// # Errors
    /// Rejects sample-phase encodings20..31.
    pub fn from_bytes(bytes: &[u8; 8]) -> Result<Self, SimulationError> {
        let encoded = u64::from_be_bytes(*bytes);
        Self::new(
            encoded >> COARSE_SHIFT,
            ((encoded >> FRACTIONAL_PHASE_BITS) & 0x1f) as u8,
            (encoded & 0x0f) as u8,
        )
    }
}
