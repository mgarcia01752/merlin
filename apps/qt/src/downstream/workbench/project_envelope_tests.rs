/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Idle editor/plan/observation transport preservation; no duplicate run suite.
use super::*;
use merlin::downstream::{FullLoopStage, FullLoopStageRetention};
use serde_json::{json, Value};

const RUNNABLE: &[u8] =
    include_bytes!("../../../../../examples/workbench/single/workbench.merlin.json");
const UPSTREAM: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");

// Independent small explicit observation choices, not new application defaults.
const GLOBAL_EVENTS: usize = 3;
const FFT_EVENTS: usize = 2;
const PLC_EVENTS: usize = 1;
const ELEMENTS_PER_EVENT: usize = 32;
const TOTAL_ELEMENTS: usize = 96;
const RETAINED_BYTES: usize = 4096;
// Tiny in-memory serializer fixture for exact/+1 append rejection.
const SINK_BYTES: usize = 8;

fn runnable() -> Project {
    let mut project: Project = serde_json::from_slice(RUNNABLE).unwrap();
    project.receiver.as_mut().unwrap().all_profiles = false;
    project
}

fn blank(version: u32) -> Project {
    Project {
        schema_version: version,
        application: "MERLIN".into(),
        transmission: Default::default(),
        sections: Default::default(),
        receiver: None,
        comparison_enabled: false,
        dark: true,
    }
}

fn observations() -> FullLoopObservationInput {
    FullLoopObservationInput {
        stages: vec![
            FullLoopStageRetention {
                stage: FullLoopStage::RxFft,
                max_events: FFT_EVENTS,
                observation_only: false,
            },
            FullLoopStageRetention {
                stage: FullLoopStage::PlcMessage,
                max_events: PLC_EVENTS,
                observation_only: true,
            },
        ],
        max_events: GLOBAL_EVENTS,
        max_elements_per_event: ELEMENTS_PER_EVENT,
        max_total_elements: TOTAL_ELEMENTS,
        max_retained_bytes: RETAINED_BYTES,
    }
}

fn envelope(workbench: &str, observations: Option<&FullLoopObservationInput>) -> Vec<u8> {
    // Independent schema/direction contract anchor, checked by the public owner.
    serde_json::to_vec(&json!({"schema_version":3,"application":"MERLIN",
        "direction":"downstream","contents":{"workbench":workbench,
        "observations":observations}}))
    .unwrap()
}

fn assert_legacy_blank(version: u32) {
    let project = blank(version);
    let original = serde_json::to_vec_pretty(&project).unwrap();
    assert!(PublicProject::from_bytes(&original).is_err());
    let decoded = decode(&original).unwrap();
    assert!(decoded.public_plan.is_none());
    assert_eq!(decoded.project.schema_version, version);
    assert!(decoded.project.sections.is_empty());
    assert!(decoded.project.receiver.is_none());
    assert_eq!(
        serde_json::to_vec_pretty(&decoded.project).unwrap(),
        original
    );
}

#[test]
fn schema_one_blank_editor_remains_decodable_without_public_runnable_admission() {
    assert_legacy_blank(1);
}

#[test]
fn schema_two_blank_editor_remains_decodable_without_public_runnable_admission() {
    assert_legacy_blank(2);
}

#[test]
fn partial_legacy_keeps_real_source_section_without_inventing_missing_inputs() {
    let mut project = runnable();
    project.schema_version = 2;
    project.sections.retain(|name, _| name == "source");
    project.receiver = None;
    let source = project.sections["source"].clone();
    let bytes = serde_json::to_vec_pretty(&project).unwrap();
    assert!(PublicProject::from_bytes(&bytes).is_err());
    let decoded = decode(&bytes).unwrap();
    assert!(decoded.public_plan.is_none());
    assert_eq!(decoded.project.sections.len(), 1);
    assert_eq!(decoded.project.sections["source"], source);
    assert_eq!(serde_json::to_vec_pretty(&decoded.project).unwrap(), bytes);
}

