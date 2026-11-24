/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual Ready products, worker rejection and deterministic native spawn failure.
use super::*;
use crate::{
    bridge::measurement_input::MeasurementInputsRust,
    downstream::{
        inputs::measurement_input::designer::Session,
        workbench::{project_file::Project, session::Context},
    },
    shell::{ApplicationSessions, RetainedResources},
};
use std::{cell::RefCell, io};

// Counter-only fixture limits; not a desktop/scientific policy.
const TEST_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
// Existing one-based single-capture JSON export selection.
const SELECTION: &str = r#"{"first":1,"last":1,"clamp":true,"collection":false}"#;

/// Reuses the original single-capture, validated Session and Ready export fixture.
/// # Returns
/// Actual owner with admitted Session/Prepared and its unchanged recipe document.
/// # Panics
/// Panics if the checked workbench or existing preflight no longer loads.
pub(super) fn prepared_owner() -> (MeasurementInputsRust, String) {
    let mut context = Context::isolated();
    context.application = ApplicationSessions::new(TEST_LIMIT).unwrap();
    let project: Project = serde_json::from_slice(include_bytes!(
        "../../../../../examples/workbench/single/workbench.merlin.json"
    ))
    .unwrap();
    let (inputs, editors) = project.prepare().unwrap();
    let session = Session::begin(
        &editors.measurements.accepted_rx,
        Arc::new(inputs.profile.unwrap()),
        inputs.profile_capture.unwrap(),
    )
    .unwrap();
    let document = serde_json::to_string(&session.request).unwrap();
    let mut owner = MeasurementInputsRust {
        context,
        ..Default::default()
    };
    owner.replace_designer(session).unwrap();
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        task();
        Ok(())
    })
    .unwrap();
    let result = lifecycle::poll(&mut owner.designer_export);
    assert_eq!(result["can_json"], true);
    assert!(matches!(owner.designer_export, Some(State::Ready(_))));
    (owner, document)
}

/// Borrows the actual prepared product from the original Ready-state fixture.
/// # Arguments
/// * `owner` - Adapter after successful existing preflight/poll.
/// # Returns
/// Shared immutable product borrowed for this owner's lifetime.
/// # Panics
/// Panics when the test precondition is not Ready.
pub(super) fn ready(owner: &MeasurementInputsRust) -> &SharedPrepared {
    let Some(State::Ready(prepared)) = &owner.designer_export else {
        panic!("Existing actual Ready product");
    };
    prepared
}

