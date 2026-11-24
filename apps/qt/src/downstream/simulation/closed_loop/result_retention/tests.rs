/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public result targets; dynamic presentation copies are explicitly separate.
use super::super::{
    latest, prepare, prepare_sweep, publish, request_retention, worker, Context, Model, Options,
    ResultData,
};
use super::*;
use crate::shell::{ApplicationSessions, WorkerEvent};
use merlin::downstream::{ChannelJobStatus, ErrorCode, FullLoopState};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

// Supported small science fixture and independent host scheduling watchdogs.
const OBSERVATION_MS: u64 = 1;
const ACTIVE_RUNTIME_MS: u64 = 60000;
const WATCHDOG: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(5);

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn request() -> FullLoopRequest {
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    prepare(
        &transmission,
        &inputs,
        &Options {
            duration_ms: OBSERVATION_MS,
            max_runtime_ms: ACTIVE_RUNTIME_MS,
            ..Default::default()
        },
        &sections,
    )
    .unwrap()
}

fn sum(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
    }
}

fn one_less(value: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: value.payload_bytes.checked_sub(1).unwrap(),
            ..value
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: value.ordered_entries.checked_sub(1).unwrap(),
            ..value
        },
        Dimension::Roots => RetainedResources {
            shared_roots: value.shared_roots.checked_sub(1).unwrap(),
            ..value
        },
    }
}

fn limited(value: RetainedResources, dimension: Dimension) -> RetainedResources {
    let unlimited = RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
    };
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: value.payload_bytes,
            ..unlimited
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: value.ordered_entries,
            ..unlimited
        },
        Dimension::Roots => RetainedResources {
            shared_roots: value.shared_roots,
            ..unlimited
        },
    }
}

fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !ready() {
        assert!(Instant::now() < deadline, "Real result admission watchdog");
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn working_allowance(request: &FullLoopRequest) -> RetainedResources {
    let budget = fixture_budget();
    let owner = request_retention::admit_run_request(
        request.clone(),
        request.options().max_runtime_ms,
        &budget,
    )
    .unwrap();
    let actual = budget.usage();
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
    actual
}

fn prepared_model(dimension: Dimension) -> (Model, RetainedResources, RetainedResources) {
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    let options = Options {
        duration_ms: OBSERVATION_MS,
        max_runtime_ms: ACTIVE_RUNTIME_MS,
        ..Default::default()
    };
    let plan = prepare_sweep(&transmission, &inputs, &options, &sections).unwrap();
    let request = plan.request(0).unwrap();
    let report_charge = resources(request.result_retained_bound().unwrap()).unwrap();
    let working_charge = working_allowance(&request);
    let source = fixture_budget();
    let request = request_retention::admit_request(request, &source).unwrap();
    let plan = request_retention::admit_plan(plan, &source).unwrap();
    let accepted_charge = source.usage();
    let metadata = ApplicationSessions::new(fixture_budget_limits())
        .unwrap()
        .budget()
        .usage();

    // Exactly one complete report fits in the selected dimension. A second
    // actual target must reject while the first immutable report remains owned.
    let maximum = limited(
        sum(
            metadata,
            sum(accepted_charge, sum(working_charge, report_charge)),
        ),
        dimension,
    );
    let mut context = Context::isolated();
    context.application = ApplicationSessions::new(maximum).unwrap();
    crate::downstream::workbench::session::restore(&context, inputs, sections);
    let request =
        request_retention::rebind_request(&request, context.application.budget()).unwrap();
    let plan = request_retention::rebind_plan(&plan, context.application.budget()).unwrap();
    let key = context.application.configure_downstream(true).unwrap();
    let mut model = Model::default();
    model.context = context;
    model.options = Some(options);
    model.verify = true;
    model.request = Some(request);
    model.plan = Some(plan);
    model.generation = Some(key);
    (model, sum(metadata, accepted_charge), report_charge)
}

fn fixture_budget_limits() -> RetainedResources {
    // Test-only policy, never an application default or science-size estimate.
    RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
    }
}

