/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Lossless Qt transport of public configured-resource metadata, without grid math.
use merlin::upstream::ResourceGridSnapshot;
use serde_json::Value;

/// Converts a typed public snapshot while preserving every unmodified field.
///
/// # Arguments
/// * `source` - Owned public query result; no observation JSON is decoded again.
///
/// # Returns
/// Direct serde projection with the explicit u64 identities/sample positions
/// represented as decimal strings. Bounded spatial indices remain JSON numbers;
/// role strings, missing values, counts and reference planes stay unchanged.
///
/// # Errors
/// A serde serialization failure, without changing the borrowed source.
pub(super) fn snapshot(source: &ResourceGridSnapshot) -> Result<Value, String> {
    let mut value = serde_json::to_value(source).map_err(|error| error.to_string())?;
    value["configuration_generation"] = source.configuration_generation.to_string().into();
    let grid = &source.grid;
    let target = &mut value["grid"];
    target["grant_id"] = grid.grant_id.to_string().into();
    target["grant_epoch_sample"] = grid.grant_epoch_sample.to_string().into();
    target["grant_first_frame"] = grid.grant_first_frame.to_string().into();
    target["absolute_frame"] = grid.absolute_frame.to_string().into();
    target["frame_prefix_sample"] = grid.frame_prefix_sample.to_string().into();
    if let Some(cells) = target["cells"].as_array_mut() {
        for (target, source) in cells.iter_mut().zip(&grid.cells) {
            target["prefix_start_sample"] = source.prefix_start_sample.to_string().into();
            target["useful_start_sample"] = source.useful_start_sample.to_string().into();
        }
    }
    Ok(value)
}
