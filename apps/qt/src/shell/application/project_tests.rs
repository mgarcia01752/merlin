/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual desktop-host project guards; no scientific work or filesystem mocks.
use super::*;
use crate::shell::{
    DispatchError, DispatchPhase, RetainedResources, RunCancellation, SessionError,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

// The existing application test module's private control is not a sibling API.
// This local witness observes the same nonblocking RunCancellation contract.
#[derive(Default)]
struct CancellationCount(AtomicUsize);

impl RunCancellation for CancellationCount {
    /// Records cancellation without acknowledging the still-owned run lease.
    ///
    /// # Returns
    /// Unit after incrementing the independent request count.
    fn request_cancel(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn other(direction: Direction) -> Direction {
    match direction {
        Direction::Downstream => Direction::Upstream,
        Direction::Upstream => Direction::Downstream,
    }
}

#[test]
fn replacement_uses_captured_direction_and_new_clean_identity_while_foreign_tab_is_selected() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        let selected = other(direction);
        host.select_direction(selected);
        let original = host.snapshot(direction);
        let foreign = host.snapshot(selected);
        let usage = host.budget().usage();
        let key = host
            .replace_project(original, ReplacePermission::RequireClean, true)
            .unwrap();
        let replaced = host.snapshot(direction);
        assert_eq!(replaced.key, key);
        assert_eq!(key.direction, direction);
        assert_ne!(key.session, original.key.session);
        assert!(!replaced.dirty);
        assert_eq!(host.snapshot(selected), foreign);
        assert_eq!(host.selected_direction(), selected);
        assert_eq!(host.budget().usage(), usage);

        let lease = host
            .begin(key, Arc::new(CancellationCount::default()))
            .unwrap();
        assert_eq!(host.active().unwrap().token.key, key);
        host.finish(lease).unwrap();
        assert!(host.active().is_none());
        assert_eq!(host.budget().usage(), usage);
    }
}

#[test]
fn save_acknowledgement_rejects_newer_metadata_revision_and_cleans_only_its_actual_owner() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        host.select_direction(other(direction));
        host.mark_project_dirty(direction).unwrap();
        let saved = host.snapshot(direction);
        let foreign = host.snapshot(other(direction));
        let usage = host.budget().usage();
        host.mark_project_dirty(direction).unwrap();
        let newer = host.snapshot(direction);
        assert_eq!(newer.key, saved.key);
        assert_ne!(newer.revision, saved.revision);
        assert!(newer.dirty);
        assert_eq!(
            host.acknowledge_save(saved),
            Err(ApplicationError::Session(SessionError::StaleGuard))
        );
        assert_eq!(host.snapshot(direction), newer);
        host.acknowledge_save(newer).unwrap();
        let clean = host.snapshot(direction);
        assert_eq!(clean.key, newer.key);
        assert_eq!(clean.revision, newer.revision);
        assert!(!clean.dirty);
        assert_eq!(host.snapshot(other(direction)), foreign);
        assert_eq!(host.budget().usage(), usage);
    }
}

#[test]
fn old_snapshot_cannot_discard_an_intervening_metadata_edit_or_foreign_host() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        let foreign_host = ApplicationSessions::desktop().unwrap();
        let old = host.snapshot(direction);
        host.mark_project_dirty(direction).unwrap();
        let changed = host.snapshot(direction);
        let foreign_direction = host.snapshot(other(direction));
        let usage = host.budget().usage();
        for stale in [old, foreign_host.snapshot(direction)] {
            assert_eq!(
                host.replace_project(stale, ReplacePermission::DiscardConfirmed, true),
                Err(ApplicationError::Session(SessionError::StaleGuard))
            );
            assert_eq!(host.snapshot(direction), changed);
            assert_eq!(host.snapshot(other(direction)), foreign_direction);
            assert_eq!(host.budget().usage(), usage);
        }
    }
}

#[test]
fn dirty_permission_and_blank_readiness_preserve_both_projects_on_rejection() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        host.mark_project_dirty(direction).unwrap();
        let dirty = host.snapshot(direction);
        let foreign = host.snapshot(other(direction));
        let usage = host.budget().usage();
        assert_eq!(
            host.replace_project(dirty, ReplacePermission::RequireClean, false),
            Err(ApplicationError::Session(SessionError::DirtyTarget))
        );
        assert_eq!(host.snapshot(direction), dirty);
        assert_eq!(host.snapshot(other(direction)), foreign);
        assert_eq!(host.budget().usage(), usage);

        let key = host
            .replace_project(dirty, ReplacePermission::DiscardConfirmed, false)
            .unwrap();
        assert_ne!(key.session, dirty.key.session);
        assert_eq!(host.snapshot(direction).key, key);
        assert!(!host.snapshot(direction).dirty);
        assert_eq!(host.snapshot(other(direction)), foreign);
        assert!(matches!(
            host.begin(key, Arc::new(CancellationCount::default())),
            Err(ApplicationError::NotConfigured(owner)) if owner == direction
        ));
        assert!(host.active().is_none());
        assert_eq!(host.budget().usage(), usage);
    }
}

