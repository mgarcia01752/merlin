/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Simulation command adaptation to the public Rust engine.
use crate::{
    arguments::Arguments,
    json_io::{read_json, write_json},
};
use docsis_ofdm_fec::{
    profile::document::CandidateDocument,
    scenario::document::ScenarioDocument,
    simulation::{simulate, MeasuredNoise, SimulationSettings},
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
    if arguments.settings.is_some() || arguments.profile.is_some() {
        return Err("--settings and --profile apply to evaluate only.".into());
    }
    let settings: SimulationSettings = read_json(&arguments.input)?;
    let scenario: Option<ScenarioDocument> =
        arguments.scenario.as_deref().map(read_json).transpose()?;
    let candidate: Option<CandidateDocument> =
        arguments.candidate.as_deref().map(read_json).transpose()?;
    if scenario.is_some() != candidate.is_some() {
        return Err("Measured simulation requires both --scenario and --candidate.".into());
    }
    if let Some(scenario) = &scenario {
        scenario.validate().map_err(|error| error.to_string())?;
    }
    if let Some(candidate) = &candidate {
        candidate.validate().map_err(|error| error.to_string())?;
    }
    let measured = match (&scenario, &candidate) {
        (Some(scenario), Some(candidate)) => {
            let frame = scenario
                .frames
                .get(arguments.frame)
                .ok_or("Frame index is outside the scenario.")?;
            Some(MeasuredNoise {
                frame,
                candidate: &candidate.candidate,
                first: 0,
                count: frame.mer_db.len(),
            })
        }
        _ => None,
    };
    let result = simulate(&settings, measured.as_ref(), cancel, |progress| {
        log::debug!(
            "simulation-progress completed-codewords={} requested-codewords={} elapsed-ms={}",
            progress.completed_codewords,
            progress.requested_codewords,
            progress.elapsed_ms
        )
    })
    .map_err(|error| error.to_string())?;
    write_json(&result, &arguments.output)?;
    log::info!("simulation-complete status={} completed-codewords={} requested-codewords={} ldpc-failures={} bch-failures={}",
        result.status, result.completed_codewords, settings.codewords,
        result.ldpc_detected_failures, result.bch_detected_failures);
    Ok(())
}
