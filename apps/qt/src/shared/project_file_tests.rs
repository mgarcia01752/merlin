/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual filesystem cases for the byte writer, independent of Qt and science.

use super::{write_bytes, write_bytes_with_limit, write_sibling, MAX_PROJECT_BYTES};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicUsize, Ordering},
};

// A small independent policy fixture exercises exact/over-limit paths without a
// 144 MiB allocation. Production still passes its original transport ceiling.
const FIXTURE_MAXIMUM_BYTES: usize = 8;
const EXACT_LIMIT_BYTES: &[u8] = b"01234567";
const OVER_LIMIT_BYTES: &[u8] = b"012345678";
const ORIGINAL_BYTES: &[u8] = b"original accepted project";
const REPLACEMENT_BYTES: &[u8] = b"\0\xffreplacement\n";
static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new() -> Self {
        let sequence = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "merlin-project-file-tests-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&path).unwrap();
        Self { path }
    }

    fn file(&self, name: &str) -> PathBuf {
        self.path.join(name)
    }

    fn entries(&self) -> Vec<PathBuf> {
        let mut entries: Vec<_> = fs::read_dir(&self.path)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        entries
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).unwrap();
    }
}

fn assert_bytes(path: &Path, expected: &[u8]) {
    assert_eq!(fs::read(path).unwrap(), expected);
}

#[test]
fn production_limit_preserves_original_144_mib_policy() {
    // Independent decimal expectation verifies the documented binary conversion.
    assert_eq!(MAX_PROJECT_BYTES, 150_994_944);
}

