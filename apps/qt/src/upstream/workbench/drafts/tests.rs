/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Configuration transactions exercise the actual public preparation and host.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources, RunCancellation};
use merlin::upstream::{ConfigurationDocument, ReportJson};
use std::time::{Duration, Instant};

const DOCUMENT: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");
const COMPLETION_DEADLINE: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(2);

// Independent exact-integer transport anchor, beyond IEEE-754's contiguous range.
const WIDE_GRANT_ID: &str = "9007199254740993";

// Independent valid PRBS seed edit, distinct from the loaded source seed.
const PROJECT_EDIT_SOURCE_SEED: &str = "2";

fn drain(model: &mut Model) {
    let deadline = Instant::now() + COMPLETION_DEADLINE;
    while model.poll()["preparing"] == true {
        assert!(Instant::now() < deadline, "{}", model.state());
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn configured() -> (ApplicationSessions, Box<Model>) {
    let application = ApplicationSessions::desktop().unwrap();
    let model = configured_with(application.clone());
    (application, model)
}

fn configured_with(application: ApplicationSessions) -> Box<Model> {
    let baseline = application.budget().usage();
    let mut model = Model::new(application.clone()).unwrap();
    model.apply(DOCUMENT).unwrap();
    drain(&mut model);
    assert!(model.current(), "{}", model.state());
    let expected = baseline
        .checked_add(model._fixed_guard.fixture_reserved_resources())
        .unwrap()
        .checked_add(
            model
                .controller
                .as_ref()
                .unwrap()
                .fixture_session_resources(),
        )
        .unwrap();
    let deadline = Instant::now() + COMPLETION_DEADLINE;
    while application.budget().usage() != expected {
        assert!(Instant::now() < deadline);
        std::thread::sleep(POLL_INTERVAL);
    }
    model
}

fn source(model: &Model) -> ConfigurationDocument {
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| simulation.document().unwrap().clone())
        .unwrap()
}

fn field(model: &Model, name: &str) -> (String, String) {
    let state = model.draft_state();
    let field = state["options"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|field| field["field"] == name)
        .unwrap_or_else(|| panic!("Missing {name}: {state}"));
    (
        state["token"].as_str().unwrap().into(),
        field["key"].as_str().unwrap().into(),
    )
}

fn changes(key: &str, value: &str) -> String {
    json!({key:value}).to_string()
}

#[test]
fn unconfigured_drafts_have_real_identities_and_no_fabricated_values() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let mut tokens = std::collections::BTreeSet::new();
    for section in [
        "ucd",
        "profiles",
        "grants",
        "source",
        "receiver",
        "channel",
        "preEqualization",
    ] {
        model.begin_draft(section).unwrap();
        let state = model.draft_state();
        assert!(tokens.insert(state["token"].as_str().unwrap().to_owned()));
        assert!(state["baseSession"].is_string());
        assert!(state["baseGeneration"].is_string());
        assert_eq!(state["can_apply"], false);
        assert!(state["options"]["fields"].as_array().unwrap().is_empty());
        assert!(state["values"].as_object().unwrap().is_empty());
        model
            .cancel_draft(state["token"].as_str().unwrap())
            .unwrap();
        assert_eq!(model.draft_state()["outcome"], "cancelled");
    }
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert!(application.active().is_none());
}

#[test]
fn changed_source_apply_is_queued_then_atomically_accepted_and_reopened() {
    let (application, mut model) = configured();
    let original = source(&model);
    let downstream = application.snapshot(Direction::Downstream);
    model.begin_draft("source").unwrap();
    let base = model.draft_state();
    let (token, key) = field(&model, "source_seed");
    let old_key = model.key;
    model
        .apply_draft("source", &token, &changes(&key, "2"))
        .unwrap();
    assert_eq!(model.key, old_key);
    assert_eq!(model.draft_state()["outcome"], "pending");
    assert_eq!(model.draft_state()["pending"], true);
    drain(&mut model);
    assert_eq!(
        model.draft_state()["outcome"],
        "accepted",
        "{}",
        model.state()
    );
    assert_ne!(model.key, old_key);
    assert_eq!(model.draft_state()["baseSession"], base["baseSession"]);
    assert_eq!(
        model.draft_state()["baseGeneration"],
        base["baseGeneration"]
    );
    let accepted = source(&model);
    let mut expected = original.clone();
    expected.simulation.source_seed = 2;
    assert_eq!(accepted, expected);
    assert_eq!(accepted.original_bytes(), original.original_bytes());
    assert_eq!(accepted.original_sha256(), original.original_sha256());
    let view = model.configuration_presentation();
    assert_eq!(view["details"]["source"]["rows"][0]["value"], "2");
    assert_eq!(view["blocks"]["source"]["status"], "complete");
    model.begin_draft("source").unwrap();
    let (_, key) = field(&model, "source_seed");
    assert_eq!(model.draft_state()["values"][&key], "2");
    assert_ne!(model.draft_state()["token"], token);
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert!(application.active().is_none());
}

