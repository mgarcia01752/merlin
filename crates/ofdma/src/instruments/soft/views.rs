/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned bounded views of actual normal-receiver soft observations.
use super::{store::SoftStore, SoftObservationPolicy, SoftObservationPolicyInfo};
use crate::{
    error::{invalid, limit},
    resources::CellRole,
    signal_processing::MAX_LOG_LLR_MAGNITUDE,
    simulation::BatchReport,
    types::{Identity, Iq},
    PhyError, PhyResult,
};
use serde::{Deserialize, Serialize};

/// Copied-cell page application ceiling, independent of aggregate soft-cell retention.
pub const MAX_SOFT_CELL_PAGE_CELLS: usize = 128;
/// Copied raw-plus-derandomized scalar ceiling, independent of estimate/query budgets.
pub const MAX_SOFT_CELL_PAGE_LLRS: usize = 4096;
/// Single completed-word copy ceiling, independent of aggregate input retention and normative N.
pub const MAX_SOFT_WORD_QUERY_LLRS: usize = 32_768;
/// Default presentation page count, independent of the equal-valued FEC decision-page ceiling.
const DEFAULT_SOFT_CELL_PAGE_CELLS: usize = 32;
pub(super) const LLR_CONVENTION: &str = "natural_log_p0_over_p1;positive_favors_zero";

/// Original completed-word identity, distinct from its retained history index.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SoftWordIdentity {
    /// Position in retained completed-word history.
    pub retained_index: usize,
    /// Actual run, accepted configuration generation and original grant.
    pub identity: Identity,
    /// Original absolute frame in which the codec call completed.
    pub completion_frame: u64,
    /// Original zero-based word ordinal within the grant.
    pub word_ordinal: usize,
    /// Original transmitted coded-bit start within the grant, before randomization.
    pub coded_offset: usize,
}

/// Availability summary; computed cells and completed codec calls have different boundaries.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SoftObservationSummary {
    /// Independent frozen soft retention policy; zero values when disabled.
    pub policy: SoftObservationPolicyInfo,
    /// Cells for which demapping and soft derandomization completed while observing.
    pub computed_cells: usize,
    /// Complete cell metric vectors retained within the application budget.
    pub retained_cells: usize,
    /// Computed cells omitted after cell-retention exhaustion.
    pub omitted_cells: usize,
    /// Actual completed codec calls while observing, including failed outcomes.
    pub completed_words: usize,
    /// Complete transmitted-position word-input vectors retained.
    pub retained_words: usize,
    /// Completed words omitted by word-count or input-scalar limits.
    pub omitted_words: usize,
    /// Actual retained transmitted-position word-input scalars.
    pub retained_word_llrs: usize,
    /// Original identities of retained words, without copying their input metrics.
    pub words: Vec<SoftWordIdentity>,
}

/// One computed nonpilot cell at the normal equalizer/demapper/derandomizer boundary.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SoftCellSnapshot {
    /// Original position in bounded retained cell history.
    pub retained_index: usize,
    /// Actual run, configuration generation and original grant.
    pub identity: Identity,
    /// Original absolute physical frame identity.
    pub frame: u64,
    /// Original symbol within that frame.
    pub symbol: usize,
    /// Original physical FFT carrier, without compaction.
    pub physical_carrier: usize,
    /// Actual data or complementary role; known pilots do not consume LLR positions.
    pub role: CellRole,
    /// Actual MSB-first payload-bit count for this cell.
    pub loading: u8,
    /// Original coded-bit start within the grant, including any final grant pad.
    pub coded_offset: usize,
    /// Actual equalized I/Q after role-gain removal; absent cells remain unavailable.
    pub corrected: Option<Iq>,
    /// Admitted complex reference-plane variance; not measured noise or field RxMER.
    pub reference_variance: f64,
    /// Actual finite MSB-first natural-log demapper metrics, before PHY mask inversion.
    pub demapper_llrs: Vec<f64>,
    /// Actual metrics after soft PHY mask inversion, including computed grant-pad positions.
    pub descrambled_llrs: Vec<f64>,
}

