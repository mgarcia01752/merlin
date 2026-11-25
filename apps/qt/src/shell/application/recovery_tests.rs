/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Pair recovery rollback, dirty-state restoration and original-path ownership.
use super::*;
use crate::{shared::update_recovery::OriginalPath, shell::SessionError};
use std::path::PathBuf;

struct Control;
impl crate::shell::RunCancellation for Control {
    /// Accepts cancellation without simulating terminal acknowledgement.
    fn request_cancel(&self) {}
}
fn originals(host: &ApplicationSessions, path: &str) -> crate::shell::InputPaths {
    host.prepare_input_paths(vec![OriginalPath {
        path: PathBuf::from(path),
        downstream: true,
        upstream: true,
    }])
    .unwrap()
}
fn guards(host: &ApplicationSessions) -> (ProjectSnapshot, ProjectSnapshot) {
    (
        host.snapshot(Direction::Downstream),
        host.snapshot(Direction::Upstream),
    )
}

#[test]
fn pair_restore_preserves_independent_dirty_flags_destinations_and_selected_direction() {
    for selected in [Direction::Downstream, Direction::Upstream] {
        for dirty in [(false, true), (true, false), (true, true), (false, false)] {
            let host = ApplicationSessions::desktop().unwrap();
            let expected = guards(&host);
            let keys = host
                .restore_projects(
                    expected,
                    (true, true),
                    dirty,
                    selected,
                    originals(&host, "/fixture/original"),
                    host.snapshot_input_paths().unwrap().value(),
                )
                .unwrap();
            assert_ne!(keys.0, expected.0.key);
            assert_ne!(keys.1, expected.1.key);
            assert_eq!(guards(&host).0.dirty, dirty.0);
            assert_eq!(guards(&host).1.dirty, dirty.1);
            assert_eq!(host.selected_direction(), selected);
            assert!(host.protects_input_path(std::path::Path::new("/fixture/original")));
            host.clear_input_paths(Direction::Downstream);
            assert!(host.protects_input_path(std::path::Path::new("/fixture/original")));
            host.clear_input_paths(Direction::Upstream);
            assert!(!host.protects_input_path(std::path::Path::new("/fixture/original")));
        }
    }
}

#[test]
fn stale_second_owner_rejects_pair_without_changing_first_or_path_registry() {
    for changed in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        host.register_input_path(Direction::Downstream, PathBuf::from("/fixture/old"))
            .unwrap();
        let expected = guards(&host);
        let candidate = originals(&host, "/fixture/new");
        host.mark_project_dirty(changed).unwrap();
        let current = guards(&host);
        let usage = host.budget().usage();
        let error = host
            .restore_projects(
                expected,
                (true, true),
                (false, false),
                Direction::Upstream,
                candidate,
                host.snapshot_input_paths().unwrap().value(),
            )
            .unwrap_err();
        assert_eq!(error, ApplicationError::Session(SessionError::StaleGuard));
        assert_eq!(guards(&host), current);
        assert_eq!(host.selected_direction(), Direction::Downstream);
        assert!(host.protects_input_path(std::path::Path::new("/fixture/old")));
        assert!(!host.protects_input_path(std::path::Path::new("/fixture/new")));
        assert!(host.budget().usage().payload_bytes < usage.payload_bytes);
    }
}

#[test]
fn foreign_or_invalid_path_candidate_never_changes_live_projects() {
    let host = ApplicationSessions::desktop().unwrap();
    let foreign = ApplicationSessions::desktop().unwrap();
    let expected = guards(&host);
    assert_eq!(
        host.restore_projects(
            expected,
            (true, true),
            (false, false),
            Direction::Upstream,
            originals(&foreign, "/fixture/foreign"),
            host.snapshot_input_paths().unwrap().value()
        )
        .unwrap_err(),
        ApplicationError::Session(SessionError::WrongBudget)
    );
    for paths in [
        vec![OriginalPath {
            path: "relative".into(),
            downstream: true,
            upstream: false,
        }],
        vec![OriginalPath {
            path: "/fixture/no-owner".into(),
            downstream: false,
            upstream: false,
        }],
    ] {
        assert!(host.prepare_input_paths(paths).is_err());
        assert_eq!(guards(&host), expected);
    }
    let snapshot = host.snapshot_input_paths().unwrap();
    assert!(host.input_paths_match(snapshot.value()));
    host.register_input_path(Direction::Upstream, "/fixture/added".into())
        .unwrap();
    assert!(!host.input_paths_match(snapshot.value()));
    assert_eq!(guards(&host), expected);
}

#[test]
fn active_execution_or_shutdown_rejects_recovery_before_either_owner_is_replaced() {
    use std::sync::Arc;
    for active_direction in [Direction::Downstream, Direction::Upstream] {
        let host = ApplicationSessions::desktop().unwrap();
        let expected = guards(&host);
        let key = host
            .replace_project(
                host.snapshot(active_direction),
                crate::shell::ReplacePermission::RequireClean,
                true,
            )
            .unwrap();
        let expected = (
            if active_direction == Direction::Downstream {
                host.snapshot(active_direction)
            } else {
                expected.0
            },
            if active_direction == Direction::Upstream {
                host.snapshot(active_direction)
            } else {
                expected.1
            },
        );
        let control = Arc::new(Control);
        let lease = host.begin(key, control).unwrap();
        assert!(host
            .restore_projects(
                expected,
                (true, true),
                (false, false),
                Direction::Upstream,
                originals(&host, "/fixture/new"),
                host.snapshot_input_paths().unwrap().value()
            )
            .is_err());
        assert_eq!(guards(&host), expected);
        host.finish(lease).unwrap();
        host.shutdown();
        assert_eq!(
            host.restore_projects(
                expected,
                (true, true),
                (false, false),
                Direction::Upstream,
                originals(&host, "/fixture/new"),
                host.snapshot_input_paths().unwrap().value()
            )
            .unwrap_err(),
            ApplicationError::Session(SessionError::Closing)
        );
        assert_eq!(guards(&host), expected);
    }
}
