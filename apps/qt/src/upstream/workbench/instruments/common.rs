/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared finite display envelopes and lossless original-identity transport.
use merlin::upstream::{BatchReport, PhyError, QamPoint, Tap};
use serde_json::{json, Value};

// Actual public-operation support, not a profile catalog or invented tap inventory.
/// Tests the public cell-query support set without inventing retained taps.
/// # Arguments
/// * `tap` - Actual selected public stage.
/// # Returns
/// Whether this operation can contain cell observations.
pub(super) fn cell_tap(tap: Tap) -> bool {
    matches!(
        tap,
        Tap::TxGrid | Tap::TxPreEq | Tap::RxFft | Tap::RxEqualized
    )
}

/// Tests the public PSD support set while keeping grid values distinct from time.
/// # Arguments
/// * `tap` - Actual selected public stage.
/// # Returns
/// Whether this stage is accepted by the existing spectrum operation.
pub(super) fn time_tap(tap: Tap) -> bool {
    matches!(
        tap,
        Tap::TxUseful | Tap::TxOutput | Tap::ChannelInput | Tap::ChannelOutput | Tap::RxInput
    )
}

/// Creates empty plot transport without fabricating unavailable samples.
/// # Returns
/// The finite renderer fields with no points or ticks.
pub(super) fn empty_viewport() -> Value {
    json!({"points":[],"x_ticks":[],"y_ticks":[],"axis_label":"",
        "frequency_label":"","start":0,"count":0,"total":0})
}

/// Creates a finite instrument envelope before projecting an existing query result.
/// # Arguments
/// * `kind`, `choices`, `selection` - Compiled destination and actual source/request metadata.
/// # Returns
/// An owned candidate; the caller must admit it before retaining or publishing it.
pub(super) fn envelope(kind: &str, choices: Value, selection: Value) -> Value {
    json!({"kind":kind,"status":"ready","message":"","choices":choices,
        "selection":selection,"metadata":{},"viewport":empty_viewport(),
        "scatter":{"points":[],"references":[]}})
}

/// Preserves unavailable science separately from malformed or over-budget requests.
/// # Arguments
/// * `display` - Unadmitted candidate without invented observations.
/// * `error` - Original typed public failure.
/// # Returns
/// An explicit unavailable envelope for unsupported history/planes.
/// # Errors
/// Other public failure categories remain rejected operations.
pub(super) fn unavailable(mut display: Value, error: PhyError) -> Result<Value, String> {
    match error {
        PhyError::Unsupported { .. } => {
            display["status"] = "unavailable".into();
            display["message"] = error.to_string().into();
            Ok(display)
        }
        _ => Err(error.to_string()),
    }
}

/// Reads actual retained configuration identities without copying the configuration.
/// # Arguments
/// * `report` - Existing immutable complete/partial scientific owner.
/// # Returns
/// Finite scalars with wide identities in exact decimal text.
pub(super) fn report_metadata(report: &BatchReport) -> Value {
    let grant = report.configuration.us_configuration.grant();
    json!({"report_status":report.status,"configured_grant_id":grant.id.to_string(),
        "configured_first_frame":grant.first_frame.to_string(),
        "configured_epoch_sample":grant.epoch_sample.to_string(),
        "note":"Retained actual observations; incomplete or absent history is not reconstructed."})
}

/// Projects every original point field while keeping wide identities lossless in QML.
/// # Arguments
/// * `point` - Actual public observation, never an inferred reference sample.
/// # Returns
/// Independent bounded detail without a source/report alias.
/// # Errors
/// Propagates serde transport failure without mutating the point.
pub(super) fn point_detail(point: &QamPoint) -> Result<Value, String> {
    let mut value = serde_json::to_value(point).map_err(|error| error.to_string())?;
    value["identity"]["run"] = point.identity.run.to_string().into();
    value["identity"]["generation"] = point.identity.generation.to_string().into();
    value["identity"]["grant"] = point.identity.grant.to_string().into();
    value["frame"] = point.frame.to_string().into();
    Ok(value)
}
