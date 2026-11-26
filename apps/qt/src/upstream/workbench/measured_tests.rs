/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual measured-plan admission through the Qt model, using authored wire fixtures.
// These sanitized synthetic captures are not field or vendor interoperability evidence.
use super::{
    tests::{configured, wait_for},
    *,
};
use crate::{platform::input_file, upstream::RetainedReport};
use merlin::upstream::{inputs::MeasuredExperiment, BatchReport};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{path::Path, sync::Arc};

const PLAN_50: &[u8] =
    include_bytes!("../../../../../examples/measurements/upstream/measured/capture-driven.json");
const PLAN_25: &[u8] = include_bytes!(
    "../../../../../examples/measurements/upstream/measured/capture-driven-25khz.json"
);
const SYNTHETIC: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const SECTIONS: [&str; 7] = [
    "ucd",
    "profiles",
    "grants",
    "source",
    "receiver",
    "channel",
    "preEqualization",
];

fn owner_address(model: &Model) -> usize {
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| simulation as *const _ as usize)
        .unwrap()
}

fn with_report(application: ApplicationSessions) -> Box<Model> {
    let mut model = configured(application);
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    assert!(model.report.is_some());
    model
}

fn assert_provenance(report: &BatchReport, bytes: &[u8]) {
    let plan = MeasuredExperiment::from_json(bytes).unwrap();
    let actual_text = report.measurement_provenance_json.as_deref().unwrap();
    assert_eq!(actual_text, plan.provenance_json().unwrap());
    let actual: Value = serde_json::from_str(actual_text).unwrap();
    assert_eq!(actual["format"], "merlin.upstream.measured-provenance");
    assert_eq!(actual["original_plan_bytes"], json!(bytes));
    assert_eq!(actual["plan"], serde_json::to_value(&plan).unwrap());
    assert_eq!(plan.original_bytes().unwrap(), bytes);
    let mut expected_hashes = Vec::new();
    for (family, source) in plan
        .ucd_fragments
        .iter()
        .map(|source| ("ucd", source))
        .chain(plan.rxmer.iter().map(|source| ("rxmer", source)))
        .chain(
            plan.pre_eq_initial
                .iter()
                .map(|source| ("pre_eq_initial", source)),
        )
        .chain(
            plan.pre_eq_events
                .iter()
                .map(|event| ("pre_eq_event", &event.source)),
        )
    {
        expected_hashes.push(json!({"family":family,"id":source.id,
            "sha256":format!("{:x}", Sha256::digest(&source.bytes))}));
    }
    assert_eq!(actual["source_hashes"], json!(expected_hashes));
}

fn run_measured(bytes: &[u8], spacing_hz: u32) {
    let application = ApplicationSessions::desktop().unwrap();
    let downstream = application.snapshot(Direction::Downstream);
    let mut model = Model::new(application.clone()).unwrap();
    model.apply(bytes).unwrap();
    assert!(model.report.is_none());
    assert!(application.active().is_none());
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    assert!(model.controller.as_ref().unwrap().has_measured_input());
    assert_eq!(model.configuration_presentation()["input_mode"], "measured");
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| {
            let config = simulation.configuration().unwrap();
            assert_eq!(
                config.us_configuration.waveform().numerology().spacing_hz(),
                spacing_hz
            );
            assert!(config.rxmer.is_some());
            assert!(config.pre_eq_updates.is_some());
        })
        .unwrap();
    assert!(model.report.is_none(), "Loading a plan must remain idle");
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let report = model.report.as_ref().unwrap().1.value();
    assert_eq!(report.status, RunStatus::Completed);
    assert!(report.reference_recovered, "{:?}", report.failure);
    assert!(report.compared_source_bits > 0);
    assert_eq!(report.bit_errors, 0);
    assert_provenance(report, bytes);
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert!(application.active().is_none());
}

#[test]
fn native_measured_path_is_protected_without_starting_or_changing_downstream() {
    let application = ApplicationSessions::desktop().unwrap();
    let downstream = application.snapshot(Direction::Downstream);
    let mut model = Model::new(application.clone()).unwrap();
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/measurements/upstream/measured/capture-driven.json")
        .canonicalize()
        .unwrap();
    let bytes = std::fs::read(&path).unwrap();
    assert_eq!(bytes, PLAN_50);
    assert_eq!(
        format!("{:x}", Sha256::digest(&bytes)),
        "3ac802bc33ae1137472d19f728d1be0cd30e267b2dd1926f6db34b4d84ecc374"
    );
    let uri = url::Url::from_file_path(&path).unwrap().to_string();
    assert!(model.load_configuration_uri(Some(&uri)).unwrap());
    assert!(application.active().is_none());
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    assert!(model.controller.as_ref().unwrap().has_measured_input());
    assert_eq!(
        model.state()["configuration_view"]["input_mode"],
        "measured"
    );
    assert!(model.report.is_none());
    assert!(input_file::is_original(&application, &path));
    application.clear_input_paths(Direction::Downstream);
    assert!(input_file::is_original(&application, &path));
    let key = model.key;
    assert!(!model.load_configuration_uri(None).unwrap());
    assert_eq!(model.key, key);
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert!(application.active().is_none());
    assert_eq!(std::fs::read(path).unwrap(), bytes);
}

#[test]
fn actual_50khz_measured_run_keeps_full_plan_sources_and_recovery() {
    run_measured(PLAN_50, 50_000);
}