#[test]
fn active_target_replacement_rejects_while_foreign_replacement_keeps_the_global_lease() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        let key = host
            .replace_project(
                host.snapshot(direction),
                ReplacePermission::RequireClean,
                true,
            )
            .unwrap();
        let control = Arc::new(CancellationCount::default());
        let lease = host.begin(key, control.clone()).unwrap();
        let active = host.active().unwrap();
        let original = host.snapshot(direction);
        let foreign = host.snapshot(other(direction));
        let usage = host.budget().usage();
        assert_eq!(
            host.replace_project(original, ReplacePermission::DiscardConfirmed, false),
            Err(ApplicationError::Session(SessionError::Dispatch(
                DispatchError::Busy(active)
            )))
        );
        assert_eq!(host.snapshot(direction), original);
        assert_eq!(host.snapshot(other(direction)), foreign);
        assert_eq!(host.budget().usage(), usage);
        assert_eq!(control.0.load(Ordering::SeqCst), 0);

        let foreign_key = host
            .replace_project(foreign, ReplacePermission::RequireClean, true)
            .unwrap();
        assert_eq!(host.active(), Some(active));
        assert_eq!(host.snapshot(direction), original);
        assert_eq!(control.0.load(Ordering::SeqCst), 0);
        assert!(matches!(
            host.begin(foreign_key, Arc::new(CancellationCount::default())),
            Err(ApplicationError::Session(SessionError::Dispatch(
                DispatchError::Busy(current)
            ))) if current == active
        ));
        host.finish(lease).unwrap();
        assert!(host.active().is_none());
        assert_eq!(host.budget().usage(), usage);
    }
}

#[test]
fn rejected_readiness_reservation_keeps_both_snapshots_and_releases_only_candidate_storage() {
    let host = ApplicationSessions::desktop().unwrap();
    let downstream = host.snapshot(Direction::Downstream);
    let upstream = host.snapshot(Direction::Upstream);
    let usage = host.budget().usage();
    let pressure = host
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: super::super::MAX_SESSION_PAYLOAD_BYTES - usage.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let pressured = host.budget().usage();
    for expected in [downstream, upstream] {
        assert!(matches!(
            host.replace_project(expected, ReplacePermission::RequireClean, true),
            Err(ApplicationError::Retention(_))
        ));
        assert_eq!(host.snapshot(Direction::Downstream), downstream);
        assert_eq!(host.snapshot(Direction::Upstream), upstream);
        assert_eq!(host.budget().usage(), pressured);
        assert!(host.active().is_none());
    }
    drop(pressure);
    assert_eq!(host.budget().usage(), usage);
}

#[test]
fn shutdown_cancels_once_and_rejects_project_mutation_until_actual_terminal_acknowledgement() {
    for direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        let key = host
            .replace_project(
                host.snapshot(direction),
                ReplacePermission::RequireClean,
                true,
            )
            .unwrap();
        let control = Arc::new(CancellationCount::default());
        let lease = host.begin(key, control.clone()).unwrap();
        let original = host.snapshot(direction);
        let foreign = host.snapshot(other(direction));
        let usage = host.budget().usage();
        host.shutdown();
        host.shutdown();
        assert_eq!(control.0.load(Ordering::SeqCst), 1);
        assert_eq!(host.active().unwrap().token, lease.token());
        assert_eq!(host.active().unwrap().phase, DispatchPhase::Cancelling);
        assert_eq!(
            host.acknowledge_save(original),
            Err(ApplicationError::Session(SessionError::Closing))
        );
        assert_eq!(
            host.mark_project_dirty(direction),
            Err(ApplicationError::Session(SessionError::Closing))
        );
        assert_eq!(
            host.replace_project(original, ReplacePermission::DiscardConfirmed, false),
            Err(ApplicationError::Session(SessionError::Closing))
        );
        assert_eq!(host.snapshot(direction), original);
        assert_eq!(host.snapshot(other(direction)), foreign);
        assert_eq!(host.budget().usage(), usage);
        assert_eq!(
            host.finish(lease),
            Err(ApplicationError::Session(SessionError::Closing))
        );
        assert!(host.active().is_none());
        assert_eq!(control.0.load(Ordering::SeqCst), 1);
        assert_eq!(host.budget().usage(), usage);
    }
}
