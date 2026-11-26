/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Retained measurement originals/options and exact queue-to-adapter transfer.
use super::*;
use crate::downstream::workbench::{
    project_file::Project,
    session::{self, Context},
};
use crate::shell::{ApplicationSessions, RetainedResources};

const PROJECT: &[u8] =
    include_bytes!("../../../../../examples/workbench/downstream/single/workbench.merlin.json");
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};

fn new_owner() -> MeasurementInputsRust {
    let context =
        Context::for_application(ApplicationSessions::new(TEST_MAXIMUM).unwrap()).unwrap();
    MeasurementInputsRust {
        context,
        ..Default::default()
    }
}
fn prepared() -> (crate::downstream::workbench::session::Inputs, Model) {
    let project: Project = serde_json::from_slice(PROJECT).unwrap();
    let (inputs, editors) = project.prepare().unwrap();
    (inputs, editors.measurements)
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Actual remaining capacity in independent aggregate dimensions.
    match dimension {
        0 => RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ..Default::default()
        },
        1 => RetainedResources {
            ordered_entries: usize::MAX - used.ordered_entries,
            ..Default::default()
        },
        _ => RetainedResources {
            shared_roots: usize::MAX - used.shared_roots,
            ..Default::default()
        },
    }
}

#[test]
fn untouched_measurement_defaults_borrow_static_text_and_preserve_serialized_contract() {
    let owner = MeasurementInputsRust::default();
    assert!(matches!(
        owner.model.options.interval_symbols,
        std::borrow::Cow::Borrowed(_)
    ));
    assert!(owner.model_guard.is_none());
    assert_eq!(
        owner.model.retained_resources().unwrap(),
        RetainedResources {
            payload_bytes: std::mem::size_of::<Model>(),
            ..Default::default()
        }
    );
    // Historical serialized default is independent of the named production policy.
    let value = serde_json::to_value(&owner.model.options).unwrap();
    assert_eq!(value["interval_symbols"], "1000");
    let parsed: Options = serde_json::from_value(value.clone()).unwrap();
    assert!(matches!(
        parsed.interval_symbols,
        std::borrow::Cow::Owned(_)
    ));
    assert_eq!(serde_json::to_value(parsed).unwrap(), value);
}

#[test]
fn measurement_draft_and_apply_rejection_keep_exact_accepted_originals_and_model_guard() {
    let mut owner = new_owner();
    let budget = owner.context.application.budget().clone();
    let (inputs, model) = prepared();
    let project: Project = serde_json::from_slice(PROJECT).unwrap();
    session::restore(&owner.context, inputs, project.sections).unwrap();
    owner.replace_model(model).unwrap();
    owner
        .apply_measurement(true, owner.model.options.clone(), &project.transmission)
        .unwrap();
    let old = owner.model.export();
    let saved = session::saved(&owner.context, "measurements");
    let usage = budget.usage();
    assert!(owner.model_guard.is_some());
    for dimension in 0..3 {
        // Only positive actual model dimensions require model-level rejection.
        let guard = budget
            .reserve_resources(pressure(usage, dimension))
            .unwrap();
        let mut candidate = owner.model.clone();
        candidate.options.interval_symbols.to_mut().push_str("00");
        let resources = candidate.retained_resources().unwrap();
        let positive = [
            resources.payload_bytes,
            resources.ordered_entries,
            resources.shared_roots,
        ];
        if positive[dimension] > 0 {
            assert!(owner.replace_model(candidate).is_err());
        }
        // Complete accepted Inputs/section always includes ordered JSON entries,
        // even when this ordinary binary capture model has no private JSON map.
        assert!(owner
            .apply_measurement(true, owner.model.options.clone(), &project.transmission)
            .is_err());
        assert_eq!(owner.model.export(), old);
        assert_eq!(&*session::saved(&owner.context, "measurements"), &*saved);
        drop(guard);
        assert_eq!(budget.usage(), usage);
    }
    drop(saved);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn admitted_measurement_transfer_survives_full_pressure_without_copy_or_new_charge() {
    let mut owner = new_owner();
    let foreign = new_owner();
    let budget = owner.context.application.budget().clone();
    let (_, model) = prepared();
    assert!(owner
        .adopt_model(
            model
                .clone()
                .admit(foreign.context.application.budget())
                .unwrap()
        )
        .is_err());
    assert!(owner.model.accepted_rx.is_empty());
    let originals = model
        .accepted_rx
        .iter()
        .map(|capture| capture.hash.clone())
        .collect::<Vec<_>>();
    let prepared = model.admit(&budget).unwrap();
    let used = budget.usage();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ordered_entries: usize::MAX - used.ordered_entries,
            shared_roots: usize::MAX - used.shared_roots,
            channel_count: usize::MAX - used.channel_count,
            slot_count: usize::MAX - used.slot_count,
            presentation_utf16_bytes: 0,
            presentation_entries: 0,
            retained_views: 0,
            open_views: 0,
        })
        .unwrap();
    owner.adopt_model(prepared).unwrap();
    assert_eq!(budget.usage(), TEST_MAXIMUM);
    assert_eq!(
        owner
            .model
            .accepted_rx
            .iter()
            .map(|capture| capture.hash.clone())
            .collect::<Vec<_>>(),
        originals
    );
    drop(pressure);
    assert_eq!(budget.usage(), used);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
