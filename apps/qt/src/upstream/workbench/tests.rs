/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public upstream runs through the production Qt application model.
use super::*;
use crate::shell::DispatchPhase;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

const DOCUMENT: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
// Async scheduling policies are independent of scientific sample/runtime limits.
const COMPLETION_DEADLINE: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(2);
// Enough actual grants to observe cooperative pause before normal completion.
const PAUSE_FIXTURE_GRANTS: usize = 16;

fn wait_for(mut predicate: impl FnMut() -> bool) {
    let deadline = Instant::now() + COMPLETION_DEADLINE;
    while !predicate() {
        assert!(
            Instant::now() < deadline,
            "Upstream model did not reach its bounded acceptance state"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn configured(application: ApplicationSessions) -> Model {
    let mut model = Model::new(application);
    model.apply(DOCUMENT).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    assert_eq!(model.state()["has_result"], false);
    model
}

#[test]
fn complete_run_uses_actual_recovery_and_invalid_apply_preserves_accepted_owners() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    let key = model.key;
    assert!(model.state()["session"].is_string());
    assert!(model.state()["generation"].is_string());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let (token, report) = model
        .report
        .as_ref()
        .expect("Actual scientific completion must publish its report");
    assert_eq!(report.value().status, RunStatus::Completed);
    assert_eq!(
        model.state()["result"]["bits_tested"],
        report.value().compared_source_bits
    );
    assert_eq!(
        model.state()["result"]["bit_errors"],
        report.value().bit_errors
    );
    assert_eq!(
        model.state()["result"]["ber"],
        serde_json::to_value(report.value().ber).unwrap()
    );
    assert_eq!(model.state()["result"]["run"], token.run.to_string());
    let report = Arc::clone(report);
    model.apply(b"{}").unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(!model.error.message().is_empty());
    assert_eq!(model.key, key);
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert!(model.state()["can_start"].as_bool().unwrap());
    assert!(application.active().is_none());
}

#[test]
fn stopping_target_preparation_restores_the_same_controller_without_starting_science() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    let key = model.key;
    model.start().unwrap();
    assert!(model.pending.is_some());
    model.stop().unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.key, key);
    assert!(model.controller.is_some());
    assert!(model.report.is_none());
    assert!(application.active().is_none());
    assert!(model.state()["can_start"].as_bool().unwrap());
}

#[test]
fn queued_configuration_cannot_overwrite_a_newer_canonical_generation() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = configured(application.clone());
    let old = model.key;
    model.apply(DOCUMENT).unwrap();
    let replacement = application.configure_upstream(true).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.key, old);
    assert_eq!(application.snapshot(Direction::Upstream).key, replacement);
    assert!(model.error.message().contains("changed during preparation"));
    assert!(model.start().is_err());
    assert!(application.active().is_none());
}

