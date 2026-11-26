/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Accepted edited configuration and exact original provenance remain independent.
use docsis_ofdm_fec::api::ErrorCode;
use docsis_ofdma::api::{ConfigurationDocument, ExperimentOptions, ReportJson, Simulation, Tap};
use pnm_input::project::Project;
use serde_json::{json, Value};

const SOURCE_50: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const SOURCE_25: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");

// Independent accepted edits; wide identity is beyond contiguous IEEE-754 integers.
const EDITED_SOURCE_SEED: u16 = 2;
const EDITED_GRANT_ID: u64 = 9_007_199_254_740_993;
const EDITED_PRESET_ID: &str = "accepted-edited-selection";
const EDITED_DESCRIPTION: &str = "Edited typed inventory, preserving original source bytes.";

// Explicit saved run overrides, independent of the original document's budgets.
const SAVED_GRANTS: usize = 2;
const SAVED_CHUNK_SAMPLES: usize = 313;
const SAVED_RETAINED_POINTS: usize = 29;

fn settings() -> ExperimentOptions {
    ExperimentOptions {
        grants: SAVED_GRANTS,
        chunk_samples: SAVED_CHUNK_SAMPLES,
        retained_points: SAVED_RETAINED_POINTS,
        retained_taps: vec![Tap::TxOutput],
        ..Default::default()
    }
}

fn envelope(document: &str, original: Option<&str>) -> Value {
    let mut input = json!({"kind":"configuration","document":document});
    if let Some(original) = original {
        input["original_document"] = json!(original);
    }

    // Independent schema-3 anchor: this version introduced direction projects.
    json!({"schema_version":3,"application":"MERLIN","direction":"upstream",
        "contents":{"input":input,"options":settings(),
        "receiver_observations":{"maximum_frames":0,"maximum_scalar_slots":0,"soft":null}}})
}

fn report_value(report: &impl ReportJson) -> Value {
    serde_json::from_str(&report.to_json().unwrap()).unwrap()
}

#[test]
fn edited_public_fields_runtime_overrides_and_exact_originals_survive_both_mode_runs() {
    let mut evidence = Vec::new();
    for (name, source) in [("50khz", SOURCE_50), ("25khz", SOURCE_25)] {
        let mut original = b" \n".to_vec();
        original.extend_from_slice(source);
        original.extend_from_slice(b"\r\n ");
        let original_text = std::str::from_utf8(&original).unwrap();
        let mut accepted = ConfigurationDocument::from_json(&original).unwrap();
        let mut selected = accepted.presets[0].clone();
        selected.id = EDITED_PRESET_ID.into();
        selected.description = EDITED_DESCRIPTION.into();
        accepted.presets.push(selected);
        accepted.selected_preset = EDITED_PRESET_ID.into();
        accepted.simulation.source_seed = EDITED_SOURCE_SEED;
        accepted.simulation.grant_id = EDITED_GRANT_ID;
        let current = accepted.to_json().unwrap();
        let wire = envelope(&current, Some(original_text));
        let bytes = serde_json::to_vec(&wire).unwrap();
        let project = Project::from_bytes(&bytes).unwrap();
        let exported: Value = serde_json::from_str(&project.to_json().unwrap()).unwrap();
        assert_eq!(exported["contents"]["input"], wire["contents"]["input"]);
        let info = Project::collection_info(&[&project]).unwrap();
        assert_eq!(info.original_input_bytes, original.len());
        let mut actual = project.upstream_simulation().unwrap();
        assert_eq!(actual.document().unwrap(), &accepted);
        assert_eq!(
            actual.document().unwrap().original_bytes(),
            Some(original.as_slice())
        );
        assert_eq!(
            actual.document().unwrap().original_sha256(),
            accepted.original_sha256()
        );
        assert_eq!(actual.options(), &settings());
        assert_eq!(actual.document().unwrap().to_json().unwrap(), current);
        drop(project);
        drop(bytes);
        let mut reference = Simulation::new(Default::default()).unwrap();
        reference.configure_document(accepted).unwrap();
        reference.set_options(settings()).unwrap();
        let actual = report_value(&actual.run().unwrap());
        let reference = report_value(&reference.run().unwrap());
        for key in [
            "configuration",
            "options",
            "planned",
            "status",
            "generated_source_bits",
            "compared_source_bits",
            "bit_errors",
            "ber",
            "reference_recovered",
            "not_started_grants",
            "source_configuration_json",
            "source_configuration_original",
            "source_configuration_sha256",
            "measurement_provenance_json",
        ] {
            assert_eq!(actual[key], reference[key], "{name}: {key}");
        }
        let words = |report: &Value| {
            report["grants"]
                .as_array()
                .unwrap()
                .iter()
                .map(|grant| grant["words"].clone())
                .collect::<Vec<_>>()
        };
        assert_eq!(words(&actual), words(&reference));
        assert_eq!(actual["reference_recovered"], true);
        assert_eq!(actual["bit_errors"], 0);
        evidence.push(
            json!({"name":name,"project":serde_json::to_string(&wire).unwrap(),
            "report":reference,"original_input_bytes":original.len()}),
        );
    }

    // Explicit ignored-local destination for a separate installed-wheel process.
    if let Some(destination) = std::env::var_os("MERLIN_PROJECT_EDITS_RUST_REFERENCE_JSON") {
        std::fs::write(
            destination,
            serde_json::to_vec(&json!({"cases":evidence})).unwrap(),
        )
        .unwrap();
    }
}

