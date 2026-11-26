/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Guarded actual rejection data and common terminal disposal boundaries.
use super::*;
use crate::shell::{spawn_worker, WorkerEvent};
use crate::shell::{
    ApplicationError, ApplicationSessions, DispatchError, SessionError, WorkerStatus,
};
use std::sync::mpsc;
use std::time::{Duration, Instant};

// Independent host ceilings and asynchronous watchdog, not scientific policies.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};
const WATCHDOG: Duration = Duration::from_secs(30);

// Small existing complete observation; rejected control never advances this source.
const OBSERVATION_MS: u64 = 1;
const TERMINAL_MESSAGE: &str = "Application terminal error with guarded owned text.";

fn wait_for(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !condition() {
        assert!(Instant::now() < deadline, "Failure disposal watchdog");
        std::thread::yield_now();
    }
}

fn channel_resources() -> RetainedResources {
    let budget = RetentionBudget::with_limits(LIMITS);
    let (sender, receiver) =
        crate::shell::bounded_channel::<()>(&budget, PROFILE_COMPLETION_CAPACITY).unwrap();
    let actual = budget.usage();
    drop(sender);
    drop(receiver);
    assert_eq!(budget.usage(), RetainedResources::default());
    actual
}

#[test]
fn actual_rejection_keeps_its_guard_until_stale_data_disposal() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget();
    let control = DownstreamControl::new(budget).unwrap();
    let request = super::tests::request(OBSERVATION_MS, budget);
    let settings = serde_json::to_value(request.value().settings()).unwrap();
    let timing = RunTiming::new(RunTiming::prepare(budget).unwrap());
    let key = application.configure_downstream(true).unwrap();
    let baseline = budget.usage();
    let prepared = Worker::prepare(
        Arc::clone(&request),
        None,
        Arc::clone(&control),
        budget.clone(),
    )
    .unwrap();
    let lease = application.begin(key, control.clone()).unwrap();
    let token = lease.token();
    let before = budget.usage();

    // The actual transport already fits; pressure reaches the profile control.
    // Synthetic root pressure rejects the real per-profile control, leaving bytes
    // available for the actual error. Request, timing and adapter are already admitted.
    let pressure_resources = RetainedResources {
        shared_roots: LIMITS.shared_roots - before.shared_roots,
        ..Default::default()
    };
    let pressure = budget.reserve_resources(pressure_resources).unwrap();
    let worker = prepared
        .spawn(Arc::clone(&timing), lease)
        .unwrap_or_else(|error| {
            let (cause, lease) = error.into_parts();
            application.finish(lease).unwrap();
            panic!("Actual rejection worker must start: {cause}");
        });
    let WorkerEvent::Message {
        token: actual,
        value,
    } = worker.receiver.recv_timeout(WATCHDOG).unwrap()
    else {
        panic!("Root pressure must reject before any scientific profile");
    };
    let frame = value.fixture_reserved_resources();
    let delivery_guard;
    let (message, guard) = value.into_parts();
    delivery_guard = guard;
    let WorkerMessage::Rejected(_, _, failure) = message else {
        panic!("Original root rejection must be preserved");
    };
    assert_eq!(actual, token);
    let WorkerFailure::Detailed(retained) = &failure else {
        panic!("The actual error fits the independent byte ceiling");
    };
    let charge = retained.fixture_reserved_resources();
    assert!(failure.message().contains("retained-data limit"));
    assert!(matches!(
        worker.receiver.recv_timeout(WATCHDOG).unwrap(),
        WorkerEvent::Message { value, .. }
            if matches!(value.value(), WorkerMessage::Finished(_))
    ));
    let WorkerEvent::Terminal(terminal) = worker.receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("Actual rejection must still produce its sole terminal");
    };
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    let without_failure = baseline
        .checked_add(pressure_resources)
        .unwrap()
        .checked_add(channel_resources())
        .unwrap()
        .checked_add(frame)
        .unwrap()
        .checked_add(terminal.fixture_reserved_resources())
        .unwrap();
    let retained = without_failure.checked_add(charge).unwrap();
    // The real task may outlive recv. Wait for its exact disposal while the stale
    // data and known terminal retain their independent fixed frame reservations.
    wait_for(|| budget.usage() == retained);
    assert_eq!(budget.usage(), retained);
    assert!(lock(&control.state).active.is_none());
    assert_eq!(
        serde_json::to_value(request.value().settings()).unwrap(),
        settings
    );
    application.configure_downstream(false).unwrap();
    assert!(application.validate_callback(token).is_err());
    assert_eq!(budget.usage(), retained);
    drop(failure);
    assert_eq!(budget.usage(), without_failure);
    drop(delivery_guard);
    assert!(matches!(
        application.finish(terminal.into_lease()),
        Err(ApplicationError::Session(SessionError::Dispatch(
            DispatchError::StaleCallback
        )))
    ));
    assert!(application.active().is_none());
    drop(worker);
    drop(pressure);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn guarded_terminal_releases_after_received_or_disconnected_actual_completion() {
    for disconnect in [false, true] {
        let application = ApplicationSessions::new(LIMITS).unwrap();
        let budget = application.budget().clone();
        let control = DownstreamControl::new(&budget).unwrap();
        let key = application.configure_downstream(true).unwrap();
        let lease = application.begin(key, control.clone()).unwrap();
        let before = budget.usage();
        let failure = WorkerFailure::admit_with_fallback(
            TERMINAL_MESSAGE.to_owned().into(),
            &budget,
            WORKER_FAILURE_ADMISSION_MESSAGE,
        );
        let charged = budget.usage();
        assert!(charged.payload_bytes > before.payload_bytes);
        let (finish, await_finish) = mpsc::channel();
        let receiver = spawn_worker::<(), WorkerFailure>(
            "downstream-guarded-terminal-fixture",
            PROFILE_COMPLETION_CAPACITY,
            0, // This error-only fixture publishes no scientific data callback.
            &budget,
            lease,
            move |_| {
                await_finish.recv_timeout(WATCHDOG).unwrap();
                Err(failure)
            },
        )
        .unwrap_or_else(|error| {
            let (cause, lease) = error.into_parts();
            application.finish(lease).unwrap();
            panic!("Terminal fixture must start: {cause}");
        });
        let executing = budget.usage();
        if disconnect {
            drop(receiver);
            assert_eq!(budget.usage(), executing);
            finish.send(()).unwrap();
            wait_for(|| application.active().is_none());
            wait_for(|| budget.usage() == before);
        } else {
            finish.send(()).unwrap();
            let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
                panic!("Closure failure must produce a terminal");
            };
            let WorkerStatus::Failed(error) = terminal.status() else {
                panic!("Original application failure must remain typed terminal data");
            };
            assert_eq!(error.message(), TERMINAL_MESSAGE);
            let held = charged
                .checked_add(channel_resources())
                .unwrap()
                .checked_add(terminal.fixture_reserved_resources())
                .unwrap();
            wait_for(|| budget.usage() == held);
            assert_eq!(budget.usage(), held);
            assert!(application.active().is_some());
            drop(terminal);
            assert!(application.active().is_none());
            assert_eq!(
                budget.usage(),
                before.checked_add(channel_resources()).unwrap()
            );
            drop(receiver);
            assert_eq!(budget.usage(), before);
        }
    }
}
