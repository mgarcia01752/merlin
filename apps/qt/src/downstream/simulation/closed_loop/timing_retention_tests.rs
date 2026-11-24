/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real Model Start and stale-worker timing ownership, without a fake backend.
use super::*;
use crate::downstream::{instruments::receiver::performance, simulation::run_timing};
use crate::shell::{ApplicationSessions, DispatchPhase, RetainedResources};
use std::time::{Duration, Instant};

// Independent test-only host ceilings and scheduling watchdogs.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(5);

// Existing supported complete/pausable observation and separate host execution budget.
const COMPLETE_OBSERVATION_MS: u64 = 1;
const PAUSABLE_OBSERVATION_MS: u64 = 1_000;
const ACTIVE_RUNTIME_MS: u64 = 60_000;

// Existing public performance-view selectors: latest entry and complete source span.
const LATEST_PROFILE: i32 = -1;
const FIRST_POINT: i32 = 0;
const FULL_SPAN: i32 = 0;

fn prepared_model(duration_ms: u64) -> Model {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let (inputs, transmission, sections) = tests::accepted_inputs();
    crate::downstream::workbench::session::restore(&context, inputs, sections).unwrap();
    let mut model = Model::for_context(context);
    model
        .accept_options(
            Some(Options {
                duration_ms,
                max_runtime_ms: ACTIVE_RUNTIME_MS,
                ..Default::default()
            }),
            true,
            &serde_json::to_string(&transmission).unwrap(),
        )
        .unwrap();
    model
}

fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !ready() {
        assert!(
            Instant::now() < deadline,
            "Actual timing-owner lifecycle watchdog"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn timing_rejection_after_adapter_admission_preserves_actual_old_model_and_history() {
    for bytes in [true, false] {
        let mut model = prepared_model(COMPLETE_OBSERVATION_MS);
        model.start();
        wait_for(|| model.poll()["busy"] == false);
        let context = model.context.clone();
        let old_result = latest(&context).unwrap();
        assert_eq!(
            old_result.job.status,
            merlin::downstream::ChannelJobStatus::Complete
        );
        let old_timing = Arc::clone(model.timing.as_ref().unwrap());
        let old_request = Arc::clone(model.request.as_ref().unwrap());
        let old_plan = Arc::clone(model.plan.as_ref().unwrap());
        let token = model.token;
        let generation = model.generation;
        let snapshot = context.application.snapshot(Direction::Downstream);
        let history = performance::view(&context, LATEST_PROFILE, FIRST_POINT, FULL_SPAN);
        let timing = old_timing.snapshot();
        let science = serde_json::to_value(old_result.backend().report()).unwrap();
        let budget = context.application.budget();
        let baseline = budget.usage();
        let adapter = worker::fixture_control_resources().0;
        let target = run_timing::fixture_resources();

        // Exact synthetic pressure leaves the real adapter fitting and timing one short.
        let pressure = budget
            .reserve_resources(if bytes {
                RetainedResources {
                    payload_bytes: LIMITS.payload_bytes
                        - baseline.payload_bytes
                        - adapter.payload_bytes
                        - target.payload_bytes
                        + 1,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    shared_roots: LIMITS.shared_roots
                        - baseline.shared_roots
                        - adapter.shared_roots
                        - target.shared_roots
                        + 1,
                    ..Default::default()
                }
            })
            .unwrap();
        let probe = DownstreamControl::new(budget).unwrap();
        assert!(RunTiming::prepare(budget).is_err());
        drop(probe);
        model.start();
        assert!(model.worker.is_none());
        assert!(context.application.active().is_none());
        assert_eq!(
            context.application.snapshot(Direction::Downstream),
            snapshot
        );
        assert!(Arc::ptr_eq(model.timing.as_ref().unwrap(), &old_timing));
        assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &old_request));
        assert!(Arc::ptr_eq(model.plan.as_ref().unwrap(), &old_plan));
        assert!(Arc::ptr_eq(&latest(&context).unwrap(), &old_result));
        assert_eq!(model.token, token);
        assert_eq!(model.generation, generation);
        assert_eq!(old_timing.snapshot(), timing);
        assert_eq!(
            serde_json::to_value(old_result.backend().report()).unwrap(),
            science
        );
        assert_eq!(
            performance::view(&context, LATEST_PROFILE, FIRST_POINT, FULL_SPAN),
            history
        );
        assert!(model.error.message().contains("retained-data limit"));
        drop(pressure);
        let mut with_status = baseline;
        crate::downstream::inputs::retention::append(
            &mut with_status,
            match &model.error {
                WorkerFailure::Detailed(error) => error.fixture_reserved_resources(),
                _ => RetainedResources::default(),
            },
        )
        .unwrap();
        assert_eq!(budget.usage(), with_status);

        // Dispose only the new persistent status after checking its actual charge;
        // preserve the historical remaining science/alias baseline independently.
        model.error = WorkerFailure::default();
        assert_eq!(budget.usage(), baseline);
    }
}

#[test]
fn stale_paused_timing_releases_at_actual_execution_and_final_alias_boundaries() {
    let mut model = prepared_model(PAUSABLE_OBSERVATION_MS);
    let application = model.context.application.clone();
    model.start();
    model.pause();
    wait_for(|| {
        model.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    let weak = Arc::downgrade(model.timing.as_ref().unwrap());
    assert!(weak.upgrade().is_some());
    assert!(model.invalidate());
    assert!(model.timing.is_none());

    // Cancellation may already have completed; only an actual observed alias is live.
    if let Some(observed) = weak.upgrade() {
        let usage = application.budget().usage();
        let charge = run_timing::fixture_resources();
        assert!(usage.payload_bytes >= charge.payload_bytes);
        assert!(usage.shared_roots >= charge.shared_roots);
        drop(observed);
    }
    assert_eq!(
        application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    wait_for(|| model.poll()["busy"] == false);
    assert!(application.active().is_none());
    wait_for(|| weak.upgrade().is_none());
    assert!(latest(&model.context).is_none());
}
