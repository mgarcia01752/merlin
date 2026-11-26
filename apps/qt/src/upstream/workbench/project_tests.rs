/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real asynchronous file transactions and exact target-revision preservation.
use super::*;
use crate::shell::ApplicationSessions;
use merlin::{
    common::Project,
    upstream::{ConfigurationDocument, RunStatus},
};
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

const SOURCE: &[u8] = include_bytes!("../../../../../examples/cli/upstream/us-50khz.json");
const WATCHDOG: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(2);
// These bytes belong only to the filesystem rollback fixture.
const SENTINEL: &[u8] = b"Preserve this existing destination.";
static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let sequence = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "merlin-upstream-project-{}-{sequence}",
            std::process::id()
        ));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn drain(model: &mut Model) {
    let deadline = Instant::now() + WATCHDOG;
    while model.poll()["preparing"] == true {
        assert!(Instant::now() < deadline, "{}", model.state());
        std::thread::sleep(POLL_INTERVAL);
    }
}
fn configured() -> (ApplicationSessions, Box<Model>) {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    model.apply(SOURCE).unwrap();
    drain(&mut model);
    assert!(model.current(), "{}", model.state());
    (application, model)
}

#[test]
fn curated_upstream_projects_open_idle_and_complete_through_the_qt_model() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/workbench/upstream");
    for name in [
        "ideal-25khz",
        "ideal-50khz",
        "synthetic-rxmer-25khz",
        "synthetic-rxmer-50khz",
    ] {
        let application = ApplicationSessions::desktop().unwrap();
        let mut model = Model::new(application.clone()).unwrap();
        let path = root.join(name).join("workbench.merlin.json");
        let original = std::fs::read(&path).unwrap();
        let blank = application.snapshot(Direction::Upstream);
        model
            .open_project(path.clone(), blank, ReplacePermission::RequireClean)
            .unwrap();
        drain(&mut model);
        assert_eq!(model.project_state()["status"], "opened", "{name}");
        assert!(model.current(), "{name}: {}", model.state());
        assert!(model.report.is_none());
        assert!(application.active().is_none());
        assert!(!application.snapshot(Direction::Upstream).dirty);
        model.start().unwrap();
        let deadline = Instant::now() + WATCHDOG;
        while model.poll()["busy"] == true {
            assert!(Instant::now() < deadline, "{name}: {}", model.state());
            std::thread::sleep(POLL_INTERVAL);
        }
        let report = model.report.as_ref().expect("Actual Qt worker completion");
        assert_eq!(report.1.value().status, RunStatus::Completed, "{name}");
        assert!(report.1.value().reference_recovered, "{name}");
        assert!(report.1.value().compared_source_bits > 0);
        assert_eq!(report.1.value().bit_errors, 0);
        assert!(application.active().is_none());
        assert_eq!(std::fs::read(path).unwrap(), original);
    }
}

#[test]
fn native_save_open_new_uses_actual_public_project_and_independent_target() {
    let (application, mut model) = configured();
    let downstream = application.snapshot(Direction::Downstream);
    let directory = Directory::new();
    let raw = directory.0.join("saved");
    let target = directory.0.join("saved.merlin.json");
    let before = application.snapshot(Direction::Upstream);
    assert!(before.dirty);
    model.save_project(Some(raw)).unwrap();
    assert_eq!(application.snapshot(Direction::Upstream), before);
    drain(&mut model);
    assert_eq!(
        model.project_state()["status"],
        "saved",
        "{}",
        model.state()
    );
    assert!(!application.snapshot(Direction::Upstream).dirty);
    assert_eq!(model.project_path(), Some(target.as_path()));
    let bytes = std::fs::read(&target).unwrap();
    let original = Project::from_bytes(&bytes)
        .unwrap()
        .upstream_simulation()
        .unwrap();
    assert_eq!(original.document().unwrap().original_bytes(), Some(SOURCE));
    application.select_direction(Direction::Downstream);
    let guard = application.snapshot(Direction::Upstream);
    model
        .new_project(guard, ReplacePermission::RequireClean)
        .unwrap();
    assert!(!model.current());
    assert!(model.project_path().is_none());
    assert!(model.save_project(Some(target.clone())).is_err());
    let blank = application.snapshot(Direction::Upstream);
    model
        .open_project(target.clone(), blank, ReplacePermission::RequireClean)
        .unwrap();
    drain(&mut model);
    assert_eq!(
        model.project_state()["status"],
        "opened",
        "{}",
        model.state()
    );
    assert!(model.current());
    assert!(!application.snapshot(Direction::Upstream).dirty);
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|actual| {
            assert_eq!(actual.options(), original.options());
            assert_eq!(
                actual.receiver_observation_policy(),
                original.receiver_observation_policy()
            );
            assert_eq!(actual.document().unwrap().original_bytes(), Some(SOURCE));
        })
        .unwrap();
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.selected_direction(), Direction::Downstream);
    assert_eq!(std::fs::read(target).unwrap(), bytes);
}

