/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public-API fixture behavior, with explicitly synthetic accounting anchors.
//! These counter charges test transport ownership, NOT backend deep-memory bounds.
use super::*;
use crate::shell::{
    DispatchError, DispatchGate, DispatchPhase, ReplacePermission, RetainedBytes, RetentionBudget,
    SessionError, ShellSessions, WorkerStatus,
};
use merlin::upstream::{ErrorCode, RunStatus};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const SOURCE_DOCUMENT: &[u8] = include_bytes!("../../../../examples/cli/upstream/us-50khz.json");

// Independent counter ticks identify configuration versus report reservation life.
// These values intentionally make no claim about actual allocation byte counts.
const CONFIGURATION_ACCOUNTING_ANCHOR: RetainedBytes = RetainedBytes(1);
const REPORT_ACCOUNTING_ANCHOR: RetainedBytes = RetainedBytes(2);

// Harness scheduling deadline, unrelated to scientific timeout in the document.
const ASYNC_TEST_DEADLINE: Duration = Duration::from_secs(10);

// One actual report plus its terminal; this is a test transport policy.
const CALLBACK_CAPACITY: usize = 2;

type Identities = ShellSessions<(), (), (), ()>;

fn identities(budget: &RetentionBudget, gate: &DispatchGate) -> Identities {
    // Unit payloads issue real shell identities; they are not scientific fixtures.
    ShellSessions::new(
        budget.clone(),
        gate.clone(),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
    )
    .unwrap()
}

fn prepared(budget: &RetentionBudget) -> PreparedSession {
    let reservation = budget.reserve(CONFIGURATION_ACCOUNTING_ANCHOR).unwrap();
    let document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    PreparedSession::from_document(
        document,
        ReceiverObservationPolicy::new(0, 0).unwrap(),
        reservation,
    )
    .unwrap()
}

fn fixture() -> (
    RetentionBudget,
    DispatchGate,
    Identities,
    UpstreamController,
) {
    let budget = RetentionBudget::new(RetainedBytes(usize::MAX));
    let gate = DispatchGate::default();
    let identities = identities(&budget, &gate);
    let key = identities.open_target(Direction::Upstream).key;
    let controller = UpstreamController::new(key, prepared(&budget)).unwrap();
    (budget, gate, identities, controller)
}

fn start(
    controller: &mut UpstreamController,
    identities: &mut Identities,
    budget: &RetentionBudget,
    backend: RunControl,
    capacity: usize,
) -> RunToken {
    let control = Arc::new(UpstreamControl::new(backend));
    let lease = identities
        .begin(Direction::Upstream, control.clone())
        .unwrap();
    let token = lease.token();
    let charge = budget.reserve(REPORT_ACCOUNTING_ANCHOR).unwrap();
    if let Err(error) = controller.start(lease, control, charge, capacity) {
        let (reason, lease) = error.into_parts();
        lease.acknowledge_termination().unwrap();
        panic!("Unexpected Start rejection: {reason:?}");
    }
    token
}

fn wait_for(mut condition: impl FnMut() -> bool) {
    let deadline = Instant::now() + ASYNC_TEST_DEADLINE;
    while !condition() {
        assert!(Instant::now() < deadline, "Asynchronous fixture deadline");
        thread::yield_now();
    }
}

fn terminal(controller: &mut UpstreamController) -> WorkerTerminal<ApiError> {
    let deadline = Instant::now() + ASYNC_TEST_DEADLINE;
    loop {
        match controller.poll().unwrap() {
            ControllerEvent::Terminal(terminal) => return terminal,
            ControllerEvent::Report { .. } | ControllerEvent::Pending => {}
            ControllerEvent::Idle => panic!("Missing terminal"),
        }
        assert!(Instant::now() < deadline, "Terminal fixture deadline");
        thread::yield_now();
    }
}

fn complete(
    controller: &mut UpstreamController,
    identities: &mut Identities,
    budget: &RetentionBudget,
) -> RunToken {
    let token = start(
        controller,
        identities,
        budget,
        RunControl::new(),
        CALLBACK_CAPACITY,
    );
    let terminal = terminal(controller);
    assert_eq!(terminal.token(), token);
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    terminal.into_lease().acknowledge_termination().unwrap();
    token
}

#[test]
fn actual_completion_preserves_configuration_provenance_and_outcome() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let source_bits = controller
        .inspect(|session| {
            assert_eq!(
                session.document().unwrap().original_bytes(),
                Some(SOURCE_DOCUMENT)
            );
            assert!(session.configuration().is_some());
            session.preview().unwrap().source_bits
        })
        .unwrap();
    let token = complete(&mut controller, &mut identities, &budget);
    let (report_token, report) = controller.report().unwrap();
    let report = report.value();
    assert_eq!(report_token, token);
    assert_eq!(report.status, RunStatus::Completed);
    assert_eq!(
        report.source_configuration_original.as_deref(),
        Some(SOURCE_DOCUMENT)
    );
    assert!(report.reference_recovered);
    assert_eq!(report.generated_source_bits, source_bits);
    assert_eq!(report.compared_source_bits, source_bits);
    assert_eq!(report.bit_errors, 0);
    assert_eq!(report.ber, Some(0.0));
    assert!(!report.grants.is_empty());
    assert!(gate.active().is_none());
    assert!(matches!(controller.poll().unwrap(), ControllerEvent::Idle));
    controller
        .inspect(|session| session.preview().unwrap())
        .unwrap();
}

