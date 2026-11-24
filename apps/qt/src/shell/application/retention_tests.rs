/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual fixed application owners and genuine transport/lease disposal fixtures.
use super::*;
use crate::shell::{spawn_worker, DispatchPhase, WorkerEvent, WorkerSender, WorkerStatus};
use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    time::{Duration, Instant},
};

// Probe constructor-owned quantities; these ceilings are not application policy.
const PROBE_LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

// Preserve the three original accounting/gate/Sessions roots independently.
const CORE_HOST_ROOTS: usize = 3;

// The source registry adds one distinct fixed root without changing the core.
const FIXED_HOST_ROOTS: usize = CORE_HOST_ROOTS + 1;

// One terminal fits the existing bounded worker channel without a data callback.
const CALLBACK_CAPACITY: usize = 1;

// Deadlock/final-disposal watchdog only; no scientific runtime or polling policy.
const WATCHDOG: Duration = Duration::from_secs(5);
const WORKER_LABEL: &str = "fixed-host-retention";
const INVALID_WORKER_LABEL: &str = "fixed-host\0invalid";
const WORKER_FAILURE: &str = "fixed-host worker failure fixture";

fn resources(payload_bytes: usize, shared_roots: usize) -> RetainedResources {
    RetainedResources {
        payload_bytes,
        ordered_entries: 0,
        shared_roots,
        ..Default::default()
    }
}

fn add(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    left.checked_add(right).unwrap()
}

fn difference(total: RetainedResources, base: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: total.payload_bytes.checked_sub(base.payload_bytes).unwrap(),
        ordered_entries: total
            .ordered_entries
            .checked_sub(base.ordered_entries)
            .unwrap(),
        shared_roots: total.shared_roots.checked_sub(base.shared_roots).unwrap(),
        channel_count: total.channel_count.checked_sub(base.channel_count).unwrap(),
        slot_count: total.slot_count.checked_sub(base.slot_count).unwrap(),
    }
}

fn fixed_parts() -> (RetainedResources, RetainedResources, RetainedResources) {
    let budget = RetentionBudget::for_application(PROBE_LIMITS).unwrap();
    let accounting = budget.usage();
    let gate = DispatchGate::for_application(&budget).unwrap();
    let accounting_and_gate = budget.usage();
    assert_eq!(accounting.shared_roots, 1);
    assert_eq!(
        accounting_and_gate.shared_roots,
        accounting.shared_roots + 1
    );
    assert_eq!(accounting.ordered_entries, 0);
    assert_eq!(accounting_and_gate.ordered_entries, 0);
    let core = add(
        accounting_and_gate,
        resources(size_of::<Retained<Mutex<Sessions>>>(), 1),
    );
    assert_eq!(core.shared_roots, CORE_HOST_ROOTS);
    let registry = InputPaths::new(&budget).unwrap();
    let with_registry = budget.usage();
    let registry_resources = RetainedResources {
        payload_bytes: with_registry.payload_bytes - accounting_and_gate.payload_bytes,
        ordered_entries: with_registry.ordered_entries - accounting_and_gate.ordered_entries,
        shared_roots: with_registry.shared_roots - accounting_and_gate.shared_roots,
        ..Default::default()
    };
    assert!(registry_resources.payload_bytes > 0);
    assert_eq!(registry_resources.ordered_entries, 0);
    assert_eq!(registry_resources.shared_roots, 1);
    let full = add(core, registry_resources);
    drop(registry);
    assert_eq!(budget.usage(), accounting_and_gate);
    drop(gate);
    assert_eq!(budget.usage(), accounting);
    (accounting, accounting_and_gate, full)
}

fn configure(host: &ApplicationSessions, direction: Direction) -> SessionKey {
    match direction {
        Direction::Downstream => host.configure_downstream(true).unwrap(),
        Direction::Upstream => host.configure_upstream(true).unwrap(),
    }
}

