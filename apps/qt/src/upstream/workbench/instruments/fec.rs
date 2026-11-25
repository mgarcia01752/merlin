/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Original upstream decoder outcomes; independent source verification stays separate.
use super::common;
use merlin::upstream::{
    BatchReport, FecWordRequest, MAX_FEC_PAGE_DECISION_BITS, MAX_FEC_PAGE_WORDS,
    MAX_SOFT_CELL_PAGE_LLRS, MAX_SOFT_WORD_QUERY_LLRS,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Independent native table population, not a receiver or scientific copy ceiling.
pub(super) const MAX_LLR_VIEW_ROWS: usize = 256;
/// Initial word-input display window, independent of retained-cell population.
pub(super) const DEFAULT_LLR_VIEW_ROWS: usize = 64;
/// Eight maximum-loading cells fit one finite native scalar table without truncation.
/// The larger public cell-page ceiling remains available to independent consumers.
pub(super) const MAX_CELL_VIEW_CELLS: usize = 8;

/// Complete strict selector for three distinct actual decoder observation planes.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub(in crate::upstream::workbench) enum FecRequest {
    Words {
        grant_index: usize,
        first_word: usize,
        maximum_words: usize,
        max_decision_bits: usize,
    },
    SoftCells {
        first_index: usize,
        maximum_cells: usize,
        maximum_llrs: usize,
    },
    SoftWord {
        retained_index: usize,
        maximum_llrs: usize,
        first_bit: usize,
        maximum_bits: usize,
    },
}

impl Default for FecRequest {
    /// Reuses the public default completed-word query without starting science.
    /// # Returns
    /// The original first-grant/word selection and independent public copy budget.
    fn default() -> Self {
        let request = FecWordRequest::default();
        Self::Words {
            grant_index: request.grant_index,
            first_word: request.first_word,
            maximum_words: request.maximum_words,
            max_decision_bits: request.max_decision_bits,
        }
    }
}

/// Queries actual outcomes and projects only finite original scalar/LLR tables.
/// # Arguments
/// * `report`, `request` - Immutable actual complete/partial report and typed selection.
/// # Returns
/// A candidate requiring existing aggregate admission before persistent ownership.
/// # Errors
/// Original public validation, malformed display bounds or lossless transport failure.
pub(super) fn project(report: &BatchReport, request: FecRequest) -> Result<Value, String> {
    let mut display = observe(report, request)?;
    let mut rows = Vec::new();
    if let Some(metadata) = display["metadata"].as_object() {
        for (name, value) in metadata {
            if name != "verification" {
                rows.push(json!({"label":name.replace('_'," "),"value":value.as_str().map(str::to_owned).unwrap_or_else(||value.to_string())}));
            }
        }
    }
    let verification = &display["metadata"]["verification"];
    for (label, name) in [
        ("PRBS15 source seed", "source_seed"),
        ("Lifecycle", "status"),
        ("Full independent source recovery", "reference_recovered"),
        ("Planned meaningful bits", "planned_source_bits"),
        ("Generated meaningful bits", "generated_source_bits"),
        (
            "Compared meaningful bits (BER denominator)",
            "compared_source_bits",
        ),
        ("Unverified generated bits", "unverified_source_bits"),
        ("Independent bit errors", "bit_errors"),
    ] {
        let value = &verification[name];
        rows.push(json!({"label":label,"value":value.as_str().map(str::to_owned).unwrap_or_else(||value.to_string())}));
    }
    rows.push(json!({"label":"BER","value":super::super::execution_view::ber_label(verification["ber"].as_f64())}));
    display["metadata_rows"] = rows.into();
    Ok(display)
}