#[test]
fn schema_three_preserves_actual_public_plan_originals_and_nondefault_observations() {
    let editor = runnable();
    let legacy = serde_json::to_string_pretty(&editor).unwrap();
    let observations = observations();
    let bytes = envelope(&legacy, Some(&observations));
    let admitted = PublicProject::from_bytes(&bytes).unwrap();
    let expected = admitted.downstream_plan().unwrap();
    let decoded = decode(&bytes).unwrap();
    assert_eq!(
        serde_json::to_string_pretty(&decoded.project).unwrap(),
        legacy
    );
    let plan = decoded.public_plan.unwrap();
    assert_eq!(
        serde_json::to_value(plan.info()).unwrap(),
        serde_json::to_value(expected.info()).unwrap()
    );
    drop(admitted);
    drop(bytes);
    let request = plan.request(0).unwrap();
    let expected_request = expected.request(0).unwrap();
    assert_eq!(
        serde_json::to_value(request.options()).unwrap(),
        serde_json::to_value(expected_request.options()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(request.observation_policy().unwrap().settings()).unwrap(),
        serde_json::to_value(&observations).unwrap()
    );
}

#[test]
fn schema_three_absent_observations_stay_absent_in_the_retained_plan() {
    let legacy = serde_json::to_string_pretty(&runnable()).unwrap();
    let decoded = decode(&envelope(&legacy, None)).unwrap();
    assert!(decoded
        .public_plan
        .unwrap()
        .request(0)
        .unwrap()
        .observation_policy()
        .is_none());
}

#[test]
fn runnable_encoding_preserves_exact_typed_editor_and_borrowed_observation_values() {
    let project = runnable();
    let legacy = serde_json::to_vec_pretty(&project).unwrap();
    let observations = observations();
    let before = serde_json::to_value(&observations).unwrap();
    let bytes = encode_runnable(&project, Some(&observations)).unwrap();
    let wire: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(wire["schema_version"], 3);
    assert_eq!(wire["direction"], "downstream");
    assert_eq!(
        wire["contents"]["workbench"].as_str().unwrap().as_bytes(),
        legacy
    );
    assert_eq!(wire["contents"]["observations"], before);
    assert_eq!(serde_json::to_value(&observations).unwrap(), before);
    assert_eq!(serde_json::to_vec_pretty(&project).unwrap(), legacy);
    let decoded = decode(&bytes).unwrap();
    assert!(decoded.public_plan.is_some());
    assert_eq!(serde_json::to_vec_pretty(&decoded.project).unwrap(), legacy);
}

#[test]
fn borrowed_sections_encode_identically_to_owned_sections() {
    let project = runnable();
    let second: Project = serde_json::from_slice(&serde_json::to_vec(&project).unwrap()).unwrap();
    let borrowed = Project {
        schema_version: second.schema_version,
        application: second.application,
        transmission: second.transmission,
        sections: &project.sections,
        receiver: second.receiver,
        comparison_enabled: second.comparison_enabled,
        dark: second.dark,
    };
    let observations = observations();
    assert_eq!(
        encode_runnable(&borrowed, Some(&observations)).unwrap(),
        encode_runnable(&project, Some(&observations)).unwrap()
    );
}

#[test]
fn runnable_encoding_rejects_incomplete_editors_without_schema_two_fallback() {
    let project = blank(2);
    let before = serde_json::to_vec_pretty(&project).unwrap();
    assert!(encode_runnable(&project, None).is_err());
    assert_eq!(serde_json::to_vec_pretty(&project).unwrap(), before);
    assert!(decode(&before).unwrap().public_plan.is_none());
}

#[test]
fn invalid_observations_reject_without_suppressing_policy_or_mutating_editor() {
    let project = runnable();
    let before = serde_json::to_vec_pretty(&project).unwrap();
    let mut observations = observations();
    observations.max_events = 0;
    assert!(encode_runnable(&project, Some(&observations)).is_err());
    assert_eq!(observations.max_events, 0);
    assert_eq!(serde_json::to_vec_pretty(&project).unwrap(), before);
}

#[test]
fn schema_three_cannot_hide_an_incomplete_editor_behind_an_envelope() {
    let legacy = serde_json::to_string_pretty(&blank(2)).unwrap();
    let bytes = envelope(&legacy, None);
    let expected = PublicProject::from_bytes(&bytes).err().unwrap().to_string();
    assert_eq!(decode(&bytes).err().unwrap(), expected);
}

#[test]
fn actual_public_upstream_project_is_rejected_as_wrong_direction() {
    let bytes = serde_json::to_vec(&json!({"schema_version":3,"application":"MERLIN",
        "direction":"upstream","contents":{"input":{"kind":"configuration",
        "document":std::str::from_utf8(UPSTREAM).unwrap()},
        "options":merlin::upstream::ExperimentOptions::default(),
        "receiver_observations":{"maximum_frames":0,"maximum_scalar_slots":0}}}))
    .unwrap();
    PublicProject::from_bytes(&bytes).unwrap();
    assert_eq!(
        decode(&bytes).err().unwrap(),
        "Choose a downstream project."
    );
}

#[test]
fn malformed_schema_three_keeps_public_strict_rejection() {
    let legacy = serde_json::to_string_pretty(&runnable()).unwrap();
    let mut wire: Value = serde_json::from_slice(&envelope(&legacy, None)).unwrap();
    wire["contents"]["unexpected"] = json!(true);
    let bytes = serde_json::to_vec(&wire).unwrap();
    let expected = PublicProject::from_bytes(&bytes).err().unwrap().to_string();
    assert_eq!(decode(&bytes).err().unwrap(), expected);
}

#[test]
fn malformed_legacy_and_unknown_versions_are_not_reinterpreted() {
    let mut legacy = serde_json::to_value(blank(2)).unwrap();
    legacy["unexpected"] = json!(true);
    assert!(decode(&serde_json::to_vec(&legacy).unwrap()).is_err());
    let unknown = br#"{"schema_version":99}"#;
    let expected = PublicProject::from_bytes(unknown)
        .err()
        .unwrap()
        .to_string();
    assert_eq!(decode(unknown).err().unwrap(), expected);
}

#[test]
fn decode_and_encode_admit_exact_boundaries_without_oversized_test_allocations() {
    let bytes = serde_json::to_vec_pretty(&blank(2)).unwrap();
    assert!(decode_with_limit(&bytes, bytes.len()).is_ok());
    assert!(decode_with_limit(&bytes, bytes.len() - 1).is_err());
    let project = runnable();
    let encoded = encode_runnable(&project, None).unwrap();
    assert_eq!(
        encode_runnable_with_limit(&project, None, encoded.len()).unwrap(),
        encoded
    );
    assert!(encode_runnable_with_limit(&project, None, encoded.len() - 1).is_err());
}

#[test]
fn bounded_serializer_rejects_the_next_byte_before_append() {
    let mut output = ProjectOutput {
        bytes: Vec::new(),
        maximum_bytes: SINK_BYTES,
    };
    output.write_all(b"01234567").unwrap();
    assert_eq!(output.bytes.len(), SINK_BYTES);
    assert!(output.write_all(b"8").is_err());
    assert_eq!(output.bytes, b"01234567");
}

#[test]
fn native_instruments_accept_exact_default_policy_and_explicitly_reject_other_public_policies() {
    let editor = runnable();
    let legacy = serde_json::to_string(&editor).unwrap();
    for policy in [
        None,
        Some(observations()),
        Some(FullLoopObservationInput::default()),
    ] {
        let supported = policy.as_ref().is_some_and(|policy| {
            serde_json::to_value(policy).unwrap()
                == serde_json::to_value(FullLoopObservationInput::default()).unwrap()
        });
        let project = PublicProject::from_bytes(&envelope(&legacy, policy.as_ref())).unwrap();
        let context = crate::downstream::workbench::session::Context::isolated();
        let host = context.application.clone();
        let before = host.snapshot(crate::shell::Direction::Downstream);
        let mut model =
            crate::downstream::simulation::closed_loop::Model::for_context(context).unwrap();
        model
            .prepare_options_for_budget(
                editor.receiver.clone(),
                editor.comparison_enabled,
                host.budget(),
            )
            .unwrap();
        let result =
            model.prepare_public_plan_for_budget(project.downstream_plan().unwrap(), host.budget());
        if supported {
            result.unwrap();
        } else {
            assert!(result
                .unwrap_err()
                .contains("observation policy is not supported"));
        }
        assert_eq!(host.snapshot(crate::shell::Direction::Downstream), before);
        assert!(host.active().is_none());
    }
}
