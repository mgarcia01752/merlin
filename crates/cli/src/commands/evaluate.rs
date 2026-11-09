/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Candidate command adaptation to exact-grid profile evaluation.
use crate::{
    arguments::Arguments,
    json_io::{read_json, write_json},
};
use docsis_ofdm_fec::{
    profile::{
        document::{verify_captured_roles, CandidateDocument},
        evaluation::{evaluate, generate_candidate, EvaluationSettings},
    },
    scenario::document::ScenarioDocument,
};
use std::sync::atomic::AtomicBool;
/// Dispatches one validated command request to the shared scientific library.
/// # Arguments
/// * `arguments` - Paths and indices parsed from the command line.
/// * `cancel` - Cooperative signal token shared with the job.
/// # Returns
/// Unit after exporting the actual Rust result.
/// # Errors
/// Reports input, scientific-validation, cancellation and output failures.
pub(crate) fn run(arguments: Arguments, cancel: &AtomicBool) -> Result<(), String> {
    if arguments.scenario.is_some() {
        return Err("evaluate reads the scenario through --input.".into());
    }
    let scenario: ScenarioDocument = read_json(&arguments.input)?;
    scenario.validate().map_err(|error| error.to_string())?;
    let selected = scenario
        .frames
        .get(arguments.frame)
        .ok_or("Frame index is outside the scenario.")?;
    let frames: Vec<_> = scenario
        .frames
        .iter()
        .filter(|frame| frame.key == selected.key)
        .collect();
    let mut document = if let Some(path) = arguments.candidate {
        let document: CandidateDocument = read_json(&path)?;
        document.validate().map_err(|error| error.to_string())?;
        document
    } else {
        let settings: EvaluationSettings = arguments
            .settings
            .as_deref()
            .map(read_json)
            .transpose()?
            .unwrap_or_default();
        let template = arguments
            .profile
            .map(|index| {
                scenario
                    .profiles
                    .get(index)
                    .ok_or("Profile index is outside the scenario.")
            })
            .transpose()?;
        if template.is_some_and(|profile| {
            profile.capture_time.abs_diff(selected.capture_time) > settings.time_tolerance_seconds
        }) {
            return Err("Profile timestamp exceeds explicit association tolerance.".into());
        }
        let candidate = generate_candidate(&frames, template, &settings, scenario.protocol, cancel)
            .map_err(|error| error.to_string())?;
        CandidateDocument {
            document_kind: "docsis_ofdm_fec_candidate".into(),
            schema_version: 1,
            candidate,
            settings,
            evaluation: None,
        }
    };
    verify_captured_roles(&mut document.candidate, &scenario).map_err(|error| error.to_string())?;
    document.evaluation = Some(
        evaluate(&document.candidate, &frames, &document.settings, cancel)
            .map_err(|error| error.to_string())?,
    );
    write_json(&document, &arguments.output)?;
    log::info!("evaluation-complete frames={}", frames.len());
    Ok(())
}
