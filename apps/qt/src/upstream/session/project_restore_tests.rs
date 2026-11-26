/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Idle source/settings preservation and retained-owner rollback; no run suite.
use super::super::session_resources;
use super::*;
use crate::shell::RetainedResources;
use merlin::upstream::{
    ConfigurationDocument, ExperimentOptions, ReceiverObservationPolicy, SoftObservationPolicy, Tap,
};
use serde_json::json;
use std::sync::Arc;

const CONFIGURATION_50: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const CONFIGURATION_25: &[u8] =
    include_bytes!("../../../../../examples/cli/upstream/us-25khz.json");
const MEASURED_50: &[u8] =
    include_bytes!("../../../../../examples/measurements/upstream/measured/capture-driven.json");
const MEASURED_25: &[u8] = include_bytes!(
    "../../../../../examples/measurements/upstream/measured/capture-driven-25khz.json"
);
const DOWNSTREAM: &[u8] =
    include_bytes!("../../../../../examples/workbench/downstream/single/workbench.merlin.json");

// Independent saved overrides differ from the original source document settings.
const SAVED_GRANTS: usize = 2;
const SAVED_CHUNK_SAMPLES: usize = 313;
const SAVED_RETAINED_POINTS: usize = 29;
// One estimate fits either fixture's FFT geometry, independent of soft histories.
const ESTIMATE_FRAMES: usize = 1;
const ESTIMATE_SCALAR_SLOTS: usize = 8192;
const SOFT_CELLS: usize = 4;
const SOFT_WORDS: usize = 1;
const SOFT_WORD_LLRS: usize = 16_200;
// Explicit accepted edit and wide identity from the provenance regression.
const EDITED_SOURCE_SEED: u16 = 2;
const EDITED_GRANT_ID: u64 = 9_007_199_254_740_993;
const EDITED_PRESET_ID: &str = "restored-edited-selection";
// Held reservation keeps existing application usage observable during rollback.
const EXISTING_RESERVED_BYTES: usize = 8;

// Test-only unconstrained scientific dimensions; no worker/presentation owners.
const TEST_LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: 0,
    slot_count: 0,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};

fn options() -> ExperimentOptions {
    ExperimentOptions {
        grants: SAVED_GRANTS,
        chunk_samples: SAVED_CHUNK_SAMPLES,
        retained_points: SAVED_RETAINED_POINTS,
        retained_taps: vec![Tap::TxOutput],
        ..Default::default()
    }
}

fn observations() -> ReceiverObservationPolicy {
    ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_SCALAR_SLOTS)
        .unwrap()
        .with_soft_metrics(
            SoftObservationPolicy::new(SOFT_CELLS, SOFT_WORDS, SOFT_WORD_LLRS).unwrap(),
        )
}

fn project(kind: &str, document: &[u8], original: Option<&[u8]>) -> Project {
    let mut input = json!({"kind":kind,"document":std::str::from_utf8(document).unwrap()});
    if let Some(original) = original {
        input["original_document"] = json!(std::str::from_utf8(original).unwrap());
    }

    // Independent schema-3 and observation wire anchors; public admission owns them.
    let wire = json!({"schema_version":3,"application":"MERLIN","direction":"upstream",
        "contents":{"input":input,"options":options(),"receiver_observations":{
        "maximum_frames":ESTIMATE_FRAMES,"maximum_scalar_slots":ESTIMATE_SCALAR_SLOTS,
        "soft":{"maximum_cells":SOFT_CELLS,"maximum_words":SOFT_WORDS,
        "maximum_word_llrs":SOFT_WORD_LLRS}}}});
    Project::from_bytes(&serde_json::to_vec(&wire).unwrap()).unwrap()
}

