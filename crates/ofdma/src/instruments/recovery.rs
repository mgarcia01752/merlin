/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive bounded views of original decoder outcomes and independent source verification.
use crate::{
    configuration::MAX_CODEWORDS,
    error::{invalid, limit},
    grant::CodewordPlan,
    receiver::DecodedWord,
    simulation::{BatchReport, GrantReport, RunStatus},
    types::Identity,
    PhyResult,
};
use fec_codec::upstream_ldpc::DocsisLdpcCode;
use serde::{Deserialize, Serialize};

/// Application word-count ceiling, independent of decoder iterations and bit storage.
pub const MAX_FEC_PAGE_WORDS: usize = 32;
/// Application copied-bit ceiling over mother, information and payload decision vectors.
pub const MAX_FEC_PAGE_DECISION_BITS: usize = 262_144;
/// The initial page selects one completed word, independently of the maximum page size.
const DEFAULT_FEC_PAGE_WORDS: usize = 1;

/// Original started-grant index and completed-word range, with independent copy budgets.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FecWordRequest {
    /// Original zero-based position among started grants; not a grant ID.
    pub grant_index: usize,
    /// Original zero-based completed-word ordinal within that grant.
    pub first_word: usize,
    /// Positive maximum returned words, at most MAX_FEC_PAGE_WORDS.
    pub maximum_words: usize,
    /// Positive total copied decision bits, at most MAX_FEC_PAGE_DECISION_BITS.
    pub max_decision_bits: usize,
}
impl Default for FecWordRequest {
    /// Selects the first actual completed word with a finite independent decision-copy budget.
    /// # Returns
    /// An owned passive request; constructing it starts no decoder or source.
    fn default() -> Self {
        Self {
            grant_index: 0,
            first_word: 0,
            maximum_words: DEFAULT_FEC_PAGE_WORDS,
            max_decision_bits: MAX_FEC_PAGE_DECISION_BITS,
        }
    }
}

/// Whole-report actual decoder totals; no flag is interpreted as original-source recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct FecSummary {
    /// Originally planned words, including grants that never started.
    pub planned_words: usize,
    /// Actual completed decoder outcomes, including failed decisions.
    pub completed_words: usize,
    /// Planned words without completed outcomes; cancellation never turns them into failures.
    pub pending_words: usize,
    /// Actual convergence flags, independent of parity and source truth.
    pub converged_words: usize,
    /// Completed outcomes whose actual convergence flag is false.
    pub nonconverged_words: usize,
    /// Completed outcomes whose independently checked final parity is invalid.
    pub parity_invalid_words: usize,
    /// Completed outcomes whose shortened known-zero coordinates are inconsistent.
    pub shortening_invalid_words: usize,
    /// Completed outcomes whose information-fill-one region is inconsistent.
    pub fill_invalid_words: usize,
    /// Sum of actual decoder iterations over completed outcomes.
    pub total_iterations: usize,
    /// Largest actual iteration count, absent if no word completed.
    pub maximum_iterations: Option<usize>,
    /// Sum of actual changes from nonzero transmitted-LLR signs, excluding restored zeros.
    pub hard_decision_changes: usize,
}

/// Independent PRBS15 comparison and actual coverage, distinct from decoder consistency.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct VerificationSnapshot {
    /// Actual terminal lifecycle state, independent of recovery.
    pub status: RunStatus,
    /// Original continuous PRBS15 source seed, never a PHY randomizer seed.
    pub source_seed: u16,
    /// Originally planned meaningful source bits, excluding fill and parity.
    pub planned_source_bits: usize,
    /// Actual meaningful source bits generated across started grants.
    pub generated_source_bits: usize,
    /// Actual recovered decisions independently compared with the original source.
    pub compared_source_bits: usize,
    /// Generated source positions without a completed comparison.
    pub unverified_source_bits: usize,
    /// Independent original-source Hamming distance over compared decisions.
    pub bit_errors: usize,
    /// Actual optional BER; absent if nothing was compared.
    pub ber: Option<f64>,
    /// Original complete nonempty source/FEC/fill/coverage acceptance flag.
    pub reference_recovered: bool,
    /// Original finite scheduled grant count.
    pub planned_grants: usize,
    /// Actual started grant count, including partial and failed grants.
    pub started_grants: usize,
    /// Normally completed grants, independently of their decoder/source success.
    pub completed_grants: usize,
    /// Scheduled grants that never started source admission.
    pub not_started_grants: usize,
    /// Actual useful symbols with incomplete input coverage.
    pub missing_symbols: usize,
    /// Actual completed decoder outcomes, including failures.
    pub completed_codewords: usize,
    /// Scheduled codewords without a completed outcome.
    pub pending_codewords: usize,
}