#[test]
fn stale_save_completion_keeps_old_destination_and_newer_dirty_revision() {
    let (application, mut model) = configured();
    let directory = Directory::new();
    let original = directory.0.join("original.merlin.json");
    model.save_project(Some(original.clone())).unwrap();
    drain(&mut model);
    let candidate = directory.0.join("stale.merlin.json");
    model.save_project(Some(candidate)).unwrap();
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let newer = application.snapshot(Direction::Upstream);
    drain(&mut model);
    assert_eq!(model.project_state()["status"], "rejected");
    assert_eq!(model.project_path(), Some(original.as_path()));
    assert_eq!(application.snapshot(Direction::Upstream), newer);
    assert!(newer.dirty);
    assert!(model.current());
    assert!(model.controller.is_some());
}

#[test]
fn failed_save_and_original_input_guard_preserve_destination_and_source() {
    let (application, mut model) = configured();
    let directory = Directory::new();
    let original = directory.0.join("original.merlin.json");
    model.save_project(Some(original.clone())).unwrap();
    drain(&mut model);
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let before = application.snapshot(Direction::Upstream);
    let source = directory.0.join("source.merlin.json");
    std::fs::write(&source, SENTINEL).unwrap();
    application
        .register_input_path(Direction::Downstream, source.canonicalize().unwrap())
        .unwrap();
    for target in [
        source.clone(),
        directory.0.join("missing/target.merlin.json"),
    ] {
        model.save_project(Some(target)).unwrap();
        drain(&mut model);
        assert_eq!(model.project_state()["status"], "rejected");
        assert_eq!(model.project_path(), Some(original.as_path()));
        assert_eq!(application.snapshot(Direction::Upstream), before);
        assert!(model.current());
    }
    assert_eq!(std::fs::read(source).unwrap(), SENTINEL);
}

#[test]
fn dirty_stale_failed_and_cancelled_open_do_not_replace_accepted_owner() {
    let (application, mut model) = configured();
    let directory = Directory::new();
    let target = directory.0.join("saved.merlin.json");
    model.save_project(Some(target.clone())).unwrap();
    drain(&mut model);
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let dirty = application.snapshot(Direction::Upstream);
    assert!(model
        .open_project(target.clone(), dirty, ReplacePermission::RequireClean)
        .is_err());
    assert!(model
        .new_project(dirty, ReplacePermission::RequireClean)
        .is_err());
    application.mark_project_dirty(Direction::Upstream).unwrap();
    assert!(model
        .open_project(target.clone(), dirty, ReplacePermission::DiscardConfirmed)
        .is_err());
    let before = application.snapshot(Direction::Upstream);
    for (path, cancel) in [
        (directory.0.join("missing.merlin.json"), false),
        (target.clone(), true),
    ] {
        model
            .open_project(path, before, ReplacePermission::DiscardConfirmed)
            .unwrap();
        if cancel {
            model.stop().unwrap();
        }
        drain(&mut model);
        assert_eq!(application.snapshot(Direction::Upstream), before);
        assert_eq!(model.project_path(), Some(target.as_path()));
        assert!(model.current());
        assert!(model.controller.is_some());
    }
    let wrong = directory.0.join("downstream.merlin.json");
    std::fs::write(
        &wrong,
        include_bytes!("../../../../../examples/workbench/downstream/single/workbench.merlin.json"),
    )
    .unwrap();
    model
        .open_project(wrong, before, ReplacePermission::DiscardConfirmed)
        .unwrap();
    drain(&mut model);
    assert_eq!(application.snapshot(Direction::Upstream), before);
    assert_eq!(model.project_state()["status"], "rejected");
    assert_eq!(model.project_path(), Some(target.as_path()));
    assert_eq!(
        model
            .controller
            .as_ref()
            .unwrap()
            .inspect(|simulation| simulation.document().unwrap().clone())
            .unwrap(),
        ConfigurationDocument::from_json(SOURCE).unwrap()
    );
}