#[test]
fn native_apply_project_export_preserves_accepted_fields_and_exact_original_provenance() {
    let (application, mut model) = configured();
    let original = source(&model);
    model.begin_draft("source").unwrap();
    let (token, key) = field(&model, "source_seed");
    model
        .apply_draft("source", &token, &changes(&key, PROJECT_EDIT_SOURCE_SEED))
        .unwrap();
    drain(&mut model);
    assert_eq!(model.draft_state()["outcome"], "accepted");
    let accepted = source(&model);
    let bytes = model
        .controller
        .as_ref()
        .unwrap()
        .project_document()
        .unwrap();
    let project = merlin::common::Project::from_bytes(&bytes).unwrap();
    let mut restored = project.upstream_simulation().unwrap();
    let document = restored.document().unwrap();
    assert_eq!(
        document.simulation.source_seed,
        accepted.simulation.source_seed
    );
    assert_eq!(document.original_bytes(), original.original_bytes());
    assert_eq!(document.original_sha256(), original.original_sha256());
    let actual = restored.run().unwrap();
    model.start().unwrap();
    let deadline = Instant::now() + COMPLETION_DEADLINE;
    while model.poll()["busy"] == true {
        assert!(Instant::now() < deadline, "{}", model.state());
        std::thread::sleep(POLL_INTERVAL);
    }
    let expected = model.report.as_ref().unwrap().1.value();
    assert_eq!(actual.configuration, expected.configuration);
    assert_eq!(actual.options, expected.options);
    assert_eq!(actual.generated_source_bits, expected.generated_source_bits);
    assert_eq!(actual.compared_source_bits, expected.compared_source_bits);
    assert_eq!(actual.bit_errors, expected.bit_errors);
    assert_eq!(
        actual.source_configuration_original,
        expected.source_configuration_original
    );
    assert_eq!(
        actual.source_configuration_sha256,
        expected.source_configuration_sha256
    );
    assert_eq!(
        actual
            .grants
            .iter()
            .map(|grant| &grant.words)
            .collect::<Vec<_>>(),
        expected
            .grants
            .iter()
            .map(|grant| &grant.words)
            .collect::<Vec<_>>()
    );
    // Optional acceptance receipt for the installed native consumer. The reference
    // is this actual native Apply/run, independently of the saved-project loader.
    if let Some(directory) = std::env::var_os("MERLIN_QT_PROJECT_REFERENCE_DIRECTORY") {
        let destination = std::path::PathBuf::from(directory).join("native-edited-source.json");
        let value = serde_json::json!({
            "project": std::str::from_utf8(&bytes).unwrap(),
            "reference": serde_json::from_str::<serde_json::Value>(
                &expected.to_json().unwrap()
            ).unwrap()
        });
        std::fs::write(destination, serde_json::to_vec(&value).unwrap()).unwrap();
    }
    assert!(application.active().is_none());
}

#[test]
fn wide_integer_apply_preserves_exact_bits_and_other_source_fields() {
    let (_, mut model) = configured();
    model.begin_draft("grants").unwrap();
    let (token, key) = field(&model, "grant_id");
    let original = source(&model);
    model
        .apply_draft("grants", &token, &changes(&key, WIDE_GRANT_ID))
        .unwrap();
    drain(&mut model);
    assert_eq!(
        model.draft_state()["outcome"],
        "accepted",
        "{}",
        model.state()
    );
    let mut expected = original;
    expected.simulation.grant_id = WIDE_GRANT_ID.parse().unwrap();
    assert_eq!(source(&model), expected);
}

#[test]
fn public_validation_failure_preserves_owner_key_and_plain_error_then_allows_retry() {
    let (_, mut model) = configured();
    let original = source(&model);
    let key_before = model.key;
    model.begin_draft("receiver").unwrap();
    let (token, key) = field(&model, "ldpc_iterations");
    model
        .apply_draft("receiver", &token, &changes(&key, "0"))
        .unwrap();
    drain(&mut model);
    assert_eq!(model.key, key_before);
    assert_eq!(source(&model), original);
    assert_eq!(model.draft_state()["outcome"], "rejected");
    assert!(!model.draft_state()["error"].as_str().unwrap().is_empty());
    assert_eq!(model.draft_state()["can_apply"], true);
    model
        .apply_draft("receiver", &token, &changes(&key, "1"))
        .unwrap();
    drain(&mut model);
    assert_eq!(
        model.draft_state()["outcome"],
        "accepted",
        "{}",
        model.state()
    );
    assert_eq!(source(&model).simulation.ldpc_iterations, 1);
}