#[test]
fn every_actual_result_dimension_rejects_before_public_control_or_source_advance() {
    let request = request();
    let target = resources(request.result_retained_bound().unwrap()).unwrap();
    assert!(target.payload_bytes > 0 && target.ordered_entries > 0 && target.shared_roots > 0);
    let settings = serde_json::to_value(request.settings()).unwrap();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let budget = RetentionBudget::with_limits(one_less(
            sum(working_allowance(&request), target),
            dimension,
        ));
        let control = FullLoopControl::new();
        assert!(
            worker::run_one_admitted(request.clone(), ACTIVE_RUNTIME_MS, &control, &budget)
                .is_err()
        );
        assert_eq!(control.snapshot().state, FullLoopState::Idle);
        assert!(control.snapshot().progress.is_none());
        assert_eq!(budget.usage(), RetainedResources::default());
        assert_eq!(serde_json::to_value(request.settings()).unwrap(), settings);
        let working =
            request_retention::admit_run_request(request.clone(), ACTIVE_RUNTIME_MS, &budget)
                .unwrap();
        let Err(ResultError::Retention(error)) = PreparedResult::new(working.value(), &budget)
        else {
            panic!("Insufficient real target must retain its typed dimension error");
        };
        assert!(matches!(
            (dimension, error),
            (Dimension::Payload, RetentionError::LimitExceeded { .. })
                | (
                    Dimension::Entries,
                    RetentionError::OrderedEntriesExceeded { .. }
                )
                | (Dimension::Roots, RetentionError::SharedRootsExceeded { .. })
        ));
    }
}

#[test]
fn exact_complete_and_precancelled_science_retain_the_preexecution_target() {
    let request = request();
    let expected = resources(request.result_retained_bound().unwrap()).unwrap();
    for cancelled in [false, true] {
        let budget = RetentionBudget::with_limits(expected);
        let control = FullLoopControl::new();
        let reference_control = FullLoopControl::new();
        if cancelled {
            control.cancel();
            reference_control.cancel();
        }
        let prepared = PreparedResult::new(&request, &budget).unwrap();
        assert_eq!(budget.usage(), expected);
        let result = ResultData::from_retained(prepared.run(&control).unwrap());
        let reference = request.run(&reference_control).unwrap();
        assert_eq!(result.job.status, reference.report().status);
        assert_eq!(
            serde_json::to_value(&result.job.summary).unwrap(),
            serde_json::to_value(&reference.report().summary).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&result.job.settings).unwrap(),
            serde_json::to_value(&reference.report().settings).unwrap()
        );
        assert_eq!(
            serde_json::to_value(result.backend().observation_stats()).unwrap(),
            serde_json::to_value(reference.observation_stats()).unwrap()
        );
        assert_eq!(
            result.job.status,
            if cancelled {
                ChannelJobStatus::Cancelled
            } else {
                ChannelJobStatus::Complete
            }
        );
        assert_eq!(result.job.summary.comparison.bit_errors, 0);
        assert_eq!(budget.usage(), expected);
        let actual = result.backend().retained_footprint().unwrap();
        let bound = request.result_retained_bound().unwrap();
        assert!(actual.payload_bytes <= bound.payload_bytes);
        assert!(actual.ordered_entries <= bound.ordered_entries);
        assert!(actual.shared_roots <= bound.shared_roots);
        drop(result);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn public_busy_failure_releases_only_the_new_target_and_preserves_old_aliases() {
    let request = request();
    let budget = fixture_budget();
    let control = FullLoopControl::new();
    let current = Arc::new(ResultData::from_retained(
        PreparedResult::new(&request, &budget)
            .unwrap()
            .run(&control)
            .unwrap(),
    ));
    let old = Arc::clone(&current);
    let charge = budget.usage();
    let before = serde_json::to_value(current.backend().report()).unwrap();
    let attempt = PreparedResult::new(&request, &budget).unwrap();
    assert_eq!(budget.usage(), sum(charge, charge));
    let Err(ResultError::Backend(error)) = attempt.run(&control) else {
        panic!("Existing one-shot public control must reject Busy");
    };
    assert_eq!(error.code(), ErrorCode::Busy);
    assert_eq!(budget.usage(), charge);
    assert_eq!(
        serde_json::to_value(current.backend().report()).unwrap(),
        before
    );
    drop(current);
    assert_eq!(budget.usage(), charge);
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn genuine_worker_queue_and_final_result_alias_keep_the_target_after_terminal_disposal() {
    let application = ApplicationSessions::new(fixture_budget_limits()).unwrap();
    let budget = application.budget();
    let accepted = request_retention::admit_request(request(), budget).unwrap();
    let expected = resources(accepted.value().result_retained_bound().unwrap()).unwrap();
    let baseline = budget.usage();
    let key = application.configure_downstream(true).unwrap();
    let control = Arc::new(worker::DownstreamControl::new());
    let lease = application.begin(key, control.clone()).unwrap();
    let worker = worker::Worker::spawn(
        Arc::clone(&accepted),
        None,
        Arc::new(super::super::RunTiming::new()),
        control,
        lease,
        budget.clone(),
    )
    .unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        application.finish(lease).unwrap();
        panic!("Real worker thread must start: {cause}");
    });
    let WorkerEvent::Message {
        token,
        value: worker::WorkerMessage::Profile(queued),
    } = worker.receiver.recv_timeout(WATCHDOG).unwrap()
    else {
        panic!("Actual worker must deliver its pre-admitted report");
    };
    application.validate_callback(token).unwrap();
    assert_eq!(budget.usage(), sum(baseline, expected));
    assert_eq!(queued.job.status, ChannelJobStatus::Complete);
    let current = Arc::new(*queued);
    let old = Arc::clone(&current);
    drop(worker);
    wait_for(|| application.active().is_none());
    drop(accepted);

    // Worker and accepted-request aliases have now dropped; the old/current
    // scientific report still owns the complete original target reservation.
    let metadata = ApplicationSessions::new(fixture_budget_limits())
        .unwrap()
        .budget()
        .usage();
    assert_eq!(budget.usage(), sum(metadata, expected));
    drop(current);
    assert_eq!(budget.usage(), sum(metadata, expected));
    drop(old);
    assert_eq!(budget.usage(), metadata);
}

