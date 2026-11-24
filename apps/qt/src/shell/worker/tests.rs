/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real application threads and leases; no simulated engine acceptance.
use super::super::{
    Direction, DispatchGate, RetainedBytes, RetainedResources, RetentionBudget, RunCancellation,
    RunId,
};
use super::super::{SessionId, SessionKey};
use super::*;
use std::time::Duration;

// Test-only deadlock watchdog; not a scientific runtime or UI polling interval.
const WATCHDOG: Duration = Duration::from_secs(5);

// One queued callback deliberately exercises the application's bounded transport.
const CALLBACK_CAPACITY: usize = 1;
// Actual finite fixture publication counts, separate from queue capacity.
const ONE_DATA_MESSAGE: usize = 1;
const NO_DATA_MESSAGES: usize = 0;

fn budget() -> RetentionBudget {
    RetentionBudget::with_limits(RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    })
}

impl<T, E> WorkerReceiver<T, E> {
    /// Creates the original stale-event fixture using the admitted production queue.
    /// # Arguments
    /// * `budget` - Exact host covering the genuine prior report and injected frame.
    /// * `token` - Deliberately stale application identity, without a new worker lease.
    /// * `value` - Existing real guarded report; scientific data is never fabricated.
    /// # Returns
    /// One queued identified message, with the same source/adopted frame coverage.
    /// # Panics
    /// Panics only if the fixture budget rejects this single frame.
    pub(crate) fn fixture_message(budget: &RetentionBudget, token: RunToken, value: T) -> Self {
        let (sender, receiver) = bounded_channel(budget, CALLBACK_CAPACITY).unwrap();
        let permit = sender
            .reserve_bytes(std::mem::size_of::<WorkerEvent<T, E>>())
            .unwrap();
        assert!(permit.send(Completion::Message { token, value }).is_ok());
        Self { receiver }
    }
}

impl<E> WorkerTerminal<E> {
    /// Reads the actual extracted terminal frame charge for fixture assertions.
    ///
    /// # Returns
    /// Its existing reservation; reading never acknowledges or releases the lease.
    pub(crate) fn fixture_reserved_resources(&self) -> RetainedResources {
        self._delivery.fixture_reserved_resources()
    }
}

struct CancellationProbe(Option<mpsc::Sender<()>>);

impl RunCancellation for CancellationProbe {
    /// No scientific control is needed for these transport ownership tests.
    fn request_cancel(&self) {}
}

impl Drop for CancellationProbe {
    /// Signals destruction after the gate releases its last control owner.
    fn drop(&mut self) {
        if let Some(sender) = &self.0 {
            let _ = sender.send(());
        }
    }
}

fn admit(
    gate: &DispatchGate,
    direction: Direction,
    destroyed: Option<mpsc::Sender<()>>,
) -> RunLease {
    // First issued run is an independent application identity anchor.
    let token = RunToken {
        key: SessionKey {
            direction,
            session: SessionId::allocate().unwrap(),
            generation: Default::default(),
        },
        run: RunId(1),
    };
    gate.admit(token, Arc::new(CancellationProbe(destroyed)))
        .unwrap()
}

#[test]
fn successful_direction_callbacks_keep_the_original_identity_until_host_finish() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let gate = DispatchGate::default();
        let lease = admit(&gate, direction, None);
        let token = lease.token();
        let receiver = spawn_worker(
            "transport-success",
            CALLBACK_CAPACITY,
            ONE_DATA_MESSAGE,
            &budget(),
            lease,
            |mut sender| {
                sender.send("owned callback").unwrap();
                Ok::<_, &'static str>(())
            },
        )
        .unwrap_or_else(|_| panic!("Test thread must start"));
        match receiver.recv_timeout(WATCHDOG).unwrap() {
            WorkerEvent::Message {
                token: received,
                value,
            } => {
                assert_eq!(received, token);
                assert_eq!(*value.value(), "owned callback");
            }
            WorkerEvent::Terminal(_) => panic!("Data precedes its terminal"),
        }
        let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
            panic!("Exactly one terminal follows data");
        };
        assert_eq!(terminal.token(), token);
        assert_eq!(terminal.status(), &WorkerStatus::Completed);
        assert_eq!(gate.active().unwrap().token, token);
        terminal.into_lease().acknowledge_termination().unwrap();
        assert!(gate.active().is_none());
        assert!(matches!(
            receiver.recv_timeout(WATCHDOG),
            Err(mpsc::RecvTimeoutError::Disconnected)
        ));
    }
}

#[test]
fn errors_and_panics_deliver_known_completion_without_abandoning_the_lease() {
    for panic_operation in [false, true] {
        let gate = DispatchGate::default();
        let lease = admit(&gate, Direction::Downstream, None);
        let token = lease.token();
        let receiver = spawn_worker(
            "transport-error",
            CALLBACK_CAPACITY,
            NO_DATA_MESSAGES,
            &budget(),
            lease,
            move |_: WorkerSender<(), &'static str>| {
                if panic_operation {
                    panic!("Independent worker unwind fixture");
                }
                Err("original scientific error")
            },
        )
        .unwrap_or_else(|_| panic!("Test thread must start"));
        let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
            panic!("A failed operation must terminate");
        };
        if panic_operation {
            assert_eq!(terminal.status(), &WorkerStatus::Panicked);
        } else {
            assert_eq!(
                terminal.status(),
                &WorkerStatus::Failed("original scientific error")
            );
        }
        assert_eq!(gate.active().unwrap().token, token);
        drop(terminal);
        assert!(gate.active().is_none());
    }
}