#[test]
fn absent_original_retains_old_wire_shape_and_admission_behavior() {
    let text = std::str::from_utf8(SOURCE_50).unwrap();
    let wire = envelope(text, None);
    let bytes = serde_json::to_vec(&wire).unwrap();
    let project = Project::from_bytes(&bytes).unwrap();
    let exported: Value = serde_json::from_str(&project.to_json().unwrap()).unwrap();
    assert_eq!(exported["contents"]["input"], wire["contents"]["input"]);
    assert!(exported["contents"]["input"]
        .get("original_document")
        .is_none());
    let simulation = project.upstream_simulation().unwrap();
    assert_eq!(
        simulation.document().unwrap().original_bytes(),
        Some(SOURCE_50)
    );
    assert_eq!(
        Project::collection_info(&[&project])
            .unwrap()
            .original_input_bytes,
        SOURCE_50.len()
    );
}

#[test]
fn invalid_original_current_or_wrong_family_reject_without_changing_accepted_project() {
    let text = std::str::from_utf8(SOURCE_50).unwrap();
    let accepted =
        Project::from_bytes(&serde_json::to_vec(&envelope(text, None)).unwrap()).unwrap();
    let before = accepted.to_json().unwrap();
    for original in [
        json!("{"),
        json!("{\"format\":\"unknown\"}"),
        json!(42),
        json!([]),
    ] {
        let mut wire = envelope(text, None);
        wire["contents"]["input"]["original_document"] = original;
        assert_eq!(
            Project::from_bytes(&serde_json::to_vec(&wire).unwrap())
                .err()
                .unwrap()
                .code(),
            ErrorCode::InvalidInput
        );
    }
    let wire = envelope("{\"format\":\"unknown\"}", Some(text));
    assert_eq!(
        Project::from_bytes(&serde_json::to_vec(&wire).unwrap())
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    let mut wire = envelope(text, Some(text));
    wire["contents"]["input"]["kind"] = json!("measured");
    assert_eq!(
        Project::from_bytes(&serde_json::to_vec(&wire).unwrap())
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    assert_eq!(accepted.to_json().unwrap(), before);
    assert!(accepted.upstream_simulation().unwrap().preview().is_ok());
}

#[test]
fn original_configuration_keeps_the_existing_independent_document_byte_ceiling() {
    use docsis_ofdma::common::constants::MAX_CONFIGURATION_DOCUMENT_BYTES;
    let text = std::str::from_utf8(SOURCE_50).unwrap();
    let original = " ".repeat(MAX_CONFIGURATION_DOCUMENT_BYTES + 1);
    let wire = envelope(text, Some(&original));
    assert_eq!(
        Project::from_bytes(&serde_json::to_vec(&wire).unwrap())
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
}