#[test]
fn cancel_suppresses_only_its_queued_edit_and_restores_original_controller() {
    let (application, mut model) = configured();
    let original = source(&model);
    let before = model.key;
    model.begin_draft("source").unwrap();
    let (token, key) = field(&model, "source_seed");
    model
        .apply_draft("source", &token, &changes(&key, "2"))
        .unwrap();
    model.cancel_draft(&token).unwrap();
    drain(&mut model);
    assert_eq!(model.draft_state()["outcome"], "cancelled");
    assert_eq!(model.key, before);
    assert_eq!(source(&model), original);
    assert!(application.active().is_none());
    model.begin_draft("source").unwrap();
    let token = model.draft_state()["token"].as_str().unwrap().to_owned();
    model.apply(DOCUMENT).unwrap();
    model.cancel_draft(&token).unwrap();
    drain(&mut model);
    assert_ne!(
        model.key, before,
        "Old draft cancellation must not cancel a raw import"
    );
    assert!(model.current());
}

#[test]
fn foreign_tokens_and_sections_cannot_poison_the_current_draft() {
    let (_, mut model) = configured();
    model.begin_draft("source").unwrap();
    let before = model.draft_state();
    let (token, key) = field(&model, "source_seed");
    assert!(model
        .apply_draft("source", "foreign", &changes(&key, "2"))
        .is_err());
    assert!(model
        .apply_draft("receiver", &token, &changes(&key, "2"))
        .is_err());
    assert!(model.cancel_draft("foreign").is_err());
    assert_eq!(model.draft_state(), before);
}

#[test]
fn canonical_upstream_commit_guard_rejects_stale_and_foreign_keys_atomically() {
    let application = ApplicationSessions::desktop().unwrap();
    let initial = application.snapshot(Direction::Upstream).key;
    let downstream = application.snapshot(Direction::Downstream);
    let current = application.configure_upstream_if(initial, true).unwrap();
    let accepted = application.snapshot(Direction::Upstream);
    for invalid in [initial, downstream.key] {
        assert!(application.configure_upstream_if(invalid, false).is_err());
        assert_eq!(application.snapshot(Direction::Upstream), accepted);
        assert_eq!(application.snapshot(Direction::Downstream), downstream);
        assert!(application.active().is_none());
    }
    assert_ne!(initial, current);
}

#[test]
fn stale_open_base_and_stale_preparation_preserve_the_new_canonical_generation() {
    for after_queue in [false, true] {
        let (application, mut model) = configured();
        let original = source(&model);
        let old = model.key;
        model.begin_draft("source").unwrap();
        let (token, field) = field(&model, "source_seed");
        if after_queue {
            model
                .apply_draft("source", &token, &changes(&field, "2"))
                .unwrap();
        }
        let replacement = application.configure_upstream(true).unwrap();
        if after_queue {
            drain(&mut model);
        } else {
            assert!(model
                .apply_draft("source", &token, &changes(&field, "2"))
                .is_err());
        }
        assert_eq!(model.draft_state()["outcome"], "stale");
        assert_eq!(application.snapshot(Direction::Upstream).key, replacement);
        assert_eq!(model.key, old);
        assert_eq!(source(&model), original);
    }
}

struct Control;
impl RunCancellation for Control {
    /// Preserves the fixture lease until explicit terminal disposal.
    /// This nonblocking control never starts a scientific engine.
    fn request_cancel(&self) {}
}

#[test]
fn cancel_draft_during_downstream_execution_does_not_touch_its_lease() {
    let (application, mut model) = configured();
    model.begin_draft("source").unwrap();
    let token = model.draft_state()["token"].as_str().unwrap().to_owned();
    let key = application.configure_downstream(true).unwrap();
    let lease = application.begin(key, Arc::new(Control)).unwrap();
    let run = lease.token();
    model.cancel_draft(&token).unwrap();
    assert_eq!(application.active().unwrap().token, run);
    assert_eq!(application.snapshot(Direction::Downstream).key, key);
    application.finish(lease).unwrap();
}

#[test]
fn draft_admission_rejection_keeps_its_previous_guard_and_releases_after_final_alias() {
    // Synthetic pressure uses an independent accounting ceiling, never an allocation.
    let application = ApplicationSessions::new(RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
        ..Default::default()
    })
    .unwrap();
    let mut model = configured_with(application.clone());
    model.begin_draft("source").unwrap();
    let original = Arc::clone(model.draft.as_ref().unwrap());
    let usage = application.budget().usage();
    let pressure = application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(model.begin_draft("receiver").is_err());
    assert!(Arc::ptr_eq(&original, model.draft.as_ref().unwrap()));
    drop(pressure);
    assert_eq!(application.budget().usage(), usage);
    let guard = original.fixture_reserved_resources();
    model.draft = None;
    assert_eq!(application.budget().usage(), usage);
    drop(original);
    let after = application.budget().usage();
    assert_eq!(after.checked_add(guard).unwrap(), usage);
}