#[test]
fn destroyed_gui_owner_cannot_release_admission_before_the_operation_returns() {
    let gate = DispatchGate::default();
    let (destroyed, observed_destruction) = mpsc::channel();
    let lease = admit(&gate, Direction::Upstream, Some(destroyed));
    let token = lease.token();
    let (entered, observed_entry) = mpsc::channel();
    let (finish, await_finish) = mpsc::channel();
    let receiver = spawn_worker(
        "transport-destroyed-owner",
        CALLBACK_CAPACITY,
        NO_DATA_MESSAGES,
        &budget(),
        lease,
        move |_: WorkerSender<(), ()>| {
            entered.send(()).unwrap();
            await_finish.recv_timeout(WATCHDOG).unwrap();
            Ok(())
        },
    )
    .unwrap_or_else(|_| panic!("Test thread must start"));
    observed_entry.recv_timeout(WATCHDOG).unwrap();
    drop(receiver);
    assert_eq!(gate.active().unwrap().token, token);
    finish.send(()).unwrap();
    observed_destruction.recv_timeout(WATCHDOG).unwrap();
    assert!(gate.active().is_none());
}

#[test]
fn queued_payload_charges_follow_actual_release_when_the_gui_disappears() {
    // Eight real fixture bytes make the accounting oracle independent of policy.
    let budget = budget();
    let bytes = std::mem::size_of::<super::super::Retained<[u8; 8]>>();
    let reserved = Box::new(
        budget
            .reserve(RetainedBytes(bytes))
            .unwrap()
            .retain([0_u8; 8]),
    );
    assert_eq!(budget.used(), RetainedBytes(bytes));
    let gate = DispatchGate::default();
    let (destroyed, observed_destruction) = mpsc::channel();
    let lease = admit(&gate, Direction::Downstream, Some(destroyed));
    let (sent, observed_send) = mpsc::channel();
    let receiver = spawn_worker(
        "transport-queued-payload",
        CALLBACK_CAPACITY,
        ONE_DATA_MESSAGE,
        &budget,
        lease,
        move |mut sender| {
            assert!(sender.send(reserved).is_ok());
            sent.send(()).unwrap();
            Ok::<_, ()>(())
        },
    )
    .unwrap_or_else(|_| panic!("Test thread must start"));
    observed_send.recv_timeout(WATCHDOG).unwrap();
    assert!(budget.used().0 >= bytes);
    assert_eq!(budget.usage().channel_count, 1);
    assert_eq!(budget.usage().slot_count, CALLBACK_CAPACITY);
    assert!(gate.active().is_some());
    drop(receiver);
    observed_destruction.recv_timeout(WATCHDOG).unwrap();

    // Terminal acknowledgement precedes the admitted outer task's final Drop.
    let deadline = std::time::Instant::now() + WATCHDOG;
    while budget.used() != RetainedBytes(0) {
        assert!(
            std::time::Instant::now() < deadline,
            "Worker storage did not drop"
        );
        std::thread::yield_now();
    }
    assert_eq!(budget.used(), RetainedBytes(0));
    assert!(gate.active().is_none());
}

#[test]
fn deterministic_os_spawn_failure_returns_the_unstarted_original_lease() {
    let gate = DispatchGate::default();
    let lease = admit(&gate, Direction::Upstream, None);
    let token = lease.token();
    let prepared = prepare_worker(
        "transport-failed-spawn",
        CALLBACK_CAPACITY,
        NO_DATA_MESSAGES,
        &budget(),
        |(), _: WorkerSender<(), ()>| panic!("Failed spawn cannot execute science"),
    )
    .unwrap();
    let result = prepared.spawn_with(lease, (), |_name, _task| {
        Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "OS spawn fixture",
        ))
    });
    let error = result.err().expect("Injected spawn failure must reject");
    let (cause, lease) = error.into_parts();
    assert_eq!(cause.kind(), io::ErrorKind::WouldBlock);
    assert_eq!(lease.token(), token);
    assert_eq!(gate.active().unwrap().token, token);
    lease.acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
}

#[test]
fn invalid_static_thread_label_returns_the_unstarted_lease() {
    let gate = DispatchGate::default();
    let lease = admit(&gate, Direction::Downstream, None);
    let result = spawn_worker(
        "invalid\0label",
        CALLBACK_CAPACITY,
        NO_DATA_MESSAGES,
        &budget(),
        lease,
        |_: WorkerSender<(), ()>| panic!("Invalid name cannot execute science"),
    );
    let (cause, lease) = result
        .err()
        .expect("Invalid label must reject")
        .into_parts();
    assert_eq!(cause.kind(), io::ErrorKind::InvalidInput);
    assert!(gate.active().is_some());
    lease.acknowledge_termination().unwrap();
    assert!(gate.active().is_none());
}
