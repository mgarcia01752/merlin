/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public scientific metadata and bounded-report controller admission.
//! This covers the declared scientific/fixed-wrapper owners, not full host aggregation.
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

// These are test-only independent policy ceilings, never application defaults.
const TEST_MAXIMUM_RESOURCES: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
// This fixture disables receiver-estimate history; tap observations stay genuine.
const DISABLED_ESTIMATE_FRAMES: usize = 0;
const DISABLED_ESTIMATE_POINTS: usize = 0;

// Harness scheduling deadline, unrelated to scientific timeout in the document.
const ASYNC_TEST_DEADLINE: Duration = Duration::from_secs(10);

// One actual report plus its terminal; this is a test transport policy.
const CALLBACK_CAPACITY: usize = 2;

type Identities = ShellSessions<(), (), (), ()>;

fn make_identities(budget: &RetentionBudget, gate: &DispatchGate) -> Identities {
    // Unit payloads issue real shell identities; they are not scientific fixtures.
    ShellSessions::new(
        budget.clone(),
        gate.clone(),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
    )
    .unwrap()
}

fn disabled_receiver_history() -> ReceiverObservationPolicy {
    ReceiverObservationPolicy::new(DISABLED_ESTIMATE_FRAMES, DISABLED_ESTIMATE_POINTS).unwrap()
}

fn prepared(budget: &RetentionBudget) -> PreparedSession {
    let document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    PreparedSession::from_document(document, disabled_receiver_history(), budget).unwrap()
}

fn fixture() -> (
    RetentionBudget,
    DispatchGate,
    Identities,
    UpstreamController,
) {
    // Test-only independent ceilings; production policy remains host-supplied.
    let budget = RetentionBudget::with_limits(TEST_MAXIMUM_RESOURCES);
    let gate = DispatchGate::default();
    let identities = make_identities(&budget, &gate);
    let key = identities.open_target(Direction::Upstream).key;
    let controller = UpstreamController::new(key, prepared(&budget)).unwrap();
    (budget, gate, identities, controller)
}

