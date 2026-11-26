/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual project product capacities, guarded transfer and rejected adoption.
use super::*;
use crate::{
    downstream::workbench::session,
    platform::startup::AdmittedSource,
    shell::{ApplicationSessions, Direction},
};
use std::{mem::size_of, sync::mpsc, time::Duration};

// Independent host ceilings and scheduling watchdog; no scientific byte estimates.
const LIMITS: RetainedResources = RetainedResources {
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
const WATCHDOG: Duration = Duration::from_secs(60);
const POLL_INTERVAL: Duration = Duration::from_millis(1);
const FIXTURE_SIGNAL_CAPACITY: usize = 1;

// Deliberate spare requested storage, with distinct byte/slot purposes.
const PATH_SPARE_BYTES: usize = 97;
const JSON_TEXT_SPARE_BYTES: usize = 61;
const JSON_KEY_SPARE_BYTES: usize = 37;
const JSON_ARRAY_SPARE_SLOTS: usize = 5;
const PROJECT: &[u8] =
    include_bytes!("../../../../../../examples/workbench/downstream/single/workbench.merlin.json");

#[derive(Clone, Copy)]
enum MetadataDimension {
    Bytes,
    Entries,
}

#[derive(Clone, Copy)]
enum ProductMismatch {
    Path,
    Metadata,
    Fixed,
    Stale,
}

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap()
}
fn path() -> PathBuf {
    let mut path = PathBuf::with_capacity(PATH_SPARE_BYTES);
    path.push("retained-project.merlin.json");
    path
}
fn metadata() -> (Value, RetainedResources) {
    let mut text = String::with_capacity(JSON_TEXT_SPARE_BYTES);
    text.push_str("original coordinates");
    let mut key = String::with_capacity(JSON_KEY_SPARE_BYTES);
    key.push_str("samples");
    let mut values = Vec::with_capacity(JSON_ARRAY_SPARE_SLOTS);
    values.push(Value::Null);
    let text_capacity = text.capacity();
    values.push(Value::String(text));
    let buffer = values.capacity() * size_of::<Value>();
    let key_capacity = key.capacity();
    let mut object = serde_json::Map::new();
    object.insert(key, Value::Array(values));
    let resources = RetainedResources {
        payload_bytes: size_of::<Retained<Value>>()
            + object.len() * size_of::<(String, Value)>()
            + key_capacity
            + buffer
            + text_capacity,
        ordered_entries: object.len(),
        ..Default::default()
    };
    (Value::Object(object), resources)
}
fn candidate(context: &Context, path: Option<PathBuf>) -> PreparedProject {
    PreparedProject::prepare(
        serde_json::from_slice(PROJECT).unwrap(),
        path,
        &context.application,
        context.application.snapshot(Direction::Downstream).key,
        session::snapshot(context),
    )
    .unwrap()
}
fn difference(value: RetainedResources, before: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: value
            .payload_bytes
            .checked_sub(before.payload_bytes)
            .unwrap(),
        ordered_entries: value
            .ordered_entries
            .checked_sub(before.ordered_entries)
            .unwrap(),
        shared_roots: value.shared_roots.checked_sub(before.shared_roots).unwrap(),
        channel_count: value
            .channel_count
            .checked_sub(before.channel_count)
            .unwrap(),
        slot_count: value.slot_count.checked_sub(before.slot_count).unwrap(),
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    }
}
fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !ready() {
        assert!(
            Instant::now() < deadline,
            "Project product disposal watchdog"
        );
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn actual_path_and_json_spare_storage_are_passive_and_count_pointees_once() {
    let (value, expected) = metadata();
    let original = serde_json::to_string(&value).unwrap();
    let exact = RetentionBudget::with_limits(expected);
    let owner = admit_metadata(value, &exact).unwrap();
    assert_eq!(owner.fixture_reserved_resources(), expected);
    assert_eq!(exact.usage(), expected);
    assert_eq!(serde_json::to_string(owner.value()).unwrap(), original);
    assert_eq!(owner.value()["samples"][0], Value::Null);
    assert_eq!(owner.value()["samples"][1], "original coordinates");
    drop(owner);
    assert_eq!(exact.usage(), RetainedResources::default());

    let path = path();
    let capacity = path.capacity();
    let expected = RetainedResources {
        payload_bytes: size_of::<Retained<PathBuf>>() + capacity,
        ..Default::default()
    };
    let exact = RetentionBudget::with_limits(expected);
    let owner = project_path::admit(path, &exact).unwrap();
    assert_eq!(owner.fixture_reserved_resources(), expected);
    assert_eq!(owner.value().capacity(), capacity);
    assert_eq!(
        owner.value(),
        &PathBuf::from("retained-project.merlin.json")
    );
    assert_eq!(exact.usage(), expected);
    drop(owner);
    assert_eq!(exact.usage(), RetainedResources::default());
}

#[test]
fn metadata_bytes_entries_and_raw_recovery_path_reject_before_target_mutation() {
    let context = context();
    session::update(&context, |_, sections| {
        sections.insert("source".into(), json!({"accepted":"preserve"}));
        Ok(())
    })
    .unwrap();
    let budget = context.application.budget();
    let baseline = budget.usage();
    let before = context.application.snapshot(Direction::Downstream);
    for dimension in [MetadataDimension::Bytes, MetadataDimension::Entries] {
        let (value, actual) = metadata();
        let pressure = match dimension {
            MetadataDimension::Bytes => RetainedResources {
                payload_bytes: LIMITS.payload_bytes
                    - baseline.payload_bytes
                    - (actual.payload_bytes - 1),
                ..Default::default()
            },
            MetadataDimension::Entries => RetainedResources {
                ordered_entries: LIMITS.ordered_entries
                    - baseline.ordered_entries
                    - (actual.ordered_entries - 1),
                ..Default::default()
            },
        };
        let pressure = budget.reserve_resources(pressure).unwrap();
        let full = budget.usage();
        // Each rejection uses this same actual candidate, without cloning buffers.
        assert!(admit_metadata(value, budget).is_err());
        assert_eq!(budget.usage(), full);
        assert_eq!(context.application.snapshot(Direction::Downstream), before);
        assert_eq!(session::saved(&context, "source")["accepted"], "preserve");
        drop(pressure);
        assert_eq!(budget.usage(), baseline);
    }
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let full = budget.usage();
    let mut loader = Loader::default();
    assert!(loader.start(&context, Some(path()))["error"]
        .as_str()
        .unwrap()
        .contains("retained-data limit"));
    assert!(!loader.is_loading());
    assert_eq!(budget.usage(), full);
    assert_eq!(context.application.snapshot(Direction::Downstream), before);
    drop(pressure);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn foreign_product_guards_and_stale_identity_reject_before_accepted_state_changes() {
    for mismatch in [
        ProductMismatch::Path,
        ProductMismatch::Metadata,
        ProductMismatch::Fixed,
        ProductMismatch::Stale,
    ] {
        let context = context();
        session::update(&context, |_, sections| {
            sections.insert("source".into(), json!({"accepted":"preserve"}));
            Ok(())
        })
        .unwrap();
        let baseline = context.application.budget().usage();
        let mut prepared = candidate(&context, Some(path()));
        let foreign = RetentionBudget::with_limits(LIMITS);
        match mismatch {
            ProductMismatch::Path => {
                prepared.path = Some(project_path::admit(path(), &foreign).unwrap());
            }
            ProductMismatch::Metadata => {
                prepared.metadata = admit_metadata(metadata().0, &foreign).unwrap();
            }
            ProductMismatch::Fixed => {
                prepared._fixed_guard = foreign
                    .reserve_resources(RetainedResources {
                        payload_bytes: size_of::<PreparedProject>(),
                        ..Default::default()
                    })
                    .unwrap()
                    .retain(());
            }
            ProductMismatch::Stale => {
                context.application.configure_downstream(false).unwrap();
            }
        }
        let target = context.application.snapshot(Direction::Downstream);
        let sections = session::saved_sections(&context);
        assert!(prepared.adopt(&context).is_err());
        assert_eq!(context.application.snapshot(Direction::Downstream), target);
        assert_eq!(&*session::saved_sections(&context), &*sections);
        assert!(context.prepared.borrow().source.is_none());
        assert_eq!(context.application.budget().usage(), baseline);
        assert_eq!(foreign.usage(), RetainedResources::default());
    }
}

#[test]
fn opened_products_move_under_pressure_and_new_flags_are_final_before_admission() {
    let context = context();
    let budget = context.application.budget();
    let prepared = candidate(&context, Some(path()));
    assert_eq!(
        prepared._fixed_guard.fixture_reserved_resources(),
        RetainedResources {
            payload_bytes: size_of::<PreparedProject>(),
            ..Default::default()
        }
    );
    let path_pointer = prepared.path.as_ref().unwrap().value() as *const PathBuf;
    let metadata_pointer = prepared.metadata.value() as *const Value;
    let metadata_before = serde_json::to_string(prepared.metadata.value()).unwrap();
    let adopted = prepared.adopt(&context).unwrap();
    let usage = budget.usage();
    let pressure = budget.reserve_resources(difference(LIMITS, usage)).unwrap();
    let AdoptedProject { path, metadata } = adopted;
    assert_eq!(budget.usage(), LIMITS);
    assert_eq!(
        path.as_ref().unwrap().value() as *const PathBuf,
        path_pointer
    );
    assert_eq!(metadata.value() as *const Value, metadata_pointer);
    assert_eq!(
        serde_json::to_string(metadata.value()).unwrap(),
        metadata_before
    );
    assert!(budget.owns(path.as_ref().unwrap()));
    assert!(budget.owns(&metadata));
    drop(pressure);
    assert_eq!(budget.usage(), usage);
    drop(path);
    drop(metadata);

    let blank = PreparedProject::blank(&context).unwrap();
    assert_eq!(blank.metadata.value()["created"], true);
    assert_eq!(blank.metadata.value()["opened"], false);
    assert_eq!(blank.metadata.value()["dark"], Value::Null);
    assert_eq!(blank.metadata.value()["path"], Value::Null);
    let mut actual =
        crate::downstream::inputs::retention::json_children(blank.metadata.value()).unwrap();
    actual.payload_bytes += size_of::<Retained<Value>>();
    assert_eq!(blank.metadata.fixture_reserved_resources(), actual);
    let before = serde_json::to_string(blank.metadata.value()).unwrap();
    let adopted = blank.adopt(&context).unwrap();
    assert!(adopted.path.is_none());
    assert_eq!(
        serde_json::to_string(adopted.metadata.value()).unwrap(),
        before
    );
}

#[test]
fn received_project_stays_charged_after_loader_cancel_until_final_delivery_drop() {
    let context = context();
    let budget = context.application.budget().clone();
    let application = context.application.clone();
    let target = application.snapshot(Direction::Downstream).key;
    let expected = session::snapshot(&context);
    let baseline = budget.usage();
    let worker_budget = budget.clone();
    // Fixture-only synchronization reports the actual product delta; production
    // delivery still uses the existing admitted bounded channel and frame.
    let (measured, measurement) = mpsc::sync_channel(FIXTURE_SIGNAL_CAPACITY);
    let mut loader = Loader::default();
    assert_eq!(
        loader.start_with(&budget, move |_, _| {
            let before = worker_budget.usage();
            let prepared = PreparedProject::prepare(
                serde_json::from_slice(PROJECT).unwrap(),
                Some(path()),
                &application,
                target,
                expected,
            )?;
            let actual = difference(worker_budget.usage(), before);
            measured.send(actual).unwrap();
            Ok(Some(prepared))
        })["loading"],
        true
    );
    let delivery = loader
        .job
        .as_ref()
        .unwrap()
        .receiver
        .recv_timeout(WATCHDOG)
        .unwrap();
    let product = measurement.recv_timeout(WATCHDOG).unwrap();
    let frame = delivery.fixture_reserved_resources();
    let candidate = delivery.value().as_ref().unwrap().as_ref().unwrap();
    let super::LoadCandidate::Downstream(prepared) = candidate else {
        panic!("The legacy fixture must preserve its downstream candidate.");
    };
    assert!(budget.owns(prepared.path.as_ref().unwrap()));
    assert!(budget.owns(&prepared.metadata));
    assert!(budget.owns(&prepared._fixed_guard));
    loader.cancel();
    drop(loader);
    let remaining = baseline
        .checked_add(product)
        .unwrap()
        .checked_add(frame)
        .unwrap();
    wait_for(|| budget.usage() == remaining);
    assert_eq!(budget.usage(), remaining);
    assert_eq!(
        context.application.snapshot(Direction::Downstream).key,
        target
    );
    assert!(session::saved_sections(&context).is_empty());
    drop(delivery);
    wait_for(|| budget.usage() == baseline);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn requested_file_moves_original_guard_through_loader_and_adoption_without_capacity_clone() {
    let context = context();
    let budget = context.application.budget().clone();
    let mut original = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/workbench/downstream/single/workbench.merlin.json");
    original.reserve(PATH_SPARE_BYTES);
    let capacity = original.capacity();
    let path = project_path::admit(original, &budget).unwrap();
    let pointer = path.value() as *const PathBuf;
    let mut loader = Loader::default();
    assert_eq!(
        loader.start_requested(&context, AdmittedSource::File(path))["loading"],
        true
    );
    let (status, prepared) = super::tests::drain(&mut loader);
    let prepared = prepared.expect("Real file must prepare");
    assert_eq!(status["loading"], false);
    assert_eq!(
        prepared.path.as_ref().unwrap().value() as *const PathBuf,
        pointer
    );
    assert_eq!(prepared.path.as_ref().unwrap().value().capacity(), capacity);
    let adopted = prepared.adopt(&context).unwrap();
    assert_eq!(
        adopted.path.as_ref().unwrap().value() as *const PathBuf,
        pointer
    );
    assert_eq!(adopted.path.as_ref().unwrap().value().capacity(), capacity);
    drop(adopted);
    drop(loader);
    drop(context);
    wait_for(|| budget.usage() == ApplicationSessions::fixture_counter_resources());
    assert_eq!(
        budget.usage(),
        ApplicationSessions::fixture_counter_resources()
    );
}
