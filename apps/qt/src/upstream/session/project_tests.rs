/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Transport/ownership fixtures and public scientific parity; no Qt-side science.
use super::*;
use merlin::upstream::{SoftObservationPolicy, Tap};
use serde_json::{json, Value};

const CONFIGURATION_50: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const CONFIGURATION_25: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-25khz.json");
const MEASURED_50: &[u8] =
    include_bytes!("../../../../../examples/measurements/upstream/measured/capture-driven.json");
const MEASURED_25: &[u8] = include_bytes!(
    "../../../../../examples/measurements/upstream/measured/capture-driven-25khz.json"
);

// Independent transport-splitting fixture, deliberately unlike the source options.
const CURRENT_CHUNK_SAMPLES: usize = 257;
// Small retained I/Q history, independent of transport chunks and estimate storage.
const CURRENT_RETAINED_POINTS: usize = 128;
// One complete estimate fits either fixture's physical FFT geometry.
const ESTIMATE_FRAMES: usize = 1;
const ESTIMATE_SCALAR_SLOTS: usize = 8192;
// Separate finite soft-cell and completed-word input storage fixtures.
const SOFT_CELLS: usize = 4;
const SOFT_WORDS: usize = 1;
const SOFT_WORD_LLRS: usize = 16200;
// Tiny serializer sink ceiling for cumulative exact/+1 tests without huge buffers.
const SINK_FIXTURE_BYTES: usize = 8;

fn options() -> ExperimentOptions {
    ExperimentOptions {
        chunk_samples: CURRENT_CHUNK_SAMPLES,
        retained_points: CURRENT_RETAINED_POINTS,
        retained_taps: vec![Tap::TxOutput, Tap::RxEqualized],
        ..ExperimentOptions::default()
    }
}

fn observations() -> ReceiverObservationPolicy {
    ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS)
        .unwrap()
        .with_soft_metrics(
            SoftObservationPolicy::new(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS).unwrap(),
        )
}

fn assert_roundtrip(input: Input<'_>, original: &[u8], kind: &str) {
    let options = options();
    let before = options.clone();
    let observations = observations();
    let bytes = encode(input, &options, observations).unwrap();
    assert!(bytes.len() <= MAX_PROJECT_BYTES);
    assert_eq!(options, before);
    let wire: Value = serde_json::from_slice(&bytes).unwrap();

    // Independent schema/application/direction and wire-name anchors.
    assert_eq!(wire["schema_version"], 3);
    assert_eq!(wire["application"], "MERLIN");
    assert_eq!(wire["direction"], "upstream");
    assert_eq!(wire["contents"]["input"]["kind"], kind);
    assert_eq!(
        wire["contents"]["input"]["document"]
            .as_str()
            .unwrap()
            .as_bytes(),
        original
    );
    assert_eq!(
        wire["contents"]["options"],
        serde_json::to_value(&options).unwrap()
    );
    let policy = &wire["contents"]["receiver_observations"];
    assert_eq!(policy["maximum_frames"], ESTIMATE_FRAMES);
    assert_eq!(policy["maximum_scalar_slots"], ESTIMATE_SCALAR_SLOTS);
    assert_eq!(policy["soft"]["maximum_cells"], SOFT_CELLS);
    assert_eq!(policy["soft"]["maximum_words"], SOFT_WORDS);
    assert_eq!(policy["soft"]["maximum_word_llrs"], SOFT_WORD_LLRS);
    assert!(policy.get("soft_metrics").is_none());
    let project = Project::from_bytes(&bytes).unwrap();
    assert_eq!(project.direction(), ProjectDirection::Upstream);
    assert_eq!(project.original_document(), bytes);
    assert_eq!(
        project
            .upstream_simulation()
            .unwrap()
            .receiver_observation_policy()
            .info(),
        observations.info()
    );
}

fn public_rejection(kind: &str, document: &str, options: &ExperimentOptions) -> String {
    let wire = json!({
        "schema_version":3,"application":"MERLIN","direction":"upstream",
        "contents":{"input":{"kind":kind,"document":document},"options":options,
        "receiver_observations":{"maximum_frames":0,"maximum_scalar_slots":0,"soft":null}}
    });
    Project::from_bytes(&serde_json::to_vec(&wire).unwrap())
        .err()
        .unwrap()
        .to_string()
}

#[test]
fn configuration_50_preserves_source_current_options_and_soft_wire() {
    assert_roundtrip(
        Input::Configuration(CONFIGURATION_50),
        CONFIGURATION_50,
        "configuration",
    );
}

#[test]
fn configuration_25_preserves_source_current_options_and_soft_wire() {
    assert_roundtrip(
        Input::Configuration(CONFIGURATION_25),
        CONFIGURATION_25,
        "configuration",
    );
}

#[test]
fn measured_50_preserves_complete_capture_association_document() {
    assert_roundtrip(Input::Measured(MEASURED_50), MEASURED_50, "measured");
}

#[test]
fn measured_25_preserves_complete_capture_association_document() {
    assert_roundtrip(Input::Measured(MEASURED_25), MEASURED_25, "measured");
}

