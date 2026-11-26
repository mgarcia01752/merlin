/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original-preserving project acceptance through the facade's public API.
use docsis_ofdm_fec::api::{FullLoopControl, FullLoopObservationInput};
use docsis_ofdma::api::{ExperimentOptions, ReportJson};
use pnm_input::project::{Project, ProjectDirection};
use serde_json::{json, Value};

const DOWNSTREAM: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/single/workbench.merlin.json");
const UPSTREAM_50: &str = include_str!("../../../examples/cli/upstream/us-50khz.json");
const UPSTREAM_25: &str = include_str!("../../../examples/cli/upstream/us-25khz.json");
const MEASURED_50: &str =
    include_str!("../../../examples/measurements/upstream/measured/capture-driven.json");
const MEASURED_25: &str =
    include_str!("../../../examples/measurements/upstream/measured/capture-driven-25khz.json");
// Deliberately not a waveform multiple: saved transport splitting must remain independent.
const PROJECT_CHUNK_SAMPLES: usize = 257;
// Enough Cartesian slots for one 4096-bin estimate, also valid for the smaller numerology.
const ESTIMATE_SCALAR_SLOTS: usize = 8192;
// Retain a small actual sample history, independent of chunk size and estimate budgets.
const RETAINED_IQ_POINTS: usize = 128;
// Finite computed-cell and completed-word histories; neither is a decoder iteration limit.
const SOFT_CELL_BUDGET: usize = 4;
const SOFT_WORD_BUDGET: usize = 1;
// Explicit reference metric storage policy, independent of cell retention and FFT size.
const WORD_INPUT_LLR_BUDGET: usize = 16200;

fn downstream() -> Value {
    let mut value: Value = serde_json::from_slice(DOWNSTREAM).unwrap();
    value["receiver"]["all_profiles"] = json!(false);
    value
}
fn envelope(direction: &str, contents: Value) -> Value {
    // Independent format-version anchor: schema 3 introduces the direction tag.
    json!({"schema_version":3,"application":"MERLIN","direction":direction,"contents":contents})
}
fn upstream(kind: &str, document: &str) -> Value {
    let mut options = serde_json::to_value(ExperimentOptions::default()).unwrap();
    options["chunk_samples"] = json!(PROJECT_CHUNK_SAMPLES);
    options["retained_points"] = json!(RETAINED_IQ_POINTS);
    envelope(
        "upstream",
        json!({"input":{"kind":kind,"document":document},"options":options,
        "receiver_observations":{"maximum_frames":1,"maximum_scalar_slots":ESTIMATE_SCALAR_SLOTS,
        "soft":{"maximum_cells":SOFT_CELL_BUDGET,"maximum_words":SOFT_WORD_BUDGET,"maximum_word_llrs":WORD_INPUT_LLR_BUDGET}}}),
    )
}
fn admit(value: &Value) -> Project {
    Project::from_bytes(&serde_json::to_vec(value).unwrap()).unwrap()
}

#[test]
fn legacy_and_schema3_keep_exact_originals_and_idle_public_settings() {
    for version in [1, 2] {
        // Both historical downstream file-format versions remain accepted.
        let mut legacy = downstream();
        legacy["schema_version"] = json!(version);
        if version == 2 {
            legacy["receiver"]["selected_profile_ids"] =
                json!([legacy["sections"]["profile"]["profile_id"]]);
        }
        let original = serde_json::to_vec_pretty(&legacy).unwrap();
        let project = Project::from_bytes(&original).unwrap();
        assert_eq!(project.direction(), ProjectDirection::Downstream);
        assert_eq!(project.original_document(), original);
        let exported = project.to_json().unwrap();
        let wire: Value = serde_json::from_str(&exported).unwrap();
        assert_eq!(wire["schema_version"], 3);
        assert_eq!(
            wire["contents"]["workbench"].as_str().unwrap().as_bytes(),
            original
        );
        let restored = Project::from_bytes(exported.as_bytes()).unwrap();
        let plan = restored.downstream_plan().unwrap();
        let request = plan.request(0).unwrap();
        assert_eq!(
            request.options().noise_seed,
            legacy["receiver"]["noise_seed"].as_u64().unwrap()
        );
        assert_eq!(
            request.options().duration_ms,
            legacy["receiver"]["duration_ms"].as_u64().unwrap()
        );
        let control = FullLoopControl::new();
        control.cancel();
        assert_eq!(
            request
                .run(&control)
                .unwrap()
                .report()
                .summary
                .comparison
                .bits_tested,
            0
        );
        assert_eq!(
            project.upstream_simulation().err().unwrap().code(),
            docsis_ofdm_fec::api::ErrorCode::InvalidInput
        );
    }
}