#[test]
fn host_counts_three_core_roots_and_one_registry_without_inline_double_count() {
    let (_, _, full) = fixed_parts();
    assert_eq!(full.shared_roots, FIXED_HOST_ROOTS);
    assert_eq!(full.ordered_entries, 0);
    let host = ApplicationSessions::new(full).unwrap();
    assert_eq!(host.budget().usage(), full);
    assert!(matches!(
        ApplicationSessions::new(RetainedResources {
            payload_bytes: full.payload_bytes - 1,
            ..full
        }),
        Err(ApplicationError::Retention(
            RetentionError::LimitExceeded { .. }
        ))
    ));
    for shared_roots in 0..FIXED_HOST_ROOTS {
        assert!(matches!(
            ApplicationSessions::new(RetainedResources {
                shared_roots,
                ..full
            }),
            Err(ApplicationError::Retention(
                RetentionError::SharedRootsExceeded { .. }
            ))
        ));
    }
    assert_eq!(host.budget().usage(), full);
}

#[test]
fn production_constructor_rolls_back_every_partial_prefix_on_byte_or_root_rejection() {
    let (accounting, accounting_and_gate, full) = fixed_parts();
    let downstream = size_of::<Retained<DownstreamReadiness>>();
    let upstream = size_of::<Retained<UpstreamReadiness>>();
    let after_downstream = accounting_and_gate
        .payload_bytes
        .checked_add(downstream)
        .unwrap();
    let after_upstream = after_downstream.checked_add(upstream).unwrap();
    let after_sessions = accounting_and_gate
        .payload_bytes
        .checked_add(size_of::<Retained<Mutex<Sessions>>>())
        .unwrap();
    for payload_bytes in [
        accounting_and_gate.payload_bytes - 1,
        after_downstream - 1,
        after_upstream - 1,
        after_sessions - 1,
        full.payload_bytes - 1,
    ] {
        let budget = RetentionBudget::for_application(RetainedResources {
            payload_bytes,
            ..full
        })
        .unwrap();
        assert!(matches!(
            ApplicationSessions::from_budget(budget.clone()),
            Err(ApplicationError::Retention(
                RetentionError::LimitExceeded { .. }
            ))
        ));
        assert_eq!(budget.usage(), accounting);
    }
    for shared_roots in accounting.shared_roots..FIXED_HOST_ROOTS {
        let budget = RetentionBudget::for_application(RetainedResources {
            shared_roots,
            ..full
        })
        .unwrap();
        assert!(matches!(
            ApplicationSessions::from_budget(budget.clone()),
            Err(ApplicationError::Retention(
                RetentionError::SharedRootsExceeded { .. }
            ))
        ));
        assert_eq!(budget.usage(), accounting);
    }
}

#[test]
fn exact_idle_capacity_rejects_readiness_candidates_without_changing_snapshots() {
    let (_, _, full) = fixed_parts();
    let host = ApplicationSessions::new(full).unwrap();
    let downstream = host.snapshot(Direction::Downstream);
    let upstream = host.snapshot(Direction::Upstream);
    for result in [
        host.configure_downstream(true),
        host.configure_downstream_if(downstream.key, true),
        host.configure_upstream(true),
    ] {
        assert!(matches!(
            result,
            Err(ApplicationError::Retention(
                RetentionError::LimitExceeded { .. }
            ))
        ));
        assert_eq!(host.snapshot(Direction::Downstream), downstream);
        assert_eq!(host.snapshot(Direction::Upstream), upstream);
        assert!(host.active().is_none());
        assert_eq!(host.budget().usage(), full);
    }
}

#[test]
fn host_clones_share_sessions_until_final_drop_then_leave_only_accounting() {
    let (accounting, _, full) = fixed_parts();
    let host = ApplicationSessions::new(full).unwrap();
    let budget = host.budget().clone();
    let released = Arc::downgrade(&host.sessions);
    let alias = host.clone();
    assert!(Arc::ptr_eq(&host.sessions, &alias.sessions));
    assert_eq!(
        host.snapshot(Direction::Downstream),
        alias.snapshot(Direction::Downstream)
    );
    assert_eq!(
        host.snapshot(Direction::Upstream),
        alias.snapshot(Direction::Upstream)
    );
    drop(host);
    assert!(released.upgrade().is_some());
    assert_eq!(budget.usage(), full);
    drop(alias);
    assert!(released.upgrade().is_none());
    assert_eq!(budget.usage(), accounting);
}

