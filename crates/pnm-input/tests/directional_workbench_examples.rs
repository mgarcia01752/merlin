/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Curated saved-file admission, original ownership and actual upstream recovery.
use docsis_ofdm_fec::api::FullLoopControl;
use docsis_ofdma::api::{ReportJson, Tap};
use pnm_input::project::{Project, ProjectDirection};
use serde_json::{json, Value};
use std::path::PathBuf;

// Independent inventory anchors: retain every legacy bundle and both new mode/spacing pairs.
const DOWNSTREAM_BUNDLES: usize = 4;
const UPSTREAM_BUNDLES: usize = 4;
// Independent delivered-file inventory, including twelve downstream companions.
const DOWNSTREAM_PROJECTS: usize = 16;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/workbench")
}

fn bundles() -> Vec<Value> {
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(root().join("manifest.json")).unwrap()).unwrap();
    manifest["bundles"].as_array().unwrap().clone()
}

fn workbenches(bundle: &Value) -> Vec<&str> {
    let mut paths = vec![bundle["workbench"].as_str().unwrap()];
    if let Some(companions) = bundle["companions"].as_array() {
        paths.extend(companions.iter().map(|path| path.as_str().unwrap()));
    }
    paths
}

fn saved_files(directory: &std::path::Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_type().unwrap().is_dir() {
            paths.extend(saved_files(&path));
        } else if path.to_string_lossy().ends_with(".merlin.json") {
            paths.push(path);
        }
    }
    paths
}

#[test]
fn manifest_includes_every_delivered_saved_workbench_without_duplicates() {
    let root = root();
    let mut actual = saved_files(&root);
    let mut advertised: Vec<_> = bundles()
        .iter()
        .flat_map(workbenches)
        .map(|relative| root.join(relative))
        .collect();
    actual.sort();
    advertised.sort();
    assert_eq!(advertised.len(), DOWNSTREAM_PROJECTS + UPSTREAM_BUNDLES);
    assert_eq!(advertised, actual);
}

#[test]
fn complete_directional_inventory_loads_idle_and_preserves_original_documents() {
    let mut downstream = 0;
    let mut upstream = 0;
    for bundle in bundles() {
        let path = root().join(bundle["workbench"].as_str().unwrap());
        let original = std::fs::read(&path).unwrap();
        let project = Project::from_file(&path).unwrap();
        assert_eq!(project.original_document(), original);
        let restored = Project::from_bytes(project.to_json().unwrap().as_bytes()).unwrap();
        match project.direction() {
            ProjectDirection::Downstream => {
                assert_eq!(bundle["direction"], "downstream");
                assert!(restored.downstream_plan().is_ok());
                downstream += 1;
            }
            ProjectDirection::Upstream => {
                assert_eq!(bundle["direction"], "upstream");
                assert!(restored.upstream_simulation().is_ok());
                let saved: Value = serde_json::from_slice(&original).unwrap();
                let companion =
                    std::fs::read(root().join(bundle["source_document"].as_str().unwrap()))
                        .unwrap();
                assert_eq!(
                    saved["contents"]["input"]["document"]
                        .as_str()
                        .unwrap()
                        .as_bytes(),
                    companion
                );
                upstream += 1;
            }
        }
        assert_eq!(project.original_document(), original);
    }
    assert_eq!(downstream, DOWNSTREAM_BUNDLES);
    assert_eq!(upstream, UPSTREAM_BUNDLES);
}

