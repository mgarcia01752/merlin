/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual Save paths: candidate overlap, borrowed reuse and atomic failure.
use super::*;
use crate::shell::RetainedResources;
use std::{
    io::Write,
    sync::atomic::{AtomicU64, Ordering},
};

// Isolated filesystem fixture identities, independent of application generations.
static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
// Sentinel for an existing destination owned only by this fixture.
const EXISTING_DESTINATION: &[u8] = b"Preserve this existing project output.";
// Optional validation artifact directory; never read by the desktop application.
const PROJECT_EVIDENCE_DIRECTORY: &str = "MERLIN_QT_PROJECT_EVIDENCE_DIR";

struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let sequence = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "merlin-qt-project-path-{}-{sequence}",
            std::process::id(),
        ));
        // Never adopt or clean an existing directory on collision.
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    /// Removes only the directory successfully created by this fixture.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn settings() -> (String, String, bool, bool) {
    let project: Project = serde_json::from_slice(include_bytes!(
        "../../../../../examples/workbench/downstream/single/workbench.merlin.json"
    ))
    .unwrap();
    (
        serde_json::to_string(&project.transmission).unwrap(),
        serde_json::to_string(&project.receiver).unwrap(),
        project.comparison_enabled,
        project.dark,
    )
}

fn save(
    owner: &mut WorkbenchFileRust,
    target: Option<PathBuf>,
) -> Result<serde_json::Value, String> {
    let (settings, receiver, verified, dark) = settings();
    owner.save_to(target, &settings, &receiver, verified, dark)
}

fn preserve_evidence(name: &str, bytes: &[u8]) {
    if let Some(directory) = std::env::var_os(PROJECT_EVIDENCE_DIRECTORY) {
        let directory = PathBuf::from(directory);
        assert!(
            directory.is_dir(),
            "Validation creates the artifact directory first"
        );
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(directory.join(name))
            .unwrap();
        file.write_all(bytes).unwrap();
    }
}

#[test]
fn rejected_save_path_preserves_old_destination_state_and_existing_file_bytes() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let target = directory.0.join("existing.merlin.json");
    std::fs::write(&target, EXISTING_DESTINATION).unwrap();
    let budget = owner.context.application.budget().clone();
    let baseline = budget.usage();
    let original = owner.path.as_ref().unwrap().value() as *const PathBuf;
    let saved = workbench_session::saved_sections(&owner.context);
    let before = owner
        .context
        .application
        .snapshot(crate::shell::Direction::Downstream);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    assert!(save(&mut owner, Some(target.clone()))
        .unwrap_err()
        .contains("retained-data limit"));
    assert_eq!(
        owner.path.as_ref().unwrap().value() as *const PathBuf,
        original
    );
    assert_eq!(
        owner
            .context
            .application
            .snapshot(crate::shell::Direction::Downstream),
        before
    );
    assert_eq!(&*workbench_session::saved_sections(&owner.context), &*saved);
    assert_eq!(std::fs::read(target).unwrap(), EXISTING_DESTINATION);
    assert_eq!(std::fs::read_dir(&directory.0).unwrap().count(), 1); // Only the original sentinel.
    assert_eq!(budget.usage(), full);
    drop(pressure);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn normalized_path_rejects_before_file_creation_and_failed_writer_preserves_old_guard() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let raw = directory.0.join("new-project");
    let normalized = directory.0.join("new-project.merlin.json");
    let budget = owner.context.application.budget().clone();
    let baseline = budget.usage();
    let previous = owner.path.as_ref().unwrap().value() as *const PathBuf;
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(save(&mut owner, Some(raw.clone())).is_err());
    assert!(!raw.exists() && !normalized.exists());
    drop(pressure);
    let nonregular = directory.0.join("directory.merlin.json");
    std::fs::create_dir(&nonregular).unwrap();
    assert!(save(&mut owner, Some(nonregular))
        .unwrap_err()
        .contains("regular project"));
    assert_eq!(
        owner.path.as_ref().unwrap().value() as *const PathBuf,
        previous
    );
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn successful_save_moves_candidate_and_borrows_same_guard_at_full_pressure() {
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let raw = directory.0.join("saved");
    let target = directory.0.join("saved.merlin.json");
    assert_eq!(save(&mut owner, Some(raw)).unwrap()["saved"], true);
    let budget = owner.context.application.budget().clone();
    let current = owner.path.as_ref().unwrap();
    assert_eq!(current.value(), &target);
    assert!(budget.owns(current));
    let pointer = current.value() as *const PathBuf;
    let baseline = budget.usage();
    let original = std::fs::read(&target).unwrap();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    assert_eq!(save(&mut owner, None).unwrap()["saved"], true);
    assert_eq!(
        owner.path.as_ref().unwrap().value() as *const PathBuf,
        pointer
    );
    assert_eq!(std::fs::read(&target).unwrap(), original);
    assert_eq!(budget.usage(), full);
    drop(pressure);
    assert_eq!(budget.usage(), baseline);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn actual_ui_saved_project_loads_and_runs_through_the_public_rust_api() {
    use merlin::downstream::{inputs::Workbench, FullLoopControl};
    let mut owner = editor_tests::populated();
    let directory = Directory::new();
    let target = directory.0.join("public-api.merlin.json");
    save(&mut owner, Some(target.clone())).unwrap();
    let bytes = std::fs::read(&target).unwrap();
    let plan = Workbench::prepare_full_loop(&bytes).unwrap();
    let request = plan.request(0).unwrap();
    let result = request.run(&FullLoopControl::new()).unwrap();
    let report = serde_json::to_value(result.report()).unwrap();
    assert_eq!(report["status"], "complete");
    assert!(
        report["summary"]["comparison"]["bits_tested"]
            .as_u64()
            .unwrap()
            > 0
    );
    let reference = json!({"status":report["status"],"summary":report["summary"],
        "error":report["error"],"unrecovered_generated_payloads":report["unrecovered_generated_payloads"]});
    preserve_evidence("saved-workbench.merlin.json", &bytes);
    preserve_evidence(
        "rust-result.json",
        &serde_json::to_vec_pretty(&reference).unwrap(),
    );
}
