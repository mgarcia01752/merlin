/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Synthetic IUC associations and per-minislot loading; no UCD wire parser.
use crate::{error::invalid, PhyResult};
use serde::{Deserialize, Serialize};

/// One contiguous physical minislot assignment before interleaving compaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Minislot {
    /// First physical carrier, ascending frequency; width comes from numerology.
    pub first_carrier: usize,
    /// Ordinary data bits per cell (2..12); zero denotes a zero-valued minislot.
    pub loading: u8,
    /// Pinned pilot pattern 1..14, or exactly 0 for zero loading,, validated against numerology.
    pub pilot_pattern: u8,
}

/// Immutable profile; IUC selects this record rather than a fixed code/QAM size.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Profile {
    id: u16,
    iuc: u8,
    seed: u32,
    minislots: Vec<Minislot>,
}

impl Profile {
    /// Validates a bounded synthetic profile before channel-specific admission.
    ///
    /// `id` is session-local; `iuc` is a data IUC, `seed` is a raw 23-bit seed,
    /// and `minislots` retains physical ordering. Returns an immutable profile.
    /// Rejects unsupported IUC/loading/seed, empty or excessive inventories.
    pub fn new(id: u16, iuc: u8, seed: u32, minislots: Vec<Minislot>) -> PhyResult<Self> {
        if ![5, 6, 9, 10, 11, 12, 13].contains(&iuc)
            || seed > 0x7f_ffff
            || minislots.is_empty()
            || minislots.len() > 256
            || minislots
                .iter()
                .any(|slot| slot.loading != 0 && !(2..=12).contains(&slot.loading))
        {
            return Err(invalid("profile"));
        }
        Ok(Self {
            id,
            iuc,
            seed,
            minislots,
        })
    }
    /// Returns the retained profile identity without guessing from its IUC.
    ///
    /// IDs remain distinct when profiles happen to share loading values.
    pub fn id(&self) -> u16 {
        self.id
    }
    /// Returns this profile's explicit scheduled-data IUC association.
    ///
    /// No LDPC family or constellation is inferred from this identifier.
    pub fn iuc(&self) -> u8 {
        self.iuc
    }
    /// Returns the accepted raw numeric Figure 7 register seed.
    ///
    /// This is not a PRBS15 seed or a serialized UCD byte field.
    pub fn seed(&self) -> u32 {
        self.seed
    }
    /// Borrows all original physical minislot assignments in order.
    ///
    /// Caller cannot mutate the admitted profile through this view.
    pub fn minislots(&self) -> &[Minislot] {
        &self.minislots
    }
}
