/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Accounting fixtures exercise independent counters, not scientific footprints.
use super::*;
use crate::shell::{
    Direction, DispatchError, DispatchGate, DispatchPhase, ReplacePermission, RunCancellation,
    SessionError, ShellSessions,
};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{mpsc, Barrier};
use std::thread;

fn quantities(
    payload_bytes: usize,
    ordered_entries: usize,
    shared_roots: usize,
) -> RetainedResources {
    RetainedResources {
        payload_bytes,
        ordered_entries,
        shared_roots,
    }
}

#[test]
fn each_inclusive_limit_rejects_one_more_without_partial_counter_changes() {
    // Independent synthetic ceilings: seven bytes, two entries, three roots.
    let maximum = quantities(7, 2, 3);
    let budget = RetentionBudget::with_limits(maximum);
    let retained = budget.reserve_resources(maximum).unwrap();
    for (additional, expected) in [
        (
            quantities(1, 0, 0),
            RetentionError::LimitExceeded {
                maximum: RetainedBytes(7),
                requested: RetainedBytes(8),
            },
        ),
        (
            quantities(0, 1, 0),
            RetentionError::OrderedEntriesExceeded {
                maximum: 2,
                requested: 3,
            },
        ),
        (
            quantities(0, 0, 1),
            RetentionError::SharedRootsExceeded {
                maximum: 3,
                requested: 4,
            },
        ),
    ] {
        assert_eq!(
            budget.clone().reserve_resources(additional).unwrap_err(),
            expected
        );
        assert_eq!(budget.usage(), maximum);
    }
    let zero = budget
        .reserve_resources(RetainedResources::default())
        .unwrap();
    drop(retained);
    assert_eq!(budget.usage(), RetainedResources::default());
    drop(zero);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn overflow_in_each_dimension_preserves_all_charges_without_allocating_payloads() {
    let maximum = quantities(usize::MAX, usize::MAX, usize::MAX);
    for additional in [
        quantities(1, 0, 0),
        quantities(0, 1, 0),
        quantities(0, 0, 1),
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        let retained = budget.reserve_resources(maximum).unwrap();
        assert_eq!(
            budget.reserve_resources(additional).unwrap_err(),
            RetentionError::ArithmeticOverflow
        );
        assert_eq!(budget.usage(), maximum);
        drop(retained);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn failed_combined_reservation_cannot_consume_other_dimensions() {
    // Each candidate fits two ceilings and exceeds exactly the third.
    let budget = RetentionBudget::with_limits(quantities(7, 2, 3));
    for candidate in [
        quantities(8, 1, 1),
        quantities(1, 3, 1),
        quantities(1, 1, 4),
    ] {
        assert!(budget.reserve_resources(candidate).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    let accepted = budget.reserve_resources(quantities(7, 2, 3)).unwrap();
    assert_eq!(budget.usage(), quantities(7, 2, 3));
    drop(accepted);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn byte_only_constructor_does_not_silently_admit_unbounded_populations() {
    // The historical byte-only primitive retains its seven-byte fixture ceiling.
    let budget = RetentionBudget::new(RetainedBytes(7));
    assert!(matches!(
        budget.reserve_resources(quantities(0, 1, 0)),
        Err(RetentionError::OrderedEntriesExceeded { maximum: 0, .. })
    ));
    assert!(matches!(
        budget.reserve_resources(quantities(0, 0, 1)),
        Err(RetentionError::SharedRootsExceeded { maximum: 0, .. })
    ));
    let retained = budget.reserve(RetainedBytes(7)).unwrap();
    assert_eq!(budget.usage(), quantities(7, 0, 0));
    drop(retained);
    assert_eq!(budget.used(), RetainedBytes(0));
}

#[test]
fn concurrent_clones_share_atomic_admission_across_all_dimensions() {
    // Eight contenders race for three reservations, each charging (2, 1, 1).
    const CONTENDERS: usize = 8;
    const ADMITTED: usize = 3;
    let budget = RetentionBudget::with_limits(quantities(6, 3, 3));
    let barrier = Arc::new(Barrier::new(CONTENDERS + 1));
    let workers: Vec<_> = (0..CONTENDERS)
        .map(|_| {
            let budget = budget.clone();
            let barrier = barrier.clone();
            thread::spawn(move || {
                barrier.wait();
                budget.reserve_resources(quantities(2, 1, 1))
            })
        })
        .collect();
    barrier.wait();
    // Returned reservations stay alive in either the join handle or this vector.
    let reservations: Vec<_> = workers
        .into_iter()
        .filter_map(|worker| worker.join().unwrap().ok())
        .collect();
    assert_eq!(reservations.len(), ADMITTED);
    assert_eq!(budget.usage(), quantities(6, 3, 3));
    drop(reservations);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn queued_and_replaced_aliases_keep_every_charge_until_final_drop() {
    // The synthetic charge is a counter fixture, not a bound for this string.
    let charge = quantities(7, 2, 3);
    let budget = RetentionBudget::with_limits(quantities(14, 4, 6));
    let old = Arc::new(budget.reserve_resources(charge).unwrap().retain("old"));
    let (sender, receiver) = mpsc::sync_channel(1);
    sender.send(old.clone()).unwrap();
    let accepted = budget.reserve_resources(charge).unwrap().retain("new");
    drop(old);
    assert_eq!(budget.usage(), quantities(14, 4, 6));
    drop(accepted);
    drop(sender);
    assert_eq!(budget.usage(), charge);
    let queued_alias = receiver.recv().unwrap();
    drop(receiver);
    assert_eq!(budget.usage(), charge);
    drop(queued_alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn data_destructor_and_unwind_release_all_quantities_in_order() {
    struct ObservedDrop {
        budget: RetentionBudget,
        expected: RetainedResources,
        observed: Arc<AtomicBool>,
    }

    impl Drop for ObservedDrop {
        fn drop(&mut self) {
            self.observed
                .store(self.budget.usage() == self.expected, Ordering::SeqCst);
        }
    }

    let charge = quantities(7, 2, 3);
    let budget = RetentionBudget::with_limits(charge);
    let observed = Arc::new(AtomicBool::new(false));
    let retained = budget
        .reserve_resources(charge)
        .unwrap()
        .retain(ObservedDrop {
            budget: budget.clone(),
            expected: charge,
            observed: observed.clone(),
        });
    assert!(std::panic::catch_unwind(move || {
        let _retained = retained;
        panic!("synthetic owner unwind");
    })
    .is_err());
    assert!(observed.load(Ordering::SeqCst));
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn paused_cancel_rejected_replacement_and_destroyed_owner_keep_all_charges() {
    #[derive(Default)]
    struct CancellationCounter(AtomicUsize);

    impl RunCancellation for CancellationCounter {
        /// Records cancellation without changing the simulated worker's lifetime.
        fn request_cancel(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    // Four synthetic charges fit: two configurations, an old report, a queue item.
    // Values test lifetime/accounting only, not these array payload footprints.
    let charge = quantities(7, 2, 3);
    let budget = RetentionBudget::with_limits(quantities(28, 8, 12));
    let gate = DispatchGate::default();
    let cancellation = Arc::new(CancellationCounter::default());
    let configuration = || budget.reserve_resources(charge).unwrap().retain([0_u8]);
    let mut sessions: ShellSessions<_, [u8; 1], _, [u8; 1]> = ShellSessions::new(
        budget.clone(),
        gate.clone(),
        configuration(),
        configuration(),
    )
    .unwrap();
    let completed = sessions
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    sessions
        .finish_upstream(completed, Some(configuration()))
        .unwrap();
    let lease = sessions
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    gate.acknowledge_pause(lease.token(), true).unwrap();
    let guard = sessions.open_target(Direction::Upstream);
    assert!(matches!(
        sessions.replace_upstream(guard, ReplacePermission::RequireClean, configuration()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    assert_eq!(budget.usage(), quantities(21, 6, 9));
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
    let queued = Arc::new(configuration());
    let worker_alias = queued.clone();
    assert_eq!(budget.usage(), quantities(28, 8, 12));
    gate.request_cancel(lease.token()).unwrap();
    drop(sessions);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    assert_eq!(budget.usage(), charge);
    drop(queued);
    assert_eq!(budget.usage(), charge);
    // Only the real terminal owner may release admission, independently of data.
    lease.acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
    assert_eq!(budget.usage(), charge);
    drop(worker_alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}