#[test]
fn schema3_downstream_observations_and_actual_fec_use_the_existing_owner() {
    let saved = serde_json::to_string(&downstream()).unwrap();
    let value = envelope(
        "downstream",
        json!({"workbench":saved,
        "observations":FullLoopObservationInput::default()}),
    );
    let project = admit(&value);
    let plan = project.downstream_plan().unwrap();
    drop(project);
    let request = plan.request(0).unwrap();
    assert!(request.observation_policy().is_some());
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert!(result.report().summary.comparison.bits_tested > 0);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    assert_eq!(result.report().summary.payload_ldpc_failures, 0);
    assert_eq!(result.report().summary.payload_bch_failures, 0);
    assert!(result.observation_stats().retained_events > 0);
}

#[test]
fn both_upstream_numerologies_and_measured_lineage_survive_owned_round_trips() {
    for (kind, original) in [
        ("configuration", UPSTREAM_50),
        ("configuration", UPSTREAM_25),
        ("measured", MEASURED_50),
        ("measured", MEASURED_25),
    ] {
        let project = admit(&upstream(kind, original));
        assert_eq!(project.direction(), ProjectDirection::Upstream);
        assert!(project.downstream_plan().is_err());
        let wire: Value = serde_json::from_str(&project.to_json().unwrap()).unwrap();
        assert_eq!(wire["contents"]["input"]["document"], original);
        let roundtrip = admit(&wire);
        let mut simulation = roundtrip.upstream_simulation().unwrap();
        assert_eq!(
            simulation
                .receiver_observation_policy()
                .info()
                .maximum_scalar_slots,
            ESTIMATE_SCALAR_SLOTS
        );
        assert_eq!(
            simulation
                .receiver_observation_policy()
                .info()
                .soft_metrics
                .unwrap()
                .maximum_cells,
            4
        );
        drop(roundtrip);
        let report = simulation.run().unwrap();
        assert!(report.reference_recovered);
        assert_eq!(report.options.chunk_samples, PROJECT_CHUNK_SAMPLES);
        assert_eq!(report.options.retained_points, RETAINED_IQ_POINTS);
        assert_eq!(report.grants.len(), 1);
        assert!(report.receiver_observations().retained_frames > 0);
        if kind == "measured" {
            let provenance: Value =
                serde_json::from_str(report.measurement_provenance_json.as_ref().unwrap()).unwrap();
            assert_eq!(
                provenance["original_plan_bytes"],
                json!(original.as_bytes())
            );
            assert!(!provenance["source_hashes"].as_array().unwrap().is_empty());
        } else {
            assert!(report.measurement_provenance_json.is_none());
            assert_eq!(
                simulation.document().unwrap().original_bytes().unwrap(),
                original.as_bytes()
            );
        }
        let serialized = report.to_json().unwrap();
        assert!(!serialized.is_empty());
    }
}

