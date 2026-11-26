/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Explicit synthetic scheduled grants, independent of a MAC/scheduler.
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};

/// Candidate finite grant; the enclosing configuration validates all fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SyntheticGrant {
    /// Unique within the accepted run.
    pub id: u64,
    /// Retained profile selected explicitly; its IUC does not imply loading.
    pub profile_id: u16,
    /// Absolute first prefix sample tick at 102.4 MHz.
    pub epoch_sample: u64,
    /// Absolute first frame identity.
    pub first_frame: u64,
    /// Consecutive frames using this assignment, bounded before execution.
    pub frames: usize,
    /// Selected indices in the profile's physical minislot inventory.
    pub minislots: Vec<usize>,
}

impl RetainedPayload for SyntheticGrant {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            id,
            profile_id,
            epoch_sample,
            first_frame,
            frames,
            minislots,
        } = self;
        sum([
            id.retained_children(),
            profile_id.retained_children(),
            epoch_sample.retained_children(),
            first_frame.retained_children(),
            frames.retained_children(),
            minislots.retained_children(),
        ])
    }
}
