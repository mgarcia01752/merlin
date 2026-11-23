/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Pure application ownership tests; fixed payloads are not scientific engines.
use super::*;
use std::mem::size_of;
use std::sync::atomic::{AtomicUsize, Ordering};

// Deliberately ample for these tiny fixed-size fixtures, not a production policy.
const FIXTURE_BUDGET_BYTES: usize = 4096;
type Sessions = ShellSessions<[u8; 4], [u8; 12], [u8; 8], [u8; 16]>;

#[derive(Default)]
struct CancellationCounter(AtomicUsize);

impl RunCancellation for CancellationCounter {
    /// Records an immediate cancellation request without worker/blocking behavior.
    fn request_cancel(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn retained<T>(budget: &RetentionBudget, payload: T) -> Retained<T> {
    budget
        .reserve(RetainedBytes(size_of::<T>()))
        .unwrap()
        .retain(payload)
}

fn fixture() -> (
    Sessions,
    RetentionBudget,
    DispatchGate,
    Arc<CancellationCounter>,
) {
    let budget = RetentionBudget::new(RetainedBytes(FIXTURE_BUDGET_BYTES));
    let gate = DispatchGate::default();
    let sessions = Sessions::new(
        budget.clone(),
        gate.clone(),
        retained(&budget, [1; 4]),
        retained(&budget, [2; 8]),
    )
    .unwrap();
    (
        sessions,
        budget,
        gate,
        Arc::new(CancellationCounter::default()),
    )
}

#[test]
fn distinct_typed_directions_keep_configuration_results_and_dirty_state() {
    let (mut sessions, budget, _, cancellation) = fixture();
    let lease = sessions.begin(Direction::Downstream, cancellation).unwrap();
    let run = lease.token();
    sessions
        .finish_downstream(lease, Some(retained(&budget, [3; 12])))
        .unwrap();
    let downstream = sessions.save_target();
    sessions.select(Direction::Upstream);
    assert_eq!(sessions.save_target().key.direction, Direction::Upstream);
    sessions
        .configure_upstream(retained(&budget, [4; 8]))
        .unwrap();
    assert!(sessions.save_target().dirty);
    assert_eq!(sessions.downstream().configuration(), &[1; 4]);
    assert_eq!(sessions.upstream().configuration(), &[4; 8]);
    assert_eq!(sessions.downstream().result(), Some((run, &[3; 12])));
    assert!(sessions.upstream().result().is_none());
    sessions.select(Direction::Downstream);
    assert_eq!(sessions.save_target(), downstream);
}

#[test]
fn running_and_paused_runs_reject_second_start_and_keep_scoped_controls() {
    let (mut sessions, _, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let token = lease.token();
    sessions.select(Direction::Upstream);
    assert!(matches!(
        sessions.begin(Direction::Upstream, cancellation.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    gate.acknowledge_pause(token, true).unwrap();
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Paused);
    assert!(matches!(
        sessions.begin(Direction::Upstream, cancellation.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    let wrong_direction = RunToken {
        key: sessions.save_target().key,
        run: token.run,
    };
    assert_eq!(
        gate.request_cancel(wrong_direction),
        Err(DispatchError::StaleCallback)
    );
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
    gate.acknowledge_pause(token, false).unwrap();
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Running);
    sessions.finish_downstream(lease, None).unwrap();
    assert!(gate.active().is_none());
}

#[test]
fn cancel_while_paused_is_once_and_holds_slot_until_partial_terminal_report() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    let token = lease.token();
    gate.acknowledge_pause(token, true).unwrap();
    gate.request_cancel(token).unwrap();
    gate.request_cancel(token).unwrap();
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    assert_eq!(
        gate.acknowledge_pause(token, false),
        Err(DispatchError::Cancelling)
    );
    assert!(matches!(
        sessions.begin(Direction::Downstream, cancellation),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    sessions
        .finish_upstream(lease, Some(retained(&budget, [9; 16])))
        .unwrap();
    assert_eq!(sessions.upstream().result(), Some((token, &[9; 16])));
    assert!(gate.active().is_none());
}

#[test]
fn callbacks_require_direction_session_generation_and_run_identity() {
    let (mut sessions, budget, _, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let old_generation = lease.token();
    sessions.finish_downstream(lease, None).unwrap();
    sessions
        .configure_downstream(retained(&budget, [7; 4]))
        .unwrap();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let old_run = lease.token();
    sessions.finish_downstream(lease, None).unwrap();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let current = lease.token();
    assert_eq!(sessions.validate_callback(current), Ok(()));
    for stale in [old_generation, old_run] {
        assert_eq!(
            sessions.validate_callback(stale),
            Err(SessionError::Dispatch(DispatchError::StaleCallback))
        );
        assert_eq!(
            sessions.publish_downstream(stale, retained(&budget, [4; 12])),
            Err(SessionError::Dispatch(DispatchError::StaleCallback))
        );
    }
    assert_eq!(
        sessions.publish_upstream(current, retained(&budget, [4; 16])),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    sessions
        .finish_downstream(lease, Some(retained(&budget, [5; 12])))
        .unwrap();
    let guard = sessions.open_target(Direction::Downstream);
    sessions
        .replace_downstream(
            guard,
            ReplacePermission::DiscardConfirmed,
            retained(&budget, [8; 4]),
        )
        .unwrap();
    let lease = sessions.begin(Direction::Downstream, cancellation).unwrap();
    assert_ne!(lease.token().key.session, current.key.session);
    assert_eq!(
        sessions.publish_downstream(current, retained(&budget, [4; 12])),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert!(sessions.downstream().result().is_none());
    sessions.finish_downstream(lease, None).unwrap();
}

#[test]
fn failed_terminal_and_rejected_candidate_preserve_previous_accepted_report() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let original = lease.token();
    sessions
        .finish_downstream(lease, Some(retained(&budget, [6; 12])))
        .unwrap();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    assert!(matches!(
        sessions.configure_downstream(retained(&budget, [9; 4])),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    sessions.finish_downstream(lease, None).unwrap();
    assert_eq!(sessions.downstream().result(), Some((original, &[6; 12])));
    assert_eq!(sessions.downstream().configuration(), &[1; 4]);
    let foreign = RetentionBudget::new(RetainedBytes(FIXTURE_BUDGET_BYTES));
    let lease = sessions.begin(Direction::Downstream, cancellation).unwrap();
    assert_eq!(
        sessions.finish_downstream(lease, Some(retained(&foreign, [7; 12]))),
        Err(SessionError::WrongBudget)
    );
    assert!(gate.active().is_none());
    assert_eq!(foreign.used(), RetainedBytes(0));
    assert_eq!(sessions.downstream().result(), Some((original, &[6; 12])));
}

#[test]
fn queue_old_run_replacement_and_owner_drop_keep_byte_charges() {
    let (mut sessions, budget, _, cancellation) = fixture();
    // Literal independent anchors are the two fixed config array sizes (4 + 8).
    assert_eq!(budget.used(), RetainedBytes(12));
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let token = lease.token();
    let queued_old = retained(&budget, [5; 12]);
    sessions
        .finish_downstream(lease, Some(retained(&budget, [3; 12])))
        .unwrap();
    // Two report payloads are owned, even though only one has been adopted.
    assert_eq!(budget.used(), RetainedBytes(36));
    let lease = sessions.begin(Direction::Downstream, cancellation).unwrap();
    assert_eq!(
        sessions.publish_downstream(token, queued_old),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert_eq!(budget.used(), RetainedBytes(24));
    sessions
        .finish_downstream(lease, Some(retained(&budget, [4; 12])))
        .unwrap();
    assert_eq!(budget.used(), RetainedBytes(24));
    let queued_after_owner = retained(&budget, [8_u8; 16]);
    drop(sessions);
    assert_eq!(budget.used(), RetainedBytes(16));
    drop(queued_after_owner);
    assert_eq!(budget.used(), RetainedBytes(0));
}

#[test]
fn byte_limits_are_inclusive_shared_and_checked_without_large_allocations() {
    let budget = RetentionBudget::new(RetainedBytes(7));
    let reserved = budget.reserve(RetainedBytes(7)).unwrap();
    assert!(matches!(
        budget.clone().reserve(RetainedBytes(1)),
        Err(RetentionError::LimitExceeded {
            maximum: RetainedBytes(7),
            requested: RetainedBytes(8)
        })
    ));
    assert_eq!(budget.used(), RetainedBytes(7));
    drop(reserved);
    assert_eq!(budget.used(), RetainedBytes(0));
    let budget = RetentionBudget::new(RetainedBytes(usize::MAX));
    let reserved = budget.reserve(RetainedBytes(usize::MAX)).unwrap();
    assert!(matches!(
        budget.reserve(RetainedBytes(1)),
        Err(RetentionError::ArithmeticOverflow)
    ));
    assert_eq!(budget.used(), RetainedBytes(usize::MAX));
    drop(reserved);
    assert_eq!(budget.used(), RetainedBytes(0));
}

#[test]
fn dirty_guard_rechecks_actual_target_and_save_does_not_clean_newer_edits() {
    let (mut sessions, budget, _, _) = fixture();
    sessions.mark_metadata_dirty(Direction::Upstream).unwrap();
    let guard = sessions.open_target(Direction::Upstream);
    // Save still targets selected downstream, not the dirty Open destination.
    assert_eq!(sessions.save_target().key.direction, Direction::Downstream);
    assert_eq!(
        sessions.replace_upstream(
            guard,
            ReplacePermission::RequireClean,
            retained(&budget, [9; 8])
        ),
        Err(SessionError::DirtyTarget)
    );
    sessions.mark_metadata_dirty(Direction::Upstream).unwrap();
    assert_eq!(
        sessions.replace_upstream(
            guard,
            ReplacePermission::DiscardConfirmed,
            retained(&budget, [9; 8])
        ),
        Err(SessionError::StaleGuard)
    );
    sessions.select(Direction::Upstream);
    let metadata_save = sessions.save_target();
    sessions.mark_metadata_dirty(Direction::Upstream).unwrap();
    assert_eq!(
        sessions.acknowledge_save(metadata_save),
        Err(SessionError::StaleGuard)
    );
    assert!(sessions.save_target().dirty);
    let save = sessions.save_target();
    sessions
        .configure_upstream(retained(&budget, [8; 8]))
        .unwrap();
    assert_eq!(
        sessions.acknowledge_save(save),
        Err(SessionError::StaleGuard)
    );
    assert!(sessions.save_target().dirty);
    let fresh = sessions.open_target(Direction::Upstream);
    sessions.acknowledge_save(fresh).unwrap();
    // A dirty prompt obtained before a successful Save must also be rechecked.
    assert_eq!(
        sessions.replace_upstream(
            fresh,
            ReplacePermission::DiscardConfirmed,
            retained(&budget, [9; 8])
        ),
        Err(SessionError::StaleGuard)
    );
    assert_eq!(sessions.upstream().configuration(), &[8; 8]);
    let fresh = sessions.open_target(Direction::Upstream);
    sessions
        .replace_upstream(
            fresh,
            ReplacePermission::RequireClean,
            retained(&budget, [9; 8]),
        )
        .unwrap();
    assert!(!sessions.save_target().dirty);
}

#[test]
fn generation_revision_and_run_exhaustion_preserve_state_without_wrap() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    sessions.downstream.snapshot.key.generation = ConfigurationGeneration(u64::MAX);
    let original = sessions.save_target();
    assert_eq!(
        sessions.configure_downstream(retained(&budget, [9; 4])),
        Err(SessionError::IdentityExhausted)
    );
    assert_eq!(sessions.save_target(), original);
    assert_eq!(sessions.downstream().configuration(), &[1; 4]);
    sessions.downstream.snapshot.key.generation = ConfigurationGeneration::default();
    sessions.downstream.snapshot.revision = EditRevision(u64::MAX);
    assert_eq!(
        sessions.configure_downstream(retained(&budget, [9; 4])),
        Err(SessionError::IdentityExhausted)
    );
    assert_eq!(
        sessions.mark_metadata_dirty(Direction::Downstream),
        Err(SessionError::IdentityExhausted)
    );
    assert_eq!(sessions.downstream().configuration(), &[1; 4]);
    sessions.downstream.last_run = RunId(u64::MAX);
    assert!(matches!(
        sessions.begin(Direction::Downstream, cancellation),
        Err(SessionError::IdentityExhausted)
    ));
    assert!(gate.active().is_none());
}

#[test]
fn destruction_requests_once_and_worker_lease_prevents_successor_overlap() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let old = lease.token();
    sessions.shutdown();
    sessions.shutdown();
    drop(sessions);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().token, old);
    let mut successor = Sessions::new(
        budget.clone(),
        gate.clone(),
        retained(&budget, [7; 4]),
        retained(&budget, [8; 8]),
    )
    .unwrap();
    assert!(matches!(
        successor.begin(Direction::Upstream, cancellation.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    assert_eq!(
        successor.finish_downstream(lease, Some(retained(&budget, [6; 12]))),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert!(successor.downstream().result().is_none());
    let next = successor
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    assert_ne!(next.token().key.session, old.key.session);
    successor.finish_upstream(next, None).unwrap();
    let terminal = successor
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    drop(successor);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 2);
    terminal.acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
}

#[test]
fn closing_or_wrong_direction_terminal_rejection_still_consumes_lease() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    assert_eq!(
        sessions.finish_downstream(lease, None),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert!(gate.active().is_none());
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    sessions.shutdown();
    assert_eq!(
        sessions.finish_downstream(lease, Some(retained(&budget, [3; 12]))),
        Err(SessionError::Closing)
    );
    assert!(gate.active().is_none());
    assert!(sessions.downstream().result().is_none());
    assert!(matches!(
        sessions.begin(Direction::Downstream, cancellation),
        Err(SessionError::Closing)
    ));
}

#[test]
fn dropped_unacknowledged_lease_cancels_but_never_reopens_gate() {
    let (mut sessions, _, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    drop(lease);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    assert!(matches!(
        sessions.begin(Direction::Upstream, cancellation.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    drop(sessions);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
}

#[test]
fn accepted_downstream_edits_cancel_once_and_keep_busy_until_stale_terminal() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let old = lease.token();
    sessions
        .publish_downstream(old, retained(&budget, [3; 12]))
        .unwrap();
    gate.acknowledge_pause(old, true).unwrap();
    assert!(matches!(
        sessions.configure_downstream(retained(&budget, [7; 4])),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
    sessions
        .configure_downstream_cancelling(retained(&budget, [7; 4]))
        .unwrap();
    assert_eq!(sessions.downstream().configuration(), &[7; 4]);
    assert!(sessions.downstream().result().is_none());
    assert_eq!(sessions.upstream().configuration(), &[2; 8]);
    assert_eq!(gate.active().unwrap().token, old);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert!(sessions.validate_callback(old).is_err());
    sessions
        .configure_downstream_cancelling(retained(&budget, [8; 4]))
        .unwrap();
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert!(matches!(
        sessions.begin(Direction::Upstream, cancellation.clone()),
        Err(SessionError::Dispatch(DispatchError::Busy(_)))
    ));
    assert_eq!(
        sessions.finish_downstream(lease, Some(retained(&budget, [9; 12]))),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert!(gate.active().is_none());
    assert!(sessions.downstream().result().is_none());
    let next = sessions.begin(Direction::Downstream, cancellation).unwrap();
    assert_eq!(next.token().key.session, old.key.session);
    assert_ne!(next.token().key.generation, old.key.generation);
    sessions.finish_downstream(next, None).unwrap();
}

#[test]
fn cancelling_edit_of_idle_direction_never_touches_the_other_active_control() {
    for active in [Direction::Downstream, Direction::Upstream] {
        let (mut sessions, budget, gate, cancellation) = fixture();
        let lease = sessions.begin(active, cancellation.clone()).unwrap();
        let token = lease.token();
        match active {
            Direction::Downstream => {
                sessions
                    .configure_upstream_cancelling(retained(&budget, [7; 8]))
                    .unwrap();
                assert_eq!(sessions.downstream().configuration(), &[1; 4]);
            }
            Direction::Upstream => {
                sessions
                    .configure_downstream_cancelling(retained(&budget, [7; 4]))
                    .unwrap();
                assert_eq!(sessions.upstream().configuration(), &[2; 8]);
            }
        }
        assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
        assert_eq!(gate.active().unwrap().token, token);
        assert_eq!(gate.active().unwrap().phase, DispatchPhase::Running);
        sessions.validate_callback(token).unwrap();
        match active {
            Direction::Downstream => sessions.finish_downstream(lease, None).unwrap(),
            Direction::Upstream => sessions.finish_upstream(lease, None).unwrap(),
        }
    }
}

#[test]
fn accepted_upstream_edit_cancels_only_its_old_generation_until_terminal() {
    let (mut sessions, budget, gate, cancellation) = fixture();
    let lease = sessions
        .begin(Direction::Upstream, cancellation.clone())
        .unwrap();
    let old = lease.token();
    sessions
        .publish_upstream(old, retained(&budget, [3; 16]))
        .unwrap();
    sessions
        .configure_upstream_cancelling(retained(&budget, [7; 8]))
        .unwrap();
    assert_eq!(sessions.upstream().configuration(), &[7; 8]);
    assert!(sessions.upstream().result().is_none());
    assert_eq!(sessions.downstream().configuration(), &[1; 4]);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().token, old);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    assert_eq!(
        sessions.finish_upstream(lease, Some(retained(&budget, [9; 16]))),
        Err(SessionError::Dispatch(DispatchError::StaleCallback))
    );
    assert!(gate.active().is_none());
    assert!(sessions.upstream().result().is_none());
}

#[test]
fn invalid_cancelling_edits_preserve_active_owners_before_any_cancel_request() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let (mut sessions, budget, gate, cancellation) = fixture();
        let foreign = RetentionBudget::new(RetainedBytes(FIXTURE_BUDGET_BYTES));
        let lease = sessions.begin(direction, cancellation.clone()).unwrap();
        let token = lease.token();
        let before = sessions.open_target(direction);
        let error = match direction {
            Direction::Downstream => {
                sessions.configure_downstream_cancelling(retained(&foreign, [7; 4]))
            }
            Direction::Upstream => {
                sessions.configure_upstream_cancelling(retained(&foreign, [7; 8]))
            }
        };
        assert_eq!(error, Err(SessionError::WrongBudget));
        assert_eq!(sessions.open_target(direction), before);
        assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
        assert_eq!(gate.active().unwrap().token, token);
        sessions.shutdown();
        let error = match direction {
            Direction::Downstream => {
                sessions.configure_downstream_cancelling(retained(&budget, [7; 4]))
            }
            Direction::Upstream => {
                sessions.configure_upstream_cancelling(retained(&budget, [7; 8]))
            }
        };
        assert_eq!(error, Err(SessionError::Closing));
        assert_eq!(sessions.open_target(direction), before);
        assert_eq!(cancellation.0.load(Ordering::SeqCst), 1);
        lease.acknowledge_termination().unwrap();
    }
}

#[test]
fn cancelling_edit_revision_exhaustion_preserves_configuration_and_active_work() {
    for exhaust_generation in [false, true] {
        let (mut sessions, budget, gate, cancellation) = fixture();
        if exhaust_generation {
            sessions.downstream.snapshot.key.generation = ConfigurationGeneration(u64::MAX);
        } else {
            sessions.downstream.snapshot.revision = EditRevision(u64::MAX);
        }
        let lease = sessions
            .begin(Direction::Downstream, cancellation.clone())
            .unwrap();
        let before = sessions.open_target(Direction::Downstream);
        assert_eq!(
            sessions.configure_downstream_cancelling(retained(&budget, [7; 4])),
            Err(SessionError::IdentityExhausted)
        );
        assert_eq!(sessions.open_target(Direction::Downstream), before);
        assert_eq!(sessions.downstream().configuration(), &[1; 4]);
        assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
        assert_eq!(gate.active().unwrap().phase, DispatchPhase::Running);
        sessions.validate_callback(lease.token()).unwrap();
        sessions.finish_downstream(lease, None).unwrap();
    }
}

#[test]
fn foreign_owner_destruction_never_cancels_another_sessions_worker() {
    let (mut owner, budget, gate, cancellation) = fixture();
    let lease = owner
        .begin(Direction::Downstream, cancellation.clone())
        .unwrap();
    let other = Sessions::new(
        budget.clone(),
        gate.clone(),
        retained(&budget, [7; 4]),
        retained(&budget, [8; 8]),
    )
    .unwrap();
    drop(other);
    assert_eq!(cancellation.0.load(Ordering::SeqCst), 0);
    assert_eq!(gate.active().unwrap().token, lease.token());
    owner.finish_downstream(lease, None).unwrap();
}

#[test]
fn retained_payload_is_destroyed_before_its_byte_reservation_is_released() {
    struct DropProbe {
        budget: RetentionBudget,
        observed: Arc<AtomicUsize>,
    }
    impl Drop for DropProbe {
        /// Observes reservation ownership while this payload is being destroyed.
        fn drop(&mut self) {
            self.observed.store(self.budget.used().0, Ordering::SeqCst);
        }
    }
    let bytes = RetainedBytes(size_of::<DropProbe>());
    let budget = RetentionBudget::new(bytes);
    let observed = Arc::new(AtomicUsize::new(0));
    let reservation = budget.reserve(bytes).unwrap();
    let payload = reservation.retain(DropProbe {
        budget: budget.clone(),
        observed: observed.clone(),
    });
    drop(payload);
    assert_eq!(observed.load(Ordering::SeqCst), bytes.0);
    assert_eq!(budget.used(), RetainedBytes(0));
}