#[test]
fn new_destination_preserves_arbitrary_bytes_and_leaves_no_sibling() {
    let directory = TestDirectory::new();
    let destination = directory.file("new.merlin.json");
    write_bytes(&destination, REPLACEMENT_BYTES).unwrap();
    assert_bytes(&destination, REPLACEMENT_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn replacement_replaces_complete_regular_file_without_appending() {
    let directory = TestDirectory::new();
    let destination = directory.file("existing.merlin.json");
    fs::write(&destination, ORIGINAL_BYTES).unwrap();
    write_bytes(&destination, REPLACEMENT_BYTES).unwrap();
    assert_bytes(&destination, REPLACEMENT_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn empty_bytes_are_written_without_interpreting_project_content() {
    let directory = TestDirectory::new();
    let destination = directory.file("empty.merlin.json");
    fs::write(&destination, ORIGINAL_BYTES).unwrap();
    write_bytes(&destination, &[]).unwrap();
    assert_bytes(&destination, &[]);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn exact_limit_is_inclusive_and_creates_complete_output() {
    let directory = TestDirectory::new();
    let destination = directory.file("exact.merlin.json");
    assert_eq!(EXACT_LIMIT_BYTES.len(), FIXTURE_MAXIMUM_BYTES);
    write_bytes_with_limit(&destination, EXACT_LIMIT_BYTES, FIXTURE_MAXIMUM_BYTES).unwrap();
    assert_bytes(&destination, EXACT_LIMIT_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn one_byte_over_limit_preserves_existing_content_without_sibling() {
    let directory = TestDirectory::new();
    let destination = directory.file("existing.merlin.json");
    fs::write(&destination, ORIGINAL_BYTES).unwrap();
    assert_eq!(OVER_LIMIT_BYTES.len(), FIXTURE_MAXIMUM_BYTES + 1);
    assert_eq!(
        write_bytes_with_limit(&destination, OVER_LIMIT_BYTES, FIXTURE_MAXIMUM_BYTES),
        Err("Workbench exceeds 144 MiB.".into())
    );
    assert_bytes(&destination, ORIGINAL_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn over_limit_new_destination_creates_nothing() {
    let directory = TestDirectory::new();
    let destination = directory.file("oversized.merlin.json");
    assert!(write_bytes_with_limit(&destination, OVER_LIMIT_BYTES, FIXTURE_MAXIMUM_BYTES).is_err());
    assert!(directory.entries().is_empty());
}

#[test]
fn directory_destination_rejects_and_preserves_its_contents() {
    let directory = TestDirectory::new();
    let destination = directory.file("project-directory");
    fs::create_dir(&destination).unwrap();
    let original = destination.join("original");
    fs::write(&original, ORIGINAL_BYTES).unwrap();
    assert_eq!(
        write_bytes(&destination, REPLACEMENT_BYTES),
        Err("Choose a regular project destination.".into())
    );
    assert_bytes(&original, ORIGINAL_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[cfg(unix)]
#[test]
fn symlink_destination_rejects_without_touching_target_or_link() {
    let directory = TestDirectory::new();
    let target = directory.file("original.capture");
    let destination = directory.file("linked.merlin.json");
    fs::write(&target, ORIGINAL_BYTES).unwrap();
    std::os::unix::fs::symlink(&target, &destination).unwrap();
    let before = directory.entries();
    assert!(write_bytes(&destination, REPLACEMENT_BYTES).is_err());
    assert_eq!(fs::read_link(&destination).unwrap(), target);
    assert_bytes(&target, ORIGINAL_BYTES);
    assert_eq!(directory.entries(), before);
}

#[cfg(unix)]
#[test]
fn dangling_symlink_rejects_without_creating_target_or_replacing_link() {
    let directory = TestDirectory::new();
    let target = directory.file("absent.capture");
    let destination = directory.file("dangling.merlin.json");
    std::os::unix::fs::symlink(&target, &destination).unwrap();
    assert!(!destination.exists());
    assert!(write_bytes(&destination, REPLACEMENT_BYTES).is_err());
    assert_eq!(fs::read_link(&destination).unwrap(), target);
    assert!(!target.exists());
    assert_eq!(directory.entries(), vec![destination]);
}

#[cfg(unix)]
#[test]
fn socket_destination_rejects_without_removing_socket() {
    let directory = TestDirectory::new();
    let destination = directory.file("socket");
    let listener = std::os::unix::net::UnixListener::bind(&destination).unwrap();
    assert!(write_bytes(&destination, REPLACEMENT_BYTES).is_err());
    assert_eq!(directory.entries(), vec![destination]);
    drop(listener);
}

#[test]
fn missing_parent_returns_error_and_leaves_no_partial_output() {
    let directory = TestDirectory::new();
    let destination = directory.file("missing-parent/project.merlin.json");
    assert!(write_bytes(&destination, REPLACEMENT_BYTES).is_err());
    assert!(directory.entries().is_empty());
}

#[test]
fn non_directory_parent_returns_error_without_touching_file() {
    let directory = TestDirectory::new();
    let parent_file = directory.file("parent-file");
    fs::write(&parent_file, ORIGINAL_BYTES).unwrap();
    let destination = parent_file.join("project.merlin.json");
    assert!(write_bytes(&destination, REPLACEMENT_BYTES).is_err());
    assert_bytes(&parent_file, ORIGINAL_BYTES);
    assert_eq!(directory.entries(), vec![parent_file]);
}

#[test]
fn rename_failure_cleans_created_sibling_and_preserves_blocking_destination() {
    let directory = TestDirectory::new();
    let destination = directory.file("blocking-directory");
    let temporary = directory.file("owned-sibling");
    fs::create_dir(&destination).unwrap();
    let original = destination.join("original");
    fs::write(&original, ORIGINAL_BYTES).unwrap();

    // Exercise the actual post-create failure kernel independently of preflight.
    assert!(write_sibling(&destination, &temporary, REPLACEMENT_BYTES).is_err());
    assert!(!temporary.exists());
    assert_bytes(&original, ORIGINAL_BYTES);
    assert_eq!(directory.entries(), vec![destination]);
}

#[test]
fn exclusive_create_collision_preserves_other_sibling_and_destination() {
    let directory = TestDirectory::new();
    let destination = directory.file("existing.merlin.json");
    let temporary = directory.file("other-writer-sibling");
    fs::write(&destination, ORIGINAL_BYTES).unwrap();
    fs::write(&temporary, EXACT_LIMIT_BYTES).unwrap();
    let before = directory.entries();
    assert!(write_sibling(&destination, &temporary, REPLACEMENT_BYTES).is_err());
    assert_bytes(&destination, ORIGINAL_BYTES);
    assert_bytes(&temporary, EXACT_LIMIT_BYTES);
    assert_eq!(directory.entries(), before);
}

#[cfg(unix)]
#[test]
fn exclusive_create_collision_never_follows_or_removes_other_symlink() {
    let directory = TestDirectory::new();
    let destination = directory.file("existing.merlin.json");
    let target = directory.file("other-writer-input");
    let temporary = directory.file("other-writer-sibling");
    fs::write(&destination, ORIGINAL_BYTES).unwrap();
    fs::write(&target, EXACT_LIMIT_BYTES).unwrap();
    std::os::unix::fs::symlink(&target, &temporary).unwrap();
    let before = directory.entries();
    assert!(write_sibling(&destination, &temporary, REPLACEMENT_BYTES).is_err());
    assert_eq!(fs::read_link(&temporary).unwrap(), target);
    assert_bytes(&destination, ORIGINAL_BYTES);
    assert_bytes(&target, EXACT_LIMIT_BYTES);
    assert_eq!(directory.entries(), before);
}