#[test]
fn acknowledged_pause_keeps_global_slot_and_resume_uses_actual_backend() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    backend.pause();
    let token = start(
        &mut controller,
        &mut identities,
        &budget,
        backend,
        CALLBACK_CAPACITY,
    );
    wait_for(|| controller.progress().unwrap().1.status == RunStatus::Paused);
    gate.acknowledge_pause(token, true).unwrap();
    identities.select(Direction::Downstream);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Paused);
    assert_eq!(controller.inspect(|_| ()), Err(ControllerError::Busy));
    let other = Arc::new(UpstreamControl::new(RunControl::new()));
    assert!(matches!(
        identities.begin(Direction::Downstream, other),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    let stale = RunToken {
        key: identities.open_target(Direction::Downstream).key,
        run: token.run,
    };
    assert_eq!(controller.resume(stale), Err(ControllerError::StaleToken));
    assert_eq!(controller.progress().unwrap().1.status, RunStatus::Paused);
    controller.resume(token).unwrap();
    let terminal = terminal(&mut controller);
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    assert_eq!(gate.active().unwrap().token, token);
    drop(terminal);
    assert!(gate.active().is_none());
    assert!(controller.report().unwrap().1.value().reference_recovered);
}

#[test]
fn paused_cancel_retains_actual_partial_report_and_old_report_charge() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let old_token = complete(&mut controller, &mut identities, &budget);
    let (_, old_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let token = start(
        &mut controller,
        &mut identities,
        &budget,
        backend,
        CALLBACK_CAPACITY,
    );
    wait_for(|| controller.progress().unwrap().1.status == RunStatus::Paused);
    assert_eq!(
        controller.cancel(old_token),
        Err(ControllerError::StaleToken)
    );
    controller.cancel(token).unwrap();
    gate.request_cancel(token).unwrap();
    let terminal = terminal(&mut controller);
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    drop(terminal);
    let (_, partial) = controller.report().unwrap();
    assert_eq!(partial.value().status, RunStatus::Cancelled);
    assert_eq!(partial.value().generated_source_bits, 0);
    assert_eq!(partial.value().compared_source_bits, 0);
    assert_eq!(partial.value().ber, None);
    assert!(!partial.value().not_started_grants.is_empty());
    assert_eq!(old_report.value().status, RunStatus::Completed);
    assert_eq!(
        budget.used(),
        RetainedBytes(
            CONFIGURATION_ACCOUNTING_ANCHOR.0
                + REPORT_ACCOUNTING_ANCHOR.0
                + REPORT_ACCOUNTING_ANCHOR.0
        )
    );
    drop(controller);
    assert_eq!(
        budget.used(),
        RetainedBytes(REPORT_ACCOUNTING_ANCHOR.0 + REPORT_ACCOUNTING_ANCHOR.0)
    );
    drop(partial);
    assert_eq!(budget.used(), REPORT_ACCOUNTING_ANCHOR);
    drop(old_report);
    assert_eq!(budget.used(), RetainedBytes(0));
}

#[test]
fn failed_document_admission_preserves_existing_owner_and_releases_candidate() {
    let (budget, _, _, controller) = fixture();
    let charge = budget.reserve(CONFIGURATION_ACCOUNTING_ANCHOR).unwrap();
    let mut document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    document.simulation.source_seed = 0;
    let Err(error) = PreparedSession::from_document(
        document,
        ReceiverObservationPolicy::new(0, 0).unwrap(),
        charge,
    ) else {
        panic!("Invalid real fixture must reject");
    };
    assert_eq!(error.category(), ErrorCode::InvalidInput);
    assert_eq!(budget.used(), CONFIGURATION_ACCOUNTING_ANCHOR);
    controller
        .inspect(|session| {
            assert_eq!(
                session.document().unwrap().original_bytes(),
                Some(SOURCE_DOCUMENT)
            );
            session.preview().unwrap();
        })
        .unwrap();
}

#[test]
fn genuine_public_control_busy_failure_keeps_old_report_and_restores_session() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let old_token = complete(&mut controller, &mut identities, &budget);
    let (_, old_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let external_control = backend.clone();
    let external_session = prepared(&budget);
    let external = thread::spawn(move || {
        let mut session = RunningSession::take(external_session.session).unwrap();
        session
            .simulation
            .as_mut()
            .unwrap()
            .run_with_control(&external_control)
    });
    wait_for(|| backend.progress().status == RunStatus::Paused);
    let token = start(
        &mut controller,
        &mut identities,
        &budget,
        backend.clone(),
        CALLBACK_CAPACITY,
    );
    let terminal = terminal(&mut controller);
    assert_eq!(terminal.token(), token);
    assert!(matches!(
        terminal.status(),
        WorkerStatus::Failed(error) if error.category() == ErrorCode::Busy
    ));
    assert_eq!(controller.report().unwrap().0, old_token);
    assert!(Arc::ptr_eq(&controller.report().unwrap().1, &old_report));
    drop(terminal);
    assert!(gate.active().is_none());
    controller
        .inspect(|session| session.preview().unwrap())
        .unwrap();
    backend.cancel();
    assert_eq!(
        external.join().unwrap().unwrap().status,
        RunStatus::Cancelled
    );
}

#[test]
fn changed_generation_and_replaced_session_reject_returning_unstarted_lease() {
    for replace in [false, true] {
        let (budget, gate, mut identities, mut controller) = fixture();
        let candidate = budget.reserve(RetainedBytes(0)).unwrap().retain(());
        if replace {
            let guard = identities.open_target(Direction::Upstream);
            identities
                .replace_upstream(guard, ReplacePermission::RequireClean, candidate)
                .unwrap();
        } else {
            identities.configure_upstream(candidate).unwrap();
        }
        let control = Arc::new(UpstreamControl::new(RunControl::new()));
        let lease = identities
            .begin(Direction::Upstream, control.clone())
            .unwrap();
        let token = lease.token();
        let charge = budget.reserve(REPORT_ACCOUNTING_ANCHOR).unwrap();
        let Err(error) = controller.start(lease, control, charge, CALLBACK_CAPACITY) else {
            panic!("Stale controller must reject Start");
        };
        let (reason, lease) = error.into_parts();
        assert!(matches!(
            reason,
            StartFailure::Controller(ControllerError::StaleToken)
        ));
        assert_eq!(lease.token(), token);
        assert_eq!(gate.active().unwrap().token, token);
        lease.acknowledge_termination().unwrap();
        assert!(gate.active().is_none());
        assert_eq!(budget.used(), CONFIGURATION_ACCOUNTING_ANCHOR);
        controller
            .inspect(|session| session.preview().unwrap())
            .unwrap();
    }
}

#[test]
fn stale_transport_message_cannot_adopt_actual_prior_report() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let old_token = complete(&mut controller, &mut identities, &budget);
    let (_, actual_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let current = start(
        &mut controller,
        &mut identities,
        &budget,
        backend,
        CALLBACK_CAPACITY,
    );
    wait_for(|| controller.progress().unwrap().1.status == RunStatus::Paused);

    // Inject only a stale application envelope containing a real prior report.
    // The genuine current worker/receiver are retained and restored below.
    let (sender, receiver) = mpsc::channel();
    let real_receiver =
        std::mem::replace(&mut controller.active.as_mut().unwrap().events, receiver);
    sender
        .send(WorkerEvent::Message {
            token: old_token,
            value: actual_report,
        })
        .ok()
        .unwrap();
    assert!(matches!(
        controller.poll(),
        Err(ControllerError::StaleToken)
    ));
    assert_eq!(controller.progress().unwrap().0, current);
    controller.active.as_mut().unwrap().events = real_receiver;
    controller.cancel(current).unwrap();
    drop(terminal(&mut controller));
    assert!(gate.active().is_none());
}

#[test]
fn disconnected_queue_and_controller_drop_release_only_after_actual_termination() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    let observer = backend.clone();
    start(&mut controller, &mut identities, &budget, backend, 0);

    // Rendezvous report delivery keeps the real worker terminal pending.
    wait_for(|| observer.progress().status == RunStatus::Completed);
    assert_eq!(
        budget.used(),
        RetainedBytes(CONFIGURATION_ACCOUNTING_ANCHOR.0 + REPORT_ACCOUNTING_ANCHOR.0)
    );
    assert!(gate.active().is_some());
    drop(controller);
    wait_for(|| gate.active().is_none());
    wait_for(|| budget.used() == RetainedBytes(0));
}

#[test]
fn paused_controller_destruction_cancels_without_joining() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    backend.pause();
    let observer = backend.clone();
    start(
        &mut controller,
        &mut identities,
        &budget,
        backend,
        CALLBACK_CAPACITY,
    );
    wait_for(|| observer.progress().status == RunStatus::Paused);
    assert!(Arc::strong_count(&controller.session) >= 2);
    drop(controller);
    wait_for(|| gate.active().is_none());
    assert_eq!(observer.progress().status, RunStatus::Cancelled);
    wait_for(|| budget.used() == RetainedBytes(0));
}

#[test]
fn actual_session_guard_restores_after_application_unwind() {
    let (budget, _, _, controller) = fixture();
    let owner = Arc::clone(&controller.session);
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let _guard = RunningSession::take(owner).unwrap();
        panic!("Application guard unwind fixture");
    }))
    .is_err());
    controller
        .inspect(|session| session.preview().unwrap())
        .unwrap();
    assert_eq!(budget.used(), CONFIGURATION_ACCOUNTING_ANCHOR);
}
