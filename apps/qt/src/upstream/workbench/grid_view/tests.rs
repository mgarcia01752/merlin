/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real passive grid preparations preserve scientific owners and exact callback context.
use super::super::{
    tests::{configured, wait_for},
    Model, Pending,
};
use super::*;
use crate::{
    shell::{ApplicationSessions, ProjectSnapshot, SessionKey},
    upstream::RetainedReport,
};
use merlin::upstream::RunStatus;
use std::sync::Arc;

const DOCUMENT_25: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-25khz.json");
// Independent existing thin-transport policy, unrelated to scientific cell limits.
const TRANSPORT_BYTE_LIMIT: usize = 4096;

struct Science {
    key: Option<SessionKey>,
    address: usize,
    report: RetainedReport,
    document: String,
    original: Vec<u8>,
    preview: Value,
    downstream: ProjectSnapshot,
    upstream: ProjectSnapshot,
}

impl Science {
    fn new(model: &Model, application: &ApplicationSessions) -> Self {
        let (address, document, original, preview) = model
            .controller
            .as_ref()
            .unwrap()
            .inspect(|simulation| {
                (
                    simulation as *const _ as usize,
                    simulation.document().unwrap().to_json().unwrap(),
                    simulation
                        .document()
                        .unwrap()
                        .original_bytes()
                        .unwrap()
                        .to_vec(),
                    serde_json::to_value(simulation.preview().unwrap()).unwrap(),
                )
            })
            .unwrap();
        Self {
            key: model.key,
            address,
            report: Arc::clone(&model.report.as_ref().unwrap().1),
            document,
            original,
            preview,
            downstream: application.snapshot(Direction::Downstream),
            upstream: application.snapshot(Direction::Upstream),
        }
    }

    fn assert_owner(&self, model: &Model) {
        assert_eq!(model.key, self.key);
        assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &self.report));
        model
            .controller
            .as_ref()
            .unwrap()
            .inspect(|simulation| {
                assert_eq!(simulation as *const _ as usize, self.address);
                assert_eq!(
                    simulation.document().unwrap().to_json().unwrap(),
                    self.document
                );
                assert_eq!(
                    simulation.document().unwrap().original_bytes().unwrap(),
                    self.original
                );
                assert_eq!(
                    serde_json::to_value(simulation.preview().unwrap()).unwrap(),
                    self.preview
                );
            })
            .unwrap();
    }

    fn assert_unchanged(&self, model: &Model, application: &ApplicationSessions) {
        self.assert_owner(model);
        assert_eq!(application.snapshot(Direction::Downstream), self.downstream);
        assert_eq!(application.snapshot(Direction::Upstream), self.upstream);
        assert!(application.active().is_none());
    }
}

fn reported(application: ApplicationSessions) -> Box<Model> {
    let mut model = configured(application.clone());
    let idle = application.budget().usage();
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    assert_eq!(
        model.report.as_ref().unwrap().1.value().status,
        RunStatus::Completed
    );
    let completed = idle
        .checked_add(
            model
                .report
                .as_ref()
                .unwrap()
                .1
                .fixture_reserved_resources(),
        )
        .unwrap();
    wait_for(|| application.budget().usage() == completed);
    model
}

fn open(model: &mut Model) -> Value {
    model.begin_resource_grid().unwrap();
    assert!(matches!(model.pending, Some(Pending::Grid { .. })));
    assert!(model.controller.is_none());
    assert_eq!(model.state()["resource_grid_view"]["status"], "loading");
    wait_for(|| model.poll()["preparing"] == false);
    let view = model.state()["resource_grid_view"].clone();
    assert_eq!(view["status"], "ready", "{}", model.state());
    assert!(view["snapshot"].is_object());
    view
}

fn context(view: &Value) -> Value {
    json!({
        "session_token":view["session_token"],
        "configuration_generation":view["configuration_generation"],
        "target_session_token":view["snapshot_session_token"],
        "target_grant_id":view["snapshot"]["grid"]["grant_id"]
    })
}