struct ControlWitness {
    budget: RetentionBudget,
    cancellations: Arc<AtomicUsize>,
    destroyed: mpsc::Sender<RetainedResources>,
}

impl RunCancellation for ControlWitness {
    /// Counts the actual host/gate fixture request without scientific control claims.
    fn request_cancel(&self) {
        self.cancellations.fetch_add(1, Ordering::SeqCst);
    }
}

impl Drop for ControlWitness {
    /// Signals the observed charge before the gate's control fields are released.
    fn drop(&mut self) {
        let _ = self.destroyed.send(self.budget.usage());
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Completed,
    Failed,
    Panicked,
}

fn wait_for_usage(budget: &RetentionBudget, expected: RetainedResources) {
    let started = Instant::now();
    while budget.usage() != expected {
        assert!(
            started.elapsed() < WATCHDOG,
            "Final gate disposal exceeded watchdog"
        );
        std::thread::yield_now();
    }
}

fn worker_after_host_drop(
    direction: Direction,
    cancelling: bool,
    outcome: Outcome,
    disconnect: bool,
) {
    let (accounting, accounting_and_gate, full) = fixed_parts();
    let host = ApplicationSessions::new(PROBE_LIMITS).unwrap();
    let key = configure(&host, direction);
    assert_eq!(host.budget().usage(), full);
    let budget = host.budget().clone();
    let gate = host.gate.clone();
    let released_sessions = Arc::downgrade(&host.sessions);
    let cancellations = Arc::new(AtomicUsize::new(0));
    let (destroyed, observed_destruction) = mpsc::channel();

    // These controls/channels are explicit lifecycle instrumentation. This test
    // does not establish complete control, closure or transport memory admission.
    let control = Arc::new(ControlWitness {
        budget: budget.clone(),
        cancellations: Arc::clone(&cancellations),
        destroyed,
    });
    let lease = host.begin(key, control.clone()).unwrap();
    let token = lease.token();
    drop(control);
    let (entered, observed_entry) = mpsc::channel();
    let (finish, await_finish) = mpsc::channel();
    let receiver = spawn_worker(
        WORKER_LABEL,
        CALLBACK_CAPACITY,
        0, // No DATA callbacks in this terminal-only lifecycle fixture.
        &budget,
        lease,
        move |_: WorkerSender<(), &'static str>| {
            entered.send(()).unwrap();
            await_finish.recv_timeout(WATCHDOG).unwrap();
            match outcome {
                Outcome::Completed => Ok(()),
                Outcome::Failed => Err(WORKER_FAILURE),
                Outcome::Panicked => panic!("Fixed-host worker unwind fixture"),
            }
        },
    )
    .unwrap_or_else(|_| panic!("Fixture thread must start"));
    observed_entry.recv_timeout(WATCHDOG).unwrap();
    // The blocked actual closure keeps all its C3 owners through this boundary.
    let worker_storage = difference(budget.usage(), full);
    assert!(worker_storage.payload_bytes > 0);
    assert_eq!(worker_storage.channel_count, 1); // This actual terminal channel.
    assert_eq!(worker_storage.slot_count, CALLBACK_CAPACITY);
    let blocked_storage = add(accounting_and_gate, worker_storage);