#[test]
fn native_intent_cancel_stale_discard_and_save_continue_keep_exact_revision() {
    let (application, mut model) = configured();
    let before = application.snapshot(Direction::Upstream);
    model.request_project_new().unwrap();
    assert!(model.request_project_new().is_err());
    model.resolve_project(false, true).unwrap();
    assert_eq!(application.snapshot(Direction::Upstream), before);
    model.request_project_new().unwrap();
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let newer = application.snapshot(Direction::Upstream);
    assert!(model.resolve_project(true, false).is_err());
    assert_eq!(application.snapshot(Direction::Upstream), newer);
    assert!(model.current());
    let directory = Directory::new();
    let target = directory.0.join("save-before-new.merlin.json");
    model.request_project_new().unwrap();
    model.save_project(Some(target.clone())).unwrap();
    drain(&mut model);
    assert_eq!(model.project_state()["status"], "saved");
    assert!(!application.snapshot(Direction::Upstream).dirty);
    // Only this exact successful Save refreshes its captured dirty intent.
    model.resolve_project(false, false).unwrap();
    assert!(!model.current());
    assert!(model.project_path().is_none());
    let blank = application.snapshot(Direction::Upstream);
    model.request_project_open(target.clone()).unwrap();
    model.resolve_project(false, true).unwrap();
    assert_eq!(application.snapshot(Direction::Upstream), blank);
    model.request_project_open(target).unwrap();
    model.resolve_project(false, false).unwrap();
    drain(&mut model);
    assert!(model.current());
    application.mark_project_dirty(Direction::Upstream).unwrap();
    model.request_project_new().unwrap();
    model.resolve_project(true, false).unwrap();
    assert!(!model.current());
    assert!(!application.snapshot(Direction::Upstream).dirty);
}

#[test]
fn stop_after_actual_write_keeps_save_completion_truthful_and_adopts_destination() {
    let (application, mut model) = configured();
    let directory = Directory::new();
    let previous = directory.0.join("previous.merlin.json");
    model.save_project(Some(previous.clone())).unwrap();
    drain(&mut model);
    application.mark_project_dirty(Direction::Upstream).unwrap();
    let expected = application.snapshot(Direction::Upstream);
    let target = directory.0.join("completed-before-poll.merlin.json");
    model.save_project(Some(target.clone())).unwrap();
    let deadline = Instant::now() + WATCHDOG;
    while !target.exists() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(POLL_INTERVAL);
    }
    // Atomic replacement happened; no GUI completion has been drained yet.
    Project::from_file(&target).unwrap();
    assert_eq!(model.project_path(), Some(previous.as_path()));
    assert_eq!(application.snapshot(Direction::Upstream), expected);
    assert!(model.stop().unwrap_err().contains("Save is finishing"));
    assert!(!model.preparation_cancelled);
    drain(&mut model);
    assert_eq!(model.project_state()["status"], "saved");
    assert_eq!(model.project_path(), Some(target.as_path()));
    let saved = application.snapshot(Direction::Upstream);
    assert_eq!(saved.key, expected.key);
    assert_eq!(saved.revision, expected.revision);
    assert!(!saved.dirty);
}