fn viewport() -> Value {
    json!({"frame_index":1,"first_symbol":2,"symbol_count":1,
        "first_carrier":127,"carrier_count":3,"max_cells":4096})
}

fn stored_snapshot(model: &Model) -> &Value {
    &model.grid.as_ref().unwrap().1.value().snapshot
}

#[test]
fn default_and_reduced_background_views_preserve_real_report_inputs_and_both_projects() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let science = Science::new(&model, &application);
    let initial_sequence = model.state()["resource_grid_view"]["query_sequence"].clone();
    let first = open(&mut model);
    // Literal desktop population policy stays distinct from the public ceiling.
    assert_eq!(first["max_cells"], 4096);
    assert!(first["max_cells"].as_u64().unwrap() < MAX_RESOURCE_GRID_CELLS as u64);
    assert_ne!(first["query_sequence"], initial_sequence);
    assert_eq!(first["snapshot"]["grid"]["frame_index"], 0);
    assert_eq!(first["snapshot"]["grid"]["symbol_count"], 1);
    assert_eq!(first["snapshot"]["grid"]["carrier_count"], 2048);
    assert_eq!(
        first["snapshot"]["grid"]["cells"].as_array().unwrap().len(),
        2048
    );
    assert_eq!(model.poll()["resource_grid_view"], first);
    science.assert_unchanged(&model, &application);

    model
        .request_resource_grid(&viewport().to_string(), &context(&first).to_string())
        .unwrap();
    assert!(matches!(model.pending, Some(Pending::Grid { .. })));
    assert!(model.controller.is_none());
    let loading = model.state()["resource_grid_view"].clone();
    assert_eq!(loading["status"], "loading");
    assert_eq!(loading["snapshot"], first["snapshot"]);
    assert_ne!(loading["query_sequence"], first["query_sequence"]);
    assert!(
        application.active().is_none(),
        "A passive query cannot take the run lease"
    );
    wait_for(|| model.poll()["preparing"] == false);
    let next = model.state()["resource_grid_view"].clone();
    assert_eq!(next["status"], "ready");
    assert_eq!(next["query_sequence"], loading["query_sequence"]);
    assert_eq!(
        next["configuration_generation"],
        first["configuration_generation"]
    );
    assert_eq!(
        next["snapshot"]["grid"]["grant_id"],
        first["snapshot"]["grid"]["grant_id"]
    );
    assert_eq!(
        next["snapshot"]["grid"]["frame_bits"],
        first["snapshot"]["grid"]["frame_bits"]
    );
    assert_eq!(
        next["snapshot"]["grid"]["grant_bits"],
        first["snapshot"]["grid"]["grant_bits"]
    );
    assert_eq!(next["snapshot"]["grid"]["first_carrier"], 127);
    assert_eq!(
        next["snapshot"]["grid"]["cells"].as_array().unwrap().len(),
        3
    );
    assert_eq!(next["snapshot"]["grid"]["cells"][0]["role"], "excluded");
    assert_eq!(
        next["snapshot"]["grid"]["cells"][1]["physical_carrier"],
        128
    );
    assert_eq!(next["snapshot"]["grid"]["cells"][1]["grant_symbol"], 8);
    assert_eq!(
        next["snapshot"]["grid"]["cells"][1]["prefix_start_sample"],
        "17920"
    );
    science.assert_unchanged(&model, &application);
}

