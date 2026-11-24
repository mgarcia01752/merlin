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

fn prepared_owner() -> (MeasurementInputsRust, String) {
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

fn ready(owner: &MeasurementInputsRust) -> &SharedPrepared {
    let Some(State::Ready(prepared)) = &owner.designer_export else {
        panic!("Existing actual Ready product");
    };
    prepared
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
