/*
 * Copyright 2026 MERLIN contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Physical holes survive compaction; capacity and inverse RX use one visit plan.
use super::{addresses, block_sizes, role};
use crate::{
    configuration::{UsConfiguration, MAX_CODED_BITS},
    error::limit,
    PhyResult,
};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::Serialize;

/// Scientific role, distinct from whether an observed value happens to be zero.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum CellRole {
    /// Ordinary loaded QAM payload.
    Data,
    /// Payload with m=max(data_m-4,1), including BPSK.
    Complementary,
    /// Known BPSK reference; consumes no randomized coded bits.
    KnownPilot,
}

/// One configured address visit with physical and logical association.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Cell {
    /// Symbol within the frame, zero-based.
    pub symbol: usize,
    /// Original physical FFT carrier in ascending frequency.
    pub carrier: usize,
    /// Configured role, independent of observed samples.
    pub role: CellRole,
    /// Payload loading; zero only for known pilots.
    pub loading: u8,
    /// Prescribed pilot/complementary amplitude gain, applied/removed once.
    pub gain: f64,
    /// Coded-bit position within a frame before the next cell's bits.
    pub bit_offset: usize,
}

/// Immutable one-frame resource plan reused across identically assigned frames.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ResourcePlan {
    /// All visits, including known pilots, in joint interleaver order.
    pub visits: Vec<Cell>,
    /// Coded capacity contributed by data and complementary cells per frame.
    pub frame_bits: usize,
    /// Total coded capacity of this finite grant.
    pub grant_bits: usize,
}

impl ResourcePlan {
    /// Builds physical masks and balanced interleaving blocks before source use.
    ///
    /// `configuration` is immutable and source-free. Returns one bounded frame
    /// visit plan and checked grant capacity, retaining physical holes/edges.
    /// Rejects resource arithmetic or coded-bit budget exhaustion.
    pub fn new(configuration: &UsConfiguration) -> PhyResult<Self> {
        let profile = configuration.selected_profile();
        let geometry = configuration.waveform();
        let q = geometry.numerology().carriers_per_minislot();
        let mut participating = Vec::new();
        let mut previous_end = None;
        for &index in &configuration.grant().minislots {
            let slot = &profile.minislots()[index];
            if slot.loading == 0 {
                previous_end = None;
                continue;
            }
            let edge = previous_end != Some(slot.first_carrier);
            participating.push((index, edge));
            previous_end = Some(slot.first_carrier + q);
        }
        let mut visits = Vec::new();
        let mut offset = 0;
        let mut first = 0;
        for count in block_sizes(participating.len())? {
            for (symbol, row) in addresses(geometry.frame_symbols(), count * q)? {
                let (index, edge) = participating[first + row / q];
                let slot = &profile.minislots()[index];
                let (role, gain) = role(
                    slot.pilot_pattern,
                    geometry.frame_symbols(),
                    symbol,
                    row % q,
                    edge,
                )?;
                let loading = match role {
                    CellRole::KnownPilot => 0,
                    CellRole::Data => slot.loading,
                    CellRole::Complementary => slot.loading.saturating_sub(4).max(1),
                };
                visits.push(Cell {
                    symbol,
                    carrier: slot.first_carrier + row % q,
                    role,
                    loading,
                    gain,
                    bit_offset: offset,
                });
                offset += loading as usize;
            }
            first += count;
        }
        let grant_bits = offset
            .checked_mul(configuration.grant().frames)
            .ok_or_else(|| limit("resources.bits"))?;
        if grant_bits > MAX_CODED_BITS {
            return Err(limit("resources.bits"));
        }
        Ok(Self {
            visits,
            frame_bits: offset,
            grant_bits,
        })
    }
}

impl RetainedPayload for CellRole {
    /// Audits every inline variant without counting borrowed or fixed data twice.
    ///
    /// # Returns
    /// Zero dynamic children; the enclosing root/container owns the fixed bytes.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Data | Self::Complementary | Self::KnownPilot => {
                Some(RetainedFootprint::default())
            }
        }
    }
}