#[test]
fn canonical_callback_identity_is_separate_from_the_actual_backend_generation() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let view = open(&mut model);
    let key = application.snapshot(Direction::Upstream).key;
    let backend = model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| {
            simulation
                .resource_grid(ResourceGridRequest::default())
                .unwrap()
                .configuration_generation
        })
        .unwrap();
    assert_eq!(view["session_token"], identity(key));
    assert_eq!(view["snapshot_session_token"], identity(key));
    assert_eq!(view["configuration_generation"], backend.to_string());
    assert_eq!(
        view["snapshot"]["configuration_generation"],
        backend.to_string()
    );
    let mut wrong = context(&view);
    // A backend scalar never substitutes for the canonical opaque callback token,
    // even if its numeric value happens to equal one component of the shell key.
    wrong["session_token"] = backend.to_string().into();
    assert!(model
        .request_resource_grid(&viewport().to_string(), &wrong.to_string())
        .is_err());
    assert!(model.pending.is_none());
    assert_eq!(stored_snapshot(&model), &view["snapshot"]);
    model
        .request_resource_grid(&viewport().to_string(), &context(&view).to_string())
        .unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.state()["resource_grid_view"]["status"], "ready");
    assert_eq!(
        model.state()["resource_grid_view"]["configuration_generation"],
        backend.to_string()
    );
}

#[test]
fn malformed_unknown_negative_and_oversized_transport_settle_without_dispatch_or_data_loss() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let view = open(&mut model);
    let science = Science::new(&model, &application);
    let request = viewport();
    let accepted_context = context(&view);
    let mut unknown_request = request.clone();
    unknown_request["roles"] = json!([]);
    let mut negative = request.clone();
    negative["first_carrier"] = json!(-1);
    let mut wrong_number = request.clone();
    wrong_number["max_cells"] = json!("16384");
    let mut unknown_context = accepted_context.clone();
    unknown_context["cell_loading"] = json!(4);
    let mut wrong_generation_type = accepted_context.clone();
    wrong_generation_type["configuration_generation"] = json!(1);
    // Keep both oversized documents valid: a parser failure must not substitute
    // for the native byte-limit boundary.
    let request_text = request.to_string();
    let oversized_request = format!(
        "{request_text}{}",
        " ".repeat(TRANSPORT_BYTE_LIMIT + 1 - request_text.len())
    );
    let context_text = accepted_context.to_string();
    let oversized_context = format!(
        "{context_text}{}",
        " ".repeat(TRANSPORT_BYTE_LIMIT + 1 - context_text.len())
    );
    let requests = [
        ("{".into(), accepted_context.to_string()),
        ("{}".into(), accepted_context.to_string()),
        (unknown_request.to_string(), accepted_context.to_string()),
        (negative.to_string(), accepted_context.to_string()),
        (wrong_number.to_string(), accepted_context.to_string()),
        (oversized_request, accepted_context.to_string()),
        (request.to_string(), "{".into()),
        (request.to_string(), "{}".into()),
        (request.to_string(), unknown_context.to_string()),
        (request.to_string(), wrong_generation_type.to_string()),
        (request.to_string(), oversized_context),
    ];
    for (request, context) in requests {
        let rejected = model.request_resource_grid(&request, &context).unwrap_err();
        if request.len() > TRANSPORT_BYTE_LIMIT || context.len() > TRANSPORT_BYTE_LIMIT {
            assert!(rejected.contains("transport byte limit"));
        }
        assert!(model.pending.is_none());
        let rejected = model.state()["resource_grid_view"].clone();
        assert_eq!(rejected["status"], "error");
        assert!(!rejected["reason"].as_str().unwrap().is_empty());
        assert_eq!(rejected["query_sequence"], view["query_sequence"]);
        assert_eq!(rejected["snapshot"], view["snapshot"]);
        assert_eq!(model.state()["preparing"], false);
        science.assert_unchanged(&model, &application);
    }
}

#[test]
fn unconfigured_default_query_rejects_without_synthesizing_a_session_or_advancing_either_project() {
    let application = ApplicationSessions::desktop().unwrap();
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let mut model = Model::new(application.clone()).unwrap();
    assert!(model.begin_resource_grid().is_err());
    assert!(model.pending.is_none());
    assert!(model.controller.is_none());
    assert!(model.grid.is_none());
    let view = model.state()["resource_grid_view"].clone();
    assert_eq!(view["status"], "error");
    assert_eq!(view["snapshot"], Value::Null);
    assert_eq!(view["query_sequence"], "0");
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert!(application.active().is_none());
}