/// Actual transmitted-position LDPC input and independently retained decoder outcomes.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SoftWordSnapshot {
    /// Original identity, completion frame, ordinal and coded offset.
    pub word: SoftWordIdentity,
    /// Existing upstream long, medium or short code-family name.
    pub code_family: &'static str,
    /// Normative mother information dimension from the actual codec owner.
    pub mother_information_bits: usize,
    /// Normative mother codeword dimension from the actual codec owner.
    pub mother_codeword_bits: usize,
    /// Retained information positions before shortened known zeros are restored internally.
    pub retained_information_bits: usize,
    /// Actual transmitted input metric count, excluding restored shortened priors.
    pub transmitted_bits: usize,
    /// Mother information positions shortened to known zeros inside the decoder.
    pub shortened_bits: usize,
    /// Admitted complex reference variance used by the actual demapper.
    pub reference_variance: f64,
    /// Natural-log P(0)/P(1), with positive values favoring zero.
    pub llr_convention: &'static str,
    /// Exact finite input vector passed to the completed LDPC call; not posterior LLRs.
    pub input_llrs: Vec<f64>,
    /// Actual decoder iterations, not a declared work ceiling.
    pub iterations: usize,
    /// Actual convergence status, distinct from parity and source comparison.
    pub converged: bool,
    /// Actual mother-code parity status.
    pub parity_valid: bool,
    /// Actual shortened-known-zero decision consistency.
    pub shortened_zeros_valid: bool,
    /// Actual information residue/fill consistency, independent of parity.
    pub information_fill_valid: bool,
    /// Actual decoder hard-decision changes.
    pub hard_decision_changes: usize,
}

/// Original retained-cell viewport and independent copied-vector budgets.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SoftCellRequest {
    /// First retained history index; exact end returns an empty page.
    pub first_index: usize,
    /// Positive cell count, at most MAX_SOFT_CELL_PAGE_CELLS.
    pub maximum_cells: usize,
    /// Positive raw-plus-derandomized copied scalar budget, at most MAX_SOFT_CELL_PAGE_LLRS.
    pub maximum_llrs: usize,
}
impl Default for SoftCellRequest {
    /// Selects a finite initial computed-cell page with independent scalar-copy budget.
    /// # Returns
    /// A read-only query; no demapping, mask clocking or scientific replay occurs.
    fn default() -> Self {
        Self {
            first_index: 0,
            maximum_cells: DEFAULT_SOFT_CELL_PAGE_CELLS,
            maximum_llrs: MAX_SOFT_CELL_PAGE_LLRS,
        }
    }
}

/// Bounded owned cell page; computed metrics do not imply completed FEC consumption.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SoftCellPage {
    /// Original first retained index, unchanged by page size.
    pub first_index: usize,
    /// Original next retained index, absent at the retained end.
    pub next_index: Option<usize>,
    /// Whole retained history count, independent of this page.
    pub retained_cells: usize,
    /// Actual sum of copied raw and derandomized metrics.
    pub copied_llrs: usize,
    /// Explicit natural-log sign/scale convention.
    pub llr_convention: &'static str,
    /// MSB-first within each actual loaded cell.
    pub bit_order: &'static str,
    /// Actual equalized unit-reference demapper and PHY derandomizer boundary.
    pub reference_plane: &'static str,
    /// Existing numerical demapper clipping magnitude, supplied by that owner.
    pub demapper_llr_limit: f64,
    /// Actual bounded original cell records.
    pub cells: Vec<SoftCellSnapshot>,
}

/// Single retained completed-word selection and finite input-vector copy budget.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SoftWordRequest {
    /// Index discovered through soft_observations, distinct from original word ordinal.
    pub retained_index: usize,
    /// Positive copied input-vector budget, at most MAX_SOFT_WORD_QUERY_LLRS.
    pub maximum_llrs: usize,
}
impl Default for SoftWordRequest {
    /// Selects the first retained completed LDPC-input vector with a finite copy ceiling.
    /// # Returns
    /// Read-only query; no shortened prior, posterior metric or missing word is invented.
    fn default() -> Self {
        Self {
            retained_index: 0,
            maximum_llrs: MAX_SOFT_WORD_QUERY_LLRS,
        }
    }
}

