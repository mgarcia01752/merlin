/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Directional CLI adaptation over the accepted upstream session and PNM owners.
use crate::{arguments::Arguments, json_io};
use merlin::{
    common::RunControl,
    upstream::{ExperimentOptions, Simulation},
};
use pnm_input::upstream::{MeasuredExperiment, PreEqCapture, RxmerCapture, UcdCapture};

/// Admits a bounded local upstream input and exports the actual Rust result.
///
/// # Arguments
/// * `arguments` - Selected operation, original input and new output destination.
/// * `cancel` - Existing process signal request; execution observes safe boundaries.
///
/// # Returns
/// Unit after inspection/preview or complete nonempty reference recovery.
/// Configuration/inspection never starts a worker or consumes source bits.
///
/// # Errors
/// Rejects malformed/unsupported input, byte limits, I/O and occupied outputs.
/// Incomplete/failed/cancelled execution retains its report before returning an error.
pub(crate) fn run(arguments: Arguments, cancel: &RunControl) -> Result<(), String> {
    let bytes = json_io::read_bytes(&arguments.input)?;
    let operation = arguments
        .upstream_operation
        .as_deref()
        .ok_or("Missing upstream operation.")?;
    let capture = match operation {
        "ucd" => Some(
            UcdCapture::from_mac_management(&bytes)
                .and_then(|capture| capture.to_json())
                .map_err(|error| error.to_string())?,
        ),
        "rxmer" => Some(
            RxmerCapture::from_pnm(&bytes)
                .and_then(|capture| capture.to_json())
                .map_err(|error| error.to_string())?,
        ),
        "pre-eq" => Some(
            PreEqCapture::from_pnm(&bytes)
                .and_then(|capture| capture.to_json())
                .map_err(|error| error.to_string())?,
        ),
        _ => None,
    };
    if let Some(document) = capture {
        let document: serde_json::Value =
            serde_json::from_str(&document).map_err(|error| error.to_string())?;
        return json_io::write_json(&document, &arguments.output);
    }
    let mut simulation =
        Simulation::new(ExperimentOptions::default()).map_err(|error| error.to_string())?;
    if operation == "measured" {
        let prepared = MeasuredExperiment::from_json(&bytes)
            .and_then(|plan| plan.admit())
            .map_err(|error| error.to_string())?;
        simulation
            .configure_measured(prepared)
            .map_err(|error| error.to_string())?;
    } else {
        simulation
            .configure_json(&bytes)
            .map_err(|error| error.to_string())?;
    }
    if operation == "preview" {
        let preview = simulation.preview().map_err(|error| error.to_string())?;
        return json_io::write_json(&preview, &arguments.output);
    }
    let control = merlin::upstream::RunControl::with_cancellation(cancel.clone());
    let report = simulation
        .run_with_control(&control)
        .map_err(|error| error.to_string())?;
    json_io::write_json(&report, &arguments.output)?;
    if !report.reference_recovered {
        return Err("Upstream source recovery incomplete; inspect the retained report.".into());
    }
    Ok(())
}
