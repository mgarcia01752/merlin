/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Runtime asynchronous Save, exact acknowledgement and pending native decisions.
use super::*;
use crate::{downstream::workbench::project_load::PreparedProject, shell::Direction};
use serde_json::Value;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

const PROJECT: &[u8] =
    include_bytes!("../../../../../examples/workbench/downstream/single/workbench.merlin.json");
const WAIT_LIMIT: Duration = Duration::from_secs(30);
const POLL_DELAY: Duration = Duration::from_millis(2);
const SENTINEL: &[u8] = b"Independent existing output sentinel";
static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let id = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("merlin-qt-file-intent-{}-{id}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    /// Removes only this fixture's successfully created directory.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn options(runnable: bool) -> Vec<u8> {
    let project: Project = serde_json::from_slice(PROJECT).unwrap();
    serde_json::to_vec(&crate::downstream::workbench::project_load::SaveOptions {
        settings: serde_json::to_string(&project.transmission).unwrap(),
        receiver: serde_json::to_string(&project.receiver).unwrap(),
        observations: serde_json::to_string(&Some(
            merlin::downstream::FullLoopObservationInput::default(),
        ))
        .unwrap(),
        verified: project.comparison_enabled,
        dark: project.dark,
        runnable,
    })
    .unwrap()
}
fn finish_write(owner: &mut WorkbenchFileRust) -> Value {
    let deadline = Instant::now() + WAIT_LIMIT;
    loop {
        let (status, project) = owner.loader.poll();
        assert!(project.is_none());
        if status["loading"] != true {
            return status;
        }
        assert!(
            Instant::now() < deadline,
            "file worker did not finish: {status}"
        );
        std::thread::sleep(POLL_DELAY);
    }
}
#[test]
fn runtime_save_and_new_or_open_continue_only_after_matching_saved_guard_is_refreshed() {
    for opening in [false, true] {
        let mut owner = editor_tests::populated();
        let directory = Directory::new();
        let application = owner.context.application.clone();
        application
            .mark_project_dirty(Direction::Downstream)
            .unwrap();
        let captured = application.snapshot(Direction::Downstream);
        let other = application.snapshot(Direction::Upstream);
        if opening {
            let candidate = PreparedProject::prepare(
                serde_json::from_slice(PROJECT).unwrap(),
                None,
                &application,
                captured.key,
                workbench_session::snapshot(&owner.context),
            )
            .unwrap();
            owner.stage_open(candidate).unwrap();
        } else {
            owner.request_new().unwrap();
        }
        let path = directory.0.join("runtime.merlin.json");
        assert_eq!(
            owner.queue_save(Some(path.clone()), options(true)).unwrap()["saving"],
            true
        );
        assert_eq!(application.snapshot(Direction::Downstream), captured);
        owner.loader.cancel();
        assert_eq!(finish_write(&mut owner)["write_completed"], true);
        let public = merlin::common::Project::from_file(&path).unwrap();
        assert_eq!(
            public.direction(),
            merlin::common::ProjectDirection::Downstream
        );
        assert_eq!(owner.adopt_saved().unwrap()["saved"], true);
        assert!(!application.snapshot(Direction::Downstream).dirty);
        assert_eq!(owner.path.as_ref().unwrap().value(), &path);
        let adopted = owner.resolve_intent(false, false).unwrap();
        assert_eq!(adopted[if opening { "opened" } else { "created" }], true);
        assert_ne!(
            application.snapshot(Direction::Downstream).key.session,
            captured.key.session
        );
        assert_eq!(application.snapshot(Direction::Upstream), other);
        assert!(owner.path.is_none());
    }
}
#[test]
fn stale_post_write_acknowledgement_keeps_newer_dirty_state_and_previous_destination() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let old_path = owner.path.as_ref().unwrap().value().clone();
    let application = owner.context.application.clone();
    application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    owner
        .queue_save(Some(directory.0.join("stale.merlin.json")), options(false))
        .unwrap();
    assert_eq!(finish_write(&mut owner)["write_completed"], true);
    application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    let changed = application.snapshot(Direction::Downstream);
    assert!(owner.adopt_saved().is_err());
    assert_eq!(application.snapshot(Direction::Downstream), changed);
    assert_eq!(owner.path.as_ref().unwrap().value(), &old_path);
    assert!(changed.dirty);
}
#[test]
fn runtime_source_protection_and_failed_write_leave_file_path_and_dirty_state_unchanged() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let old_path = owner.path.as_ref().unwrap().value().clone();
    let original = std::fs::read(&old_path).unwrap();
    let application = owner.context.application.clone();
    application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    let before = application.snapshot(Direction::Downstream);
    let invalid = directory.0.join("nonregular.merlin.json");
    std::fs::create_dir(&invalid).unwrap();
    for target in [old_path.clone(), invalid] {
        owner.queue_save(Some(target), options(false)).unwrap();
        assert!(finish_write(&mut owner)["error"].is_string());
        assert_eq!(application.snapshot(Direction::Downstream), before);
        assert_eq!(owner.path.as_ref().unwrap().value(), &old_path);
        assert_eq!(std::fs::read(&old_path).unwrap(), original);
    }
    let target = directory.0.join("packet.merlin.json");
    std::fs::write(&target, SENTINEL).unwrap();
    assert!(owner
        .queue_save(Some(target.clone()), b"{}".to_vec())
        .is_err());
    assert_eq!(std::fs::read(target).unwrap(), SENTINEL);
}
#[test]
fn cancelled_and_stale_native_file_intents_cannot_replace_actual_accepted_sections() {
    let mut owner = editor_tests::populated();
    let application = owner.context.application.clone();
    application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    let original = workbench_session::saved_sections(&owner.context);
    let before = application.snapshot(Direction::Downstream);
    owner.request_new().unwrap();
    assert_eq!(
        owner.resolve_intent(false, true).unwrap()["cancelled"],
        true
    );
    assert_eq!(application.snapshot(Direction::Downstream), before);
    owner.request_new().unwrap();
    application
        .mark_project_dirty(Direction::Downstream)
        .unwrap();
    let changed = application.snapshot(Direction::Downstream);
    assert!(owner.resolve_intent(true, false).is_err());
    assert_eq!(application.snapshot(Direction::Downstream), changed);
    assert_eq!(
        &*workbench_session::saved_sections(&owner.context),
        &*original
    );
}

#[test]
fn accepted_input_edit_without_generation_change_rejects_save_ack_and_stays_dirty() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let application = owner.context.application.clone();
    let old_path = owner.path.as_ref().unwrap().value().clone();
    let before = application.snapshot(Direction::Downstream);
    owner
        .queue_save(
            Some(directory.0.join("intervening.merlin.json")),
            options(false),
        )
        .unwrap();
    assert_eq!(finish_write(&mut owner)["write_completed"], true);
    workbench_session::update(&owner.context, |_, sections| {
        sections.insert("channel_model".into(), json!({"mode":"off"}));
        Ok(())
    })
    .unwrap();
    let changed = application.snapshot(Direction::Downstream);
    assert_eq!(changed.key, before.key);
    assert_ne!(changed.revision, before.revision);
    assert!(changed.dirty);
    assert!(owner.adopt_saved().is_err());
    assert_eq!(application.snapshot(Direction::Downstream), changed);
    assert_eq!(owner.path.as_ref().unwrap().value(), &old_path);
}
