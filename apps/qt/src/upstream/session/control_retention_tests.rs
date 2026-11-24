/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Admitted real upstream controls, optional common roots and exact host identity.
use super::*;
use crate::shell::{DispatchGate, RetainedBytes, ShellSessions};
use merlin::upstream::RunStatus;
use std::time::{Duration, Instant};

const DOCUMENT: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");

// Test-only host ceilings and scheduling deadline; no backend size estimate.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

// This established real fixture keeps optional receiver history disabled.
const ESTIMATE_FRAMES: usize = 0;
const ESTIMATE_POINTS: usize = 0;

type Identities = ShellSessions<(), (), (), ()>;
fn fixture() -> (
    RetentionBudget,
    DispatchGate,
    Identities,
    UpstreamController,
) {
    let budget = RetentionBudget::with_limits(LIMITS);
    let gate = DispatchGate::default();
    let identities = ShellSessions::new(
        budget.clone(),
        gate.clone(),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
    )
    .unwrap();
    let key = identities.open_target(Direction::Upstream).key;
    let prepared = PreparedSession::from_document(
        ConfigurationDocument::from_json(DOCUMENT).unwrap(),
        ReceiverObservationPolicy::new(ESTIMATE_FRAMES, ESTIMATE_POINTS).unwrap(),
        &budget,
    )
    .unwrap();
    (
        budget,
        gate,
        identities,
        UpstreamController::new(key, prepared).unwrap(),
    )
}

#[test]
fn optional_common_control_exact_layout_rejections_and_final_alias_are_passive() {
    for common in [false, true] {
        let backend = if common {
            RunControl::with_cancellation(merlin::common::RunControl::new())
        } else {
            RunControl::new()
        };
        let scientific = backend.retained_footprint().unwrap();
        let expected = RetainedResources {
            payload_bytes: scientific.payload_bytes - size_of::<RunControl>()
                + size_of::<UpstreamControl>(),
            ordered_entries: 0,
            shared_roots: if common { 3 } else { 2 },
            ..Default::default()
        };
        let budget = RetentionBudget::with_limits(expected);
        let owner = UpstreamControl::new(backend.clone(), &budget).unwrap();
        let alias = Arc::clone(&owner);
        let weak = Arc::downgrade(&owner);
        assert_eq!(budget.usage(), expected);
        assert!(budget.owns(&owner.guard));
        drop(owner);
        alias.request_cancel();
        assert_eq!(backend.progress().status, RunStatus::Idle);
        assert!(backend.is_cancelled());
        assert_eq!(budget.usage(), expected);
        drop(alias);
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
            let candidate = if common {
                RunControl::with_cancellation(merlin::common::RunControl::new())
            } else {
                RunControl::new()
            };
            assert!(matches!(
                UpstreamControl::new(candidate, &budget),
                Err(AdmissionError::Retention(_))
            ));
            assert_eq!(budget.usage(), RetainedResources::default());
        }
    }
}

#[test]
fn actual_foreign_control_rejects_before_claim_and_returns_the_original_lease() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let before = budget.usage();
    let target = controller.prepare_run().unwrap();
    let foreign = RetentionBudget::with_limits(LIMITS);
    let control = UpstreamControl::new(RunControl::new(), &foreign).unwrap();
    let foreign_charge = foreign.usage();
    let lease = identities
        .begin(Direction::Upstream, control.clone())
        .unwrap();
    let token = lease.token();
    let Err(error) = controller.start(lease, Arc::clone(&control), target, 0) else {
        panic!("Actual foreign control must reject");
    };
    let (reason, lease) = error.into_parts();
    assert!(matches!(
        reason,
        StartFailure::Controller(ControllerError::ForeignControl)
    ));
    assert_eq!(lease.token(), token);
    assert_eq!(gate.active().unwrap().token, token);
    assert_eq!(control.backend.progress().status, RunStatus::Idle);
    assert_eq!(budget.usage(), before);
    assert_eq!(foreign.usage(), foreign_charge);
    lease.acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
    drop(control);
    assert_eq!(foreign.usage(), RetainedResources::default());
    controller
        .inspect(|session| assert_eq!(session.document().unwrap().original_bytes(), Some(DOCUMENT)))
        .unwrap();
}

#[test]
fn paused_real_execution_and_controller_drop_keep_the_final_control_alias_charged() {
    let (budget, gate, mut identities, mut controller) = fixture();
    let backend = RunControl::new();
    backend.pause();
    let control = UpstreamControl::new(backend, &budget).unwrap();
    let expected = wrapped_resources::<UpstreamControl, RunControl>(
        control.backend.retained_footprint().unwrap(),
    )
    .unwrap();
    let target = controller.prepare_run().unwrap();
    let lease = identities
        .begin(Direction::Upstream, control.clone())
        .unwrap();
    controller
        .start(lease, Arc::clone(&control), target, 0)
        .unwrap_or_else(|error| {
            let (reason, lease) = error.into_parts();
            lease.acknowledge_termination().unwrap();
            panic!("Actual paused control must start: {reason:?}");
        });
    let deadline = Instant::now() + WATCHDOG;
    while control.backend.progress().status != RunStatus::Paused {
        assert!(Instant::now() < deadline, "Actual upstream pause watchdog");
        std::thread::yield_now();
    }
    assert!(gate.active().is_some());
    drop(controller);
    while gate.active().is_some() || budget.usage() != expected {
        assert!(
            Instant::now() < deadline,
            "Actual upstream cancellation disposal watchdog"
        );
        std::thread::yield_now();
    }
    assert_eq!(control.backend.progress().status, RunStatus::Cancelled);
    drop(control);
    assert_eq!(budget.usage(), RetainedResources::default());
}
