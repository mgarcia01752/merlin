/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite tables of actual soft observations, never reconstructed or posterior metrics.
use super::{
    common,
    fec::{identity_value, MAX_CELL_VIEW_CELLS, MAX_LLR_VIEW_ROWS},
};
use merlin::upstream::{BatchReport, SoftCellRequest, SoftWordRequest};
use serde_json::{json, Value};

/// Copies only the finite actual inventory and preserves wide frame identities.
/// # Arguments
/// * `report` - Immutable actual report, including independently disabled retention.
/// # Returns
/// Original summary; no demapper, randomizer, decoder or source executes.
/// # Errors
/// Propagates serialization or invalid identity transport without replacing data.
pub(super) fn summary(report: &BatchReport) -> Result<Value, String> {
    let source = report.soft_observations();
    let mut value = serde_json::to_value(&source).map_err(|e| e.to_string())?;
    for (original, word) in source.words.iter().zip(
        value["words"]
            .as_array_mut()
            .ok_or("Invalid soft inventory.")?,
    ) {
        word["identity"] = identity_value(&original.identity)?;
        word["completion_frame"] = original.completion_frame.to_string().into();
    }
    Ok(value)
}

/// Reads original cell metrics before and after the actual PHY mask inversion.
/// # Arguments
/// * `report`, `display` - Existing report and unadmitted finite display candidate.
/// * `first_index`, `maximum_cells`, `maximum_llrs` - Original public copy request.
/// # Returns
/// Original bounded cells plus paired scalar rows, without sign/scale changes.
/// # Errors
/// Public failures, native table limits or nonfinite original scalar transport.
pub(super) fn cells(
    report: &BatchReport,
    mut display: Value,
    first_index: usize,
    maximum_cells: usize,
    maximum_llrs: usize,
) -> Result<Value, String> {
    if maximum_cells == 0 || maximum_cells > MAX_CELL_VIEW_CELLS {
        return Err("Choose a positive cell count within the native table limit.".into());
    }
    let source = match report.soft_cells(SoftCellRequest {
        first_index,
        maximum_cells,
        maximum_llrs,
    }) {
        Ok(value) => value,
        Err(error) => return common::unavailable(display, error),
    };
    let mut data = serde_json::to_value(&source).map_err(|e| e.to_string())?;
    let mut rows = Vec::new();
    for (cell, value) in source
        .cells
        .iter()
        .zip(data["cells"].as_array_mut().ok_or("Invalid soft cells.")?)
    {
        value["identity"] = identity_value(&cell.identity)?;
        value["frame"] = cell.frame.to_string().into();
        if cell.demapper_llrs.len() != cell.descrambled_llrs.len() {
            return Err("Original raw/derandomized soft lengths differ.".into());
        }
        for (bit, (raw, derandomized)) in cell
            .demapper_llrs
            .iter()
            .zip(&cell.descrambled_llrs)
            .enumerate()
        {
            if rows.len() >= MAX_LLR_VIEW_ROWS {
                return Err("Original soft cells exceed the finite native scalar table.".into());
            }
            rows.push(vec![
                cell.retained_index.to_string(),
                cell.frame.to_string(),
                cell.symbol.to_string(),
                cell.physical_carrier.to_string(),
                bit.to_string(),
                scalar(*raw)?,
                scalar(*derandomized)?,
            ]);
        }
    }
    display["metadata"]["reference_plane"] = source.reference_plane.into();
    display["metadata"]["llr_convention"] = source.llr_convention.into();
    display["metadata"]["bit_order"] = source.bit_order.into();
    display["metadata"]["demapper_llr_limit"] = source.demapper_llr_limit.into();
    display["metadata"]["note"]="Computed cells can exist before any completed word. Raw demapper and actual derandomized input remain separate; positive favors zero.".into();
    display["page"] = json!({"first":source.first_index,"next":source.next_index,"total":source.retained_cells,
            "previous":(source.first_index>0).then(||source.first_index.saturating_sub(maximum_cells))});
    display["table"] = json!({"headings":["Retained cell","Frame","Symbol","Physical carrier","MSB-first bit","Raw LLR","Derandomized LLR"],"rows":rows});
    display["data"] = data;
    Ok(display)
}

/// Reads an actual complete LDPC input and retains only a finite original-bit window.
/// # Arguments
/// * `report`, `display` - Original report and unadmitted candidate.
/// * `retained_index`, `maximum_llrs` - Public retained-word selection/copy budget.
/// * `first_bit`, `maximum_bits` - Native original-bit display viewport.
/// # Returns
/// Exact input values and original word metadata; no priors or posterior claims.
/// # Errors
/// Public failures, invalid table extents or nonfinite original metrics.
pub(super) fn word(
    report: &BatchReport,
    mut display: Value,
    retained_index: usize,
    maximum_llrs: usize,
    first_bit: usize,
    maximum_bits: usize,
) -> Result<Value, String> {
    if maximum_bits == 0 || maximum_bits > MAX_LLR_VIEW_ROWS {
        return Err("Choose a positive LDPC-input window within the native table limit.".into());
    }
    let source = match report.soft_word(SoftWordRequest {
        retained_index,
        maximum_llrs,
    }) {
        Ok(value) => value,
        Err(error) => return common::unavailable(display, error),
    };
    let remaining = source
        .input_llrs
        .len()
        .checked_sub(first_bit)
        .ok_or("Invalid original LDPC input start.")?;
    let count = remaining.min(maximum_bits);
    let end = first_bit + count;
    let mut rows = Vec::with_capacity(count);
    for (offset, value) in source.input_llrs[first_bit..end].iter().enumerate() {
        rows.push(vec![(first_bit + offset).to_string(), scalar(*value)?]);
    }
    let next = (end < source.input_llrs.len()).then_some(end);
    display["metadata"]["reference_plane"]="Exact transmitted-position input of an actually completed LDPC call; excludes restored shortened priors and is not posterior LLRs.".into();
    display["metadata"]["llr_convention"] = source.llr_convention.into();
    display["metadata"]["reference_variance"] = source.reference_variance.into();
    display["metadata"]["code_family"] = source.code_family.into();
    display["metadata"]["word"] = json!({"retained_index":source.word.retained_index,
        "identity":identity_value(&source.word.identity)?,"completion_frame":source.word.completion_frame.to_string(),
        "word_ordinal":source.word.word_ordinal,"coded_offset":source.word.coded_offset,
        "mother_information_bits":source.mother_information_bits,"mother_codeword_bits":source.mother_codeword_bits,
        "retained_information_bits":source.retained_information_bits,"transmitted_bits":source.transmitted_bits,
        "shortened_bits":source.shortened_bits,"iterations":source.iterations,"converged":source.converged,
        "parity_valid":source.parity_valid,"shortened_zeros_valid":source.shortened_zeros_valid,
        "information_fill_valid":source.information_fill_valid,"hard_decision_changes":source.hard_decision_changes});
    display["page"] = json!({"first":first_bit,"next":next,"total":source.input_llrs.len(),
        "previous":(first_bit>0).then(||first_bit.saturating_sub(maximum_bits))});
    display["data"] = json!({"retained_index":retained_index,"first_bit":first_bit,"input_llrs":&source.input_llrs[first_bit..end]});
    display["table"] =
        json!({"headings":["Original transmitted bit","Actual LDPC input LLR"],"rows":rows});
    Ok(display)
}

fn scalar(value: f64) -> Result<String, String> {
    if !value.is_finite() {
        return Err("Nonfinite original soft observation.".into());
    }
    Ok(value.to_string())
}
