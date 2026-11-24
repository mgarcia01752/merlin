/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Guarded gate allocation, real lease lifetime and data-before-guard destruction.
use super::*;
use crate::shell::{Direction, RunId, SessionKey};
use std::sync::atomic::{AtomicUsize, Ordering};

// Independent run identities exercise the existing global Busy transition.
const FIRST_RUN: RunId = RunId(1);
const SECOND_RUN: RunId = RunId(2);

// Probe actual application accounting; these ceilings are not a production policy.
const BASELINE_PROBE_LIMITS: RetainedResources = RetainedResources {
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

fn gate_resources() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<GateStorage>(),
        ordered_entries: 0,
        shared_roots: 1,
        ..Default::default()
    }
}

fn add(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
        ..Default::default()
    }
}

fn token(run: RunId) -> RunToken {
    RunToken {
        key: SessionKey {
            direction: Direction::Downstream,
            session: SessionId::allocate().unwrap(),
            generation: Default::default(),
        },
        run,
    }
}

#[test]
fn fixed_gate_admits_exact_dimensions_and_default_remains_unguarded() {
    let required = gate_resources();
    let budget = RetentionBudget::with_limits(required);
    let gate = DispatchGate::for_application(&budget).unwrap();
    assert_eq!(budget.usage(), required);
    assert!(gate.0._reservation.is_some());
    assert!(gate.active().is_none());
    let weak = Arc::downgrade(&gate.0);
    let alias = gate.clone();
    assert!(Arc::ptr_eq(&gate.0, &alias.0));
    drop(gate);
    assert_eq!(budget.usage(), required);
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
    let generic = DispatchGate::default();
    assert!(generic.0._reservation.is_none());
    assert!(generic.active().is_none());
}

#[test]
fn rejected_gate_allocation_preserves_the_existing_application_baseline() {
    let probe = RetentionBudget::for_application(BASELINE_PROBE_LIMITS).unwrap();
    let baseline = probe.usage();
    drop(probe);
    let required = gate_resources();
    for space in [
        RetainedResources {
            payload_bytes: required.payload_bytes - 1,
            ..required
        },
        RetainedResources {
            shared_roots: 0,
            ..required
        },
    ] {
        let budget = RetentionBudget::for_application(add(baseline, space)).unwrap();
        let rejected = DispatchGate::for_application(&budget);
        if space.shared_roots == 0 {
            assert!(matches!(
                rejected,
                Err(RetentionError::SharedRootsExceeded { .. })
            ));
        } else {
            assert!(matches!(
                rejected,
                Err(RetentionError::LimitExceeded { .. })
            ));
        }
        assert_eq!(budget.usage(), baseline);
    }
}

struct ControlWitness {
    budget: RetentionBudget,
    cancelled: Arc<AtomicUsize>,
    dropped_with: Arc<Mutex<Option<RetainedResources>>>,
}

impl RunCancellation for ControlWitness {
    /// Records the original nonblocking cooperative cancellation request.
    fn request_cancel(&self) {
        self.cancelled.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for ControlWitness {
    /// Observes accounting while the gate's actual control is destroyed.
    fn drop(&mut self) {
        *self.dropped_with.lock().unwrap() = Some(self.budget.usage());
    }
}

#[test]
fn lease_retains_the_gate_and_control_drops_before_its_last_guard() {
    for acknowledged in [false, true] {
        let required = gate_resources();
        let budget = RetentionBudget::with_limits(required);
        let gate = DispatchGate::for_application(&budget).unwrap();
        let weak = Arc::downgrade(&gate.0);
        let cancelled = Arc::new(AtomicUsize::new(0));
        let dropped_with = Arc::new(Mutex::new(None));
        let first = token(FIRST_RUN);
        let second = RunToken {
            key: first.key,
            run: SECOND_RUN,
        };

        // These tracking handles are external fixture instrumentation, not science.
        let control = Arc::new(ControlWitness {
            budget: budget.clone(),
            cancelled: Arc::clone(&cancelled),
            dropped_with: Arc::clone(&dropped_with),
        });
        let lease = gate.admit(first, control.clone()).unwrap();
        gate.acknowledge_pause(first, true).unwrap();
        assert_eq!(gate.active().unwrap().phase, DispatchPhase::Paused);
        assert!(matches!(
            gate.admit(second, control.clone()),
            Err(DispatchError::Busy(active)) if active.token == first
        ));
        gate.request_cancel(first).unwrap();
        gate.request_cancel(first).unwrap();
        assert_eq!(cancelled.load(Ordering::SeqCst), 1);
        assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
        assert!(matches!(
            gate.admit(second, control.clone()),
            Err(DispatchError::Busy(active)) if active.token == first
        ));
        assert_eq!(
            gate.acknowledge_pause(first, false),
            Err(DispatchError::Cancelling)
        );
        drop(control);
        drop(gate);
        assert!(weak.upgrade().is_some());
        assert_eq!(budget.usage(), required);
        assert_eq!(*dropped_with.lock().unwrap(), None);
        if acknowledged {
            lease.acknowledge_termination().unwrap();
        } else {
            let observer = weak.upgrade().unwrap();
            drop(lease);
            assert_eq!(
                lock(&observer.state).as_ref().unwrap().active.phase,
                DispatchPhase::Cancelling
            );
            assert_eq!(budget.usage(), required);

            // No scientific worker was started. Dispose the last observed fixture
            // owner without reusing its unacknowledged Busy slot.
            drop(observer);
        }
        assert_eq!(cancelled.load(Ordering::SeqCst), 1);
        assert_eq!(*dropped_with.lock().unwrap(), Some(required));
        assert!(weak.upgrade().is_none());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}
