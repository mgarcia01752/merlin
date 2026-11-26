/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Both dirty decisions must resolve before irreversible host cancellation.
use super::*;
use crate::shell::{ApplicationError, RunCancellation, SessionError};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Default)]
struct CancellationCount(AtomicUsize);
impl RunCancellation for CancellationCount {
    /// Observes real host cancellation without acknowledging its active lease.
    /// # Returns
    /// Unit after recording one request.
    fn request_cancel(&self) {
        self.0.fetch_add(1, Ordering::SeqCst);
    }
}

fn owner(dirty: bool) -> DirectionShellRust {
    let application = ApplicationSessions::desktop().unwrap();
    if dirty {
        for direction in [Direction::Downstream, Direction::Upstream] {
            application.mark_project_dirty(direction).unwrap();
        }
    }
    DirectionShellRust {
        application,
        close: None,
    }
}

#[test]
fn cancel_first_or_second_dirty_prompt_keeps_both_projects_and_run_admission_live() {
    for discard_first in [false, true] {
        let mut owner = owner(true);
        let ds = owner.application.snapshot(Direction::Downstream);
        let us = owner.application.snapshot(Direction::Upstream);
        let usage = owner.application.budget().usage();
        assert_eq!(owner.begin_close().unwrap()["direction"], "downstream");
        assert!(owner.begin_close().is_err());
        if discard_first {
            assert_eq!(
                owner.close_decision(true, false).unwrap()["direction"],
                "upstream"
            );
        }
        assert_eq!(
            owner.close_decision(false, true).unwrap()["cancelled"],
            true
        );
        assert_eq!(owner.application.snapshot(Direction::Downstream), ds);
        assert_eq!(owner.application.snapshot(Direction::Upstream), us);
        assert_eq!(owner.application.budget().usage(), usage);
        owner
            .application
            .mark_project_dirty(Direction::Upstream)
            .unwrap();
        assert!(owner.finish_close().is_err());
    }
}

#[test]
fn unresolved_or_stale_close_never_shuts_down_and_releases_its_intent() {
    let mut owner = owner(true);
    let usage = owner.application.budget().usage();
    owner.begin_close().unwrap();
    assert!(owner.finish_close().is_err());
    owner
        .application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    owner.begin_close().unwrap();
    owner
        .application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    assert!(owner.close_decision(true, false).is_err());
    assert_eq!(owner.application.budget().usage(), usage);
    owner.begin_close().unwrap();
    owner.close_decision(true, false).unwrap();
    owner.close_decision(true, false).unwrap();
    owner
        .application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    assert!(owner.finish_close().is_err());
    owner
        .application
        .mark_project_dirty(Direction::Upstream)
        .unwrap();
}

#[test]
fn save_then_discard_rechecks_actual_clean_revision_before_atomic_shutdown() {
    let mut owner = owner(true);
    owner.begin_close().unwrap();
    let saved = owner.application.snapshot(Direction::Downstream);
    owner.application.acknowledge_save(saved).unwrap();
    assert_eq!(
        owner.close_decision(false, false).unwrap()["direction"],
        "upstream"
    );
    assert_eq!(owner.close_decision(true, false).unwrap()["ready"], true);
    owner.finish_close().unwrap();
    assert_eq!(
        owner.application.mark_project_dirty(Direction::Downstream),
        Err(ApplicationError::Session(SessionError::Closing))
    );
    assert!(!owner.application.snapshot(Direction::Downstream).dirty);
    assert!(owner.application.snapshot(Direction::Upstream).dirty);
}

#[test]
fn clean_close_and_active_discard_cancel_only_after_both_decisions_pass() {
    let mut clean = owner(false);
    assert_eq!(clean.begin_close().unwrap()["ready"], true);
    clean.finish_close().unwrap();
    assert!(clean
        .application
        .mark_project_dirty(Direction::Upstream)
        .is_err());

    let mut owner = owner(true);
    let key = owner
        .application
        .replace_project(
            owner.application.snapshot(Direction::Upstream),
            ReplacePermission::DiscardConfirmed,
            true,
        )
        .unwrap();
    owner
        .application
        .mark_project_dirty(Direction::Upstream)
        .unwrap();
    let control = Arc::new(CancellationCount::default());
    let lease = owner.application.begin(key, control.clone()).unwrap();
    owner.begin_close().unwrap();
    owner.close_decision(true, false).unwrap();
    assert_eq!(control.0.load(Ordering::SeqCst), 0);
    owner.close_decision(true, false).unwrap();
    assert_eq!(control.0.load(Ordering::SeqCst), 0);
    owner.finish_close().unwrap();
    assert_eq!(control.0.load(Ordering::SeqCst), 1);
    assert!(owner.application.finish(lease).is_err());
    assert!(owner.application.active().is_none());
}
