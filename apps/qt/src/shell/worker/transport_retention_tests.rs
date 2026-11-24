/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Genuine leases, finite preflight, and independently retained terminal frames.
use super::*;
use crate::shell::{Direction, DispatchGate, RunCancellation, RunId, SessionId, SessionKey};
use std::time::Instant;

const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(5);
const CALLBACK_SLOTS: usize = 1; // Backpressure, independent of total events.
const DATA_EVENTS: usize = 3; // Three held fixture callbacks exceed that capacity.

struct Control;
impl RunCancellation for Control {
    /// Fixture control has no scientific state to advance.
    ///
    /// Cancellation is immediate and does not manufacture a terminal event.
    fn request_cancel(&self) {}
}

fn lease(gate: &DispatchGate) -> RunLease {
    gate.admit(
        RunToken {
            key: SessionKey {
                direction: Direction::Downstream,
                session: SessionId::allocate().unwrap(),
                generation: Default::default(),
            },
            run: RunId(1), // Independent first-run identity anchor.
        },
        Arc::new(Control),
    )
    .unwrap()
}

fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !ready() {
        assert!(
            Instant::now() < deadline,
            "Actual transport disposal watchdog"
        );
        std::thread::yield_now();
    }
}

#[test]
fn held_data_and_terminal_keep_each_frame_after_actual_endpoint_cleanup() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let gate = DispatchGate::default();
    let prepared = prepare_worker(
        "held-frames",
        CALLBACK_SLOTS,
        DATA_EVENTS,
        &budget,
        |(), mut sender| {
            for value in [1_u8, 2, 3] {
                sender.send(value).unwrap();
            }
            Ok::<_, &'static str>(())
        },
    )
    .unwrap();
    assert!(gate.active().is_none());
    let receiver = prepared
        .spawn(lease(&gate), ())
        .unwrap_or_else(|_| panic!("Fixture thread starts"));
    let mut data = Vec::new();
    for _ in 0..DATA_EVENTS {
        let WorkerEvent::Message { value, .. } = receiver.recv_timeout(WATCHDOG).unwrap() else {
            panic!("Data precedes terminal")
        };
        data.push(value);
    }
    let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("Sole terminal follows data")
    };
    drop(receiver);
    let frame_bytes = size_of::<Retained<Completion<u8, &'static str>>>()
        .max(size_of::<WorkerEvent<u8, &'static str>>());
    let held = RetainedResources {
        payload_bytes: (data.len() + 1) * frame_bytes,
        ..Default::default()
    };
    wait_for(|| budget.usage() == held);
    assert!(gate.active().is_some());
    assert_eq!(terminal.status(), &WorkerStatus::Completed);
    drop(terminal);
    assert!(gate.active().is_none());
    assert_eq!(budget.used().0, data.len() * frame_bytes);
    drop(data);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn finite_inventory_rejects_extra_publication_and_unstarted_failure_returns_lease() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let gate = DispatchGate::default();
    let prepared = prepare_worker(
        "invalid\0thread",
        CALLBACK_SLOTS,
        0,
        &budget,
        |(), _: WorkerSender<(), ()>| panic!("Unstarted science"),
    )
    .unwrap();
    let original = lease(&gate);
    let token = original.token();
    let Err(error) = prepared.spawn(original, ()) else {
        panic!("Static invalid name must reject")
    };
    let (_, recovered) = error.into_parts();
    assert_eq!(recovered.token(), token);
    assert!(gate.active().is_some());
    recovered.acknowledge_termination().unwrap();
    assert_eq!(budget.usage(), RetainedResources::default());

    let prepared = prepare_worker(
        "finite-data",
        CALLBACK_SLOTS,
        0,
        &budget,
        |(), mut sender| {
            assert_eq!(sender.send(9_u8), Err(9)); // No DATA inventory was admitted.
            Ok::<_, ()>(())
        },
    )
    .unwrap();
    let receiver = prepared
        .spawn(lease(&gate), ())
        .unwrap_or_else(|_| panic!("Fixture thread starts"));
    let WorkerEvent::Terminal(terminal) = receiver.recv_timeout(WATCHDOG).unwrap() else {
        panic!("No data may escape inventory")
    };
    drop(terminal);
    drop(receiver);
    wait_for(|| budget.usage() == RetainedResources::default());
}

#[test]
fn huge_inventory_is_rejected_before_builder_allocation_and_rolls_back() {
    // A finite inventory gets no buffer space beyond the independent fixed prefix.
    const BUFFER_PRESSURE_EVENTS: usize = 1024;
    let frame_bytes =
        size_of::<Retained<Completion<(), ()>>>().max(size_of::<WorkerEvent<(), ()>>());
    let prefix = size_of::<Retained<Mutex<Option<(RunLease, ())>>>>()
        + size_of::<Retained<()>>()
        + frame_bytes;
    let budget = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: prefix,
        ..LIMITS
    });
    assert!(matches!(prepare_worker(
        "no-allocation",
        CALLBACK_SLOTS,
        BUFFER_PRESSURE_EVENTS,
        &budget,
        |(), _: WorkerSender<(), ()>| Ok(())
    ), Err(RetentionError::LimitExceeded { maximum, requested })
        if maximum.0 == prefix && requested.0 == prefix
            + BUFFER_PRESSURE_EVENTS * size_of::<DeliveryPermit<Completion<(), ()>>>()));
    assert_eq!(budget.usage(), RetainedResources::default());
    let budget = RetentionBudget::with_limits(LIMITS);
    assert!(matches!(
        prepare_worker(
            "checked-inventory",
            CALLBACK_SLOTS,
            usize::MAX,
            &budget,
            |(), _: WorkerSender<(), ()>| Ok(())
        ),
        Err(RetentionError::ArithmeticOverflow)
    ));
    assert_eq!(budget.usage(), RetainedResources::default());
}