#[test]
fn stale_foreign_and_wrong_grant_contexts_preserve_the_exact_previous_view_and_report() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let view = open(&mut model);
    let science = Science::new(&model, &application);
    let accepted = context(&view);
    let foreign = identity(application.snapshot(Direction::Downstream).key);
    let mut wrong_session = accepted.clone();
    wrong_session["session_token"] = foreign.clone().into();
    let mut wrong_generation = accepted.clone();
    wrong_generation["configuration_generation"] = json!("stale-backend-generation");
    let mut wrong_target = accepted.clone();
    wrong_target["target_session_token"] = foreign.into();
    let mut wrong_grant = accepted.clone();
    wrong_grant["target_grant_id"] = json!("another-grant");
    for context in [wrong_session, wrong_generation, wrong_target, wrong_grant] {
        assert!(model
            .request_resource_grid(&viewport().to_string(), &context.to_string())
            .is_err());
        assert!(model.pending.is_none());
        assert_eq!(model.state()["resource_grid_view"]["status"], "error");
        assert_eq!(
            model.state()["resource_grid_view"]["query_sequence"],
            view["query_sequence"]
        );
        assert_eq!(stored_snapshot(&model), &view["snapshot"]);
        science.assert_unchanged(&model, &application);
    }
}

#[test]
fn invalid_public_bounds_reach_the_real_grid_worker_and_preserve_old_accepted_owners() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let view = open(&mut model);
    let science = Science::new(&model, &application);
    let mut invalid_frame = viewport();
    invalid_frame["frame_index"] = json!(2);
    let mut invalid_start = viewport();
    invalid_start["first_carrier"] = json!(2048);
    let mut zero_budget = viewport();
    zero_budget["max_cells"] = json!(0);
    let mut excessive_budget = viewport();
    // Independent literal anchor for the Qt policy, not the larger public cap.
    excessive_budget["max_cells"] = json!(4097);
    assert!(model
        .request_resource_grid(&excessive_budget.to_string(), &context(&view).to_string())
        .unwrap_err()
        .contains("Qt viewport"));
    assert!(model.pending.is_none());
    assert_eq!(stored_snapshot(&model), &view["snapshot"]);
    science.assert_unchanged(&model, &application);
    let mut insufficient_budget = viewport();
    insufficient_budget["max_cells"] = json!(2);
    let mut sequence = view["query_sequence"].clone();
    for request in [
        invalid_frame,
        invalid_start,
        zero_budget,
        insufficient_budget,
    ] {
        model
            .request_resource_grid(&request.to_string(), &context(&view).to_string())
            .unwrap();
        // Parsing/context/transport succeeded: an earlier failure cannot stand
        // in for the public resource-grid bounds check under review.
        assert!(matches!(model.pending, Some(Pending::Grid { .. })));
        assert!(model.controller.is_none());
        let queued = model.state()["resource_grid_view"].clone();
        assert_eq!(queued["status"], "loading");
        assert_ne!(queued["query_sequence"], sequence);
        sequence = queued["query_sequence"].clone();
        assert!(application.active().is_none());
        wait_for(|| model.poll()["preparing"] == false);
        let rejected = model.state()["resource_grid_view"].clone();
        assert_eq!(rejected["status"], "error");
        assert!(rejected["reason"]
            .as_str()
            .unwrap()
            .contains("resource_grid"));
        assert_eq!(rejected["query_sequence"], sequence);
        assert_eq!(stored_snapshot(&model), &view["snapshot"]);
        science.assert_unchanged(&model, &application);
    }
}

