/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public-API publication with admitted or fixed truthful completion status.
use super::*;
use crate::{
    downstream::{
        common::constants::{
            DESIGNER_CANCELLED_STATUS_ADMISSION_MESSAGE, DESIGNER_FAILED_STATUS_ADMISSION_MESSAGE,
            DESIGNER_SAVED_STATUS_ADMISSION_MESSAGE,
        },
        inputs::measurement_input::document::{Capture, Measurement},
    },
    shell::RetainedResources,
};
use std::{path::PathBuf, sync::atomic::AtomicU64};

// Unique fixture directory sequence, independent of application identities.
static DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);
// Sentinel bytes prove rejected publication never overwrites the destination.
const EXISTING_DESTINATION: &[u8] = b"Preserve this existing fixture output.";

struct Directory(PathBuf);

impl Directory {
    fn new() -> Self {
        let sequence = DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "merlin-qt-saved-status-{}-{sequence}",
            std::process::id()
        ));
        // An existing path fails safely: this fixture never adopts or cleans it.
        std::fs::create_dir(&path).unwrap();
        Self(path)
    }

    fn destination(&self) -> (PathBuf, String) {
        let path = self.0.join("capture.json");
        let uri = url::Url::from_file_path(&path).unwrap().to_string();
        (path, uri)
    }
}