#[test]
fn source_whitespace_and_unicode_survive_outer_json_escaping() {
    let original = std::str::from_utf8(CONFIGURATION_50).unwrap().replace(
        "Synthetic generic example; no universal recommended loading.",
        "Exact source text: α 😀 <literal>",
    );
    let original = format!("\n\t{original}\r\n");
    assert_roundtrip(
        Input::Configuration(original.as_bytes()),
        original.as_bytes(),
        "configuration",
    );
}

#[test]
fn disabled_policy_uses_schema_soft_null_without_public_soft_metrics_key() {
    let bytes = encode(
        Input::Configuration(CONFIGURATION_50),
        &options(),
        ReceiverObservationPolicy::default(),
    )
    .unwrap();
    let wire: Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        wire["contents"]["receiver_observations"],
        json!({"maximum_frames":0,"maximum_scalar_slots":0,"soft":null})
    );
}

#[test]
fn invalid_utf8_configuration_rejects_without_mutating_input_or_options() {
    let original = [0xff];
    let options = options();
    let before = options.clone();
    assert!(encode(Input::Configuration(&original), &options, observations()).is_err());
    assert_eq!(original, [0xff]);
    assert_eq!(options, before);
}

#[test]
fn invalid_utf8_measured_rejects() {
    assert!(encode(Input::Measured(&[0xff]), &options(), observations()).is_err());
}

#[test]
fn invalid_configuration_keeps_public_admission_diagnostic() {
    let document = "{\"format\":\"unknown\"}";
    let options = options();
    assert_eq!(
        encode(
            Input::Configuration(document.as_bytes()),
            &options,
            ReceiverObservationPolicy::default()
        )
        .unwrap_err(),
        public_rejection("configuration", document, &options)
    );
}

#[test]
fn invalid_measured_keeps_public_admission_diagnostic() {
    let document = "{\"format\":\"unknown\"}";
    let options = options();
    assert_eq!(
        encode(
            Input::Measured(document.as_bytes()),
            &options,
            ReceiverObservationPolicy::default()
        )
        .unwrap_err(),
        public_rejection("measured", document, &options)
    );
}

#[test]
fn configuration_does_not_accept_a_measured_document() {
    assert!(encode(
        Input::Configuration(MEASURED_50),
        &options(),
        observations()
    )
    .is_err());
}

#[test]
fn measured_does_not_accept_a_configuration_document() {
    assert!(encode(
        Input::Measured(CONFIGURATION_50),
        &options(),
        observations()
    )
    .is_err());
}

#[test]
fn invalid_current_options_reject_through_public_owner() {
    let mut options = options();
    options.grants = 0;
    let document = std::str::from_utf8(CONFIGURATION_50).unwrap();
    assert_eq!(
        encode(
            Input::Configuration(CONFIGURATION_50),
            &options,
            ReceiverObservationPolicy::default()
        )
        .unwrap_err(),
        public_rejection("configuration", document, &options)
    );
}

#[test]
fn input_above_private_fixture_ceiling_rejects_before_serialization() {
    assert_eq!(
        encode_with_limit(
            Input::Configuration(CONFIGURATION_50),
            &options(),
            observations(),
            CONFIGURATION_50.len() - 1
        )
        .unwrap_err(),
        PROJECT_LIMIT_MESSAGE
    );
}

#[test]
fn complete_output_is_inclusive_and_one_byte_below_rejects() {
    let options = options();
    let bytes = encode(
        Input::Configuration(CONFIGURATION_50),
        &options,
        observations(),
    )
    .unwrap();
    assert!(bytes.len() > CONFIGURATION_50.len());
    assert_eq!(
        encode_with_limit(
            Input::Configuration(CONFIGURATION_50),
            &options,
            observations(),
            bytes.len()
        )
        .unwrap(),
        bytes
    );
    assert!(encode_with_limit(
        Input::Configuration(CONFIGURATION_50),
        &options,
        observations(),
        bytes.len() - 1
    )
    .is_err());
}

#[test]
fn serializer_sink_checks_cumulative_total_before_append() {
    let mut output = ProjectOutput {
        bytes: Vec::new(),
        maximum_bytes: SINK_FIXTURE_BYTES,
    };
    output.write_all(b"0123").unwrap();
    output.write_all(b"4567").unwrap();
    assert_eq!(output.bytes.len(), SINK_FIXTURE_BYTES);
    assert!(output.write_all(b"8").is_err());
    assert_eq!(output.bytes, b"01234567");
}

#[test]
fn public_project_owns_source_after_all_encoder_inputs_and_output_drop() {
    let original = CONFIGURATION_50.to_vec();
    let options = options();
    let bytes = encode(Input::Configuration(&original), &options, observations()).unwrap();
    let project = Project::from_bytes(&bytes).unwrap();
    drop(bytes);
    drop(options);
    drop(original);
    let wire: Value = serde_json::from_slice(project.original_document()).unwrap();
    assert_eq!(
        wire["contents"]["input"]["document"]
            .as_str()
            .unwrap()
            .as_bytes(),
        CONFIGURATION_50
    );
    let simulation = project.upstream_simulation().unwrap();
    drop(project);
    assert_eq!(
        simulation.document().unwrap().original_bytes().unwrap(),
        CONFIGURATION_50
    );
    assert!(simulation.preview().is_ok());
}