#[test]
fn paused_stop_keeps_the_real_global_lease_until_terminal_and_retains_partial_results() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone());
    let mut document: serde_json::Value = serde_json::from_slice(DOCUMENT).unwrap();
    document["simulation"]["grants"] = PAUSE_FIXTURE_GRANTS.into();
    model
        .apply(&serde_json::to_vec(&document).unwrap())
        .unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.current(), "{}", model.state());
    model.start().unwrap();
    wait_for(|| {
        model.poll();
        model.is_running()
    });
    model.pause().unwrap();
    wait_for(|| {
        model.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    let token = application.active().unwrap().token;
    assert!(model.start().is_err());
    model.stop().unwrap();
    assert_eq!(
        application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    assert_eq!(application.active().unwrap().token, token);
    wait_for(|| model.poll()["busy"] == false);
    assert!(application.active().is_none());
    let (actual_token, report) = model
        .report
        .as_ref()
        .expect("Actual cancellation retains its scientific prefix");
    assert_eq!(*actual_token, token);
    assert_eq!(report.value().status, RunStatus::Cancelled);
    let alias = Arc::clone(report);
    drop(model);
    assert_eq!(alias.value().status, RunStatus::Cancelled);
}

#[test]
fn stopped_or_destroyed_configuration_preparation_never_adopts_a_candidate() {
    let application = ApplicationSessions::desktop().unwrap();
    let initial_usage = application.budget().usage();
    let mut model = configured(application.clone());
    let key = model.key;
    model.apply(DOCUMENT).unwrap();
    model.stop().unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.key, key);
    assert_eq!(application.snapshot(Direction::Upstream).key, key.unwrap());
    model.apply(DOCUMENT).unwrap();
    drop(model);
    // Destruction drops the receiver, so completion can only release its candidate.
    wait_for(|| application.budget().usage() == initial_usage);
    assert_eq!(application.snapshot(Direction::Upstream).key, key.unwrap());
    assert!(application.active().is_none());
}

#[test]
fn exhausted_input_admission_rejects_before_copy_and_preserves_the_accepted_controller() {
    // Synthetic counter pressure exercises real input admission without allocating
    // a matching payload. Independent test-host policy, not a scientific estimate.
    const TEST_PAYLOAD_LIMIT: usize = 64 * 1024 * 1024;
    const TEST_ORDERED_ENTRY_LIMIT: usize = 100_000;
    const TEST_SHARED_ROOT_LIMIT: usize = 10_000;
    let application = ApplicationSessions::new(RetainedResources {
        payload_bytes: TEST_PAYLOAD_LIMIT,
        ordered_entries: TEST_ORDERED_ENTRY_LIMIT,
        shared_roots: TEST_SHARED_ROOT_LIMIT,
    })
    .unwrap();
    let mut model = configured(application.clone());
    let key = model.key;
    let before = application.budget().usage();
    let pressure = application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: TEST_PAYLOAD_LIMIT - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(model.apply(DOCUMENT).is_err());
    assert!(model.pending.is_none());
    assert_eq!(model.key, key);
    assert!(model.current());
    assert!(model.controller.is_some());
    drop(pressure);
    assert_eq!(application.budget().usage(), before);
}

// Independent accounting ceilings for the actual queued failure pressure fixtures.
const FAILURE_TEST_LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

#[test]
fn stale_configuration_error_keeps_its_charge_until_discard_without_replacing_old_report() {
    let application = ApplicationSessions::new(FAILURE_TEST_LIMITS).unwrap();
    let mut model = configured(application.clone());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let (token, report) = model.report.as_ref().unwrap();
    let token = *token;
    let report = Arc::clone(report);
    let baseline = application.budget().usage();
    let key = model.key;
    model.apply(b"{}").unwrap();
    let mut prepared = None;
    wait_for(|| {
        prepared = model.pending.as_ref().and_then(Pending::poll);
        prepared.is_some()
    });
    model.pending = None;
    let Prepared::Configuration {
        expected,
        candidate: Err(error),
    } = prepared.unwrap()
    else {
        panic!("Invalid real document must yield its original preparation error");
    };
    let crate::shared::worker_failure::WorkerFailure::Detailed(retained) = &error else {
        panic!("Original bounded preparation text fits the fixture host");
    };
    let charge = retained.fixture_reserved_resources();
    let queued = RetainedResources {
        payload_bytes: baseline.payload_bytes + charge.payload_bytes,
        ..baseline
    };

    // Receiving completion does not prove closure return; wait for input disposal.
    wait_for(|| application.budget().usage() == queued);
    application.configure_upstream(true).unwrap();
    let result = model.adopt_prepared(Prepared::Configuration {
        expected,
        candidate: Err(error),
    });
    assert!(result
        .unwrap_err()
        .message()
        .contains("changed during preparation"));
    assert_eq!(model.key, key);
    assert_eq!(model.report.as_ref().unwrap().0, token);
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert_eq!(application.budget().usage(), baseline);
}

#[test]
fn target_preparation_fallback_restores_controller_and_same_completed_report() {
    let application = ApplicationSessions::new(FAILURE_TEST_LIMITS).unwrap();
    let mut model = configured(application.clone());
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    let (token, report) = model.report.as_ref().unwrap();
    let token = *token;
    let report = Arc::clone(report);
    let key = model.key;
    let budget = application.budget();
    let before = budget.usage();

    // Actual target preflight runs before shared admission. Synthetic byte pressure
    // rejects that reservation and then the detailed error, selecting borrowed text.
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: FAILURE_TEST_LIMITS.payload_bytes - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    model.start().unwrap();
    assert!(model.pending.is_some());
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(
        model.error.message(),
        crate::shared::presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE
    );
    assert_eq!(model.key, key);
    assert!(model.controller.is_some());
    assert_eq!(model.report.as_ref().unwrap().0, token);
    assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &report));
    assert!(application.active().is_none());
    assert_eq!(
        budget.usage().payload_bytes,
        FAILURE_TEST_LIMITS.payload_bytes
    );
    drop(pressure);
    assert_eq!(budget.usage(), before);
}

#[test]
fn queued_preparation_error_moves_to_persistent_status_without_recharging_under_pressure() {
    const MESSAGE: &str = "Original admitted upstream preparation detail.";
    let application = ApplicationSessions::new(FAILURE_TEST_LIMITS).unwrap();
    let budget = application.budget().clone();
    let baseline = budget.usage();
    let mut model = Model::new(application.clone());
    let failure = WorkerFailure::try_admit(MESSAGE.to_owned().into(), &budget).unwrap();
    let used = budget.usage();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: FAILURE_TEST_LIMITS.payload_bytes - used.payload_bytes,
            ordered_entries: FAILURE_TEST_LIMITS.ordered_entries - used.ordered_entries,
            shared_roots: FAILURE_TEST_LIMITS.shared_roots - used.shared_roots,
        })
        .unwrap();
    let prepared = Prepared::Configuration {
        expected: application.snapshot(Direction::Upstream).key,
        candidate: Err(failure),
    };
    let error = model.adopt_prepared(prepared).unwrap_err();
    assert!(error.owns(&budget));
    model.error = error;
    assert_eq!(model.error.message(), MESSAGE);
    assert_eq!(model.state()["error"], MESSAGE);
    assert!(model.controller.is_none());
    assert!(model.report.is_none());
    assert_eq!(budget.usage(), FAILURE_TEST_LIMITS);
    drop(pressure);
    assert_eq!(budget.usage(), used);
    drop(model);
    assert_eq!(budget.usage(), baseline);
}
