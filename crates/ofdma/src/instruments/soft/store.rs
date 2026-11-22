/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive retention at actual computed-cell and completed-codec boundaries.
use super::{
    policy::SoftObservationPolicy,
    views::{
        SoftCellSnapshot, SoftObservationSummary, SoftWordIdentity, SoftWordSnapshot,
        LLR_CONVENTION,
    },
};
use crate::{
    grant::CodewordPlan,
    receiver::DecodedWord,
    resources::Cell,
    types::{Identity, Iq},
};
use serde::Serialize;
use std::sync::Arc;

/// Actual receiver frame context; no source bits or configured channel truth.
#[derive(Clone, Copy)]
pub(crate) struct SoftFrameContext {
    pub(crate) identity: Identity,
    pub(crate) frame: u64,
    pub(crate) reference_variance: f64,
}

/// Borrowed actual stage vectors, preserving their distinct scientific meanings.
pub(crate) struct SoftCellMetrics<'metrics> {
    pub(crate) demapper: &'metrics [f64],
    pub(crate) descrambled: &'metrics [f64],
}

/// Aggregate immutable-record owner shared by transactional receiver clones.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct SoftStore {
    policy: SoftObservationPolicy,
    computed_cells: usize,
    completed_words: usize,
    retained_word_llrs: usize,
    #[serde(serialize_with = "super::super::receiver::serialize_records")]
    pub(super) cells: Vec<Arc<SoftCellSnapshot>>,
    #[serde(serialize_with = "super::super::receiver::serialize_records")]
    pub(super) words: Vec<Arc<SoftWordSnapshot>>,
}

impl SoftStore {
    /// Creates empty finite retention metadata for an enabled validated `policy`.
    /// # Returns
    /// Owned empty history; no metric vector or scientific state is allocated or advanced.
    pub(crate) fn new(policy: SoftObservationPolicy) -> Self {
        Self {
            policy,
            computed_cells: 0,
            completed_words: 0,
            retained_word_llrs: 0,
            cells: Vec::new(),
            words: Vec::new(),
        }
    }

    /// Records the normal receiver's already computed full nonpilot cell vectors.
    /// # Arguments
    /// * `context`, `cell`, `coded_offset` - Actual identities, reference variance and physical/coded positions.
    /// * `corrected` - Actual optional I/Q after equalization and role-gain removal.
    /// * `metrics` - Borrowed raw demapper and derandomized vectors; neither is recalculated.
    /// # Returns
    /// Unit; disabled/exhausted cell scope allocates nothing and never mutates scientific state.
    pub(crate) fn record_cell(
        &mut self,
        context: SoftFrameContext,
        cell: &Cell,
        coded_offset: usize,
        corrected: Option<Iq>,
        metrics: SoftCellMetrics<'_>,
    ) {
        let policy = self.policy.info();
        if policy.maximum_cells == 0 {
            return;
        }
        self.computed_cells += 1;
        if self.cells.len() >= policy.maximum_cells {
            return;
        }
        self.cells.push(Arc::new(SoftCellSnapshot {
            retained_index: self.cells.len(),
            identity: context.identity,
            frame: context.frame,
            symbol: cell.symbol,
            physical_carrier: cell.carrier,
            role: cell.role,
            loading: cell.loading,
            coded_offset,
            corrected,
            reference_variance: context.reference_variance,
            demapper_llrs: metrics.demapper.to_vec(),
            descrambled_llrs: metrics.descrambled.to_vec(),
        }));
    }

    /// Records exact transmitted-position input only after an actual LDPC call completes.
    /// # Arguments
    /// * `context`, `plan` - Actual completion frame/identity, variance and original word dimensions.
    /// * `decoded` - Actual completed decisions and independent decoder/fill flags.
    /// * `input_llrs` - Exact buffer passed to that call, excluding restored shortened priors.
    /// # Returns
    /// Unit; disabled/exhausted word scope omits the complete vector without changing decoding.
    pub(crate) fn record_word(
        &mut self,
        context: SoftFrameContext,
        plan: &CodewordPlan,
        decoded: &DecodedWord,
        input_llrs: &[f64],
    ) {
        let policy = self.policy.info();
        if policy.maximum_words == 0 {
            return;
        }
        self.completed_words += 1;
        if self.words.len() >= policy.maximum_words
            || input_llrs.len() > policy.maximum_word_llrs - self.retained_word_llrs
        {
            return;
        }
        let dimensions = plan.code.parameters();
        self.retained_word_llrs += input_llrs.len();
        self.words.push(Arc::new(SoftWordSnapshot {
            word: SoftWordIdentity {
                retained_index: self.words.len(),
                identity: context.identity,
                completion_frame: context.frame,
                word_ordinal: decoded.ordinal,
                coded_offset: plan.coded_offset,
            },
            code_family: super::super::recovery::code_family_name(plan.code),
            mother_information_bits: dimensions.information_bits,
            mother_codeword_bits: dimensions.codeword_bits,
            retained_information_bits: plan.information_bits,
            transmitted_bits: plan.transmitted_bits,
            shortened_bits: dimensions.information_bits - plan.information_bits,
            reference_variance: context.reference_variance,
            llr_convention: LLR_CONVENTION,
            input_llrs: input_llrs.to_vec(),
            iterations: decoded.decoded.iterations,
            converged: decoded.decoded.converged,
            parity_valid: decoded.decoded.parity_valid,
            shortened_zeros_valid: decoded.decoded.shortened_zeros_valid,
            information_fill_valid: decoded.information_fill_valid,
            hard_decision_changes: decoded.decoded.hard_decision_changes,
        }));
    }

    /// Projects bounded availability and original word identities without copying metrics.
    /// # Returns
    /// Owned counts for enabled scopes, including omissions independently of actual outcomes.
    pub(super) fn summary(&self) -> SoftObservationSummary {
        SoftObservationSummary {
            policy: self.policy.info(),
            computed_cells: self.computed_cells,
            retained_cells: self.cells.len(),
            omitted_cells: self.computed_cells - self.cells.len(),
            completed_words: self.completed_words,
            retained_words: self.words.len(),
            omitted_words: self.completed_words - self.words.len(),
            retained_word_llrs: self.retained_word_llrs,
            words: self.words.iter().map(|word| word.word.clone()).collect(),
        }
    }
}
