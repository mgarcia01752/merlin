/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! UI Designer recipe workbench admission and actual engine equivalence.
use base64::{engine::general_purpose::STANDARD, Engine};
use docsis_ofdm_fec::{api::*, signal_processing::channel::impairments::NoiseModel};
use pnm_input::Workbench;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
const WB: &[u8] =
    include_bytes!("../../../examples/workbench/cl-192mhz-8mhz-notch/cmce.merlin.json");
const RECIPE: &[u8] = include_bytes!("fixtures/designer-flat.json");
fn document() -> Value {
    let mut value: Value = serde_json::from_slice(WB).unwrap();
    value["sections"]["measurements"]["rxmer"] = json!([{
        "name":"UI Designer fixture", "byte_length":RECIPE.len(),
        "sha256":format!("{:x}",Sha256::digest(RECIPE)), "base64":STANDARD.encode(RECIPE)
    }]);
    value["receiver"]["duration_ms"] = json!(1);
    value["receiver"]["receiver_mode"] = json!("merlin_estimated");
    value
}
#[test]
fn designer_load_is_idle_and_cancel_preserves_not_started() {
    let bytes = serde_json::to_vec(&document()).unwrap();
    let mut wb = Workbench::from_bytes(&bytes).unwrap();
    assert_eq!(wb.original_document().as_ref(), bytes);
    assert_eq!(wb.controllers().len(), 1);
    assert_eq!(
        wb.controllers()[0].options().receiver_mode,
        ReceiverMode::MerlinChannelEstimated
    );
    let control = RunControl::new();
    control.cancel();
    let reports = wb.run_with_control(&control).unwrap();
    assert!(reports[0].iter().all(|entry| entry.report().is_none()));
}
#[test]
fn designer_run_matches_materialized_capture_and_exports_python_parity() {
    let started = std::time::Instant::now();
    let mut doc = document();
    let bytes = serde_json::to_vec(&doc).unwrap();
    let mut wb = Workbench::from_bytes(&bytes).unwrap();
    let recipe: Value = serde_json::from_slice(RECIPE).unwrap();
    let key = &recipe["baseline"]["key"];
    let grid = &key["grid"];
    let recipe_typed: docsis_ofdm_fec::signal_processing::rxmer_designer::RxmerRecipe =
        serde_json::from_value(recipe["recipe"].clone()).unwrap();
    let grid_typed = serde_json::from_value(grid.clone()).unwrap();
    let mask: Vec<bool> = serde_json::from_value(recipe["baseline"]["valid"].clone()).unwrap();
    let baseline: Vec<_> = mask.iter().map(|v| v.then_some(48.0)).collect();
    let samples = recipe_typed.apply(&grid_typed, &baseline, 1, 1).unwrap();
    let decoded = json!({"pnm_header":{"file_type":"PNN","file_type_version":4,"major_version":1,"minor_version":0,"capture_time":recipe["baseline"]["anchor_epoch"]},
        "mac_address":"00:ca:00:00:00:01","channel_id":key["channel_id"],
        "subcarrier_zero_frequency":grid["zero_frequency_hz"],"first_active_subcarrier_index":grid["first_subcarrier_index"],
        "subcarrier_spacing":grid["spacing_hz"],"data_length":samples.len(),"value_units":"dB","values":samples});
    let decoded = serde_json::to_vec(&decoded).unwrap();
    doc["sections"]["measurements"]["rxmer"] = json!([{"name":"materialized", "byte_length":decoded.len(),
        "sha256":format!("{:x}",Sha256::digest(&decoded)),"base64":STANDARD.encode(&decoded)}]);
    let mut materialized = Workbench::from_bytes(&serde_json::to_vec(&doc).unwrap()).unwrap();
    let left = wb.controllers()[0].prepare().unwrap();
    let right = materialized.controllers()[0].prepare().unwrap();
    let (NoiseModel::Rxmer { frame: left }, NoiseModel::Rxmer { frame: right }) = (
        &left[0].engine_settings().impairments.noise,
        &right[0].engine_settings().impairments.noise,
    ) else {
        panic!("Both prepared jobs must use the RxMER noise model");
    };
    assert_eq!(
        left.mer_db, right.mer_db,
        "materialized JSON must preserve exact f64 samples"
    );
    let actual = wb.run().unwrap();
    let expected = materialized.run().unwrap();
    let actual: Value = serde_json::from_str(&actual[0].to_json().unwrap()).unwrap();
    let expected: Value = serde_json::from_str(&expected[0].to_json().unwrap()).unwrap();
    assert_eq!(actual["entries"][0]["status"], "Complete");
    for field in ["fec", "all_completed_fec", "profile"] {
        assert_eq!(
            actual["entries"][0]["report"][field], expected["entries"][0]["report"][field],
            "{field}"
        );
    }
    if let Ok(path) = std::env::var("MERLIN_DESIGNER_PARITY_OUT") {
        pnm_input::save_json(path, &serde_json::to_string(&actual).unwrap()).unwrap();
    }
    println!(
        "Designer/materialized real FEC parity: {:.3}s",
        started.elapsed().as_secs_f64()
    );
}
