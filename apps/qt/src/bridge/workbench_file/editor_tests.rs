/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! New must preflight every reset before accepted inputs, paths or GUI reload.
use super::*;
use crate::downstream::workbench::{
    project_load::{self, PreparedProject},
    session::{self, Context},
};
use crate::shell::{ApplicationSessions, Direction, RetainedResources};

const PROJECT: &[u8] =
    include_bytes!("../../../../../examples/workbench/single/workbench.merlin.json");
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn populated() -> WorkbenchFileRust {
    let context =
        Context::for_application(ApplicationSessions::new(TEST_MAXIMUM).unwrap()).unwrap();
    PreparedProject::prepare(
        serde_json::from_slice(PROJECT).unwrap(),
        None,
        &context.application,
        context.application.snapshot(Direction::Downstream).key,
        session::snapshot(&context),
    )
    .unwrap()
    .adopt(&context)
    .unwrap();
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/workbench/single/workbench.merlin.json");
    let url = QUrl::from_local_file(&QString::from(path.to_string_lossy().as_ref()));
    crate::platform::input_file::read_local(&url.to_string(), PROJECT.len()).unwrap();
    WorkbenchFileRust {
        context,
        path: Some(path),
        loader: Default::default(),
    }
}

#[test]
fn rejected_new_with_only_empty_input_capacity_preserves_old_alias_editors_and_save_path() {
    let mut owner = populated();
    let budget = owner.context.application.budget().clone();
    let old = session::snapshot(&owner.context);
    let saved = session::saved_sections(&owner.context);
    let key = owner.context.application.snapshot(Direction::Downstream);
    let path = owner.path.clone().unwrap();
    let original = std::fs::read(&path).unwrap();
    let baseline = budget.usage();
    // This independent actual empty-State charge made the old New commit possible
    // before subsequent positive-byte editor resets rejected with old aliases alive.
    let empty = session::fixture_candidate_resources(&Default::default(), &Default::default());
    let guard = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - baseline.payload_bytes - empty.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    assert!(owner.create_blank().is_err());
    assert_eq!(budget.usage(), full);
    assert_eq!(
        owner.context.application.snapshot(Direction::Downstream),
        key
    );
    assert_eq!(owner.path.as_ref(), Some(&path));
    assert!(crate::platform::input_file::is_original(&path));
    assert_eq!(std::fs::read(&path).unwrap(), original);
    assert_eq!(&*session::saved_sections(&owner.context), &*saved);
    assert_eq!(
        session::snapshot(&owner.context)
            .source
            .as_ref()
            .unwrap()
            .config
            .original_bytes(),
        old.source.as_ref().unwrap().config.original_bytes()
    );
    let pending = owner.context.prepared.borrow();
    assert!(
        pending.source.is_some() && pending.profile.is_some() && pending.measurements.is_some()
    );
    drop(pending);
    drop(guard);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn successful_new_transfers_all_blank_editors_under_full_pressure_and_preserves_theme_choice() {
    let mut owner = populated();
    let budget = owner.context.application.budget().clone();
    let path = owner.path.clone().unwrap();
    let result = owner.create_blank().unwrap();
    assert_eq!(result["created"], true);
    assert_eq!(result["prepared"], true);
    assert!(result["dark"].is_null());
    assert!(owner.path.is_none());
    assert!(!crate::platform::input_file::is_original(&path));
    assert!(session::snapshot(&owner.context).source.is_none());
    assert!(session::saved_sections(&owner.context).is_empty());
    let usage = budget.usage();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ordered_entries: usize::MAX - usage.ordered_entries,
            shared_roots: usize::MAX - usage.shared_roots,
        })
        .unwrap();
    let source = project_load::take(&owner.context, |models| {
        models.source.take().map(|model| *model)
    })
    .unwrap();
    let profile = project_load::take(&owner.context, |models| {
        models.profile.take().map(|model| *model)
    })
    .unwrap();
    let measurements = project_load::take(&owner.context, |models| {
        models.measurements.take().map(|model| *model)
    })
    .unwrap();
    assert!(source.value().snapshot().is_none());
    assert!(profile.value().accepted.is_none());
    assert!(measurements.value().accepted_rx.is_empty());
    assert!(budget.owns(&source) && budget.owns(&profile) && budget.owns(&measurements));
    let source_parts = source.into_parts();
    let profile_parts = profile.into_parts();
    let measurement_parts = measurements.into_parts();
    assert_eq!(budget.usage(), TEST_MAXIMUM);
    drop(pressure);
    assert_eq!(budget.usage(), usage);
    // Retain transferred guards through data destruction, exactly as GUI owners do.
    drop(source_parts);
    drop(profile_parts);
    drop(measurement_parts);
    drop(owner);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
