/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Immutable synthetic configuration admission shared by both PHY endpoints.
mod burst;
mod grant;
mod guards;
pub use guards::GuardIntervals;
mod profile;
mod receiver;
#[doc(hidden)]
pub mod serde_limits;
pub mod ucd;
mod waveform;

pub use burst::BurstMode;
pub use grant::SyntheticGrant;
pub use profile::{Minislot, Profile};
pub use receiver::{Estimator, ReceiverConfig};
pub use waveform::{Numerology, Waveform, SAMPLE_RATE_HZ};

use crate::{
    error::{invalid, limit},
    PhyResult,
};
use serde::Serialize;
use std::collections::BTreeSet;

/// Maximum retained profiles; application policy, not a normative catalog size.
pub const MAX_PROFILES: usize = 256;
/// Maximum expanded physical cells admitted per finite grant.
pub const MAX_RESOURCE_CELLS: usize = 1_048_576;
/// Maximum transmitted coded-bit positions admitted per finite grant.
pub const MAX_CODED_BITS: usize = 1_048_576;
/// Maximum codewords per grant, bounded before codec/output allocation.
pub const MAX_CODEWORDS: usize = 4096;
/// Maximum cumulative emitted sample count admitted per job.
pub const MAX_JOB_SAMPLES: usize = 16_777_216;

/// Immutable validated geometry/inventory/grant, without source/channel truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UsConfiguration {
    waveform: Waveform,
    profiles: Vec<Profile>,
    grant: SyntheticGrant,
    excluded: Vec<usize>,
}

