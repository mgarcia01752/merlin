/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real native selectors retain the actual owner and reject stale view context.
use super::super::tests::{configured, wait_for};
use super::*;
use crate::shell::ApplicationSessions;
use merlin::upstream::{ConfigurationDocument, ReceiverObservationPolicy, Simulation};
use std::sync::Arc;

const DOCUMENT: &[u8] = include_bytes!("../../../../../../examples/cli/upstream/us-50khz.json");

fn settle(model: &mut Model) -> Value {
    wait_for(|| model.poll()["preparing"] == false);
    model.instrument_state()
}

fn context(view: &Value) -> String {
    json!({"query_sequence":view["query_sequence"],"session_token":view["session_token"],
        "configuration_generation":view["configuration_generation"],"run":view["run"]})
    .to_string()
}

fn response(source: &str, metric: &str) -> String {
    json!({"source":source,"metric":metric,"grant_symbol":0,"retained_index":0,
        "first":0,"count":0})
    .to_string()
}

#[test]
fn idle_configured_response_is_passive_and_missing_received_history_stays_unavailable() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let original = model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| {
            simulation
                .document()
                .unwrap()
                .original_bytes()
                .unwrap()
                .to_vec()
        })
        .unwrap();
    assert!(model.begin_instrument("qam").is_err());
    model.begin_instrument("response").unwrap();
    assert!(model.controller.is_none());
    assert_eq!(model.instrument_state()["status"], "loading");
    let first = settle(&mut model);
    assert_eq!(first["status"], "ready", "{first}");
    assert_eq!(
        first["snapshot"]["metadata"]["reference_plane"],
        "configured_deterministic_physical_h"
    );
    assert_eq!(first["run"], Value::Null);
    model
        .request_instrument(
            "response",
            &response("received_estimate", "amplitude"),
            &context(&first),
        )
        .unwrap();
    let absent = settle(&mut model);
    assert_eq!(absent["status"], "ready");
    assert_eq!(absent["snapshot"]["status"], "unavailable");
    assert_eq!(absent["snapshot"]["viewport"]["points"], json!([]));
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert!(application.active().is_none());
    assert!(model.report.is_none());
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| {
            assert_eq!(
                simulation.document().unwrap().original_bytes().unwrap(),
                original
            )
        })
        .unwrap();
}

#[test]
fn workbench_retains_one_actual_estimate_without_changing_recovery_or_public_defaults() {
    let document = ConfigurationDocument::from_json(DOCUMENT).unwrap();
    let (configuration, options) = document.prepare().unwrap();
    let mut reference = Simulation::new(options).unwrap();
    reference.configure(configuration).unwrap();
    let reference = reference.run().unwrap();
    assert!(reference.receiver_observations().frames.is_empty());
    assert_eq!(
        ReceiverObservationPolicy::default().info().maximum_frames,
        0
    );
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let retained = Arc::clone(&model.report.as_ref().unwrap().1);
    let actual = retained.value();
    assert_eq!(actual.receiver_observations().frames.len(), 1);
    assert_eq!(actual.compared_source_bits, reference.compared_source_bits);
    assert_eq!(actual.bit_errors, reference.bit_errors);
    assert_eq!(actual.reference_recovered, reference.reference_recovered);
    model.begin_instrument("response").unwrap();
    let configured = settle(&mut model);
    model
        .request_instrument(
            "response",
            &response("received_estimate", "phase"),
            &context(&configured),
        )
        .unwrap();
    let received = settle(&mut model);
    assert_eq!(received["snapshot"]["status"], "ready", "{received}");
    assert_eq!(
        received["snapshot"]["choices"]["frames"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        received["snapshot"]["metadata"]["frame"]["identity"]["run"],
        actual.receiver_observations().frames[0]
            .identity
            .run
            .to_string()
    );
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &retained));
    assert!(application.active().is_none());
}

#[test]
fn superseded_query_context_cancellation_and_configuration_changes_preserve_science() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    model.begin_instrument("response").unwrap();
    let first = settle(&mut model);
    model
        .request_instrument(
            "response",
            &response("cm_pre_eq", "phase"),
            &context(&first),
        )
        .unwrap();
    let second = settle(&mut model);
    assert_ne!(first["query_sequence"], second["query_sequence"]);
    assert!(model
        .request_instrument(
            "response",
            &response("physical_channel", "amplitude"),
            &context(&first)
        )
        .unwrap_err()
        .contains("context changed"));
    assert_eq!(model.instrument_state()["snapshot"], second["snapshot"]);
    model
        .request_instrument(
            "response",
            &response("physical_channel", "amplitude"),
            &context(&second),
        )
        .unwrap();
    model.stop().unwrap();
    assert_eq!(settle(&mut model)["status"], "unavailable");
    assert_eq!(model.instrument_state()["snapshot"], second["snapshot"]);
    assert!(model.controller.is_some());
    assert!(application.active().is_none());
    model.begin_instrument("response").unwrap();
    application.configure_upstream(true).unwrap();
    let stale = settle(&mut model);
    assert_eq!(stale["status"], "error");
    assert_eq!(stale["snapshot"], Value::Null);
    assert!(model.controller.is_some());
}