impl Drop for Directory {
    /// Removes only the directory successfully created by this fixture.
    /// # Returns
    /// Unit; a cleanup failure is ignored during unwinding rather than replacing
    /// the original assertion failure. No source or user destination is involved.
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn publish(owner: &mut crate::bridge::measurement_input::MeasurementInputsRust, uri: String) {
    lifecycle::save_with_picker(
        owner,
        ExportFormat::Json,
        |task| {
            task();
            Ok(())
        },
        move |collection, json| {
            assert!(!collection);
            assert!(json);
            Ok(Some(uri))
        },
    )
    .unwrap();
}

fn assert_capture(path: &std::path::Path, expected: &RxMerFrame) {
    let bytes = std::fs::read(path).unwrap();
    let capture = Capture::decode("saved.json".into(), bytes, true).unwrap();
    let Measurement::Rxmer(frame) = capture.measurement else {
        panic!("Actual JSON export remains an RxMER capture");
    };
    assert_eq!(frame.mer_db, expected.mer_db);
    assert_eq!(frame.capture_time, expected.capture_time);
    assert_eq!(
        serde_json::to_value(&frame.key).unwrap(),
        serde_json::to_value(&expected.key).unwrap()
    );
}

#[test]
fn actual_json_publication_retains_queued_status_then_preserves_exact_ready_and_values() {
    let directory = Directory::new();
    let (path, uri) = directory.destination();
    let (mut owner, _) = admission_tests::prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(admission_tests::ready(&owner));
    let before = budget.usage();
    publish(&mut owner, uri);
    assert_capture(&path, &original.value().frames[0]);
    assert!(budget.usage().payload_bytes > before.payload_bytes);
    assert!(budget.usage().ordered_entries > before.ordered_entries);
    let reply = lifecycle::poll(&mut owner.designer_export);
    assert_eq!(reply["saved"], true);
    assert_eq!(reply["path"], path.to_string_lossy().as_ref());
    assert!(reply.get("error").is_none());
    assert!(Arc::ptr_eq(admission_tests::ready(&owner), &original));
    assert_eq!(budget.usage(), before);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(original.fixture_reserved_resources())
            .unwrap()
    );
    drop(original);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn rejected_status_after_publication_keeps_the_saved_file_and_truthful_static_reply() {
    #[derive(Clone, Copy)]
    enum Dimension {
        Payload,
        Entries,
    }
    // JSON has positive payload/ordered-entry demand and no shared roots.
    for dimension in [Dimension::Payload, Dimension::Entries] {
        let directory = Directory::new();
        let (path, uri) = directory.destination();
        let (mut owner, _) = admission_tests::prepared_owner();
        let budget = owner.context.application.budget().clone();
        let original = Arc::clone(admission_tests::ready(&owner));
        let before = budget.usage();
        let worker_budget = budget.clone();
        let held = Arc::new(std::sync::Mutex::new(None));
        let worker_held = Arc::clone(&held);
        lifecycle::save_with_picker(
            &mut owner,
            ExportFormat::Json,
            |task| {
                task();
                Ok(())
            },
            move |_, _| {
                let used = worker_budget.usage();
                let pressure = match dimension {
                    Dimension::Payload => RetainedResources {
                        payload_bytes: usize::MAX - used.payload_bytes,
                        ..Default::default()
                    },
                    Dimension::Entries => RetainedResources {
                        ordered_entries: usize::MAX - used.ordered_entries,
                        ..Default::default()
                    },
                };
                *worker_held.lock().unwrap() =
                    Some(worker_budget.reserve_resources(pressure).unwrap());
                Ok(Some(uri))
            },
        )
        .unwrap();
        assert_capture(&path, &original.value().frames[0]);
        let full = budget.usage();

        // The operation already returned: only Working, its two atomic pointees,
        // channel guard and the queued rejected-status frame remain beyond Ready.
        let pending = RetainedResources {
            payload_bytes: std::mem::size_of::<State>()
                + std::mem::size_of::<Retained<AtomicBool>>()
                + std::mem::size_of::<Retained<AtomicUsize>>()
                + std::mem::size_of::<Retained<()>>()
                + std::mem::size_of::<Retained<Result<Outcome, WorkerFailure>>>(),
            shared_roots: 3,  // Two actual controls and one channel allocation.
            channel_count: 1, // One real export completion queue.
            slot_count: crate::downstream::common::constants::EXPORT_RESULT_QUEUE_CAPACITY,
            ..Default::default()
        };
        let reply = lifecycle::poll(&mut owner.designer_export);
        assert_eq!(reply["error"], DESIGNER_SAVED_STATUS_ADMISSION_MESSAGE);
        assert!(reply.get("saved").is_none());
        assert!(reply.get("path").is_none());
        assert!(Arc::ptr_eq(admission_tests::ready(&owner), &original));
        assert_eq!(
            budget.usage(),
            RetainedResources {
                payload_bytes: full.payload_bytes - pending.payload_bytes,
                ordered_entries: full.ordered_entries,
                shared_roots: full.shared_roots - pending.shared_roots,
                channel_count: full.channel_count - pending.channel_count,
                slot_count: full.slot_count - pending.slot_count,
                presentation_utf16_bytes: 0,
                presentation_entries: 0,
                retained_views: 0,
                open_views: 0,
            }
        );
        drop(held.lock().unwrap().take());
        assert_eq!(budget.usage(), before);
        drop(owner);
        drop(original);
        assert_eq!(
            budget.usage(),
            crate::shell::ApplicationSessions::fixture_counter_resources()
        );
    }
}

#[test]
fn picker_cancel_and_rejected_existing_output_preserve_ready_and_destination_bytes() {
    let directory = Directory::new();
    let (path, uri) = directory.destination();
    let (mut owner, _) = admission_tests::prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(admission_tests::ready(&owner));
    let before = budget.usage();
    lifecycle::save_with_picker(
        &mut owner,
        ExportFormat::Json,
        |task| {
            task();
            Ok(())
        },
        |_, _| Ok(None),
    )
    .unwrap();
    let reply = lifecycle::poll(&mut owner.designer_export);
    assert_eq!(reply["cancelled"], true);
    assert!(reply.get("saved").is_none());
    assert!(reply.get("error").is_none());
    assert_eq!(budget.usage(), before);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let fallback = saved::Saved::admit(reply, &budget);
    assert!(matches!(
        fallback,
        saved::Saved::Rejected(saved::Status::Cancelled)
    ));
    assert_eq!(
        fallback.reply()["error"],
        DESIGNER_CANCELLED_STATUS_ADMISSION_MESSAGE
    );
    assert_eq!(fallback.reply()["cancelled"], true);
    drop(fallback);
    drop(pressure);
    assert_eq!(budget.usage(), before);
    std::fs::write(&path, EXISTING_DESTINATION).unwrap();
    publish(&mut owner, uri);
    let reply = lifecycle::poll(&mut owner.designer_export);
    assert!(reply["error"]
        .as_str()
        .is_some_and(|message| !message.is_empty()));
    assert!(reply.get("saved").is_none());
    assert_eq!(std::fs::read(&path).unwrap(), EXISTING_DESTINATION);
    assert!(Arc::ptr_eq(admission_tests::ready(&owner), &original));
    assert_eq!(budget.usage(), before);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let fallback = saved::Saved::admit(reply, &budget);
    assert!(matches!(
        fallback,
        saved::Saved::Rejected(saved::Status::Failed)
    ));
    assert_eq!(
        fallback.reply()["error"],
        DESIGNER_FAILED_STATUS_ADMISSION_MESSAGE
    );
    assert!(fallback.reply().get("saved").is_none());
    drop(fallback);
    drop(pressure);
    assert_eq!(budget.usage(), before);
    drop(owner);
    drop(original);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