#[test]
fn real_queued_export_error_preserves_original_message_and_previous_ready() {
    let (mut owner, document) = prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(ready(&owner));
    let before = budget.usage();
    let mut request: Request = serde_json::from_str(&document).unwrap();
    request.source = "unrecognized-source".into();
    let expected = owner
        .designer
        .as_ref()
        .unwrap()
        .value()
        .export_frames(
            &request,
            1,
            1,
            &AtomicBool::new(false),
            &AtomicUsize::new(0),
        )
        .unwrap_err();
    let document = serde_json::to_string(&request).unwrap();
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        task();
        Ok(())
    })
    .unwrap();
    assert!(budget.usage().payload_bytes > before.payload_bytes);
    let reply = lifecycle::poll(&mut owner.designer_export);
    assert_eq!(reply["error"], expected);
    assert_eq!(reply["running"], false);
    assert_eq!(reply["can_export"], false);
    assert!(Arc::ptr_eq(ready(&owner), &original));
    assert_eq!(budget.usage(), before);
    drop(owner);
    drop(original);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn parsed_export_input_rejection_after_session_copy_preserves_exact_ready_owner() {
    let (mut owner, document) = prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(ready(&owner));
    let before = budget.usage();
    let copied_session = owner.designer_copy().unwrap();
    let copy_charge = copied_session.fixture_reserved_resources();
    drop(copied_session);
    // Admit the exact Session copy, leaving no byte allowance for its separately
    // parsed Request/Selection input. This exercises the later input gate.
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - before.payload_bytes - copy_charge.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    let admitted_copy = owner
        .designer_copy()
        .expect("Session copy fits this later gate");
    assert_eq!(budget.usage().payload_bytes, usize::MAX);
    drop(admitted_copy);
    assert!(
        lifecycle::prepare(&mut owner, &document, SELECTION, |_task| panic!(
            "Rejected parsed input cannot spawn"
        ))
        .is_err()
    );
    assert!(Arc::ptr_eq(ready(&owner), &original));
    assert_eq!(budget.usage(), full);
    drop(pressure);
    assert_eq!(budget.usage(), before);
    drop(owner);
    assert_eq!(budget.usage(), original.fixture_reserved_resources());
    drop(original);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn detached_export_preparation_retains_parsed_input_after_gui_cancel_and_disposal() {
    let (mut owner, document) = prepared_owner();
    let budget = owner.context.application.budget().clone();
    let before = budget.usage();
    let input = input::Input {
        request: serde_json::from_str(&document).unwrap(),
        selection: serde_json::from_str(SELECTION).unwrap(),
    }
    .admit(&budget)
    .unwrap();
    let input_charge = input.fixture_reserved_resources();
    drop(input);
    let session = owner.designer_copy().unwrap();
    let session_charge = session.fixture_reserved_resources();
    drop(session);
    let expected = RetainedResources {
        payload_bytes: input_charge.payload_bytes + session_charge.payload_bytes,
        ordered_entries: input_charge.ordered_entries + session_charge.ordered_entries,
        shared_roots: input_charge.shared_roots + session_charge.shared_roots,
    };
    let held = RefCell::new(None);
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        held.replace(Some(task));
        Ok(())
    })
    .unwrap();
    let total = budget.usage();
    assert_eq!(
        total,
        RetainedResources {
            payload_bytes: before.payload_bytes + expected.payload_bytes,
            ordered_entries: before.ordered_entries + expected.ordered_entries,
            shared_roots: before.shared_roots + expected.shared_roots,
        }
    );
    let Some(State::Working { cancel, .. }) = &owner.designer_export else {
        panic!("Actual pending export worker");
    };
    cancel.store(true, Ordering::Relaxed);
    assert_eq!(budget.usage(), total);
    drop(owner);
    assert_eq!(budget.usage(), expected);
    // Execute the genuine cancelled operation on a native thread after its GUI
    // and terminal receiver are gone; failed delivery must drop both inputs.
    std::thread::spawn(held.into_inner().unwrap())
        .join()
        .unwrap();
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn pressure(usage: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ..Default::default()
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: usize::MAX - usage.ordered_entries,
            ..Default::default()
        },
        Dimension::Roots => RetainedResources {
            shared_roots: usize::MAX - usage.shared_roots,
            ..Default::default()
        },
    }
}

