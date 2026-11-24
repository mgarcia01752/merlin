/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real controller transport rejection returns genuine admission and old science.
use super::*;
use crate::shell::{DispatchGate, RetainedBytes, ShellSessions, WorkerStatus};
use merlin::upstream::RunStatus;
use std::time::{Duration, Instant};

const DOCUMENT: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
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
const CALLBACK_SLOTS: usize = 1; // One-report fixture backpressure policy.

#[test]
fn actual_transport_rejection_preserves_old_report_and_returns_original_unstarted_lease() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let gate = DispatchGate::default();
    let mut identities: ShellSessions<(), (), (), ()> = ShellSessions::new(
        budget.clone(),
        gate.clone(),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
        budget.reserve(RetainedBytes(0)).unwrap().retain(()),
    )
    .unwrap();
    let key = identities.open_target(Direction::Upstream).key;
    let prepared = PreparedSession::from_document(
        ConfigurationDocument::from_json(DOCUMENT).unwrap(),
        ReceiverObservationPolicy::new(0, 0).unwrap(),
        &budget,
    )
    .unwrap(); // Receiver history disabled, unchanged supported fixture.
    let mut controller = UpstreamController::new(key, prepared).unwrap();
    let target = controller.prepare_run().unwrap();
    let control = UpstreamControl::new(RunControl::new(), &budget).unwrap();
    let lease = identities
        .begin(Direction::Upstream, control.clone())
        .unwrap();
    if let Err(error) = controller.start(lease, control, target, CALLBACK_SLOTS) {
        let (reason, lease) = error.into_parts();
        lease.acknowledge_termination().unwrap();
        panic!("Real initial fixture start failed: {reason:?}");
    }
    let deadline = Instant::now() + WATCHDOG;
    loop {
        if let ControllerEvent::Terminal(terminal) = controller.poll().unwrap() {
            assert!(matches!(terminal.status(), WorkerStatus::Completed));
            terminal.into_lease().acknowledge_termination().unwrap();
            break;
        }
        assert!(
            Instant::now() < deadline,
            "Actual upstream terminal watchdog"
        );
        std::thread::yield_now();
    }
    while budget.usage().channel_count != 0 {
        assert!(
            Instant::now() < deadline,
            "Actual upstream endpoint cleanup watchdog"
        );
        std::thread::yield_now();
    }
    let old = controller.report().unwrap();
    assert_eq!(old.1.value().status, RunStatus::Completed);
    for channels in [true, false] {
        let target = controller.prepare_run().unwrap();
        let control = UpstreamControl::new(RunControl::new(), &budget).unwrap();
        let observer = Arc::clone(&control);
        let lease = identities
            .begin(Direction::Upstream, control.clone())
            .unwrap();
        let token = lease.token();
        let used = budget.usage();
        let pressure = budget
            .reserve_resources(if channels {
                RetainedResources {
                    channel_count: LIMITS.channel_count - used.channel_count,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    slot_count: LIMITS.slot_count - used.slot_count,
                    ..Default::default()
                }
            })
            .unwrap();
        let Err(error) = controller.start(lease, control, target, CALLBACK_SLOTS) else {
            panic!("Actual transport must reject")
        };
        let (reason, lease) = error.into_parts();
        assert!(matches!(reason, StartFailure::Spawn(_)));
        assert_eq!(lease.token(), token);
        assert_eq!(observer.backend.progress().status, RunStatus::Idle);
        assert!(controller.active.is_none());
        let current = controller.report().unwrap();
        assert_eq!(current.0, old.0);
        assert!(Arc::ptr_eq(&current.1, &old.1));
        assert!(gate.active().is_some());
        lease.acknowledge_termination().unwrap();
        assert!(gate.active().is_none());
        drop(pressure);
    }
}