    // Acknowledged lifecycle phase only; no backend pause behavior is simulated.
    host.acknowledge_pause(token, true).unwrap();
    assert_eq!(host.active().unwrap().phase, DispatchPhase::Paused);
    if cancelling {
        host.cancel(token).unwrap();
        assert_eq!(host.active().unwrap().phase, DispatchPhase::Cancelling);
    }
    drop(host);
    assert!(released_sessions.upgrade().is_none());
    assert_eq!(budget.usage(), blocked_storage);
    assert_eq!(cancellations.load(Ordering::SeqCst), 1);
    assert_eq!(gate.active().unwrap().token, token);
    assert_eq!(gate.active().unwrap().phase, DispatchPhase::Cancelling);
    if disconnect {
        drop(receiver);
        assert_eq!(budget.usage(), blocked_storage);
        finish.send(()).unwrap();
    } else {
        finish.send(()).unwrap();
        let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
            panic!("Operation must produce its sole terminal");
        };
        assert_eq!(terminal.token(), token);
        match outcome {
            Outcome::Completed => assert_eq!(terminal.status(), &WorkerStatus::Completed),
            Outcome::Failed => assert_eq!(terminal.status(), &WorkerStatus::Failed(WORKER_FAILURE)),
            Outcome::Panicked => assert_eq!(terminal.status(), &WorkerStatus::Panicked),
        }
        assert_eq!(gate.active().unwrap().token, token);
        // Dispose the endpoint and wait for actual task cleanup before checking
        // the sole extracted terminal's independently retained fixed frame.
        drop(receiver);
        let held_terminal = add(accounting_and_gate, terminal.fixture_reserved_resources());
        wait_for_usage(&budget, held_terminal);
        assert_eq!(budget.usage(), held_terminal);
        drop(terminal);
    }
    let observed = observed_destruction.recv_timeout(WATCHDOG).unwrap();
    // On disconnect the destructor runs inside the still-admitted task; on
    // receipt it runs inside the separately guarded terminal. Both keep the
    // exact original accounting/gate owners. Final disposal is checked below.
    assert!(observed.payload_bytes >= accounting_and_gate.payload_bytes);
    assert_eq!(
        observed.ordered_entries,
        accounting_and_gate.ordered_entries
    );
    assert!(observed.shared_roots >= accounting_and_gate.shared_roots);
    assert!(gate.active().is_none());
    wait_for_usage(&budget, accounting_and_gate);
    assert_eq!(budget.usage(), accounting_and_gate);
    drop(gate);

    // On disconnected delivery, the worker may still be releasing its final
    // lease handle after signalling control destruction. Wait only for disposal.
    wait_for_usage(&budget, accounting);
    assert_eq!(cancellations.load(Ordering::SeqCst), 1);
}

#[test]
fn real_terminal_outcomes_keep_the_gate_charged_after_both_direction_hosts_drop() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        for cancelling in [false, true] {
            for outcome in [Outcome::Completed, Outcome::Failed, Outcome::Panicked] {
                worker_after_host_drop(direction, cancelling, outcome, false);
            }
        }
    }
}

#[test]
fn disconnected_receivers_acknowledge_only_after_the_real_operation_returns() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        for cancelling in [false, true] {
            worker_after_host_drop(direction, cancelling, Outcome::Completed, true);
        }
    }
}

#[test]
fn invalid_static_thread_label_returns_the_original_unstarted_host_lease() {
    let (accounting, accounting_and_gate, _) = fixed_parts();
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::new(PROBE_LIMITS).unwrap();
        let key = configure(&host, direction);
        let budget = host.budget().clone();
        let gate = host.gate.clone();
        let cancellations = Arc::new(AtomicUsize::new(0));
        let (destroyed, observed_destruction) = mpsc::channel();
        let lease = host
            .begin(
                key,
                Arc::new(ControlWitness {
                    budget: budget.clone(),
                    cancellations: Arc::clone(&cancellations),
                    destroyed,
                }),
            )
            .unwrap();
        let token = lease.token();
        let result = spawn_worker(
            INVALID_WORKER_LABEL,
            CALLBACK_CAPACITY,
            0, // No DATA callbacks in this invalid-name fixture.
            &budget,
            lease,
            |_: WorkerSender<(), &'static str>| panic!("Invalid label cannot start an operation"),
        );
        let Err(error) = result else {
            panic!("Invalid static thread label must reject");
        };
        let (cause, lease) = error.into_parts();
        assert_eq!(cause.kind(), std::io::ErrorKind::InvalidInput);
        assert_eq!(lease.token(), token);
        assert_eq!(gate.active().unwrap().token, token);
        drop(host);
        assert_eq!(budget.usage(), accounting_and_gate);
        assert_eq!(cancellations.load(Ordering::SeqCst), 1);
        lease.acknowledge_termination().unwrap();
        assert_eq!(
            observed_destruction.recv_timeout(WATCHDOG).unwrap(),
            accounting_and_gate
        );
        assert!(gate.active().is_none());
        assert_eq!(budget.usage(), accounting_and_gate);
        drop(gate);
        assert_eq!(budget.usage(), accounting);
    }
}