#[test]
fn prior_ready_survives_real_input_admission_parse_and_prepare_or_save_spawn_failure() {
    let (mut owner, document) = prepared_owner();
    let budget = owner.context.application.budget().clone();
    let original = Arc::clone(ready(&owner));
    let frames = serde_json::to_value(&original.value().frames).unwrap();
    let before = budget.usage();
    assert!(
        lifecycle::prepare(&mut owner, "invalid JSON", SELECTION, |_task| panic!(
            "Invalid input cannot spawn"
        ))
        .is_err()
    );
    assert!(Arc::ptr_eq(&original, ready(&owner)));
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let reservation = budget
            .reserve_resources(pressure(before, dimension))
            .unwrap();
        let full = budget.usage();
        assert!(
            lifecycle::prepare(&mut owner, &document, SELECTION, |_task| panic!(
                "Rejected copy cannot spawn"
            ))
            .is_err()
        );
        assert!(Arc::ptr_eq(&original, ready(&owner)));
        assert_eq!(budget.usage(), full);
        drop(reservation);
        assert_eq!(budget.usage(), before);
    }
    let fail = |_task: Box<dyn FnOnce() + Send>| {
        Err(io::Error::new(
            io::ErrorKind::WouldBlock,
            "OS spawn fixture",
        ))
    };
    assert!(lifecycle::prepare(&mut owner, &document, SELECTION, fail)
        .unwrap_err()
        .contains("OS spawn fixture"));
    assert!(Arc::ptr_eq(&original, ready(&owner)));
    assert_eq!(budget.usage(), before);
    assert!(lifecycle::save(&mut owner, ExportFormat::Json, fail)
        .unwrap_err()
        .contains("OS spawn fixture"));
    assert!(Arc::ptr_eq(&original, ready(&owner)));
    assert_eq!(
        serde_json::to_value(&ready(&owner).value().frames).unwrap(),
        frames
    );
    assert_eq!(budget.usage(), before);
    drop(owner);
    assert_eq!(budget.usage(), original.fixture_reserved_resources());
    drop(original);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn late_product_admission_rejection_restores_prior_ready_in_every_positive_dimension() {
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let (mut owner, document) = prepared_owner();
        let budget = owner.context.application.budget().clone();
        let original = Arc::clone(ready(&owner));
        let before = budget.usage();
        let reservation = RefCell::new(None);
        lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
            // Session input already admitted; saturate only before actual product production.
            *reservation.borrow_mut() = Some(
                budget
                    .reserve_resources(pressure(budget.usage(), dimension))
                    .unwrap(),
            );
            task();
            Ok(())
        })
        .unwrap();
        let result = lifecycle::poll(&mut owner.designer_export);
        assert!(result["error"]
            .as_str()
            .unwrap()
            .contains("retained-data limit"));
        assert!(Arc::ptr_eq(&original, ready(&owner)));
        drop(reservation.into_inner());
        assert_eq!(budget.usage(), before);
        drop(owner);
        assert_eq!(budget.usage(), original.fixture_reserved_resources());
        drop(original);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn queued_success_cancel_disconnect_and_final_aliases_keep_actual_product_guards() {
    let (mut owner, document) = prepared_owner();
    let budget = owner.context.application.budget().clone();
    let old = Arc::clone(ready(&owner));
    let before = budget.usage();
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        task();
        Ok(())
    })
    .unwrap();
    assert!(budget.usage().payload_bytes > before.payload_bytes);
    let Some(State::Working { cancel, .. }) = &owner.designer_export else {
        panic!("Queued actual product");
    };
    cancel.store(true, Ordering::Relaxed);
    assert_eq!(
        lifecycle::poll(&mut owner.designer_export)["error"],
        "PNM export cancelled."
    );
    assert!(Arc::ptr_eq(&old, ready(&owner)));
    assert_eq!(budget.usage(), before);
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        drop(task);
        Ok(())
    })
    .unwrap();
    assert_eq!(
        lifecycle::poll(&mut owner.designer_export)["error"],
        "Export worker ended unexpectedly."
    );
    assert!(Arc::ptr_eq(&old, ready(&owner)));
    assert_eq!(budget.usage(), before);
    lifecycle::prepare(&mut owner, &document, SELECTION, |task| {
        task();
        Ok(())
    })
    .unwrap();
    assert_eq!(
        lifecycle::poll(&mut owner.designer_export)["can_json"],
        true
    );
    let new = Arc::clone(ready(&owner));
    assert!(!Arc::ptr_eq(&old, &new));
    assert_eq!(
        serde_json::to_value(&old.value().frames).unwrap(),
        serde_json::to_value(&new.value().frames).unwrap()
    );
    let old_charge = old.fixture_reserved_resources();
    let new_charge = new.fixture_reserved_resources();
    drop(owner);
    assert_eq!(
        budget.usage().payload_bytes,
        old_charge.payload_bytes + new_charge.payload_bytes
    );
    drop(new);
    assert_eq!(budget.usage(), old_charge);
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
}
