/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Curated saved-file admission, original ownership and actual upstream recovery.
use docsis_ofdma::api::ReportJson;
use pnm_input::project::{Project, ProjectDirection};
use serde_json::{json, Value};
use std::path::PathBuf;

// Independent inventory anchors: retain every legacy bundle and both new mode/spacing pairs.
const DOWNSTREAM_BUNDLES: usize = 4;
const UPSTREAM_BUNDLES: usize = 4;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/workbench")
}

fn bundles() -> Vec<Value> {
    let manifest: Value =
        serde_json::from_slice(&std::fs::read(root().join("manifest.json")).unwrap()).unwrap();
    manifest["bundles"].as_array().unwrap().clone()
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