impl BatchReport {
    /// Inspects actual computed-cell and completed-word availability without copying metrics.
    /// # Returns
    /// Owned budgets, independent observed/retained/omitted counts and bounded word identities.
    /// Disabled observations return zero availability, without claiming the receiver did no work.
    pub fn soft_observations(&self) -> SoftObservationSummary {
        match self
            .observations
            .receiver_estimates
            .as_ref()
            .and_then(|store| store.soft.as_ref())
        {
            Some(store) => store.summary(),
            None => SoftObservationSummary {
                policy: SoftObservationPolicy::default().info(),
                computed_cells: 0,
                retained_cells: 0,
                omitted_cells: 0,
                completed_words: 0,
                retained_words: 0,
                omitted_words: 0,
                retained_word_llrs: 0,
                words: Vec::new(),
            },
        }
    }

    /// Copies bounded actual computed-cell metrics in original retained order.
    /// # Arguments
    /// * `request` - Retained history start and independent positive cell/scalar copy limits.
    /// # Returns
    /// Owned original cell metadata and raw/derandomized vectors; exact end is an empty page.
    /// No scientific processing or state mutation occurs, including at omitted/partial boundaries.
    /// # Errors
    /// Disabled observation is unavailable; invalid starts/budgets and excessive copy costs reject
    /// before allocation. Computed cells can exist without a completed word after cancellation.
    pub fn soft_cells(&self, request: SoftCellRequest) -> PhyResult<SoftCellPage> {
        if !(1..=MAX_SOFT_CELL_PAGE_CELLS).contains(&request.maximum_cells)
            || !(1..=MAX_SOFT_CELL_PAGE_LLRS).contains(&request.maximum_llrs)
        {
            return Err(limit("soft_cells.query_budget"));
        }
        let store = soft_store(self)?;
        if request.first_index > store.cells.len() {
            return Err(invalid("soft_cells.first_index"));
        }
        let count = request
            .maximum_cells
            .min(store.cells.len() - request.first_index);
        let end = request.first_index + count;
        let mut copied_llrs = 0_usize;
        for cell in &store.cells[request.first_index..end] {
            copied_llrs = copied_llrs
                .checked_add(cell.demapper_llrs.len())
                .and_then(|count| count.checked_add(cell.descrambled_llrs.len()))
                .filter(|count| *count <= request.maximum_llrs)
                .ok_or_else(|| limit("soft_cells.copy_llrs"))?;
        }
        Ok(SoftCellPage {
            first_index: request.first_index,
            next_index: (end < store.cells.len()).then_some(end),
            retained_cells: store.cells.len(),
            copied_llrs,
            llr_convention: LLR_CONVENTION,
            bit_order: "msb_first",
            reference_plane: "equalized_role_gain_removed_demapper_and_phy_derandomizer",
            demapper_llr_limit: MAX_LOG_LLR_MAGNITUDE,
            cells: store.cells[request.first_index..end]
                .iter()
                .map(|cell| cell.as_ref().clone())
                .collect(),
        })
    }

    /// Copies the exact transmitted-position input of one actually completed LDPC call.
    /// # Arguments
    /// * `request` - Retained word index and positive finite input-vector copy ceiling.
    /// # Returns
    /// Owned actual input LLRs, original identity/dimensions and independent decoder outcomes.
    /// No posterior values, shortened priors, source comparison or codec rerun is substituted.
    /// # Errors
    /// Disabled/unretained/incomplete words are unavailable; invalid/insufficient budgets reject
    /// before copying. Failed completed decoder decisions remain available when retained.
    pub fn soft_word(&self, request: SoftWordRequest) -> PhyResult<SoftWordSnapshot> {
        if !(1..=MAX_SOFT_WORD_QUERY_LLRS).contains(&request.maximum_llrs) {
            return Err(limit("soft_word.query_budget"));
        }
        let word =
            soft_store(self)?
                .words
                .get(request.retained_index)
                .ok_or(PhyError::Unsupported {
                    field: "soft_word.unavailable",
                })?;
        if word.input_llrs.len() > request.maximum_llrs {
            return Err(limit("soft_word.copy_llrs"));
        }
        Ok(word.as_ref().clone())
    }
}

fn soft_store(report: &BatchReport) -> PhyResult<&SoftStore> {
    report
        .observations
        .receiver_estimates
        .as_ref()
        .and_then(|store| store.soft.as_ref())
        .ok_or(PhyError::Unsupported {
            field: "soft_observations.unavailable",
        })
}
