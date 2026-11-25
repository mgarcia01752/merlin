/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real dual-project worker, restart and rollback consumers of private recovery.
use super::*;
use crate::shell::ReplacePermission;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};
const DOWNSTREAM: &[u8] =
    include_bytes!("../../../../../examples/workbench/single/workbench.merlin.json");
const UPSTREAM: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const WATCHDOG: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(2);
const ORIGINAL: &[u8] = b"Original source bytes must survive recovery and Save rejection.";
static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let id = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("merlin-qt-recovery-{}-{id}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    /// Removes only the temporary directory successfully created by this fixture.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn blank() -> (WorkbenchFileRust, Box<Model>) {
    let application = ApplicationSessions::desktop().unwrap();
    let context = session::Context::for_application(application.clone()).unwrap();
    let upstream = Model::new(application).unwrap();
    (
        WorkbenchFileRust {
            context,
            path: None,
            loader: Default::default(),
            intent: None,
            recovery: None,
        },
        upstream,
    )
}
fn configured(directory: &Directory) -> (WorkbenchFileRust, Box<Model>) {
    let (mut file, mut upstream) = blank();
    let application = file.context.application.clone();
    PreparedProject::prepare(
        serde_json::from_slice(DOWNSTREAM).unwrap(),
        None,
        &application,
        application.snapshot(Direction::Downstream).key,
        session::snapshot(&file.context),
    )
    .unwrap()
    .adopt(&file.context)
    .unwrap();
    file.path = Some(
        project_path::admit(
            directory.0.join("downstream.merlin.json"),
            application.budget(),
        )
        .unwrap(),
    );
    upstream.apply(UPSTREAM).unwrap();
    let deadline = Instant::now() + WATCHDOG;
    while upstream.poll()["preparing"] == true {
        assert!(Instant::now() < deadline);
        std::thread::sleep(POLL_INTERVAL);
    }
    upstream
        .save_project(Some(directory.0.join("upstream.merlin.json")))
        .unwrap();
    while upstream.poll()["preparing"] == true {
        assert!(Instant::now() < deadline);
        std::thread::sleep(POLL_INTERVAL);
    }
    assert!(upstream.project_path().is_some(), "{}", upstream.state());
    (file, upstream)
}
fn options(runnable: bool) -> Vec<u8> {
    let project: Project = serde_json::from_slice(DOWNSTREAM).unwrap();
    serde_json::to_vec(&SaveOptions {
        settings: serde_json::to_string(&project.transmission).unwrap(),
        receiver: serde_json::to_string(&project.receiver).unwrap(),
        observations: serde_json::to_string(&Some(
            merlin::downstream::FullLoopObservationInput::default(),
        ))
        .unwrap(),
        verified: project.comparison_enabled,
        dark: true,
        runnable,
    })
    .unwrap()
}
fn finish(file: &mut WorkbenchFileRust, upstream: &mut Model) -> Result<serde_json::Value, String> {
    let deadline = Instant::now() + WATCHDOG;
    loop {
        let result = file.poll_recovery(upstream)?;
        if result["preparing"] != true {
            return Ok(result);
        }
        assert!(Instant::now() < deadline, "Recovery worker timed out");
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn real_worker_restart_restores_both_public_projects_originals_dirty_flags_and_destinations() {
    for selected in [Direction::Downstream, Direction::Upstream] {
        let directory = Directory::new();
        let (mut file, mut upstream) = configured(&directory);
        let application = file.context.application.clone();
        // A CLI-relative destination must survive an installer changing cwd.
        // Do not change process cwd inside concurrent Rust tests.
        let live_downstream_path = if selected == Direction::Upstream {
            PathBuf::from("examples/workbench/single/workbench.merlin.json")
        } else {
            directory.0.join("downstream.merlin.json")
        };
        file.path =
            Some(project_path::admit(live_downstream_path.clone(), application.budget()).unwrap());
        let recovered_downstream_path =
            std::env::current_dir().unwrap().join(&live_downstream_path);
        application.select_direction(selected);
        application
            .mark_project_dirty(Direction::Downstream)
            .unwrap();
        application.mark_project_dirty(Direction::Upstream).unwrap();
        let original = directory.0.join("original.bin");
        std::fs::write(&original, ORIGINAL).unwrap();
        let canonical = std::fs::canonicalize(&original).unwrap();
        for direction in [Direction::Downstream, Direction::Upstream] {
            application
                .register_input_path(direction, canonical.clone())
                .unwrap();
        }
        let expected = guards(&application);
        let destination = directory.0.join("private-recovery.json");
        assert_eq!(
            file.start_recovery_write(&upstream, options(true), destination.clone())
                .unwrap()["preparing"],
            true
        );
        assert_eq!(finish(&mut file, &mut upstream).unwrap()["saved"], true);
        assert_eq!(guards(&application), expected);
        assert_eq!(file.path.as_ref().unwrap().value(), &live_downstream_path);
        assert_eq!(
            upstream.project_path(),
            Some(directory.0.join("upstream.merlin.json").as_path())
        );
        let bytes = std::fs::read(&destination).unwrap();
        let bundle = Bundle::decode(&bytes).unwrap();
        assert_eq!(
            bundle.downstream.path,
            Some(recovered_downstream_path.clone())
        );
        let ds = merlin::common::Project::from_bytes(bundle.downstream.document.as_ref().unwrap())
            .unwrap();
        let us = merlin::common::Project::from_bytes(bundle.upstream.document.as_ref().unwrap())
            .unwrap();
        assert_eq!(ds.direction(), merlin::common::ProjectDirection::Downstream);
        assert_eq!(
            us.upstream_simulation()
                .unwrap()
                .document()
                .unwrap()
                .original_bytes(),
            Some(UPSTREAM)
        );
        let (mut restored, mut restored_upstream) = blank();
        let target = restored.context.application.clone();
        restored
            .start_recovery_restore(&restored_upstream, destination.clone())
            .unwrap();
        let metadata = finish(&mut restored, &mut restored_upstream).unwrap();
        assert_eq!(metadata["restored"], true);
        assert!(guards(&target).0.dirty && guards(&target).1.dirty);
        assert_eq!(target.selected_direction(), selected);
        assert_eq!(
            restored.path.as_ref().unwrap().value(),
            &recovered_downstream_path
        );
        assert_eq!(
            restored_upstream.project_path(),
            Some(directory.0.join("upstream.merlin.json").as_path())
        );
        assert!(target.protects_input_path(&canonical));
        #[cfg(unix)]
        {
            let alias = directory.0.join("original-alias.bin");
            std::os::unix::fs::symlink(&original, &alias).unwrap();
            assert!(crate::platform::input_file::is_original(&target, &alias));
        }
        target.clear_input_paths(Direction::Downstream);
        assert!(target.protects_input_path(&canonical));
        assert_eq!(std::fs::read(original).unwrap(), ORIGINAL);
        assert_eq!(std::fs::read(destination).unwrap(), bytes);
    }
}

#[test]
fn partial_downstream_and_blank_upstream_remain_nonrunnable_native_states() {
    let directory = Directory::new();
    let (mut file, mut upstream) = blank();
    let packet = serde_json::to_vec(&SaveOptions {
        settings: serde_json::to_string(&merlin::downstream::TransmissionSettings::default())
            .unwrap(),
        receiver: "null".into(),
        observations: "null".into(),
        verified: false,
        dark: false,
        runnable: false,
    })
    .unwrap();
    let destination = directory.0.join("partial.json");
    file.context
        .application
        .select_direction(Direction::Upstream);
    file.start_recovery_write(&upstream, packet, destination.clone())
        .unwrap();
    finish(&mut file, &mut upstream).unwrap();
    let (mut restored, mut target) = blank();
    restored
        .start_recovery_restore(&target, destination)
        .unwrap();
    let result = finish(&mut restored, &mut target).unwrap();
    assert_eq!(result["restored"], true);
    assert_eq!(result["dark"], false);
    assert_eq!(target.state()["configured"], false);
    assert!(target.project_path().is_none());
    assert!(restored.path.is_none());
    assert!(!guards(&restored.context.application).0.dirty);
    assert_eq!(
        restored.context.application.selected_direction(),
        Direction::Upstream
    );
}

#[test]
fn invalid_either_candidate_and_stale_final_guard_preserve_both_live_owners() {
    let directory = Directory::new();
    let (mut file, mut upstream) = configured(&directory);
    let application = file.context.application.clone();
    let destination = directory.0.join("valid.json");
    file.start_recovery_write(&upstream, options(true), destination.clone())
        .unwrap();
    finish(&mut file, &mut upstream).unwrap();
    let valid = std::fs::read(&destination).unwrap();
    let before = guards(&application);
    let old_path = file.path.as_ref().unwrap().value().clone();
    let old_upstream = upstream.project_path().unwrap().to_path_buf();
    for invalid_upstream in [false, true] {
        let mut bundle = Bundle::decode(&valid).unwrap();
        if invalid_upstream {
            bundle.upstream.document = Some(b"invalid second project".to_vec());
        } else {
            bundle.downstream.document =
                Some(b"invalid downstream candidate after upstream admission".to_vec());
        }
        let invalid = directory.0.join("invalid.json");
        std::fs::write(&invalid, bundle.encode().unwrap()).unwrap();
        file.start_recovery_restore(&upstream, invalid).unwrap();
        assert!(finish(&mut file, &mut upstream).is_err());
        assert_eq!(guards(&application), before);
        assert_eq!(file.path.as_ref().unwrap().value(), &old_path);
        assert_eq!(upstream.project_path(), Some(old_upstream.as_path()));
    }
    // Pre-completed preparation isolates the final stale-guard boundary from I/O scheduling.
    let product = prepare_bundle(
        application.clone(),
        *project_path::admit(destination, application.budget()).unwrap(),
        before,
        session::snapshot(&file.context),
        application.snapshot_input_paths().unwrap(),
    )
    .unwrap();
    file.recovery = Some(launch(&application, move || Ok(product)).unwrap());
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let current = guards(&application);
    assert!(finish(&mut file, &mut upstream).is_err());
    assert_eq!(guards(&application), current);
    assert_eq!(file.path.as_ref().unwrap().value(), &old_path);
    assert_eq!(upstream.project_path(), Some(old_upstream.as_path()));
}

#[test]
fn late_original_registration_or_failed_write_never_clears_dirty_or_changes_normal_paths() {
    let directory = Directory::new();
    let (mut file, mut upstream) = configured(&directory);
    let application = file.context.application.clone();
    let destination = directory.0.join("recovery.json");
    file.start_recovery_write(&upstream, options(true), destination.clone())
        .unwrap();
    finish(&mut file, &mut upstream).unwrap();
    let before = guards(&application);
    let bytes = std::fs::read(&destination).unwrap();
    let product = prepare_bundle(
        application.clone(),
        *project_path::admit(destination.clone(), application.budget()).unwrap(),
        before,
        session::snapshot(&file.context),
        application.snapshot_input_paths().unwrap(),
    )
    .unwrap();
    file.recovery = Some(launch(&application, move || Ok(product)).unwrap());
    application
        .register_input_path(Direction::Downstream, directory.0.join("late-original"))
        .unwrap();
    assert!(finish(&mut file, &mut upstream).is_err());
    assert_eq!(guards(&application), before);
    assert!(application.protects_input_path(&directory.0.join("late-original")));
    application
        .register_input_path(
            Direction::Downstream,
            std::fs::canonicalize(&destination).unwrap(),
        )
        .unwrap();
    file.start_recovery_write(&upstream, options(true), destination.clone())
        .unwrap();
    assert!(finish(&mut file, &mut upstream)
        .unwrap_err()
        .contains("original"));
    assert_eq!(std::fs::read(&destination).unwrap(), bytes);
    assert_eq!(guards(&application), before);
    let blank = application.snapshot(Direction::Upstream);
    // Verify ordinary New still owns its established path/protection cleanup.
    upstream
        .new_project(blank, ReplacePermission::DiscardConfirmed)
        .unwrap();
    assert!(upstream.project_path().is_none());
}

#[test]
fn previous_single_downstream_recovery_migrates_without_inventing_destination_or_clean_state() {
    let directory = Directory::new();
    let path = directory.0.join("legacy.json");
    std::fs::write(&path, DOWNSTREAM).unwrap();
    let (mut file, mut upstream) = blank();
    file.start_recovery_restore(&upstream, path).unwrap();
    let result = finish(&mut file, &mut upstream).unwrap();
    assert_eq!(result["restored"], true);
    assert!(file.path.is_none());
    assert!(
        file.context
            .application
            .snapshot(Direction::Downstream)
            .dirty
    );
    assert_eq!(upstream.state()["configured"], false);
    assert_eq!(
        file.context.application.selected_direction(),
        Direction::Downstream
    );
    let disguised = serde_json::json!({"format":"merlin.qt.update-recovery","version":99,
        "application":"MERLIN","schema_version":2});
    assert!(decode_bundle(&serde_json::to_vec(&disguised).unwrap()).is_err());
    // A present null format belongs to strict new-bundle decoding, never legacy.
    let mixed = format!(
        "{{\"format\":null,{}",
        std::str::from_utf8(DOWNSTREAM)
            .unwrap()
            .trim_start()
            .strip_prefix('{')
            .unwrap()
    );
    assert!(decode_bundle(mixed.as_bytes())
        .unwrap_err()
        .contains("invalid type: null"));
}

#[test]
fn malformed_bundle_errors_are_bounded_before_worker_retention_and_qt_publication() {
    let directory = Directory::new();
    let (mut file, mut upstream) = blank();
    let path = directory.0.join("oversized-error.json");
    // A deliberately oversized unknown field produces a parser error beyond the
    // independent existing 4096-character UI diagnostic policy.
    let name = "x".repeat(5000);
    let malformed = format!("{{\"format\":\"merlin.qt.update-recovery\",\"{name}\":false}}");
    std::fs::write(&path, malformed).unwrap();
    file.start_recovery_restore(&upstream, path).unwrap();
    let error = finish(&mut file, &mut upstream).unwrap_err();
    assert_eq!(error.chars().count(), 4096);
    assert!(error.starts_with("unknown field"));
    let response: serde_json::Value =
        serde_json::from_str(&super::response(Err(name)).to_string()).unwrap();
    assert_eq!(response["error"].as_str().unwrap().chars().count(), 4096);
}

#[test]
fn private_file_cannot_become_either_normal_destination_on_export_or_restore() {
    for colliding_direction in [Direction::Downstream, Direction::Upstream] {
        let directory = Directory::new();
        let (mut file, mut upstream) = configured(&directory);
        let application = file.context.application.clone();
        let path = directory.0.join("workbench.merlin.json");
        file.start_recovery_write(&upstream, options(true), path.clone())
            .unwrap();
        finish(&mut file, &mut upstream).unwrap();
        let bytes = std::fs::read(&path).unwrap();
        let mut bundle = Bundle::decode(&bytes).unwrap();
        if colliding_direction == Direction::Downstream {
            bundle.downstream.path = Some(path.clone());
        } else {
            bundle.upstream.path = Some(path.clone());
        }
        let invalid = bundle.encode().unwrap();
        std::fs::write(&path, &invalid).unwrap();
        let before = guards(&application);
        let old = file.path.as_ref().unwrap().value().clone();
        file.start_recovery_restore(&upstream, path.clone())
            .unwrap();
        assert!(finish(&mut file, &mut upstream)
            .unwrap_err()
            .contains("private update recovery"));
        assert_eq!(guards(&application), before);
        assert_eq!(file.path.as_ref().unwrap().value(), &old);
        assert_eq!(std::fs::read(&path).unwrap(), invalid);
        #[cfg(unix)]
        {
            let alias = directory.0.join("private-alias.json");
            std::os::unix::fs::symlink(&path, &alias).unwrap();
            assert!(reject_private_destinations(Some(&alias), None, &path).is_err());
        }
        if colliding_direction == Direction::Downstream {
            file.path = Some(project_path::admit(path.clone(), application.budget()).unwrap());
        } else {
            upstream.save_project(Some(path.clone())).unwrap();
            let deadline = Instant::now() + WATCHDOG;
            while upstream.poll()["preparing"] == true {
                assert!(Instant::now() < deadline);
                std::thread::sleep(POLL_INTERVAL);
            }
        }
        // The accepted explicit project remains unchanged when private export is denied.
        let before = std::fs::read(&path).unwrap();
        let expected = guards(&application);
        file.start_recovery_write(&upstream, options(true), path.clone())
            .unwrap();
        assert!(finish(&mut file, &mut upstream)
            .unwrap_err()
            .contains("private update recovery"));
        assert_eq!(std::fs::read(path).unwrap(), before);
        assert_eq!(guards(&application), expected);
    }
}