#[test]
fn actual_25khz_measured_run_keeps_full_plan_sources_and_recovery() {
    run_measured(PLAN_25, 25_000);
}

#[test]
fn all_seven_measured_drafts_are_read_only_and_forged_edits_preserve_science() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    model.apply(PLAN_50).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let key = model.key;
    let owner = owner_address(&model);
    let report = Arc::clone(&model.report.as_ref().unwrap().1);
    let downstream = application.snapshot(Direction::Downstream);
    for section in SECTIONS {
        model.begin_draft(section).unwrap();
        let draft = model.draft_state();
        assert_eq!(draft["options"]["read_only"], true, "{section}: {draft}");
        assert!(draft["options"]["fields"].as_array().unwrap().is_empty());
        assert_eq!(draft["can_apply"], false);
        assert_eq!(draft["pending"], false);
        let explanation = draft["error"].as_str().unwrap().to_lowercase();
        assert!(explanation.contains("measured"), "{explanation}");
        assert!(explanation.contains("reload"), "{explanation}");
        let token = draft["token"].as_str().unwrap();
        assert!(model
            .apply_draft(section, token, r#"{"field/0":"2"}"#)
            .is_err());
        assert!(model.pending.is_none());
        model.cancel_draft(token).unwrap();
        assert_eq!(model.draft_state()["outcome"], "cancelled");
        assert_eq!(model.key, key);
        assert_eq!(owner_address(&model), owner);
        assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
        assert_provenance(report.value(), PLAN_50);
        assert!(application.active().is_none());
    }
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
}

fn rejected_measured_change(pointer: &str, replacement: Value) {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = with_report(application.clone());
    let key = model.key;
    let owner = owner_address(&model);
    let report: RetainedReport = Arc::clone(&model.report.as_ref().unwrap().1);
    let downstream = application.snapshot(Direction::Downstream);
    let mut changed: Value = serde_json::from_slice(PLAN_50).unwrap();
    *changed.pointer_mut(pointer).unwrap() = replacement;
    model.apply(&serde_json::to_vec(&changed).unwrap()).unwrap();
    assert!(model.pending.is_some());
    wait_for(|| model.poll()["preparing"] == false);
    assert!(
        !model.error.message().is_empty(),
        "{pointer}: {}",
        model.state()
    );
    assert_eq!(model.key, key);
    assert_eq!(owner_address(&model), owner);
    assert!(model.current());
    assert!(!model.controller.as_ref().unwrap().has_measured_input());
    assert_eq!(
        model.configuration_presentation()["input_mode"],
        "synthetic"
    );
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert!(report.value().measurement_provenance_json.is_none());
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert!(application.active().is_none());
}

#[test]
fn invalid_measured_association_keeps_accepted_owner_key_and_report() {
    rejected_measured_change("/association/if_index", json!(7));
}

#[test]
fn invalid_measured_update_base_keeps_accepted_owner_key_and_report() {
    rejected_measured_change("/pre_eq_events/0/base_bank", json!(99));
}

#[test]
fn unsupported_rxmer_nonunity_update_composition_keeps_accepted_owner_and_report() {
    // Existing literal .5 correction on one bin conflicts with unity-residual RxMER.
    rejected_measured_change("/pre_eq_events/0/source/bytes/34", json!(32));
}

#[test]
fn stopped_measured_preparation_preserves_original_controller_and_report() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = with_report(application.clone());
    let key = model.key;
    let owner = owner_address(&model);
    let report = Arc::clone(&model.report.as_ref().unwrap().1);
    model.apply(PLAN_50).unwrap();
    assert!(model.pending.is_some());
    model.stop().unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.key, key);
    assert_eq!(owner_address(&model), owner);
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert!(!model.controller.as_ref().unwrap().has_measured_input());
    assert!(model.state()["can_start"].as_bool().unwrap());
    assert!(application.active().is_none());
}

#[test]
fn measured_preparation_cannot_adopt_over_a_newer_canonical_generation() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = with_report(application.clone());
    let key = model.key;
    let owner = owner_address(&model);
    let report = Arc::clone(&model.report.as_ref().unwrap().1);
    model.apply(PLAN_50).unwrap();
    let replacement = application.configure_upstream(true).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.key, key);
    assert_eq!(owner_address(&model), owner);
    assert_eq!(application.snapshot(Direction::Upstream).key, replacement);
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert!(!model.controller.as_ref().unwrap().has_measured_input());
    assert!(model.error.message().contains("changed during preparation"));
    assert!(model.start().is_err());
    assert!(application.active().is_none());
}

#[test]
fn explicit_synthetic_reload_restores_editability_and_preserves_old_measured_alias() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application).unwrap();
    model.apply(PLAN_50).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let measured = Arc::clone(&model.report.as_ref().unwrap().1);
    model.apply(SYNTHETIC).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(!model.controller.as_ref().unwrap().has_measured_input());
    assert_eq!(
        model.configuration_presentation()["input_mode"],
        "synthetic"
    );
    model.begin_draft("source").unwrap();
    let draft = model.draft_state();
    assert_ne!(draft["options"]["read_only"], true);
    assert!(!draft["options"]["fields"].as_array().unwrap().is_empty());
    assert_eq!(draft["can_apply"], true);
    assert!(model.report.is_none());
    assert_provenance(measured.value(), PLAN_50);
}
