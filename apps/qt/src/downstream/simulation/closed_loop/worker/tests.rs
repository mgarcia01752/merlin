/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public downstream behavior; separate terminal fixtures execute no science.
use super::*;
use crate::{
    downstream::simulation::closed_loop::{prepare, Options},
    shell::{
        Direction, DispatchError, DispatchGate, DispatchPhase, RetainedBytes, RetainedResources,
        RetentionBudget, SessionError, ShellSessions, WorkerSender, WorkerStatus, WorkerTerminal,
    },
};
use std::time::{Duration, Instant};

// Scheduling policies independent of scientific runtime and observation duration.
const CONTROL_ACK_TIMEOUT: Duration = Duration::from_secs(15);
const CONTROL_POLL_INTERVAL: Duration = Duration::from_millis(5);
const PAUSED_OBSERVATION_INTERVAL: Duration = Duration::from_millis(30);

// Preserve the original one-second pause fixture and sixty-second active budget.
const PAUSED_OBSERVATION_MS: u64 = 1000;
const FIXTURE_ACTIVE_RUNTIME_MS: u64 = 60000;
// Small accepted observation used only to reach genuine complete worker delivery.
const COMPLETE_OBSERVATION_MS: u64 = 1;

// Test-only ceilings; real request ownership is still derived from public metadata.
const TEST_OWNER_LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

type Identities = ShellSessions<(), (), (), ()>;

fn admit(
    control: &Arc<DownstreamControl>,
) -> (RetentionBudget, DispatchGate, Identities, RunLease) {
    // Unit configuration payloads occupy zero bytes and issue genuine identities.
    // They neither stand in for scientific ownership nor claim aggregate admission.
    let budget = RetentionBudget::with_limits(TEST_OWNER_LIMITS);
    let gate = DispatchGate::default();
    let mut identities = ShellSessions::new(
        budget.clone(),
        gate.clone(),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
    )
    .unwrap();
    let lease = identities
        .begin(Direction::Downstream, control.clone())
        .unwrap();
    (budget, gate, identities, lease)
}

fn request(duration_ms: u64, budget: &RetentionBudget) -> SharedRequest {
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    let request = prepare(
        &transmission,
        &inputs,
        &Options {
            duration_ms,
            max_runtime_ms: FIXTURE_ACTIVE_RUNTIME_MS,
            ..Default::default()
        },
        &sections,
    )
    .unwrap();
    super::super::request_retention::admit_request(request, budget).unwrap()
}

fn spawned(result: Result<Worker, WorkerSpawnError>) -> Worker {
    result.unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        lease.acknowledge_termination().unwrap();
        panic!("Fixture thread must start: {cause}");
    })
}

fn busy(identities: &mut Identities, worker: &Worker) {
    assert!(matches!(
        identities.begin(Direction::Upstream, worker.control.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(active)))
            if active.token == worker.token()
    ));
}

fn data(worker: &Worker) -> WorkerMessage {
    match worker.receiver.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap() {
        WorkerEvent::Message { token, value } => {
            assert_eq!(token, worker.token());
            value
        }
        WorkerEvent::Terminal(_) => panic!("Expected original data before the terminal"),
    }
}

fn terminal(worker: &Worker) -> WorkerTerminal<String> {
    let WorkerEvent::Terminal(terminal) =
        worker.receiver.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap()
    else {
        panic!("Exactly one common terminal must follow data");
    };
    assert_eq!(terminal.token(), worker.token());
    terminal
}

fn wait_for(mut predicate: impl FnMut() -> bool) {
    let deadline = Instant::now() + CONTROL_ACK_TIMEOUT;
    while !predicate() {
        assert!(
            Instant::now() < deadline,
            "Worker fixture acknowledgement timeout"
        );
        std::thread::sleep(CONTROL_POLL_INTERVAL);
    }
}

#[test]
fn paused_public_worker_cancels_and_retains_its_actual_partial_report() {
    let timing = Arc::new(RunTiming::new());
    let adapter = Arc::new(DownstreamControl::new());
    assert_eq!(adapter.acknowledged_pause(), None);
    let (budget, gate, mut identities, lease) = admit(&adapter);
    let worker = spawned(Worker::spawn(
        request(PAUSED_OBSERVATION_MS, &budget),
        None,
        timing.clone(),
        adapter,
        lease,
        budget.clone(),
    ));
    worker.pause();
    let deadline = Instant::now() + CONTROL_ACK_TIMEOUT;
    let control = loop {
        if let Some(control) = lock(&worker.control.state).active.clone() {
            if control.snapshot().state == FullLoopState::Paused {
                break control;
            }
        }
        assert!(
            Instant::now() < deadline,
            "Public pause was not acknowledged"
        );
        std::thread::sleep(CONTROL_POLL_INTERVAL);
    };
    assert_eq!(worker.acknowledged_pause(), Some(true));
    gate.acknowledge_pause(worker.token(), true).unwrap();
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Paused);
    busy(&mut identities, &worker);
    let before = control.snapshot().progress;
    let age = control.last_progress_age_ms();
    std::thread::sleep(PAUSED_OBSERVATION_INTERVAL);
    assert_eq!(
        serde_json::to_value(before).unwrap(),
        serde_json::to_value(control.snapshot().progress).unwrap()
    );
    if let (Some(before), Some(after)) = (age, control.last_progress_age_ms()) {
        assert!(after > before);
    }
    assert!(worker.progress(&timing).0);
    gate.request_cancel(worker.token()).unwrap();
    worker.cancel();
    assert_eq!(worker.acknowledged_pause(), None);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    busy(&mut identities, &worker);
    let WorkerMessage::Profile(result) = data(&worker) else {
        panic!("Cancellation must preserve the actual partial public report");
    };
    assert_eq!(result.job.status, ChannelJobStatus::Cancelled);
    assert_eq!(
        result.backend().report().summary.comparison.bits_tested,
        result.job.summary.comparison.bits_tested
    );
    assert_eq!(control.snapshot().state, FullLoopState::Cancelled);
    let WorkerMessage::Finished(reason) = data(&worker) else {
        panic!("Cancelled single-profile worker must finish without another run");
    };
    assert!(reason.contains("cancelled"));
    assert_eq!(worker.progress(&timing).1, 0);
    busy(&mut identities, &worker);
    let terminal = terminal(&worker);
    assert_eq!(terminal.status(), &WorkerStatus::Completed);
    assert_eq!(gate.active().unwrap().token, worker.token());
    drop(terminal);
    assert!(gate.active().is_none());
    assert!(matches!(
        worker.receiver.recv_timeout(CONTROL_ACK_TIMEOUT),
        Err(mpsc::RecvTimeoutError::Disconnected)
    ));
}