#[test]
fn malformed_cross_direction_duplicate_fields_and_bad_policies_preserve_accepted_project() {
    use docsis_ofdm_fec::api::ErrorCode;
    let accepted = admit(&upstream("configuration", UPSTREAM_50));
    let before = accepted.to_json().unwrap();
    for mutate in [
        ("direction", json!("downstream")),
        ("schema_version", json!(99)),
        ("application", json!("other")),
        ("unknown", json!(true)),
    ] {
        let mut value = upstream("configuration", UPSTREAM_50);
        value[mutate.0] = mutate.1;
        assert!(Project::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
    }
    let mut value = upstream("configuration", UPSTREAM_50);
    value["contents"]["options"]["chunk_samples"] = json!(0);
    assert!(Project::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
    let mut value = upstream("configuration", UPSTREAM_50);
    value["contents"]["receiver_observations"]["maximum_scalar_slots"] = json!(1);
    assert!(Project::from_bytes(&serde_json::to_vec(&value).unwrap()).is_err());
    let value = serde_json::to_string(&upstream("configuration", UPSTREAM_50)).unwrap();
    let duplicate = value.replace(
        "\"chunk_samples\":257",
        "\"chunk_samples\":257,\"chunk_samples\":258",
    );
    assert_ne!(duplicate, value);
    assert_eq!(
        Project::from_bytes(duplicate.as_bytes())
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    let mut legacy = downstream();
    legacy["sections"]["profile"]["file"]["sha256"] = json!("corrupt");
    assert_eq!(
        Project::from_bytes(&serde_json::to_vec(&legacy).unwrap())
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    assert_eq!(accepted.to_json().unwrap(), before);
    assert!(accepted.upstream_simulation().unwrap().preview().is_ok());
}

#[test]
fn collection_counts_every_original_and_rejects_duplicate_or_extra_directions() {
    use docsis_ofdm_fec::api::ErrorCode;
    let legacy = downstream();
    let downstream = admit(&legacy);
    let upstream = admit(&upstream("measured", MEASURED_50));
    let info = Project::collection_info(&[&downstream, &upstream]).unwrap();
    assert_eq!(info.projects, 2);
    assert_eq!(
        info.directions,
        [ProjectDirection::Downstream, ProjectDirection::Upstream]
    );
    assert_eq!(
        info.serialized_bytes,
        downstream.original_document().len() + upstream.original_document().len()
    );
    let measurements = &legacy["sections"]["measurements"];
    let original_downstream = legacy["sections"]["profile"]["file"]["byte_length"]
        .as_u64()
        .unwrap()
        + measurements["rxmer"]
            .as_array()
            .unwrap()
            .iter()
            .map(|input| input["byte_length"].as_u64().unwrap())
            .sum::<u64>()
        + measurements["response"]["byte_length"].as_u64().unwrap();
    let measured: Value = serde_json::from_str(MEASURED_50).unwrap();
    let original_upstream = MEASURED_50.len()
        + ["ucd_fragments", "rxmer"]
            .into_iter()
            .map(|family| {
                measured[family]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|source| source["bytes"].as_array().unwrap().len())
                    .sum::<usize>()
            })
            .sum::<usize>()
        + measured["pre_eq_initial"]["bytes"]
            .as_array()
            .unwrap()
            .len()
        + measured["pre_eq_events"]
            .as_array()
            .unwrap()
            .iter()
            .map(|event| event["source"]["bytes"].as_array().unwrap().len())
            .sum::<usize>();
    assert_eq!(
        info.original_input_bytes,
        original_downstream as usize + original_upstream
    );
    assert_eq!(
        Project::collection_info(&[&downstream, &downstream])
            .err()
            .unwrap()
            .code(),
        ErrorCode::InvalidInput
    );
    assert_eq!(
        Project::collection_info(&[&downstream, &upstream, &downstream])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(Project::collection_info(&[]).unwrap().projects, 0);
}

#[test]
fn file_admission_preserves_bytes_and_bounds_nonregular_and_oversized_inputs() {
    use docsis_ofdm_fec::api::ErrorCode;
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("opaque.data");
    let bytes = serde_json::to_vec(&downstream()).unwrap();
    std::fs::write(&path, &bytes).unwrap();
    let project = Project::from_file(&path).unwrap();
    assert_eq!(project.original_document(), bytes);
    assert_eq!(std::fs::read(&path).unwrap(), bytes);
    assert_eq!(
        Project::from_file(directory.path()).err().unwrap().code(),
        ErrorCode::Io
    );
    // Independent 144 MiB policy anchor: a sparse oversized file must reject before reading.
    std::fs::File::create(&path)
        .unwrap()
        .set_len(144 * 1024 * 1024 + 1)
        .unwrap();
    assert_eq!(
        Project::from_file(&path).err().unwrap().code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(
        Project::from_bytes(&[0xff]).err().unwrap().code(),
        ErrorCode::InvalidInput
    );
}