/// One owned original upstream decoder outcome and its scientific plan metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FecWordSnapshot {
    /// Actual nonzero run/configuration and original grant identity.
    pub identity: Identity,
    /// Explicit upstream data LDPC family, without a downstream BCH stage.
    pub code_family: &'static str,
    /// Normative mother information dimension from the existing codec owner.
    pub mother_information_bits: usize,
    /// Normative full mother codeword dimension, including restored positions.
    pub mother_codeword_bits: usize,
    /// Original retained information count before trailing-zero shortening.
    pub retained_information_bits: usize,
    /// Original transmitted information-plus-parity count, excluding restored positions.
    pub transmitted_bits: usize,
    /// Original omitted trailing known-zero information count.
    pub shortened_bits: usize,
    /// Original encoded-grant bit offset before PHY randomization.
    pub coded_offset: usize,
    /// Absolute original meaningful-source start, without circular period reduction.
    pub source_start_bit: u64,
    /// Unmodified original completed payload, mother/information decisions and flags.
    pub word: DecodedWord,
}

/// Bounded completed-word page; scalar totals do not depend on its display limits.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct FecWordPage {
    /// Original zero-based selected started-grant position.
    pub grant_index: usize,
    /// Actual run/configuration/grant association of every returned word.
    pub identity: Identity,
    /// Original selected profile identity, independent of page position.
    pub profile_id: u16,
    /// Original selected profile IUC, without inferring a QAM order from it.
    pub iuc: u8,
    /// Complete selected-grant completed-word count, including failed decisions.
    pub total_completed_words: usize,
    /// Selected-grant words without completed outcomes.
    pub pending_words: usize,
    /// Original first completed-word ordinal selected by this query.
    pub first_word: usize,
    /// Next completed-word ordinal, absent at the actual retained end.
    pub next_word: Option<usize>,
    /// Total mother/information/payload decision bits copied into this page.
    pub copied_decision_bits: usize,
    /// Owned exact original outcomes, without reconstructed or repeated decoding.
    pub words: Vec<FecWordSnapshot>,
}

