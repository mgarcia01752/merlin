/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Guarded native project readiness; counters are not scientific byte estimates.
use super::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Barrier,
};

const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
// Independent application callers racing one captured Open target.
const RACING_OPEN_COUNT: usize = 2;
#[derive(Default)]
struct CancellationCount(AtomicUsize);
impl RunCancellation for CancellationCount {
    /// Records a nonblocking cancellation request without acknowledging termination.
    fn request_cancel(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

#[test]
fn guarded_readiness_rejects_stale_foreign_and_wrong_direction_without_mutating_either_owner() {
    let host = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let foreign = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let original = host.snapshot(Direction::Downstream);
    host.configure_downstream_if(original.key, true).unwrap();
    let downstream = host.snapshot(Direction::Downstream);
    let upstream = host.snapshot(Direction::Upstream);
    let usage = host.budget().usage();
    for rejected in [
        original.key,
        foreign.snapshot(Direction::Downstream).key,
        upstream.key,
    ] {
        assert_eq!(
            host.configure_downstream_if(rejected, false),
            Err(ApplicationError::Session(SessionError::StaleGuard))
        );
        assert_eq!(host.snapshot(Direction::Downstream), downstream);
        assert_eq!(host.snapshot(Direction::Upstream), upstream);
        assert_eq!(host.budget().usage(), usage);
    }
}

#[test]
fn final_metadata_failure_preserves_active_work_and_successful_commit_keeps_its_terminal_lease() {
    let host = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let key = host.configure_downstream(true).unwrap();
    let control = Arc::new(CancellationCount::default());
    let lease = host.begin(key, control.clone()).unwrap();
    let before = host.snapshot(Direction::Downstream);
    let phase = host.active().unwrap();
    let used = host.budget().usage();
    let pressure = host
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(matches!(
        host.configure_downstream_if(key, false),
        Err(ApplicationError::Retention(_))
    ));
    assert_eq!(host.snapshot(Direction::Downstream), before);
    assert_eq!(host.active(), Some(phase));
    assert_eq!(control.0.load(Ordering::SeqCst), 0);
    drop(pressure);
    let committed = host.configure_downstream_if(key, false).unwrap();
    assert_ne!(committed, key);
    assert_eq!(control.0.load(Ordering::SeqCst), 1);
    assert_eq!(host.active().unwrap().token, lease.token());
    assert_eq!(
        host.active().unwrap().phase,
        super::super::DispatchPhase::Cancelling
    );
    // Old callback publication can be stale while consuming its genuine terminal lease.
    let _ = host.finish(lease);
    assert!(host.active().is_none());
    assert_eq!(host.budget().usage(), used);
}

#[test]
fn competing_readiness_commits_accept_only_one_original_key() {
    let host = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let original = host.snapshot(Direction::Downstream).key;
    let before = host.budget().usage();
    let barrier = Barrier::new(RACING_OPEN_COUNT + 1);
    let results = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..RACING_OPEN_COUNT)
            .map(|_| {
                let host = &host;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    host.configure_downstream_if(original, true)
                })
            })
            .collect();
        barrier.wait();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(
        results
            .iter()
            .filter(|result| matches!(
                result,
                Err(ApplicationError::Session(SessionError::StaleGuard))
            ))
            .count(),
        RACING_OPEN_COUNT - 1
    );
    assert_eq!(host.budget().usage(), before);
}

#[test]
fn original_path_flags_share_only_with_host_aliases_and_keep_direction_union() {
    let host = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let foreign = ApplicationSessions::new(TEST_MAXIMUM).unwrap();
    let alias = host.clone();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixture-original.bin");
    let before = host.budget().usage();
    host.register_input_path(Direction::Downstream, path.clone())
        .unwrap();
    let charged = host.budget().usage();
    assert!(charged.payload_bytes > before.payload_bytes);
    assert_eq!(charged.ordered_entries, before.ordered_entries + 1);
    assert_eq!(charged.shared_roots, before.shared_roots);
    alias
        .register_input_path(Direction::Upstream, path.clone())
        .unwrap();
    assert_eq!(host.budget().usage(), charged);
    assert!(host.protects_input_path(&path));
    assert!(!foreign.protects_input_path(&path));
    let downstream = host.snapshot(Direction::Downstream);
    let upstream = host.snapshot(Direction::Upstream);
    host.clear_input_paths(Direction::Downstream);
    assert!(alias.protects_input_path(&path));
    assert_eq!(host.budget().usage(), charged);
    alias.clear_input_paths(Direction::Upstream);
    assert!(!host.protects_input_path(&path));
    assert_eq!(host.budget().usage(), before);
    assert_eq!(host.snapshot(Direction::Downstream), downstream);
    assert_eq!(host.snapshot(Direction::Upstream), upstream);
}