impl UsConfiguration {
    /// Admits a complete candidate before publishing any accepted state.
    ///
    /// Arguments are owned geometry, profile inventory, scheduled `grant` and
    /// excluded physical bins. Returns an immutable snapshot. Rejects duplicates,
    /// incompatible grids/pilot families, unknown selections, overflow and budgets.
    pub fn new(
        waveform: Waveform,
        profiles: Vec<Profile>,
        grant: SyntheticGrant,
        excluded: Vec<usize>,
    ) -> PhyResult<Self> {
        if profiles.is_empty() || profiles.len() > MAX_PROFILES {
            return Err(limit("profiles"));
        }
        let n = waveform.numerology().fft_len();
        let q = waveform.numerology().carriers_per_minislot();
        let mut profile_ids = BTreeSet::new();
        let mut iucs = BTreeSet::new();
        let exclusions: BTreeSet<_> = excluded.iter().copied().collect();
        if exclusions.len() != excluded.len() || exclusions.iter().any(|bin| *bin >= n) {
            return Err(invalid("excluded"));
        }
        let mut family = None;
        for profile in &profiles {
            if !profile_ids.insert(profile.id()) || !iucs.insert(profile.iuc()) {
                return Err(invalid("profile.identity"));
            }
            let mut previous_end = 0;
            for slot in profile.minislots() {
                let end = slot
                    .first_carrier
                    .checked_add(q)
                    .ok_or_else(|| limit("minislot.end"))?;
                if end > n
                    || slot.first_carrier < previous_end
                    || (slot.first_carrier..end).any(|bin| exclusions.contains(&bin))
                {
                    return Err(invalid("minislot.grid"));
                }
                if slot.loading == 0 {
                    if slot.pilot_pattern != 0 {
                        return Err(invalid("minislot.zero_pilot_pattern"));
                    }
                    previous_end = end;
                    continue;
                }
                let allowed = if q == 8 { 1..=7 } else { 8..=14 };
                if !allowed.contains(&slot.pilot_pattern) {
                    return Err(invalid("minislot.pilot_pattern"));
                }
                let boosted = if q == 8 {
                    slot.pilot_pattern >= 5
                } else {
                    slot.pilot_pattern >= 12
                };
                if family.is_some_and(|old| old != boosted) {
                    return Err(invalid("profile.pilot_family"));
                }
                family = Some(boosted);
                previous_end = end;
            }
            let carriers = profile
                .minislots()
                .len()
                .checked_mul(q)
                .ok_or_else(|| limit("profile.carriers"))?;
            let maximum = if q == 8 { 1900 } else { 3800 };
            if carriers > maximum {
                return Err(limit("profile.carriers"));
            }
        }
        let selected = profiles
            .iter()
            .find(|profile| profile.id() == grant.profile_id)
            .ok_or_else(|| invalid("grant.profile_id"))?;
        if grant.frames == 0 || grant.frames > 256 || grant.minislots.is_empty() {
            return Err(invalid("grant.frames"));
        }
        let mut last = None;
        for &slot in &grant.minislots {
            if slot >= selected.minislots().len() || last.is_some_and(|previous| slot <= previous) {
                return Err(invalid("grant.minislots"));
            }
            last = Some(slot);
        }
        let cells = grant
            .frames
            .checked_mul(waveform.frame_symbols())
            .and_then(|v| v.checked_mul(grant.minislots.len()))
            .and_then(|v| v.checked_mul(q))
            .ok_or_else(|| limit("grant.cells"))?;
        if cells > MAX_RESOURCE_CELLS {
            return Err(limit("grant.cells"));
        }
        let symbols = grant
            .frames
            .checked_mul(waveform.frame_symbols())
            .ok_or_else(|| limit("grant.symbols"))?;
        let support = symbols
            .checked_mul(waveform.stride())
            .and_then(|v| v.checked_add(waveform.rolloff()))
            .ok_or_else(|| limit("grant.samples"))?;
        if support > MAX_JOB_SAMPLES {
            return Err(limit("grant.samples"));
        }
        grant
            .epoch_sample
            .checked_add(support as u64)
            .ok_or_else(|| limit("grant.end"))?;
        grant
            .first_frame
            .checked_add(grant.frames as u64)
            .ok_or_else(|| limit("grant.frame_end"))?;
        Ok(Self {
            waveform,
            profiles,
            grant,
            excluded,
        })
    }
    /// Borrows accepted waveform geometry; neither endpoint sees channel truth.
    ///
    /// The borrowed snapshot stays immutable for the enclosing configuration.
    pub fn waveform(&self) -> &Waveform {
        &self.waveform
    }
    /// Borrows the complete retained inventory in original order.
    ///
    /// Querying does not select a profile or begin a run.
    pub fn profiles(&self) -> &[Profile] {
        &self.profiles
    }
    /// Borrows the explicitly selected finite grant.
    ///
    /// Source payload and injected impairment settings are absent.
    pub fn grant(&self) -> &SyntheticGrant {
        &self.grant
    }
    /// Borrows excluded physical bins without compaction or interpolation.
    ///
    /// Original input order is preserved for provenance.
    pub fn excluded(&self) -> &[usize] {
        &self.excluded
    }
    /// Finds the profile selected by the admitted grant.
    ///
    /// Returns a borrowed inventory record; admission ensures it exists.
    pub fn selected_profile(&self) -> &Profile {
        // Construction proves the selected ID is present; no mutable view escapes.
        self.profiles
            .iter()
            .find(|profile| profile.id() == self.grant.profile_id)
            .expect("validated profile")
    }
    /// Revalidates a new synthetic `grant` against the same immutable inventory.
    ///
    /// Returns a fresh snapshot, preserving this configuration on any failure.
    /// This is explicit finite repetition, not a MAC scheduling service.
    pub fn with_grant(&self, grant: SyntheticGrant) -> PhyResult<Self> {
        Self::new(
            self.waveform.clone(),
            self.profiles.clone(),
            grant,
            self.excluded.clone(),
        )
    }
}

mod document;
mod synthetic;
pub use document::{ConfigurationDocument, ConfigurationPreset, RunSettings};
pub use synthetic::{
    BankDocument, EventDocument, PreEqDocument, ProbeDocument, RxmerDocument,
    RxmerSelectionDocument, UpdateDocument,
};