fn assert_restored(kind: &str, original: &[u8]) {
    let mut source = b" \n".to_vec();
    source.extend_from_slice(original);
    source.extend_from_slice(b"\r\n ");
    let project = project(kind, &source, None);
    let accepted = project.original_document().to_vec();
    let expected = project.upstream_simulation().unwrap();
    let budget = RetentionBudget::with_limits(TEST_LIMITS);
    let prepared = PreparedSession::from_project(&project, &budget).unwrap();
    assert_eq!(project.original_document(), accepted);
    drop(project);
    assert!(budget.owns(prepared.session.as_ref()));
    let expected_charge = {
        let storage = prepared.session.value().lock().unwrap();
        let simulation = storage.simulation.as_ref().unwrap();
        assert_eq!(simulation.options(), &options());
        assert_eq!(simulation.options(), expected.options());
        assert_eq!(simulation.receiver_observation_policy(), observations());
        assert_eq!(simulation.configuration(), expected.configuration());
        assert_eq!(simulation.preview().unwrap(), expected.preview().unwrap());
        let document = simulation.document().unwrap();
        assert_eq!(
            document.to_json().unwrap(),
            expected.document().unwrap().to_json().unwrap()
        );
        assert_eq!(
            document.original_bytes(),
            expected.document().unwrap().original_bytes()
        );
        assert_eq!(
            document.original_sha256(),
            expected.document().unwrap().original_sha256()
        );
        let mut resources = session_resources(simulation.retained_footprint().unwrap()).unwrap();
        if kind == "measured" {
            let plan = MeasuredExperiment::from_json(&source).unwrap();
            let inventory = prepared.measured_input.unwrap();
            assert_eq!(inventory.ucd_fragments, plan.ucd_fragments.len());
            assert_eq!(inventory.rxmer_captures, plan.rxmer.len());
            assert_eq!(inventory.pre_eq_initial, plan.pre_eq_initial.is_some());
            assert_eq!(inventory.pre_eq_events, plan.pre_eq_events.len());
            let retained = storage.measured_source.as_ref().unwrap();
            assert!(budget.owns(retained));
            assert_eq!(retained.value().as_ref(), source);
            resources.payload_bytes = resources.payload_bytes.checked_add(source.len()).unwrap();
        } else {
            assert!(prepared.measured_input.is_none());
            assert!(storage.measured_source.is_none());
            assert_eq!(document.original_bytes(), Some(source.as_slice()));
        }
        resources
    };
    assert_eq!(budget.usage(), expected_charge);
    let alias = Arc::clone(&prepared.session);
    drop(prepared);
    assert_eq!(budget.usage(), expected_charge);
    drop(alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn configuration_50_restores_actual_options_policy_source_and_final_alias_charge() {
    assert_restored("configuration", CONFIGURATION_50);
}

#[test]
fn configuration_25_restores_actual_options_policy_source_and_final_alias_charge() {
    assert_restored("configuration", CONFIGURATION_25);
}

#[test]
fn measured_50_restores_exact_plan_inventory_overrides_and_final_alias_charge() {
    assert_restored("measured", MEASURED_50);
}

#[test]
fn measured_25_restores_exact_plan_inventory_overrides_and_final_alias_charge() {
    assert_restored("measured", MEASURED_25);
}

#[test]
fn edited_configuration_keeps_current_fields_original_identity_and_saved_overrides() {
    let mut original = b" \n".to_vec();
    original.extend_from_slice(CONFIGURATION_50);
    original.extend_from_slice(b"\r\n ");
    let mut accepted = ConfigurationDocument::from_json(&original).unwrap();
    let mut selected = accepted.presets[0].clone();
    selected.id = EDITED_PRESET_ID.into();
    accepted.presets.push(selected);
    accepted.selected_preset = EDITED_PRESET_ID.into();
    accepted.simulation.source_seed = EDITED_SOURCE_SEED;
    accepted.simulation.grant_id = EDITED_GRANT_ID;
    let current = accepted.to_json().unwrap();
    let project = project("configuration", current.as_bytes(), Some(&original));
    let before = project.to_json().unwrap();
    let budget = RetentionBudget::with_limits(TEST_LIMITS);
    let prepared = PreparedSession::from_project(&project, &budget).unwrap();
    assert_eq!(project.to_json().unwrap(), before);
    drop(project);
    let storage = prepared.session.value().lock().unwrap();
    let simulation = storage.simulation.as_ref().unwrap();
    assert_eq!(simulation.options(), &options());
    assert_eq!(simulation.receiver_observation_policy(), observations());
    let restored = simulation.document().unwrap();
    assert_eq!(restored, &accepted);
    assert_eq!(restored.to_json().unwrap(), current);
    assert_eq!(restored.original_bytes(), Some(original.as_slice()));
    assert_eq!(restored.original_sha256(), accepted.original_sha256());
    assert_eq!(restored.simulation.source_seed, EDITED_SOURCE_SEED);
    assert_eq!(restored.simulation.grant_id, EDITED_GRANT_ID);
    assert!(prepared.measured_input.is_none());
    assert!(storage.measured_source.is_none());
    drop(storage);
    drop(prepared);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn wrong_direction_preserves_public_error_and_leaves_budget_and_project_unchanged() {
    let project = Project::from_bytes(DOWNSTREAM).unwrap();
    let before = project.to_json().unwrap();
    let expected = project.upstream_simulation().err().unwrap().to_string();
    let budget = RetentionBudget::with_limits(TEST_LIMITS);
    assert_eq!(
        PreparedSession::from_project(&project, &budget)
            .err()
            .unwrap(),
        expected
    );
    assert_eq!(project.to_json().unwrap(), before);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn final_measured_session_rejection_releases_candidate_source_and_keeps_existing_reservation() {
    let project = project("measured", MEASURED_50, None);
    let before = project.to_json().unwrap();
    let maximum = RetainedResources {
        payload_bytes: MEASURED_50
            .len()
            .checked_add(EXISTING_RESERVED_BYTES)
            .unwrap(),
        ..TEST_LIMITS
    };
    let budget = RetentionBudget::with_limits(maximum);
    let existing = budget
        .reserve_resources(RetainedResources {
            payload_bytes: EXISTING_RESERVED_BYTES,
            ..Default::default()
        })
        .unwrap();
    let baseline = budget.usage();
    let error = PreparedSession::from_project(&project, &budget)
        .err()
        .unwrap();
    assert!(error.contains("Retention("), "{error}");
    assert_eq!(budget.usage(), baseline);
    assert_eq!(project.to_json().unwrap(), before);
    drop(existing);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn measured_source_rejection_does_not_leave_scientific_or_source_charge() {
    let project = project("measured", MEASURED_25, None);
    let before = project.to_json().unwrap();
    let budget = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: MEASURED_25.len() - 1,
        ..TEST_LIMITS
    });
    let error = PreparedSession::from_project(&project, &budget)
        .err()
        .unwrap();
    assert!(error.contains("Retention("), "{error}");
    assert_eq!(project.to_json().unwrap(), before);
    assert_eq!(budget.usage(), RetainedResources::default());
    let unrestricted = RetentionBudget::with_limits(TEST_LIMITS);
    let restored = PreparedSession::from_project(&project, &unrestricted).unwrap();
    drop(restored);
    assert_eq!(unrestricted.usage(), RetainedResources::default());
}
