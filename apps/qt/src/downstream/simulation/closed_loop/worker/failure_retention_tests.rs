/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Guarded actual rejection data and common terminal disposal boundaries.
use super::*;
use crate::shell::{
    ApplicationError, ApplicationSessions, DispatchError, SessionError, WorkerStatus,
};
use std::time::{Duration, Instant};

// Independent host ceilings and asynchronous watchdog, not scientific policies.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
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

#[test]
fn actual_rejection_keeps_its_guard_until_stale_data_disposal() {
    let application = ApplicationSessions::new(LIMITS).unwrap();
    let budget = application.budget();
    let control = DownstreamControl::new(budget).unwrap();
    let request = super::tests::request(OBSERVATION_MS, budget);
    let settings = serde_json::to_value(request.value().settings()).unwrap();
    let timing = RunTiming::new(RunTiming::prepare(budget).unwrap());
    let key = application.configure_downstream(true).unwrap();
    let lease = application.begin(key, control.clone()).unwrap();
    let token = lease.token();
    let before = budget.usage();

    // Synthetic root pressure rejects the real per-profile control, leaving bytes
    // available for the actual error. Request, timing and adapter are already admitted.
    let pressure = budget
        .reserve_resources(RetainedResources {
            shared_roots: LIMITS.shared_roots - before.shared_roots,
            ..Default::default()
        })
        .unwrap();
    let worker = Worker::spawn(
        Arc::clone(&request),
        None,
        Arc::clone(&timing),
        Arc::clone(&control),
        lease,
        budget.clone(),
    )
    .unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        application.finish(lease).unwrap();
        panic!("Actual rejection worker must start: {cause}");
    });
    let WorkerEvent::Message {
        token: actual,
        value: WorkerMessage::Rejected(_, _, failure),
    } = worker.receiver.recv_timeout(WATCHDOG).unwrap()
    else {
        panic!("Root pressure must reject before any scientific profile");
    };
    assert_eq!(actual, token);
    let WorkerFailure::Detailed(retained) = &failure else {
        panic!("The actual error fits the independent byte ceiling");
    };
    let charge = retained.fixture_reserved_resources();
    assert!(failure.message().contains("retained-data limit"));
    assert_eq!(
        budget.usage().payload_bytes,
        before.payload_bytes + charge.payload_bytes
    );
    assert!(lock(&control.state).active.is_none());
    assert_eq!(
        serde_json::to_value(request.value().settings()).unwrap(),
        settings
    );
    application.configure_downstream(false).unwrap();
    assert!(application.validate_callback(token).is_err());
    assert_eq!(
        budget.usage().payload_bytes,
        before.payload_bytes + charge.payload_bytes
    );
    drop(failure);
    assert_eq!(budget.usage().payload_bytes, before.payload_bytes);
    assert!(matches!(
        worker.receiver.recv_timeout(WATCHDOG).unwrap(),
        WorkerEvent::Message {
            value: WorkerMessage::Finished(_),
            ..
        }
    ));
    let WorkerEvent::Terminal(terminal) = worker.receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("Actual rejection must still produce its sole terminal");
    };
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    assert!(matches!(
        application.finish(terminal.into_lease()),
        Err(ApplicationError::Session(SessionError::Dispatch(
            DispatchError::StaleCallback
        )))
    ));
    assert!(application.active().is_none());
    drop(worker);
    drop(pressure);
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
        if disconnect {
            drop(receiver);
            assert_eq!(budget.usage(), charged);
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
            assert_eq!(budget.usage(), charged);
            assert!(application.active().is_some());
            drop(terminal);
            assert!(application.active().is_none());
            assert_eq!(budget.usage(), before);
        }
    }
}
