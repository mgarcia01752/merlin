/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual adapter/scientific-control admission and genuine lease disposal.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources, WorkerStatus};
use std::time::Duration;

// Test-only ceilings and scheduling watchdog; neither is a scientific policy.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

// Established smallest complete observation and independent active host timeout.
const OBSERVATION_MS: u64 = 1;
const RUNTIME_MS: u64 = 60_000;

#[test]
fn adapter_exact_layout_below_limits_and_final_alias_are_independent() {
    let expected = RetainedResources {
        payload_bytes: size_of::<DownstreamControl>(),
        ordered_entries: 0,
        shared_roots: 1,
    };
    let exact = RetentionBudget::with_limits(expected);
    let owner = DownstreamControl::new(&exact).unwrap();
    let weak = Arc::downgrade(&owner);
    assert_eq!(exact.usage(), expected);
    let gate_alias = Arc::clone(&owner);
    drop(owner);
    assert!(weak.upgrade().is_some());
    assert_eq!(exact.usage(), expected);
    drop(gate_alias);
    assert!(weak.upgrade().is_none());
    assert_eq!(exact.usage(), RetainedResources::default());
    for maximum in [
        RetainedResources {
            payload_bytes: expected.payload_bytes - 1,
            ..expected
        },
        RetainedResources {
            shared_roots: 0,
            ..expected
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(DownstreamControl::new(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn profile_layout_rejection_and_dispatch_execution_aliases_keep_one_charge() {
    let backend = FullLoopControl::new();
    let scientific = backend.retained_footprint().unwrap();
    let expected = RetainedResources {
        payload_bytes: scientific.payload_bytes - size_of::<FullLoopControl>()
            + size_of::<Retained<FullLoopControl>>(),
        ordered_entries: 0,
        shared_roots: 3, // Outer retained owner, backend Shared and common stop.
    };
    let exact = RetentionBudget::with_limits(expected);
    let execution = admit_profile_control(&exact).unwrap();
    let weak = Arc::downgrade(&execution);
    let mut dispatch = Dispatch {
        active: Some(Arc::clone(&execution)),
        ..Default::default()
    };
    assert!(Arc::ptr_eq(dispatch.active.as_ref().unwrap(), &execution));
    execution.value().pause().unwrap();
    assert_eq!(execution.value().snapshot().state, FullLoopState::Idle);
    assert_eq!(exact.usage(), expected);
    dispatch.active = None;
    assert_eq!(exact.usage(), expected);
    execution.value().cancel();
    assert!(execution.value().snapshot().cancellation_requested);
    drop(execution);
    assert!(weak.upgrade().is_none());
    assert_eq!(exact.usage(), RetainedResources::default());
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
        assert!(admit_profile_control(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
        assert_eq!(backend.snapshot().state, FullLoopState::Idle);
    }
    assert_eq!(
        profile_control_resources(Default::default()),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[test]
fn actual_worker_control_rejection_keeps_source_idle_and_terminal_owns_lease() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget();
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    let candidate = super::super::prepare(
        &transmission,
        &inputs,
        &super::super::Options {
            duration_ms: OBSERVATION_MS,
            max_runtime_ms: RUNTIME_MS,
            ..Default::default()
        },
        &sections,
    )
    .unwrap();
    let settings = serde_json::to_value(candidate.settings()).unwrap();
    let request = request_retention::admit_request(candidate, budget).unwrap();
    let control = DownstreamControl::new(budget).unwrap();
    let key = application.configure_downstream(true).unwrap();
    let timing = RunTiming::new(RunTiming::prepare(budget).unwrap());
    let lease = application.begin(key, control.clone()).unwrap();
    let token = lease.token();

    // Explicit accounting pressure blocks only a new profile control, before science.
    let pressure = budget
        .reserve_resources(RetainedResources {
            shared_roots: LIMITS.shared_roots - budget.usage().shared_roots,
            ..Default::default()
        })
        .unwrap();
    let worker = Worker::spawn(
        Arc::clone(&request),
        None,
        timing,
        Arc::clone(&control),
        lease,
        budget.clone(),
    )
    .unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        application.finish(lease).unwrap();
        panic!("Actual rejection worker must start: {cause}");
    });
    assert!(matches!(worker.receiver.recv_timeout(WATCHDOG).unwrap(),
        WorkerEvent::Message { token: actual, value: WorkerMessage::Rejected(_, _, ref error) }
            if actual == token && error.contains("retained-data limit")));
    assert!(lock(&control.state).active.is_none());
    assert_eq!(
        serde_json::to_value(request.value().settings()).unwrap(),
        settings
    );
    assert!(matches!(
        worker.receiver.recv_timeout(WATCHDOG).unwrap(),
        WorkerEvent::Message {
            value: WorkerMessage::Finished(_),
            ..
        }
    ));
    let WorkerEvent::Terminal(terminal) = worker.receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("Actual worker must publish its sole terminal");
    };
    assert_eq!(terminal.status(), &WorkerStatus::Completed);
    assert_eq!(application.active().unwrap().token, token);
    application.finish(terminal.into_lease()).unwrap();
    drop(worker);
    drop(pressure);
    drop(request);
    drop(control);
    assert!(application.active().is_none());
}

#[test]
fn invalid_thread_label_returns_the_original_lease_and_releases_closure_alias() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let control = DownstreamControl::new(application.budget()).unwrap();
    let weak = Arc::downgrade(&control);
    let key = application.configure_downstream(true).unwrap();
    let lease = application.begin(key, control.clone()).unwrap();
    let token = lease.token();
    let alias = Arc::clone(&control);
    let Err(error) = spawn_worker::<(), String>("control\0fixture", 0, lease, move |_| {
        drop(alias);
        Ok(())
    }) else {
        panic!("Static invalid thread label must reject before spawning");
    };
    let (_, lease) = error.into_parts();
    assert_eq!(lease.token(), token);
    drop(control);
    assert!(weak.upgrade().is_some());
    application.finish(lease).unwrap();
    assert!(weak.upgrade().is_none());
}
