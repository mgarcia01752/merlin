/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent retention policies for computed cells and completed word-input metrics.
use crate::{error::invalid, PhyResult};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;

/// Aggregate computed-cell application ceiling; each admitted cell has two bounded bit vectors.
pub const MAX_RECEIVER_SOFT_CELLS: usize = 1024;
/// Aggregate completed-word application ceiling, independent of FEC decision-page counts.
pub const MAX_RECEIVER_SOFT_WORDS: usize = 32;
/// Aggregate retained word-input scalar ceiling, independent of receiver-estimate storage.
pub const MAX_RECEIVER_SOFT_WORD_LLRS: usize = 262_144;

/// Validated immutable optional soft retention, independent of estimate retention.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct SoftObservationPolicy {
    maximum_cells: usize,
    maximum_words: usize,
    maximum_word_llrs: usize,
}

/// Owned budgets for actual receiver soft observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SoftObservationPolicyInfo {
    /// Maximum computed nonpilot cells across all grants; zero disables cell retention.
    pub maximum_cells: usize,
    /// Maximum completed words across all grants; zero disables word retention.
    pub maximum_words: usize,
    /// Aggregate copied transmitted-position input LLRs, without shortened priors.
    pub maximum_word_llrs: usize,
}

impl SoftObservationPolicy {
    /// Validates independent computed-cell, completed-word and word-input scalar budgets.
    /// # Arguments
    /// * `maximum_cells` - At most 1024 actual computed cells, each with two admitted bit vectors.
    /// * `maximum_words` - At most 32 completed codec calls, including failed decisions.
    /// * `maximum_word_llrs` - At most 262144 input metrics; zero exactly when words are disabled.
    /// # Returns
    /// An owned immutable policy; all zero values disable soft retention. No science starts.
    /// A complete word that cannot fit its remaining budget is omitted, never truncated.
    /// # Errors
    /// Rejects exceeded application ceilings and inconsistent word/scalar budgets.
    pub fn new(
        maximum_cells: usize,
        maximum_words: usize,
        maximum_word_llrs: usize,
    ) -> PhyResult<Self> {
        if maximum_cells > MAX_RECEIVER_SOFT_CELLS
            || maximum_words > MAX_RECEIVER_SOFT_WORDS
            || maximum_word_llrs > MAX_RECEIVER_SOFT_WORD_LLRS
            || (maximum_words == 0) != (maximum_word_llrs == 0)
        {
            return Err(invalid("soft_observations.policy"));
        }
        Ok(Self {
            maximum_cells,
            maximum_words,
            maximum_word_llrs,
        })
    }

    /// Projects aggregate retention budgets without changing observer or scientific state.
    /// # Returns
    /// Owned scalar limits; zero cell or word counts preserve their independently disabled scope.
    pub fn info(&self) -> SoftObservationPolicyInfo {
        SoftObservationPolicyInfo {
            maximum_cells: self.maximum_cells,
            maximum_words: self.maximum_words,
            maximum_word_llrs: self.maximum_word_llrs,
        }
    }
}

impl RetainedPayload for SoftObservationPolicy {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            maximum_cells,
            maximum_words,
            maximum_word_llrs,
        } = self;
        sum([
            maximum_cells.retained_children(),
            maximum_words.retained_children(),
            maximum_word_llrs.retained_children(),
        ])
    }
}