fn start(
    controller: &mut UpstreamController,
    identities: &mut Identities,
    backend: RunControl,
    capacity: usize,
) -> RunToken {
    let prepared = controller.prepare_run().unwrap();
    let control = Arc::new(UpstreamControl::new(backend));
    let lease = identities
        .begin(Direction::Upstream, control.clone())
        .unwrap();
    let token = lease.token();
    if let Err(error) = controller.start(lease, control, prepared, capacity) {
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

fn complete(controller: &mut UpstreamController, identities: &mut Identities) -> RunToken {
    let token = start(controller, identities, RunControl::new(), CALLBACK_CAPACITY);
    let terminal = terminal(controller);
    assert_eq!(terminal.token(), token);
    assert!(matches!(terminal.status(), WorkerStatus::Completed));
    terminal.into_lease().acknowledge_termination().unwrap();
    token
}

#[test]
fn actual_completion_preserves_configuration_provenance_and_outcome() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let session_footprint = controller
        .inspect(|session| session.retained_footprint().unwrap())
        .unwrap();
    let charge = budget.usage();
    assert_eq!(
        charge.payload_bytes,
        session_footprint.payload_bytes + size_of::<Retained<Mutex<SessionStorage>>>()
            - size_of::<Simulation>()
    );
    assert_eq!(charge.ordered_entries, session_footprint.ordered_entries);
    assert_eq!(charge.shared_roots, session_footprint.shared_roots + 1);
    let target = controller
        .inspect(|session| session.batch_report_retained_bound().unwrap())
        .unwrap();
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
    let token = complete(&mut controller, &mut identities);
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
    let actual = report.retained_footprint().unwrap();
    assert!(actual.payload_bytes <= target.payload_bytes);
    assert!(actual.ordered_entries <= target.ordered_entries);
    assert!(actual.shared_roots <= target.shared_roots);
    assert!(gate.active().is_none());
    assert!(matches!(controller.poll().unwrap(), ControllerEvent::Idle));
    controller
        .inspect(|session| session.preview().unwrap())
        .unwrap();
}

#[test]
fn acknowledged_pause_keeps_global_slot_and_resume_uses_actual_backend() {
    let (_budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    backend.pause();
    let token = start(&mut controller, &mut identities, backend, CALLBACK_CAPACITY);
    wait_for(|| controller.progress().unwrap().1.status == RunStatus::Paused);
    gate.acknowledge_pause(token, true).unwrap();
    identities.select(Direction::Downstream);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Paused);
    assert_eq!(controller.inspect(|_| ()), Err(ControllerError::Busy));
    assert!(matches!(
        controller.prepare_run(),
        Err(AdmissionError::Controller(ControllerError::Busy))
    ));
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
    let session_charge = budget.usage();
    let old_token = complete(&mut controller, &mut identities);
    let report_charge = difference(budget.usage(), session_charge);
    let (_, old_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let token = start(&mut controller, &mut identities, backend, CALLBACK_CAPACITY);
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
        budget.usage(),
        combine([session_charge, report_charge, report_charge])
    );
    drop(controller);
    assert_eq!(budget.usage(), combine([report_charge, report_charge]));
    drop(partial);
    assert_eq!(budget.usage(), report_charge);
    drop(old_report);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn failed_document_admission_preserves_existing_owner_and_releases_candidate() {
    let (budget, _, mut identities, mut controller) = fixture();
    complete(&mut controller, &mut identities);
    let (_, old_report) = controller.report().unwrap();
    let before = budget.usage();
    let mut document = ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap();
    document.simulation.source_seed = 0; // Existing public invalid PRBS seed.
    let Err(AdmissionError::Backend(error)) =
        PreparedSession::from_document(document, disabled_receiver_history(), &budget)
    else {
        panic!("Invalid real fixture must retain its backend error");
    };
    assert_eq!(error.category(), ErrorCode::InvalidInput);
    assert_eq!(budget.usage(), before);
    assert!(Arc::ptr_eq(&controller.report().unwrap().1, &old_report));
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
    let old_token = complete(&mut controller, &mut identities);
    let (_, old_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let external_control = backend.clone();
    let external_session = prepared(&budget);
    let external_budget = budget.clone();
    let external = thread::spawn(move || {
        let mut session = RunningSession::take(external_session.session).unwrap();
        let simulation = session.simulation.as_mut().unwrap();
        let limit = simulation.batch_report_retained_bound().unwrap();
        let reservation = external_budget
            .reserve_resources(report_resources(limit).unwrap())
            .unwrap();
        simulation
            .run_with_control_bounded(&external_control, limit)
            .map(|report| Arc::new(reservation.retain(report)))
    });
    wait_for(|| backend.progress().status == RunStatus::Paused);
    let token = start(
        &mut controller,
        &mut identities,
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
        external.join().unwrap().unwrap().value().status,
        RunStatus::Cancelled
    );
}

#[test]
fn changed_generation_and_replaced_session_reject_returning_unstarted_lease() {
    for replace in [false, true] {
        let (budget, gate, mut identities, mut controller) = fixture();
        let target = controller.prepare_run().unwrap();
        let configuration_charge = difference(
            budget.usage(),
            report_resources(target.backend_limit).unwrap(),
        );
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
        let Err(error) = controller.start(lease, control, target, CALLBACK_CAPACITY) else {
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
        assert_eq!(budget.usage(), configuration_charge);
        controller
            .inspect(|session| session.preview().unwrap())
            .unwrap();
    }
}

#[test]
fn stale_transport_message_cannot_adopt_actual_prior_report() {
    let (_budget, gate, mut identities, mut controller) = fixture();
    let old_token = complete(&mut controller, &mut identities);
    let (_, actual_report) = controller.report().unwrap();
    let backend = RunControl::new();
    backend.pause();
    let current = start(&mut controller, &mut identities, backend, CALLBACK_CAPACITY);
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
    let configuration_charge = budget.usage();
    let report_bound = controller
        .inspect(|session| session.batch_report_retained_bound().unwrap())
        .unwrap();
    let target_charge = report_resources(report_bound).unwrap();
    start(&mut controller, &mut identities, backend, 0);

    // Rendezvous report delivery keeps the real worker terminal pending.
    wait_for(|| observer.progress().status == RunStatus::Completed);
    assert_eq!(
        budget.usage(),
        combine([configuration_charge, target_charge])
    );
    assert!(gate.active().is_some());
    drop(controller);
    wait_for(|| gate.active().is_none());
    wait_for(|| budget.usage() == RetainedResources::default());
}

#[test]
fn paused_controller_destruction_cancels_without_joining() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    backend.pause();
    let observer = backend.clone();
    start(&mut controller, &mut identities, backend, CALLBACK_CAPACITY);
    wait_for(|| observer.progress().status == RunStatus::Paused);
    assert!(Arc::strong_count(&controller.session) >= 2);
    drop(controller);
    wait_for(|| gate.active().is_none());
    assert_eq!(observer.progress().status, RunStatus::Cancelled);
    wait_for(|| budget.usage() == RetainedResources::default());
}

#[test]
fn actual_session_guard_restores_after_application_unwind() {
    let (budget, _, _, controller) = fixture();
    let configuration_charge = budget.usage();
    let owner = Arc::clone(&controller.session);
    assert!(catch_unwind(AssertUnwindSafe(|| {
        let _guard = RunningSession::take(owner).unwrap();
        panic!("Application guard unwind fixture");
    }))
    .is_err());
    controller
        .inspect(|session| session.preview().unwrap())
        .unwrap();
    assert_eq!(budget.usage(), configuration_charge);
}

fn combine<const N: usize>(values: [RetainedResources; N]) -> RetainedResources {
    values
        .into_iter()
        .fold(RetainedResources::default(), |sum, value| {
            RetainedResources {
                payload_bytes: sum.payload_bytes.checked_add(value.payload_bytes).unwrap(),
                ordered_entries: sum
                    .ordered_entries
                    .checked_add(value.ordered_entries)
                    .unwrap(),
                shared_roots: sum.shared_roots.checked_add(value.shared_roots).unwrap(),
            }
        })
}

fn difference(total: RetainedResources, base: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: total.payload_bytes.checked_sub(base.payload_bytes).unwrap(),
        ordered_entries: total
            .ordered_entries
            .checked_sub(base.ordered_entries)
            .unwrap(),
        shared_roots: total.shared_roots.checked_sub(base.shared_roots).unwrap(),
    }
}

#[test]
fn each_actual_report_dimension_rejects_before_lease_or_backend_claim() {
    let (reference_budget, _, _, reference) = fixture();
    let configuration_charge = reference_budget.usage();
    let target = reference.prepare_run().unwrap();
    let combined = reference_budget.usage();
    let report_charge = difference(combined, configuration_charge);
    assert!(
        report_charge.payload_bytes > 0
            && report_charge.ordered_entries > 0
            && report_charge.shared_roots > 0
    );
    drop(target);
    let maxima = [
        RetainedResources {
            payload_bytes: combined.payload_bytes - 1,
            ..combined
        },
        RetainedResources {
            ordered_entries: combined.ordered_entries - 1,
            ..combined
        },
        RetainedResources {
            shared_roots: combined.shared_roots - 1,
            ..combined
        },
    ];
    for (dimension, maximum) in maxima.into_iter().enumerate() {
        let budget = RetentionBudget::with_limits(maximum);
        let gate = DispatchGate::default();
        let identities = make_identities(&budget, &gate);
        let controller = UpstreamController::new(
            identities.open_target(Direction::Upstream).key,
            prepared(&budget),
        )
        .unwrap();
        let before = budget.usage();
        let control = RunControl::new();
        let Err(AdmissionError::Retention(error)) = controller.prepare_run() else {
            panic!("Insufficient actual report dimension must reject");
        };
        assert!(matches!(
            (dimension, error),
            (0, RetentionError::LimitExceeded { .. })
                | (1, RetentionError::OrderedEntriesExceeded { .. })
                | (2, RetentionError::SharedRootsExceeded { .. })
        ));
        assert_eq!(budget.usage(), before);
        assert!(gate.active().is_none());
        assert_eq!(control.progress().status, RunStatus::Idle);
        controller
            .inspect(|session| {
                assert_eq!(
                    session.document().unwrap().original_bytes(),
                    Some(SOURCE_DOCUMENT)
                )
            })
            .unwrap();
    }
    // Incoming configuration failure also leaves an accepted source/report untouched.
    for maximum in [
        RetainedResources {
            payload_bytes: 0,
            ..combined
        },
        RetainedResources {
            shared_roots: 0,
            ..combined
        },
    ] {
        let too_small = RetentionBudget::with_limits(maximum);
        assert!(matches!(
            PreparedSession::from_document(
                ConfigurationDocument::from_json(SOURCE_DOCUMENT).unwrap(),
                disabled_receiver_history(),
                &too_small
            ),
            Err(AdmissionError::Retention(
                RetentionError::LimitExceeded { .. } | RetentionError::SharedRootsExceeded { .. }
            ))
        ));
        assert_eq!(too_small.usage(), RetainedResources::default());
    }
    assert_eq!(reference_budget.usage(), configuration_charge);
    reference
        .inspect(|session| session.preview().unwrap())
        .unwrap();
}

#[test]
fn prepared_target_from_another_owner_or_key_returns_the_original_lease() {
    for foreign_key in [false, true] {
        let (budget, gate, mut identities, mut controller) = fixture();
        let before = budget.usage();
        let foreign_budget = RetentionBudget::with_limits(TEST_MAXIMUM_RESOURCES);
        let foreign_key_value = if foreign_key {
            let separate_gate = DispatchGate::default();
            let other = make_identities(&foreign_budget, &separate_gate);
            other.open_target(Direction::Upstream).key
        } else {
            controller.key
        };
        let other = UpstreamController::new(foreign_key_value, prepared(&foreign_budget)).unwrap();
        let foreign_configuration = foreign_budget.usage();
        let target = other.prepare_run().unwrap();
        let control = Arc::new(UpstreamControl::new(RunControl::new()));
        let lease = identities
            .begin(Direction::Upstream, control.clone())
            .unwrap();
        let token = lease.token();
        let Err(error) = controller.start(lease, control.clone(), target, CALLBACK_CAPACITY) else {
            panic!("Foreign preparation must reject");
        };
        let (reason, lease) = error.into_parts();
        assert!(matches!(
            reason,
            StartFailure::Controller(ControllerError::StalePreparation)
        ));
        assert_eq!(lease.token(), token);
        assert_eq!(control.backend.progress().status, RunStatus::Idle);
        assert_eq!(budget.usage(), before);
        assert_eq!(foreign_budget.usage(), foreign_configuration);
        assert_eq!(gate.active().unwrap().token, token);
        lease.acknowledge_termination().unwrap();
        assert!(gate.active().is_none());
    }
}

#[test]
fn fixed_wrapper_adjustments_replace_scientific_root_once_and_check_overflow() {
    let scientific = RetainedFootprint {
        payload_bytes: size_of::<BatchReport>(),
        ..Default::default()
    };
    let wrapped = report_resources(scientific).unwrap();
    assert_eq!(wrapped.payload_bytes, size_of::<Retained<BatchReport>>());
    assert_eq!(wrapped.ordered_entries, 0);
    assert_eq!(wrapped.shared_roots, 1); // New outer report Arc allocation.
    assert_eq!(
        report_resources(RetainedFootprint::default()),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        session_resources(RetainedFootprint {
            payload_bytes: usize::MAX,
            ..scientific
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        report_resources(RetainedFootprint {
            shared_roots: usize::MAX,
            ..scientific
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
}
