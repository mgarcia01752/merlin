/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Admitted preview controls preserve accepted science and follow actual aliases.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources, RetentionError};
use merlin::downstream::{TransmissionSettings, TransmitterPreviewState};
use std::{
    mem::size_of,
    time::{Duration, Instant},
};

// Independent fixture policy and scheduler deadline, never scientific sizing.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

fn configured() -> Analyzer {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let mut model = Analyzer::for_context(context);
    tests::configure(
        &mut model,
        &serde_json::to_string(&TransmissionSettings::default()).unwrap(),
        tests::inputs(),
    );
    model
}

#[test]
fn exact_control_layout_below_limits_and_final_execution_alias_are_charged() {
    let scientific = TransmitterPreviewControl::new()
        .retained_footprint()
        .unwrap();
    let expected = RetainedResources {
        payload_bytes: scientific.payload_bytes - size_of::<TransmitterPreviewControl>()
            + size_of::<Retained<TransmitterPreviewControl>>(),
        ordered_entries: 0,
        shared_roots: 3, // Retained owner, backend Shared and common stop.
    };
    let budget = RetentionBudget::with_limits(expected);
    let job = retention::admit_control(&budget).unwrap();
    let weak = Arc::downgrade(&job);
    let worker = Arc::clone(&job);
    assert_eq!(budget.usage(), expected);
    drop(job);
    worker.value().cancel();
    assert_eq!(
        worker.value().snapshot().state,
        TransmitterPreviewState::Idle
    );
    assert_eq!(budget.usage(), expected);
    drop(worker);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
    for maximum in [
        RetainedResources {
            payload_bytes: expected.payload_bytes - 1,
            ..expected
        },
        RetainedResources {
            shared_roots: expected.shared_roots - 1,
            ..expected
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(matches!(
            retention::admit_control(&budget),
            Err(PreviewError::Retention(_))
        ));
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    assert_eq!(
        retention::control_resources(Default::default()),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[test]
fn failed_control_admission_preserves_actual_old_result_configuration_and_notice() {
    let mut model = configured();
    model.generate();
    let finished_control = Arc::downgrade(&model.job.as_ref().unwrap().control);
    tests::wait(&mut model);
    let deadline = Instant::now() + WATCHDOG;
    while finished_control.upgrade().is_some() {
        assert!(
            Instant::now() < deadline,
            "Initial preview control disposal watchdog"
        );
        std::thread::yield_now();
    }
    let prepared = Arc::clone(model.prepared.as_ref().unwrap());
    let old = model.result.as_ref().unwrap().value() as *const _;
    let token = model.result_token;
    let notice = model.notice;
    let budget = model.context.application.budget().clone();
    let baseline = budget.usage();
    for bytes in [true, false] {
        // Fill only one actual counter dimension; no scientific bound is guessed.
        let pressure = budget
            .reserve_resources(if bytes {
                RetainedResources {
                    payload_bytes: LIMITS.payload_bytes - baseline.payload_bytes,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    shared_roots: LIMITS.shared_roots - baseline.shared_roots,
                    ..Default::default()
                }
            })
            .unwrap();
        model.generate();
        assert!(model.job.is_none());
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &prepared));
        assert_eq!(model.result.as_ref().unwrap().value() as *const _, old);
        assert_eq!(model.result_token, token);
        assert_eq!(model.notice, notice);
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
fn genuine_stale_job_and_final_external_control_alias_release_in_order() {
    let mut model = configured();
    let budget = model.context.application.budget().clone();
    model.generate();
    let control = Arc::clone(&model.job.as_ref().unwrap().control);
    let weak = Arc::downgrade(&control);
    let charge =
        retention::control_resources(control.value().retained_footprint().unwrap()).unwrap();
    let deadline = Instant::now() + WATCHDOG;
    while control.value().snapshot().state != TransmitterPreviewState::Completed {
        assert!(
            Instant::now() < deadline,
            "Actual preview completion watchdog"
        );
        std::thread::yield_now();
    }
    tests::configure(
        &mut model,
        &serde_json::to_string(&TransmissionSettings::default()).unwrap(),
        tests::inputs(),
    );
    tests::wait(&mut model);
    assert!(model.result.is_none());
    drop(model);
    let expected = ApplicationSessions::fixture_counter_resources()
        .checked_add(charge)
        .unwrap();
    while budget.usage() != expected {
        assert!(
            Instant::now() < deadline,
            "Preview worker alias disposal watchdog"
        );
        std::thread::yield_now();
    }
    assert!(weak.upgrade().is_some());
    drop(control);
    assert!(weak.upgrade().is_none());
    assert_eq!(
        budget.usage(),
        ApplicationSessions::fixture_counter_resources()
    );
}
