/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real detached receiver archives, original queries and concrete final-alias accounting.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources, RetentionBudget, RetentionError};
use merlin::{
    common::RetainedFootprint,
    downstream::{FullLoopControl, FullLoopRequest, FullLoopResult, NamedCapture, ProfileCapture},
};
use std::mem::size_of;

// Existing supported small observation fixture; no new engine/scientific limit.
const OBSERVATION_MS: u64 = 1;

// Independent accounting-only fixture ceiling; production always uses the canonical host.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}
fn quantity(resources: RetainedResources, dimension: Dimension) -> usize {
    match dimension {
        Dimension::Payload => resources.payload_bytes,
        Dimension::Entries => resources.ordered_entries,
        Dimension::Roots => resources.shared_roots,
    }
}
fn only(dimension: Dimension, value: usize) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: value,
            ..Default::default()
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: value,
            ..Default::default()
        },
        Dimension::Roots => RetainedResources {
            shared_roots: value,
            ..Default::default()
        },
    }
}
fn positive(resources: RetainedResources) -> impl Iterator<Item = Dimension> {
    [Dimension::Payload, Dimension::Entries, Dimension::Roots]
        .into_iter()
        .filter(move |dimension| quantity(resources, *dimension) > 0)
}
fn plus(a: RetainedResources, b: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: a.payload_bytes.checked_add(b.payload_bytes).unwrap(),
        ordered_entries: a.ordered_entries.checked_add(b.ordered_entries).unwrap(),
        shared_roots: a.shared_roots.checked_add(b.shared_roots).unwrap(),
    }
}
fn one_less(mut resources: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => resources.payload_bytes -= 1,
        Dimension::Entries => resources.ordered_entries -= 1,
        Dimension::Roots => resources.shared_roots -= 1,
    }
    resources
}
fn request() -> FullLoopRequest {
    let mut document: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../../examples/workbench/single/workbench.merlin.json"
    ))
    .unwrap();
    document["receiver"]["duration_ms"] = json!(OBSERVATION_MS);
    merlin::downstream::inputs::Workbench::prepare_full_loop(
        &serde_json::to_vec(&document).unwrap(),
    )
    .unwrap()
    .request(0)
    .unwrap()
}
fn result() -> FullLoopResult {
    let request = request();
    request
        .run_bounded(
            &FullLoopControl::new(),
            request.result_retained_bound().unwrap(),
        )
        .unwrap()
}
fn source_resources(result: &FullLoopResult) -> RetainedResources {
    let metadata = result.retained_footprint().unwrap();
    RetainedResources {
        payload_bytes: metadata
            .payload_bytes
            .checked_sub(size_of::<FullLoopResult>())
            .unwrap()
            .checked_add(size_of::<Retained<FullLoopResult>>())
            .unwrap(),
        ordered_entries: metadata.ordered_entries,
        shared_roots: metadata.shared_roots,
    }
}
fn policy() -> FullLoopReceiverArchivePolicy {
    FullLoopReceiverArchivePolicy::new(Default::default()).unwrap()
}
fn profile() -> SourceProfileFrame {
    let capture = NamedCapture::new(
        "original-profile.json",
        include_bytes!(
            "../../../../../../../examples/measurements/single-channel/modulation_profile.json"
        )
        .to_vec(),
    )
    .unwrap();
    let catalog = ProfileCapture::from_bytes(&capture).unwrap();
    catalog.frame(catalog.profile_ids()[0]).unwrap().clone()
}
fn context() -> Context {
    Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
}
fn qam(archive: &FullLoopReceiverArchive) -> Result<Value, String> {
    archive
        .qam(Default::default())
        .map(|view| serde_json::to_value(view).unwrap())
        .map_err(|error| error.to_string())
}
fn decoder(archive: &FullLoopReceiverArchive) -> Result<Value, String> {
    archive
        .decoder_soft_view()
        .map(|view| serde_json::to_value(view).unwrap())
        .map_err(|error| error.to_string())
}

