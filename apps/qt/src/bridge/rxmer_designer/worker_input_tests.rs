/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual independent request/selection ownership and detached generation lifetime.
use super::*;
use crate::shell::{RetainedResources, RetentionBudget};
use worker_inputs::{admit_request, SelectionSnapshot};

// Test counter ceiling; not an execution or application policy.
const TEST_LIMIT: RetainedResources = RetainedResources {
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
// Zero capacity synchronizes fixture checkpoints without buffered messages.
const RENDEZVOUS_CAPACITY: usize = 0;

#[test]
fn actual_generation_error_stays_admitted_after_inputs_drop_until_queue_collection() {
    let budget = RetentionBudget::with_limits(TEST_LIMIT);
    let (source, _) = super::tests::fixture();
    let session = source.admit(&budget).unwrap();
    let request = admit_request(session.value().request.clone(), &budget).unwrap();
    let expected = session
        .value()
        .build(
            request.value(),
            &AtomicBool::new(true),
            &AtomicUsize::new(0),
        )
        .err()
        .expect("Cancelled generation rejects")
        .message;
    let profile = Arc::downgrade(&session.value().profile);
    let worker_budget = budget.clone();
    let (sender, receiver) = mpsc::sync_channel(DESIGNER_CAPTURE_QUEUE_CAPACITY);
    std::thread::spawn(move || {
        let result = worker_inputs::generate(
            session,
            request,
            &AtomicBool::new(true),
            &AtomicUsize::new(0),
            &worker_budget,
        );
        sender.send(result).unwrap();
    })
    .join()
    .unwrap();
    assert!(profile.upgrade().is_none());
    let charge = budget.usage();
    assert!(charge.payload_bytes > 0);
    let failure = receiver
        .recv()
        .unwrap()
        .err()
        .expect("Queued cancellation failure");
    assert_eq!(failure.message(), expected);
    assert_eq!(budget.usage(), charge);
    drop(failure);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn actual_request_and_selection_reject_without_replacing_independent_old_owners() {
    let budget = RetentionBudget::with_limits(TEST_LIMIT);
    let (session, _) = super::tests::fixture();
    let original = admit_request(session.request.clone(), &budget).unwrap();
    let selected = session.hashes.clone();
    let snapshot = SelectionSnapshot {
        hashes: session.hashes.clone(),
        profile_hash: session.profile.source.sha256.clone(),
        options: Options {
            selected_captures: Some(selected.clone()),
            ..Default::default()
        },
        imported: true,
    }
    .admit(&budget)
    .unwrap();
    let before = budget.usage();
    assert!(original.fixture_reserved_resources().payload_bytes > 0);
    assert!(snapshot.fixture_reserved_resources().payload_bytes > 0);
    // Request/selection children contain no ordered maps or shared roots.
    assert_eq!(before.ordered_entries, 0);
    assert_eq!(before.shared_roots, 0);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    assert!(admit_request(session.request.clone(), &budget).is_err());
    assert!(SelectionSnapshot {
        hashes: session.hashes.clone(),
        profile_hash: session.profile.source.sha256.clone(),
        options: snapshot.value().options.clone(),
        imported: true,
    }
    .admit(&budget)
    .is_err());
    assert_eq!(budget.usage(), full);
    assert_eq!(
        serde_json::to_value(original.value()).unwrap(),
        serde_json::to_value(&session.request).unwrap()
    );
    assert_eq!(
        snapshot.value().options.selected_captures.as_ref().unwrap(),
        &selected
    );
    drop(pressure);
    assert_eq!(budget.usage(), before);
    drop(original);
    drop(snapshot);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn detached_generation_keeps_actual_request_and_source_until_cancelled_thread_completes() {
    let budget = RetentionBudget::with_limits(TEST_LIMIT);
    let (source, _) = super::tests::fixture();
    let session = source.admit(&budget).unwrap();
    let request = admit_request(session.value().request.clone(), &budget).unwrap();
    let before = budget.usage();
    let profile = Arc::downgrade(&session.value().profile);
    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = Arc::clone(&cancel);
    let worker_budget = budget.clone();
    let (started_tx, started_rx) = mpsc::sync_channel(RENDEZVOUS_CAPACITY);
    let (finish_tx, finish_rx) = mpsc::sync_channel(RENDEZVOUS_CAPACITY);
    let thread = std::thread::spawn(move || {
        started_tx.send(()).unwrap();
        finish_rx.recv().unwrap();
        assert!(worker_inputs::generate(
            session,
            request,
            &worker_cancel,
            &AtomicUsize::new(0),
            &worker_budget
        )
        .is_err());
    });
    started_rx.recv().unwrap();
    cancel.store(true, Ordering::Relaxed);
    assert_eq!(budget.usage(), before);
    assert!(profile.upgrade().is_some());
    finish_tx.send(()).unwrap();
    thread.join().unwrap();
    assert!(profile.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}
