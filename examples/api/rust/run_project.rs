/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate-process acceptance reference for original-preserving direction projects.
use docsis_ofdm_fec::api::{FullLoopControl, FullLoopObservationInput};
use docsis_ofdma::api::{ExperimentOptions, ReportJson};
use pnm_input::project::Project;
use serde_json::{json, Value};

const DOWNSTREAM: &[u8] = include_bytes!("../../workbench/downstream/single/workbench.merlin.json");
const UPSTREAM: &str = include_str!("../../cli/upstream/us-50khz.json");
const MEASURED: &str = include_str!("../../measurements/upstream/measured/capture-driven.json");
// A finite external chunk split intentionally unrelated to upstream waveform geometry.
const CHUNK_SAMPLES: usize = 257;
// Enough Cartesian scalar storage for either upstream numerology's complete estimate.
const ESTIMATE_SCALAR_SLOTS: usize = 8192;
// Small shared reference history, independent of transport and estimate storage.
const RETAINED_IQ_POINTS: usize = 128;
// Finite computed-cell and completed-word histories; neither is a decoder iteration limit.
const SOFT_CELL_BUDGET: usize = 4;
const SOFT_WORD_BUDGET: usize = 1;
// Explicit reference metric storage policy, independent of cell retention and FFT size.
const WORD_INPUT_LLR_BUDGET: usize = 16200;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut saved: Value = serde_json::from_slice(DOWNSTREAM)?;
    saved["receiver"]["all_profiles"] = json!(false);
    let downstream = Project::from_bytes(&serde_json::to_vec(&json!({
        // File-format schema 3 is an independent acceptance anchor.
        "schema_version":3,"application":"MERLIN","direction":"downstream",
        "contents":{"workbench":serde_json::to_string(&saved)?,
            "observations":FullLoopObservationInput::default()}
    }))?)?;
    let downstream = Project::from_bytes(downstream.to_json()?.as_bytes())?;
    let plan = downstream.downstream_plan()?;
    let request = plan.request(0)?;
    let result = request.run(&FullLoopControl::new())?;
    let mut references = Vec::new();
    for (kind, original) in [("configuration", UPSTREAM), ("measured", MEASURED)] {
        let mut options = serde_json::to_value(ExperimentOptions::default())?;
        options["chunk_samples"] = json!(CHUNK_SAMPLES);
        options["retained_points"] = json!(RETAINED_IQ_POINTS);
        let bytes = serde_json::to_vec(&json!({"schema_version":3,"application":"MERLIN",
            "direction":"upstream","contents":{"input":{"kind":kind,"document":original},
            "options":options,"receiver_observations":{"maximum_frames":1,
            "maximum_scalar_slots":ESTIMATE_SCALAR_SLOTS,
            "soft":{"maximum_cells":SOFT_CELL_BUDGET,"maximum_words":SOFT_WORD_BUDGET,"maximum_word_llrs":WORD_INPUT_LLR_BUDGET}}}}))?;
        let project = Project::from_bytes(&bytes)?;
        let project = Project::from_bytes(project.to_json()?.as_bytes())?;
        let report = project.upstream_simulation()?.run()?;
        references.push(json!({"kind":kind,"project":project.to_json()?,
            "collection":Project::collection_info(&[&downstream,&project])?,
            "report":serde_json::from_str::<Value>(&report.to_json()?)?,
            "receiver_observations":report.receiver_observations()}));
    }
    println!(
        "{}",
        serde_json::to_string(&json!({
            "downstream":{"project":downstream.to_json()?,"plan":plan.info(),
            "settings":request.settings(),"status":result.report().status,
            "summary":result.report().summary,"inputs":result.inputs().summary()?,
            "observation_stats":result.observation_stats()},"upstream":references
        }))?
    );
    Ok(())
}