#[test]
fn rejected_new_targets_preserve_the_same_old_backend_report_in_every_dimension() {
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let (mut model, baseline, charge) = prepared_model(dimension);
        let context = model.context.clone();
        model.start();
        let old_token = context.application.active().unwrap().token;
        wait_for(|| model.poll()["busy"] == false);
        let old = latest(&context).unwrap();
        let before = serde_json::to_value(old.backend().report()).unwrap();
        let key = model.generation;
        assert_eq!(context.application.budget().usage(), sum(baseline, charge));
        model.start();
        let new_token = context.application.active().unwrap().token;
        assert_ne!(old_token, new_token);
        assert!(Arc::ptr_eq(&latest(&context).unwrap(), &old));
        wait_for(|| model.poll()["busy"] == false);
        assert!(model.error.contains("retained-data limit"));
        assert_eq!(model.generation, key);
        assert!(Arc::ptr_eq(&latest(&context).unwrap(), &old));
        assert_eq!(
            serde_json::to_value(old.backend().report()).unwrap(),
            before
        );
        assert_eq!(context.application.budget().usage(), sum(baseline, charge));
        assert!(context.application.active().is_none());
        publish(&context, None);
        drop(model);
        let metadata = ApplicationSessions::new(fixture_budget_limits())
            .unwrap()
            .budget()
            .usage();
        assert_eq!(context.application.budget().usage(), sum(metadata, charge));
        drop(old);
        assert_eq!(context.application.budget().usage(), metadata);
    }
}

#[test]
fn fixed_result_layout_counts_the_embedded_retained_reservation_once_and_checks_overflow() {
    let fixed = RetainedFootprint {
        payload_bytes: size_of::<FullLoopResult>(),
        ..Default::default()
    };
    let actual = resources(fixed).unwrap();
    assert_eq!(actual.payload_bytes, size_of::<ResultData>());
    assert!(size_of::<ResultData>() >= size_of::<Retained<FullLoopResult>>());
    assert_eq!(actual.ordered_entries, 0);
    assert_eq!(actual.shared_roots, FINAL_RESULT_SHARED_ROOTS);
    assert_eq!(
        resources(RetainedFootprint::default()),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        resources(RetainedFootprint {
            payload_bytes: usize::MAX,
            ..fixed
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        resources(RetainedFootprint {
            shared_roots: usize::MAX,
            ..fixed
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
}
