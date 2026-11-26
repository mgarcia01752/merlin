/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete registry admission, late rejection and actual final data disposal.
use super::*;
use crate::shell::{ApplicationSessions, RetainedResources};
use merlin::downstream::FullLoopControl;
use std::mem::size_of;

// Independent requested capacities; assertions use actual capacities below.
const SLOT_CAPACITY: usize = 7;
const STATUS_CAPACITY: usize = 37;

// Original captured identity and supported one-millisecond scientific fixture.
const PROFILE_ID: u8 = 3;
const OBSERVATION_MS: u64 = 1;
const TEST_LIMIT: RetainedResources = RetainedResources {
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

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(TEST_LIMIT).unwrap()).unwrap()
}

#[test]
fn actual_spare_slots_and_statuses_remain_charged_until_clear_or_final_store_drop() {
    let context = context();
    let budget = context.application.budget().clone();
    let baseline = budget.usage();
    let mut entries = Vec::with_capacity(SLOT_CAPACITY);
    let mut status = String::with_capacity(STATUS_CAPACITY);
    status.push_str("not_started");
    entries.push(Entry {
        id: PROFILE_ID,
        status,
        snapshot: None,
    });
    let expected = RetainedResources {
        payload_bytes: entries.capacity() * size_of::<Entry>() + entries[0].status.capacity(),
        ..Default::default()
    };
    let candidate = Registry::admit(entries, &budget).unwrap();
    let combined = RetainedResources {
        payload_bytes: baseline.payload_bytes + expected.payload_bytes,
        ..baseline
    };
    assert_eq!(budget.usage(), combined);
    *context.qam.borrow_mut() = candidate;
    let alias = context.clone();
    drop(context);
    assert_eq!(budget.usage(), combined);
    assert_eq!(alias.qam.borrow()[0].status, "not_started");
    clear(&alias);
    assert_eq!(budget.usage(), baseline);
    assert_eq!(alias.qam.borrow().entries.capacity(), 0);
    drop(alias);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}

#[test]
fn rejected_catalog_and_late_status_replacement_preserve_exact_old_scientific_alias() {
    let context = context();
    let budget = context.application.budget().clone();
    let mut document: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../../examples/workbench/downstream/single/workbench.merlin.json"
    ))
    .unwrap();
    document["receiver"]["duration_ms"] = json!(OBSERVATION_MS);
    let plan = merlin::downstream::inputs::Workbench::prepare_full_loop(
        &serde_json::to_vec(&document).unwrap(),
    )
    .unwrap();
    let request = plan.request(0).unwrap();
    let result = ResultData::new(
        request
            .run_bounded(
                &FullLoopControl::new(),
                request.result_retained_bound().unwrap(),
            )
            .unwrap(),
    );
    let profile = result
        .job
        .settings
        .playback
        .as_ref()
        .unwrap()
        .profile
        .clone();
    begin(&context, std::slice::from_ref(&profile)).unwrap();
    retain(&context, profile.profile_id, &result).unwrap();
    let old = get(&context, i32::from(profile.profile_id)).unwrap();
    let catalog_before = catalog(&context);
    let science_before = old.backend.retained_footprint().unwrap();
    let source_before = result.backend().retained_footprint().unwrap();
    let before = budget.usage();

    // First reject an initial catalog while the previous scientific alias stays live.
    let full = budget
        .reserve_resources(RetainedResources {
            payload_bytes: TEST_LIMIT.payload_bytes - before.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(begin(&context, std::slice::from_ref(&profile)).is_err());
    assert_eq!(catalog(&context), catalog_before);
    assert!(Arc::ptr_eq(
        &old,
        &get(&context, i32::from(profile.profile_id)).unwrap()
    ));
    drop(full);
    assert_eq!(budget.usage(), before);

    // Permit the complete real archive, leaving zero bytes for its new registry.
    // This reaches the later admission gate after successful scientific copying.
    let policy = FullLoopReceiverArchivePolicy::new(Default::default()).unwrap();
    let target = retention::resources(
        result
            .backend()
            .receiver_archive_retained_bound(&policy)
            .unwrap(),
    )
    .unwrap();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: TEST_LIMIT.payload_bytes - before.payload_bytes - target.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let pressured = budget.usage();
    assert!(retain(&context, profile.profile_id, &result).is_err());
    assert_eq!(budget.usage(), pressured);
    assert_eq!(catalog(&context), catalog_before);
    assert!(Arc::ptr_eq(
        &old,
        &get(&context, i32::from(profile.profile_id)).unwrap()
    ));
    assert_eq!(old.backend.retained_footprint().unwrap(), science_before);
    assert_eq!(
        result.backend().retained_footprint().unwrap(),
        source_before
    );
    drop(pressure);
    assert_eq!(budget.usage(), before);
    clear(&context);
    assert!(get(&context, i32::from(profile.profile_id)).is_err());
    assert_eq!(old.backend.retained_footprint().unwrap(), science_before);
    drop(context);
    assert!(
        budget.used().0
            > crate::shell::ApplicationSessions::fixture_counter_resources().payload_bytes
    );
    drop(old);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