impl BatchReport {
    /// Reads independent source verification and actual complete/partial coverage.
    /// # Returns
    /// Owned scalar snapshot with the original BER, recovery flag and lifecycle state.
    /// This query never generates source bits, runs a decoder or modifies retained science.
    /// # Errors
    /// Rejects inconsistent public report extents/counters or checked aggregate overflow.
    pub fn verification(&self) -> PhyResult<VerificationSnapshot> {
        let fec = self.fec_summary()?;
        if self
            .generated_source_bits
            .checked_sub(self.compared_source_bits)
            != Some(self.unverified_source_bits)
            || self.bit_errors > self.compared_source_bits
            || self.generated_source_bits > self.planned.source_bits
            || self
                .ber
                .is_some_and(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
            || (self.compared_source_bits == 0) != self.ber.is_none()
        {
            return Err(invalid("verification.report_counts"));
        }
        let mut missing_symbols = 0;
        for grant in &self.grants {
            add(&mut missing_symbols, grant.missing_symbols)?;
        }
        Ok(VerificationSnapshot {
            status: self.status,
            source_seed: self.configuration.source_seed,
            planned_source_bits: self.planned.source_bits,
            generated_source_bits: self.generated_source_bits,
            compared_source_bits: self.compared_source_bits,
            unverified_source_bits: self.unverified_source_bits,
            bit_errors: self.bit_errors,
            ber: self.ber,
            reference_recovered: self.reference_recovered,
            planned_grants: self.options.grants,
            started_grants: self.grants.len(),
            completed_grants: self.grants.iter().filter(|grant| grant.completed).count(),
            not_started_grants: self.not_started_grants.len(),
            missing_symbols,
            completed_codewords: fec.completed_words,
            pending_codewords: fec.pending_words,
        })
    }

    /// Summarizes actual completed decoder flags and iterations independently of source truth.
    /// # Returns
    /// Owned whole-report counts, including honest pending/nonconverged words and absent maxima.
    /// Page limits, observation retention and repeated inspection do not change these totals.
    /// # Errors
    /// Rejects inconsistent original report/word dimensions or checked aggregate overflow.
    pub fn fec_summary(&self) -> PhyResult<FecSummary> {
        validate_structure(self)?;
        let mut summary = FecSummary {
            planned_words: self.planned.codewords,
            completed_words: 0,
            pending_words: self.planned.codewords,
            converged_words: 0,
            nonconverged_words: 0,
            parity_invalid_words: 0,
            shortening_invalid_words: 0,
            fill_invalid_words: 0,
            total_iterations: 0,
            maximum_iterations: None,
            hard_decision_changes: 0,
        };
        for grant in &self.grants {
            for (ordinal, word) in grant.words.iter().enumerate() {
                word_plan(self, grant, ordinal, word)?;
                add(&mut summary.completed_words, 1)?;
                if word.decoded.converged {
                    add(&mut summary.converged_words, 1)?;
                } else {
                    add(&mut summary.nonconverged_words, 1)?;
                }
                if !word.decoded.parity_valid {
                    add(&mut summary.parity_invalid_words, 1)?;
                }
                if !word.decoded.shortened_zeros_valid {
                    add(&mut summary.shortening_invalid_words, 1)?;
                }
                if !word.information_fill_valid {
                    add(&mut summary.fill_invalid_words, 1)?;
                }
                add(&mut summary.total_iterations, word.decoded.iterations)?;
                summary.maximum_iterations = Some(
                    summary
                        .maximum_iterations
                        .unwrap_or(0)
                        .max(word.decoded.iterations),
                );
                add(
                    &mut summary.hard_decision_changes,
                    word.decoded.hard_decision_changes,
                )?;
            }
        }
        summary.pending_words = summary
            .planned_words
            .checked_sub(summary.completed_words)
            .ok_or_else(|| invalid("fec_view.completed_words"))?;
        Ok(summary)
    }

    /// Copies a bounded page of original completed words from one actually started grant.
    /// # Arguments
    /// * `request` - Original grant/word positions and independent positive word/decision budgets.
    /// # Returns
    /// Owned original outcomes with code-family/shortening/source metadata and a continuation.
    /// An exact end position returns an empty page; unprocessed words remain pending.
    /// All dimensions and copy costs are checked before allocating or copying decisions.
    /// # Errors
    /// Rejects missing grants, invalid starts/budgets, inconsistent metadata or overflow.
    /// A page exceeding its decision budget fails without changing the immutable report.
    pub fn fec_words(&self, request: FecWordRequest) -> PhyResult<FecWordPage> {
        validate_structure(self)?;
        if request.maximum_words == 0
            || request.maximum_words > MAX_FEC_PAGE_WORDS
            || request.max_decision_bits == 0
            || request.max_decision_bits > MAX_FEC_PAGE_DECISION_BITS
        {
            return Err(limit("fec_view.query_budget"));
        }
        let grant = self
            .grants
            .get(request.grant_index)
            .ok_or_else(|| invalid("fec_view.grant_index"))?;
        if request.first_word > grant.words.len() {
            return Err(invalid("fec_view.first_word"));
        }
        let count = request
            .maximum_words
            .min(grant.words.len() - request.first_word);
        let end = request.first_word + count;
        let mut copied_decision_bits = 0;
        for (offset, word) in grant.words[request.first_word..end].iter().enumerate() {
            word_plan(self, grant, request.first_word + offset, word)?;
            add(
                &mut copied_decision_bits,
                word.decoded.mother_codeword.len(),
            )?;
            add(
                &mut copied_decision_bits,
                word.decoded.information_bits.len(),
            )?;
            add(&mut copied_decision_bits, word.payload.len())?;
            if copied_decision_bits > request.max_decision_bits {
                return Err(limit("fec_view.decision_bits"));
            }
        }
        let mut words = Vec::with_capacity(count);
        for (offset, word) in grant.words[request.first_word..end].iter().enumerate() {
            let plan = word_plan(self, grant, request.first_word + offset, word)?;
            let dimensions = plan.code.parameters();
            let code_family = match plan.code {
                DocsisLdpcCode::UpstreamLong => "upstream_long",
                DocsisLdpcCode::UpstreamMedium => "upstream_medium",
                DocsisLdpcCode::UpstreamShort => "upstream_short",
            };
            words.push(FecWordSnapshot {
                identity: grant.identity,
                code_family,
                mother_information_bits: dimensions.information_bits,
                mother_codeword_bits: dimensions.codeword_bits,
                retained_information_bits: plan.information_bits,
                transmitted_bits: plan.transmitted_bits,
                shortened_bits: dimensions.information_bits - plan.information_bits,
                coded_offset: plan.coded_offset,
                source_start_bit: grant
                    .generated_source
                    .start()
                    .checked_add(word.payload_offset as u64)
                    .ok_or_else(|| limit("fec_view.source_position"))?,
                word: word.clone(),
            });
        }
        let next_word = if end < grant.words.len() {
            Some(end)
        } else {
            None
        };
        let profile = self.configuration.us_configuration.selected_profile();
        Ok(FecWordPage {
            grant_index: request.grant_index,
            identity: grant.identity,
            profile_id: profile.id(),
            iuc: profile.iuc(),
            total_completed_words: grant.words.len(),
            pending_words: self.planned.grant_plan.codewords.len() - grant.words.len(),
            first_word: request.first_word,
            next_word,
            copied_decision_bits,
            words,
        })
    }
}

fn validate_structure(report: &BatchReport) -> PhyResult<()> {
    report.options.validate()?;
    let words_per_grant = report.planned.grant_plan.codewords.len();
    if words_per_grant > MAX_CODEWORDS {
        return Err(limit("fec_view.planned_words"));
    }
    if words_per_grant.checked_mul(report.options.grants) != Some(report.planned.codewords)
        || report.grants.len() > report.options.grants
        || report.not_started_grants.len() != report.options.grants - report.grants.len()
        || report
            .grants
            .iter()
            .any(|grant| grant.words.len() > words_per_grant)
    {
        return Err(invalid("fec_view.report_extent"));
    }
    Ok(())
}

fn word_plan<'report>(
    report: &'report BatchReport,
    grant: &GrantReport,
    ordinal: usize,
    word: &DecodedWord,
) -> PhyResult<&'report CodewordPlan> {
    let plan = report
        .planned
        .grant_plan
        .codewords
        .get(ordinal)
        .ok_or_else(|| invalid("fec_view.word_ordinal"))?;
    let dimensions = plan.code.parameters();
    if word.ordinal != ordinal
        || word.payload_offset != plan.information_offset
        || plan.information_bits == 0
        || plan.information_bits > dimensions.information_bits
        || plan.transmitted_bits
            != dimensions.codeword_bits - (dimensions.information_bits - plan.information_bits)
        || word.decoded.information_bits.len() != plan.information_bits
        || word.decoded.mother_codeword.len() != dimensions.codeword_bits
        || word.payload.len() > plan.information_bits
        || word
            .payload_offset
            .checked_add(word.payload.len())
            .is_none_or(|end| end > grant.generated_source.length())
    {
        return Err(invalid("fec_view.word_metadata"));
    }
    grant.identity.validate()?;
    Ok(plan)
}

fn add(total: &mut usize, amount: usize) -> PhyResult<()> {
    *total = total
        .checked_add(amount)
        .ok_or_else(|| limit("fec_view.aggregate"))?;
    Ok(())
}