#[test]
fn snapshot_layout_replaces_the_scientific_handle_once_and_checks_each_overflow() {
    let footprint = RetainedFootprint {
        payload_bytes: size_of::<FullLoopReceiverArchive>(),
        ordered_entries: 0,
        shared_roots: 1,
    }; // Actual backend detached-root shape.
    assert_eq!(
        retention::resources(footprint).unwrap(),
        RetainedResources {
            payload_bytes: size_of::<Snapshot>(),
            ordered_entries: 0,
            shared_roots: 2, // Backend detached Arc plus the application Snapshot Arc.
        }
    );
    for footprint in [
        RetainedFootprint::default(),
        RetainedFootprint {
            payload_bytes: usize::MAX,
            ..footprint
        },
        RetainedFootprint {
            shared_roots: usize::MAX,
            ..footprint
        },
    ] {
        assert!(matches!(
            retention::resources(footprint),
            Err(RetentionError::ArithmeticOverflow)
        ));
    }
}

#[test]
fn exact_source_target_overlap_preserves_legacy_queries_and_archive_is_independent_after_source_drop(
) {
    let result = result();
    let before = result.retained_footprint().unwrap();
    let report = serde_json::to_value(result.report()).unwrap();
    let source_charge = source_resources(&result);
    let limit = result.receiver_archive_retained_bound(&policy()).unwrap();
    let target = retention::resources(limit).unwrap();
    let budget = RetentionBudget::with_limits(plus(source_charge, target));
    let source = budget
        .reserve_resources(source_charge)
        .unwrap()
        .retain(result);
    let snapshot = retention::snapshot(0, source.value(), policy(), &budget).unwrap();
    assert_eq!(budget.usage(), plus(source_charge, target));
    assert_eq!(snapshot.backend.retained_footprint().unwrap(), limit);
    assert_eq!(target.ordered_entries, 0, "No invented ordered-node charge");
    let legacy = source.value().receiver_archive(policy()).unwrap();
    assert_eq!(
        serde_json::to_value(snapshot.backend.info()).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    assert_eq!(qam(&snapshot.backend), qam(&legacy));
    assert_eq!(decoder(&snapshot.backend), decoder(&legacy));
    assert_eq!(source.value().retained_footprint().unwrap(), before);
    assert_eq!(
        serde_json::to_value(source.value().report()).unwrap(),
        report
    );
    drop(source);
    assert_eq!(budget.usage(), target);
    assert_eq!(qam(&snapshot.backend), qam(&legacy));
    let old = Arc::clone(&snapshot);
    drop(snapshot);
    assert_eq!(budget.usage(), target);
    drop(old);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn each_positive_target_dimension_rejects_before_copy_without_changing_the_same_source() {
    for dimension in [Dimension::Payload, Dimension::Roots] {
        let result = result();
        let source_charge = source_resources(&result);
        let before = result.retained_footprint().unwrap();
        let target =
            retention::resources(result.receiver_archive_retained_bound(&policy()).unwrap())
                .unwrap();
        assert!(quantity(target, dimension) > 0);
        assert_eq!(target.ordered_entries, 0);
        let budget = RetentionBudget::with_limits(one_less(plus(source_charge, target), dimension));
        let source = budget
            .reserve_resources(source_charge)
            .unwrap()
            .retain(result);
        assert!(matches!(
            retention::snapshot(0, source.value(), policy(), &budget),
            Err(retention::ArchiveError::Retention(_))
        ));
        assert_eq!(budget.usage(), source_charge);
        assert_eq!(source.value().retained_footprint().unwrap(), before);
        assert_eq!(source.value().report().summary.comparison.bit_errors, 0);
    }
}

#[test]
fn registry_replacement_clear_and_context_drop_keep_selected_old_aliases_until_final_drop() {
    let context = context();
    let budget = context.application.budget().clone();
    let metadata = budget.usage();
    let profile = profile();
    let id = profile.profile_id;
    let result = ResultData::new(result());
    begin(&context, std::slice::from_ref(&profile)).unwrap();
    retain(&context, id, &result).unwrap();
    let old = get(&context, i32::from(id)).unwrap();
    let target = retention::resources(
        result
            .backend()
            .receiver_archive_retained_bound(&policy())
            .unwrap(),
    )
    .unwrap();
    let registry_charge = context
        .qam
        .with(|registry| registry_retention::resources(&registry.borrow().entries).unwrap());
    assert_eq!(
        budget.usage(),
        plus(metadata, plus(target, registry_charge))
    );
    retain(&context, id, &result).unwrap();
    let current = get(&context, i32::from(id)).unwrap();
    assert!(!Arc::ptr_eq(&old, &current));
    let registry_charge = context
        .qam
        .with(|registry| registry_retention::resources(&registry.borrow().entries).unwrap());
    assert_eq!(
        budget.usage(),
        plus(metadata, plus(plus(target, target), registry_charge))
    );
    let old_values = qam(&old.backend);
    clear(&context);
    assert!(get(&context, i32::from(id)).is_err());
    assert!(get(&context, i32::from(id) + 1).is_err()); // Distinct missing source identity.
    assert_eq!(qam(&old.backend), old_values);
    assert_eq!(budget.usage(), plus(metadata, plus(target, target)));
    drop(current);
    assert_eq!(budget.usage(), plus(metadata, target));
    drop(context);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(target)
            .unwrap()
    );
    drop(old);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn host_rejection_keeps_the_same_accepted_entry_status_and_scientific_history() {
    let context = context();
    let budget = context.application.budget().clone();
    let profile = profile();
    let id = profile.profile_id;
    let result = ResultData::new(result());
    begin(&context, std::slice::from_ref(&profile)).unwrap();
    retain(&context, id, &result).unwrap();
    let old = get(&context, i32::from(id)).unwrap();
    let before = budget.usage();
    let catalog_before = catalog(&context);
    let source_before = result.backend().retained_footprint().unwrap();
    let target = retention::resources(
        result
            .backend()
            .receiver_archive_retained_bound(&policy())
            .unwrap(),
    )
    .unwrap();
    for dimension in positive(target) {
        // Explicit accounting-only synthetic pressure. No backend target or scientific
        // population is estimated, reduced or replaced by this fixture reservation.
        let pressure = budget
            .reserve_resources(only(
                dimension,
                quantity(FIXTURE_LIMIT, dimension) - quantity(before, dimension),
            ))
            .unwrap();
        assert!(retain(&context, id, &result).is_err());
        assert!(Arc::ptr_eq(&old, &get(&context, i32::from(id)).unwrap()));
        assert_eq!(catalog(&context), catalog_before);
        assert_eq!(
            result.backend().retained_footprint().unwrap(),
            source_before
        );
        drop(pressure);
        assert_eq!(budget.usage(), before);
    }
}

#[test]
fn real_precancelled_empty_archive_preserves_absence_and_original_legacy_errors() {
    let request = request();
    let control = FullLoopControl::new();
    control.cancel();
    let result = request
        .run_bounded(&control, request.result_retained_bound().unwrap())
        .unwrap();
    let before = result.retained_footprint().unwrap();
    let legacy = result.receiver_archive(policy()).unwrap();
    let limit = result.receiver_archive_retained_bound(&policy()).unwrap();
    let target = retention::resources(limit).unwrap();
    let budget = RetentionBudget::with_limits(target);
    let snapshot = retention::snapshot(0, &result, policy(), &budget).unwrap();
    assert_eq!(
        serde_json::to_value(snapshot.backend.info()).unwrap(),
        serde_json::to_value(legacy.info()).unwrap()
    );
    assert_eq!(qam(&snapshot.backend), qam(&legacy));
    assert_eq!(decoder(&snapshot.backend), decoder(&legacy));
    assert_eq!(snapshot.backend.retained_footprint().unwrap(), limit);
    assert_eq!(result.retained_footprint().unwrap(), before);
    drop(snapshot);
    assert_eq!(budget.usage(), RetainedResources::default());
}