#[test]
fn finished_data_keeps_real_completion_busy_until_terminal_disposal() {
    let adapter = Arc::new(DownstreamControl::new());
    let (budget, gate, mut identities, lease) = admit(&adapter);
    let worker = spawned(Worker::spawn(
        request(COMPLETE_OBSERVATION_MS, &budget),
        None,
        Arc::new(RunTiming::new()),
        adapter,
        lease,
        budget.clone(),
    ));
    let WorkerMessage::Profile(result) = data(&worker) else {
        panic!("Genuine completion must retain its public report");
    };
    assert_eq!(result.job.status, ChannelJobStatus::Complete);
    assert_eq!(
        result.backend().report().summary.comparison.bits_tested,
        result.job.summary.comparison.bits_tested
    );
    let WorkerMessage::Finished(reason) = data(&worker) else {
        panic!("Finished remains an ordinary data event");
    };
    assert!(reason.is_empty());
    assert_eq!(worker.acknowledged_pause(), None);
    busy(&mut identities, &worker);
    let terminal = terminal(&worker);
    assert_eq!(terminal.status(), &WorkerStatus::Completed);
    drop(worker);
    assert!(gate.active().is_some());
    terminal.into_lease().acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
}

// These fixtures exercise only the Worker adapter around the accepted transport.
// No public simulation, result or scientific error is substituted by their closure.
fn transport_worker(
    operation: impl FnOnce(WorkerSender<WorkerMessage, String>) -> Result<(), String> + Send + 'static,
) -> (DispatchGate, Identities, Worker) {
    let control = Arc::new(DownstreamControl::new());
    let (_budget, gate, identities, lease) = admit(&control);
    let token = lease.token();
    let receiver = spawn_worker(
        "downstream-terminal-fixture",
        PROFILE_COMPLETION_CAPACITY,
        lease,
        operation,
    )
    .unwrap_or_else(|error| {
        let (cause, lease) = error.into_parts();
        lease.acknowledge_termination().unwrap();
        panic!("Terminal fixture thread must start: {cause}");
    });
    let worker = Worker {
        control,
        token,
        receiver,
        profile_count: 0, // No scientific profile executes in this transport fixture.
        stale: false,
    };
    (gate, identities, worker)
}

#[test]
fn error_and_panic_terminals_keep_the_worker_lease_until_disposed() {
    for panic_operation in [false, true] {
        let (gate, _identities, worker) = transport_worker(move |_| {
            if panic_operation {
                panic!("Application terminal panic fixture");
            }
            Err("Application terminal error fixture".to_owned())
        });
        let terminal = terminal(&worker);
        if panic_operation {
            assert_eq!(terminal.status(), &WorkerStatus::Panicked);
        } else {
            assert_eq!(
                terminal.status(),
                &WorkerStatus::Failed("Application terminal error fixture".to_owned())
            );
        }
        assert_eq!(gate.active().unwrap().token, worker.token());
        drop(worker);
        assert!(gate.active().is_some());
        drop(terminal);
        assert!(gate.active().is_none());
    }
}

#[test]
fn destroyed_worker_requests_cancel_but_waits_for_real_closure_termination() {
    let (entered, observed_entry) = mpsc::channel();
    let (finish, await_finish) = mpsc::channel();
    let (gate, _identities, worker) = transport_worker(move |_| {
        entered.send(()).unwrap();
        await_finish.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap();
        Ok(())
    });
    observed_entry.recv_timeout(CONTROL_ACK_TIMEOUT).unwrap();
    let control = Arc::clone(&worker.control);
    let token = worker.token();
    drop(worker);
    assert!(lock(&control.state).cancelled);
    assert_eq!(control.acknowledged_pause(), None);
    assert_eq!(gate.active().unwrap().token, token);
    finish.send(()).unwrap();
    wait_for(|| gate.active().is_none());
}