#[test]
fn every_upstream_saved_example_recovers_source_and_exports_native_parity() {
    let mut cases = Vec::new();
    for bundle in bundles()
        .into_iter()
        .filter(|bundle| bundle["direction"] == "upstream")
    {
        let relative = bundle["workbench"].as_str().unwrap();
        let original = std::fs::read(root().join(relative)).unwrap();
        let saved: Value = serde_json::from_slice(&original).unwrap();
        let project = Project::from_file(root().join(relative)).unwrap();
        let restored = Project::from_bytes(project.to_json().unwrap().as_bytes()).unwrap();
        let result = restored.upstream_simulation().unwrap().run().unwrap();
        let report: Value = serde_json::from_str(&result.to_json().unwrap()).unwrap();
        assert_eq!(report["reference_recovered"], true, "{relative}");
        assert_eq!(report["bit_errors"], 0, "{relative}");
        assert!(report["compared_source_bits"].as_u64().unwrap() > 0);
        assert!(report["not_started_grants"].as_array().unwrap().is_empty());
        assert_eq!(report["options"], saved["contents"]["options"]);
        let mode = result
            .configuration
            .us_configuration
            .waveform()
            .numerology();
        assert_eq!(
            result.options.retained_points,
            result.options.retained_taps.len() * mode.fft_len()
        );
        for tap in [Tap::TxOutput, Tap::RxInput] {
            assert_eq!(
                result.spectrum(tap, mode).unwrap().fft_len,
                mode.fft_len(),
                "{relative}: {tap:?} must have a full actual FFT window"
            );
        }
        for tap in [Tap::TxGrid, Tap::TxPreEq, Tap::RxFft, Tap::RxEqualized] {
            assert!(
                !result
                    .qam(tap, None, docsis_ofdma::instruments::MAX_QAM_QUERY_POINTS)
                    .unwrap()
                    .points
                    .is_empty(),
                "{relative}: {tap:?}"
            );
        }
        let source = saved["contents"]["input"]["document"].as_str().unwrap();
        let provenance: Value = if bundle["mode"] == "measured" {
            serde_json::from_str(report["measurement_provenance_json"].as_str().unwrap()).unwrap()
        } else {
            json!({"original_plan_bytes":report["source_configuration_original"]})
        };
        assert_eq!(provenance["original_plan_bytes"], json!(source.as_bytes()));
        assert_eq!(project.original_document(), original);
        cases.push(json!({"workbench":relative,"report":report,
            "receiver_observations":result.receiver_observations()}));
    }
    assert_eq!(cases.len(), UPSTREAM_BUNDLES);

    // The acceptance runner selects a new ignored file for a separate native process.
    if let Some(path) = std::env::var_os("MERLIN_WORKBENCH_EXAMPLES_RUST_REFERENCE_JSON") {
        std::fs::write(path, serde_json::to_vec(&json!({"cases":cases})).unwrap()).unwrap();
    }
}

#[test]
#[ignore = "exhaustive saved-scenario acceptance; run explicitly with --release --ignored"]
fn every_downstream_saved_example_executes_each_selected_profile() {
    let mut cases = Vec::new();
    for bundle in bundles()
        .into_iter()
        .filter(|bundle| bundle["direction"] == "downstream")
    {
        for relative in workbenches(&bundle) {
            let original = std::fs::read(root().join(relative)).unwrap();
            let project = Project::from_file(root().join(relative)).unwrap();
            let plan = project.downstream_plan().unwrap();
            let info = plan.info();
            assert!(!info.selected_profile_ids.is_empty(), "{relative}");
            for (index, profile) in info.selected_profile_ids.iter().enumerate() {
                let request = plan.request(index).unwrap();
                let result = request.run(&FullLoopControl::new()).unwrap();
                let report = result.report();
                let record = json!({"workbench":relative,"profile":profile,"status":report.status,
                "summary":report.summary,"error":report.error,
                "unrecovered_generated_payloads":report.unrecovered_generated_payloads});
                println!("{}", serde_json::to_string(&record).unwrap());
                assert_eq!(record["status"], "complete", "{relative}: {record}");
                assert!(report.error.is_none(), "{relative}: {record}");
                assert!(
                    report.summary.generated_payloads > 0,
                    "{relative}: {record}"
                );
                assert!(!report.retained_words.is_empty(), "{relative}: {record}");
                cases.push(record);
            }
            assert_eq!(project.original_document(), original);
        }
    }
    assert!(cases.len() >= DOWNSTREAM_PROJECTS);
    if let Some(path) = std::env::var_os("MERLIN_DOWNSTREAM_EXAMPLES_RUST_REFERENCE_JSON") {
        std::fs::write(path, serde_json::to_vec(&json!({"cases":cases})).unwrap()).unwrap();
    }
}
