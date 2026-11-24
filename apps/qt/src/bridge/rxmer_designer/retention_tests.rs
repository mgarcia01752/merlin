/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual GUI Session and independent generation/export copy ownership.
use super::*;
use crate::{
    bridge::measurement_input::MeasurementInputsRust,
    downstream::workbench::session::Context,
    shell::{ApplicationSessions, RetainedResources},
};

// Counter-only fixture ceilings, independent of desktop and scientific policies.
const TEST_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn owner() -> MeasurementInputsRust {
    let context = Context::for_application(ApplicationSessions::new(TEST_LIMIT).unwrap()).unwrap();
    MeasurementInputsRust {
        context,
        ..Default::default()
    }
}

fn pressure(used: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ..Default::default()
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: usize::MAX - used.ordered_entries,
            ..Default::default()
        },
        Dimension::Roots => RetainedResources {
            shared_roots: usize::MAX - used.shared_roots,
            ..Default::default()
        },
    }
}

#[test]
fn rejected_gui_replacement_and_worker_copy_keep_the_same_original_session_in_all_dimensions() {
    let mut owner = owner();
    let budget = owner.context.application.budget().clone();
    let session = super::tests::fixture().0;
    let expected = serde_json::to_value(&session.request).unwrap();
    let hashes = session.hashes.clone();
    let original_profile = Arc::clone(&session.profile);
    owner.replace_designer(session).unwrap();
    let before = budget.usage();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let reservation = budget
            .reserve_resources(pressure(before, dimension))
            .unwrap();
        let full = budget.usage();
        let candidate = owner.designer.as_ref().unwrap().value().clone();
        assert!(owner.replace_designer(candidate).is_err());
        assert!(owner.designer_copy().is_err());
        let retained = owner.designer.as_ref().unwrap().value();
        assert_eq!(serde_json::to_value(&retained.request).unwrap(), expected);
        assert_eq!(retained.hashes, hashes);
        assert!(Arc::ptr_eq(&retained.profile, &original_profile));
        assert_eq!(budget.usage(), full);
        drop(reservation);
        assert_eq!(budget.usage(), before);
    }
    drop(original_profile);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn actual_worker_session_survives_gui_replacement_cancel_and_owner_destruction_until_thread_exit() {
    let mut owner = owner();
    let budget = owner.context.application.budget().clone();
    owner.replace_designer(super::tests::fixture().0).unwrap();
    let work = owner.designer_copy().unwrap();
    let work_charge = work.fixture_reserved_resources();
    let old_profile = Arc::downgrade(&work.value().profile);
    let expected = serde_json::to_value(&work.value().request).unwrap();
    let cancel = Arc::new(AtomicBool::new(false));
    let worker_cancel = Arc::clone(&cancel);
    let (started_tx, started_rx) = mpsc::sync_channel(0);
    let (finish_tx, finish_rx) = mpsc::sync_channel(0);
    let thread = std::thread::spawn(move || {
        started_tx.send(()).unwrap();
        finish_rx.recv().unwrap();
        let result =
            work.value()
                .build(&work.value().request, &worker_cancel, &AtomicUsize::new(0));
        assert!(worker_cancel.load(Ordering::Relaxed));
        assert!(result.is_err());
        assert_eq!(
            serde_json::to_value(&work.value().request).unwrap(),
            expected
        );
    });
    started_rx.recv().unwrap();
    owner.replace_designer(super::tests::fixture().0).unwrap();
    cancel.store(true, Ordering::Relaxed);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(work_charge)
            .unwrap()
    );
    assert!(old_profile.upgrade().is_some());
    finish_tx.send(()).unwrap();
    thread.join().unwrap();
    assert!(old_profile.upgrade().is_none());
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn generation_and_export_copies_are_independent_admitted_inputs_with_unchanged_previews() {
    let mut owner = owner();
    let budget = owner.context.application.budget().clone();
    owner.replace_designer(super::tests::fixture().0).unwrap();
    let before = budget.usage();
    let source = owner.designer.as_ref().unwrap().value();
    let expected = source.preview(&source.request, 1).unwrap();
    let generation = owner.designer_copy().unwrap();
    let export = owner.designer_copy().unwrap();
    assert_eq!(
        generation
            .value()
            .preview(&generation.value().request, 1)
            .unwrap(),
        expected
    );
    assert_eq!(
        export.value().preview(&export.value().request, 1).unwrap(),
        expected
    );
    assert!(Arc::ptr_eq(&source.profile, &generation.value().profile));
    assert!(Arc::ptr_eq(&source.profile, &export.value().profile));
    let total = budget.usage();
    assert_eq!(
        total.payload_bytes,
        before.payload_bytes
            + generation.fixture_reserved_resources().payload_bytes
            + export.fixture_reserved_resources().payload_bytes
    );
    assert_eq!(
        total.ordered_entries,
        before.ordered_entries
            + generation.fixture_reserved_resources().ordered_entries
            + export.fixture_reserved_resources().ordered_entries
    );
    assert_eq!(
        total.shared_roots,
        before.shared_roots
            + generation.fixture_reserved_resources().shared_roots
            + export.fixture_reserved_resources().shared_roots
    );
    drop(generation);
    drop(export);
    assert_eq!(budget.usage(), before);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