fn observe(report: &BatchReport, request: FecRequest) -> Result<Value, String> {
    let mut display = common::envelope(
        "fec",
        choices(report)?,
        serde_json::to_value(request).map_err(|e| e.to_string())?,
    );
    display["limits"] = json!({"maximum_words":MAX_FEC_PAGE_WORDS,
        "max_decision_bits":MAX_FEC_PAGE_DECISION_BITS,"maximum_cells":MAX_CELL_VIEW_CELLS,
        "maximum_cell_llrs":MAX_SOFT_CELL_PAGE_LLRS,"maximum_word_llrs":MAX_SOFT_WORD_QUERY_LLRS,
        "maximum_bit_rows":MAX_LLR_VIEW_ROWS,"initial_bit_rows":DEFAULT_LLR_VIEW_ROWS});
    display["metadata"] = common::report_metadata(report);
    display["metadata"]["verification"] =
        serde_json::to_value(report.verification().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    display["metadata"]["fec_summary"] =
        serde_json::to_value(report.fec_summary().map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    display["metadata"]["soft_observations"] = super::soft::summary(report)?;
    display["table"] = json!({"headings":[],"rows":[]});
    display["chart"] = json!({"points":[],"maximum":"0"});
    match request {
        FecRequest::Words {
            grant_index,
            first_word,
            maximum_words,
            max_decision_bits,
        } => {
            display["metadata"]["reference_plane"] = "Original completed upstream LDPC outcomes; parity/convergence are not independent source comparison.".into();
            let source = match report.fec_words(FecWordRequest {
                grant_index,
                first_word,
                maximum_words,
                max_decision_bits,
            }) {
                Ok(source) => source,
                Err(merlin::upstream::PhyError::InvalidInput {
                    field: "fec_view.grant_index",
                }) if report.grants.is_empty() => {
                    display["status"] = "unavailable".into();
                    display["message"] =
                        "No grant started; there are no completed decoder words.".into();
                    return Ok(display);
                }
                Err(error) => return Err(error.to_string()),
            };
            let mut words = Vec::with_capacity(source.words.len());
            let mut rows = Vec::with_capacity(source.words.len());
            let maximum = source
                .words
                .iter()
                .map(|word| word.word.decoded.iterations)
                .max()
                .unwrap_or(0);
            let mut bars = Vec::with_capacity(source.words.len());
            for word in &source.words {
                let decoded = &word.word.decoded;
                words.push(json!({"identity":identity_value(&word.identity)?,"code_family":word.code_family,
                    "mother_information_bits":word.mother_information_bits,"mother_codeword_bits":word.mother_codeword_bits,
                    "retained_information_bits":word.retained_information_bits,"transmitted_bits":word.transmitted_bits,
                    "shortened_bits":word.shortened_bits,"coded_offset":word.coded_offset,
                    "source_start_bit":word.source_start_bit.to_string(),"ordinal":word.word.ordinal,
                    "payload_offset":word.word.payload_offset,"payload_bits":word.word.payload.len(),
                    "iterations":decoded.iterations,"converged":decoded.converged,"parity_valid":decoded.parity_valid,
                    "shortened_zeros_valid":decoded.shortened_zeros_valid,
                    "information_fill_valid":word.word.information_fill_valid,
                    "hard_decision_changes":decoded.hard_decision_changes}));
                rows.push(vec![
                    word.word.ordinal.to_string(),
                    word.code_family.to_owned(),
                    decoded.iterations.to_string(),
                    flag(decoded.converged).into(),
                    flag(decoded.parity_valid).into(),
                    flag(decoded.shortened_zeros_valid).into(),
                    flag(word.word.information_fill_valid).into(),
                    word.source_start_bit.to_string(),
                ]);
                bars.push(json!({"label":word.word.ordinal.to_string(),"count":decoded.iterations.to_string(),
                    "fraction":if maximum==0 {
                        0.0
                    } else {
                        decoded.iterations as f64/maximum as f64
                    },
                    "attention":!decoded.converged || !decoded.parity_valid || !decoded.shortened_zeros_valid || !word.word.information_fill_valid}));
            }
            display["data"] = json!({"grant_index":source.grant_index,"identity":identity_value(&source.identity)?,
                "profile_id":source.profile_id,"iuc":source.iuc,"total_completed_words":source.total_completed_words,
                "pending_words":source.pending_words,"first_word":source.first_word,"next_word":source.next_word,
                "copied_decision_bits":source.copied_decision_bits,"words":words});
            display["page"] = json!({"first":source.first_word,"next":source.next_word,"total":source.total_completed_words,
                "previous":(source.first_word>0).then(||source.first_word.saturating_sub(maximum_words))});
            display["table"] = json!({"headings":["Word","LDPC family","Iterations","Converged","Parity","Shortened zeros","Fill","Source start bit"],"rows":rows});
            display["chart"] = json!({"points":bars,"maximum":maximum.to_string()});
        }
        FecRequest::SoftCells {
            first_index,
            maximum_cells,
            maximum_llrs,
        } => {
            return super::soft::cells(report, display, first_index, maximum_cells, maximum_llrs);
        }
        FecRequest::SoftWord {
            retained_index,
            maximum_llrs,
            first_bit,
            maximum_bits,
        } => {
            return super::soft::word(
                report,
                display,
                retained_index,
                maximum_llrs,
                first_bit,
                maximum_bits,
            );
        }
    }
    Ok(display)
}

fn choices(report: &BatchReport) -> Result<Value, String> {
    let soft = report.soft_observations();
    Ok(
        json!({"modes":[{"value":"words","label":"Completed LDPC words"},
        {"value":"soft_cells","label":"Raw / derandomized cell LLRs"},
        {"value":"soft_word","label":"Completed LDPC input LLRs"}],
        "grants":report.grants.iter().enumerate().map(|(index,grant)|json!({"value":index,"label":format!("Started grant {index} · original {}",grant.identity.grant)})).collect::<Vec<_>>(),
        "soft_words":soft.words.iter().map(|word|json!({"value":word.retained_index,
            "label":format!("Retained {} · word {} · grant {}",word.retained_index,word.word_ordinal,word.identity.grant)})).collect::<Vec<_>>(),
        "mode_requests":{"words":FecRequest::default(),
            "soft_cells":FecRequest::SoftCells{first_index:0,maximum_cells:MAX_CELL_VIEW_CELLS,maximum_llrs:MAX_SOFT_CELL_PAGE_LLRS},
            "soft_word":FecRequest::SoftWord{retained_index:0,maximum_llrs:MAX_SOFT_WORD_QUERY_LLRS,first_bit:0,maximum_bits:DEFAULT_LLR_VIEW_ROWS}}}),
    )
}

/// Preserves wide original public identity fields as decimal text for QML.
/// # Arguments
/// * `identity` - Original immutable public identity, never application metadata.
/// # Returns
/// Exact lossless run/configuration/grant fields.
/// # Errors
/// Propagates serialization failure without mutating the original value.
pub(super) fn identity_value(identity: &impl Serialize) -> Result<Value, String> {
    let mut value = serde_json::to_value(identity).map_err(|e| e.to_string())?;
    for name in ["run", "generation", "grant"] {
        value[name] = value[name]
            .as_u64()
            .ok_or("Invalid original decoder identity.")?
            .to_string()
            .into();
    }
    Ok(value)
}

fn flag(value: bool) -> &'static str {
    if value {
        "Pass"
    } else {
        "Fail"
    }
}