#[test]
fn stop_discards_a_queued_viewport_and_restores_the_same_controller_and_previous_display() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let view = open(&mut model);
    let science = Science::new(&model, &application);
    model
        .request_resource_grid(&viewport().to_string(), &context(&view).to_string())
        .unwrap();
    assert!(matches!(model.pending, Some(Pending::Grid { .. })));
    let sequence = model.state()["resource_grid_view"]["query_sequence"].clone();
    model.stop().unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    let stopped = model.state()["resource_grid_view"].clone();
    assert_eq!(stopped["status"], "unavailable");
    assert!(stopped["reason"].as_str().unwrap().contains("cancelled"));
    assert_eq!(stopped["query_sequence"], sequence);
    assert_eq!(stopped["snapshot"], view["snapshot"]);
    science.assert_unchanged(&model, &application);
    let retry = open(&mut model);
    assert_ne!(retry["query_sequence"], sequence);
    assert_eq!(retry["snapshot"], view["snapshot"]);
    science.assert_unchanged(&model, &application);
}

#[test]
fn canonical_generation_change_drops_the_queued_candidate_and_restores_the_original_controller() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let before_grid = application.budget().usage();
    let view = open(&mut model);
    let science = Science::new(&model, &application);
    // Idle worker teardown may lag Poll; measure only after the actual stable baseline.
    let stable = before_grid
        .checked_add(model.grid.as_ref().unwrap().1.fixture_reserved_resources())
        .unwrap();
    wait_for(|| application.budget().usage() == stable);
    model
        .request_resource_grid(&viewport().to_string(), &context(&view).to_string())
        .unwrap();
    assert!(matches!(model.pending, Some(Pending::Grid { .. })));
    let replacement = application.configure_upstream(true).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.state()["resource_grid_view"]["status"], "error");
    assert!(model
        .error
        .message()
        .contains("changed during resource observation"));
    assert_eq!(stored_snapshot(&model), &view["snapshot"]);
    assert_eq!(model.state()["resource_grid_view"]["snapshot"], Value::Null);
    science.assert_owner(&model);
    assert_eq!(
        application.snapshot(Direction::Downstream),
        science.downstream
    );
    assert_eq!(application.snapshot(Direction::Upstream).key, replacement);
    assert!(application.active().is_none());
    // This stale diagnostic is static and readiness retains the same fixed layout.
    // Exact final return proves that candidate/slot/task/delivery guards all release.
    wait_for(|| application.budget().usage() == stable);
}

#[test]
fn accepted_reconfiguration_clears_the_old_grid_and_rejects_its_original_callback_context() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let old = open(&mut model);
    let old_report = Arc::clone(&model.report.as_ref().unwrap().1);
    let downstream = application.snapshot(Direction::Downstream);
    model.apply(DOCUMENT_25).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    assert!(model.grid.is_none());
    assert!(model.report.is_none());
    let unavailable = model.state()["resource_grid_view"].clone();
    assert_eq!(unavailable["status"], "unavailable");
    assert_eq!(unavailable["snapshot"], Value::Null);
    assert_eq!(unavailable["query_sequence"], old["query_sequence"]);
    assert_ne!(unavailable["session_token"], old["session_token"]);
    assert!(model
        .request_resource_grid(&viewport().to_string(), &context(&old).to_string())
        .is_err());
    assert!(model.pending.is_none());
    assert!(model.grid.is_none());
    assert_eq!(
        model.state()["resource_grid_view"]["query_sequence"],
        old["query_sequence"]
    );
    assert_eq!(old_report.value().status, RunStatus::Completed);
    let current = open(&mut model);
    assert_ne!(
        current["configuration_generation"],
        old["configuration_generation"]
    );
    assert_eq!(current["snapshot"]["grid"]["spacing_hz"], 25_000);
    assert_eq!(current["snapshot"]["grid"]["carrier_count"], 4096);
    assert_eq!(current["snapshot_session_token"], current["session_token"]);
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert!(application.active().is_none());
}
