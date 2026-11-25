/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Headless current-project regression, preserving complete legacy validation.
use super::*;
const DOWNSTREAM: &[u8] =
    include_bytes!("../../../../../examples/workbench/single/workbench.merlin.json");
const UPSTREAM: &str = include_str!("../../../../../examples/cli/upstream/us-50khz.json");

#[test]
fn headless_check_accepts_complete_legacy_and_both_current_directions_without_running() {
    check_bytes(DOWNSTREAM).unwrap();
    let downstream: Project = serde_json::from_slice(DOWNSTREAM).unwrap();
    let downstream = super::super::project_envelope::encode_runnable(&downstream, None).unwrap();
    // Independent public wire-version anchor; do not derive malformed fixtures
    // or compatibility expectations from the production dispatch constant.
    let upstream = serde_json::to_vec(&json!({"schema_version":3,"application":"MERLIN",
        "direction":"upstream","contents":{"input":{"kind":"configuration",
        "document":UPSTREAM},"options":merlin::upstream::ExperimentOptions::default(),
        "receiver_observations":{"maximum_frames":0,"maximum_scalar_slots":0}}}))
    .unwrap();
    for document in [&downstream, &upstream] {
        check_bytes(document).unwrap();
        let mut invalid: Value = serde_json::from_slice(document).unwrap();
        invalid["contents"]["unexpected"] = json!(true);
        assert!(check_bytes(&serde_json::to_vec(&invalid).unwrap()).is_err());
        invalid["schema_version"] = json!(4);
        assert!(check_bytes(&serde_json::to_vec(&invalid).unwrap()).is_err());
        assert!(check_bytes(&document[..document.len() - 1]).is_err());
    }
}

#[test]
fn headless_check_keeps_partial_legacy_rejection_and_original_diagnostic() {
    let mut partial: Project = serde_json::from_slice(DOWNSTREAM).unwrap();
    partial.receiver = None;
    assert_eq!(
        check_bytes(&serde_json::to_vec(&partial).unwrap()).unwrap_err(),
        "Example has no receiver configuration."
    );
    let mut bad: Value = serde_json::from_slice(DOWNSTREAM).unwrap();
    bad["application"] = json!("Other");
    let bad = serde_json::to_vec(&bad).unwrap();
    let expected = serde_json::from_slice::<Project>(&bad)
        .unwrap()
        .validate()
        .err()
        .unwrap();
    assert_eq!(check_bytes(&bad).unwrap_err(), expected);
}
