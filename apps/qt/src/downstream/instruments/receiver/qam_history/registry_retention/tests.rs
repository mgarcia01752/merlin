/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Registry storage and alias fixtures, independent of aggregate SESSION acceptance.
use super::super::retention;
use super::*;
use crate::shell::RetentionBudget;
use merlin::downstream::{FullLoopControl, FullLoopReceiverArchivePolicy};
use serde_json::{json, Value};

// Independent requested-capacity anchors; assertions use actual capacities,
// never a Vec/String growth rule or a production retained-data policy.
const REGISTRY_SLOT_CAPACITY: usize = 7;
const STATUS_CAPACITY_BYTES: usize = 37;
const ADDITIONAL_STATUS_BYTES: usize = 53;

// Existing supported small observation fixture, not a new scientific limit.
const OBSERVATION_MS: u64 = 1;

// Independent original profile identity, unrelated to wire coding or capacity.
const FIXTURE_PROFILE_ID: u8 = 3;
const FIXTURE_UNSTARTED_PROFILE_ID: u8 = 4;

// Accounting-only fixture policy; the production caller supplies its host policy.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
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

#[test]
fn all_slots_and_status_capacity_are_passive_without_recharging_snapshot_aliases() {
    let mut document: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../../../examples/workbench/downstream/single/workbench.merlin.json"
    ))
    .unwrap();
    document["receiver"]["duration_ms"] = json!(OBSERVATION_MS);
    let request = merlin::downstream::inputs::Workbench::prepare_full_loop(
        &serde_json::to_vec(&document).unwrap(),
    )
    .unwrap()
    .request(0)
    .unwrap();
    let result = request
        .run_bounded(
            &FullLoopControl::new(),
            request.result_retained_bound().unwrap(),
        )
        .unwrap();
    let source_metadata = result.retained_footprint().unwrap();
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let policy = FullLoopReceiverArchivePolicy::new(Default::default()).unwrap();
    let snapshot = retention::snapshot(FIXTURE_PROFILE_ID, &result, policy, &budget).unwrap();
    let snapshot_charge = budget.usage();
    assert!(snapshot_charge.payload_bytes > 0 && snapshot_charge.shared_roots > 0);
    let snapshot_metadata = snapshot.backend.retained_footprint().unwrap();
    let old_alias = Arc::clone(&snapshot);
    let mut entries = Vec::with_capacity(REGISTRY_SLOT_CAPACITY);
    let mut status = String::with_capacity(STATUS_CAPACITY_BYTES);
    status.push_str("complete");
    entries.push(Entry {
        id: FIXTURE_PROFILE_ID,
        status,
        snapshot: Some(Arc::clone(&snapshot)),
    });
    let mut unstarted_status = String::with_capacity(STATUS_CAPACITY_BYTES);
    unstarted_status.push_str("not_started");
    entries.push(Entry {
        id: FIXTURE_UNSTARTED_PROFILE_ID,
        status: unstarted_status,
        snapshot: None,
    });
    let expected = RetainedResources {
        payload_bytes: entries.capacity() * size_of::<Entry>()
            + entries[0].status.capacity()
            + entries[1].status.capacity(),
        ordered_entries: 0,
        shared_roots: 0,
        ..Default::default()
    };
    let alias_count = Arc::strong_count(&snapshot);
    assert_eq!(resources(&entries).unwrap(), expected);
    assert_eq!(resources(&entries).unwrap(), expected);
    assert_eq!(Arc::strong_count(&snapshot), alias_count);
    assert!(Arc::ptr_eq(
        entries[0].snapshot.as_ref().unwrap(),
        &snapshot
    ));
    assert_eq!(entries[0].id, FIXTURE_PROFILE_ID);
    assert_eq!(entries[0].status, "complete");
    assert_eq!(entries[1].id, FIXTURE_UNSTARTED_PROFILE_ID);
    assert_eq!(entries[1].status, "not_started");
    assert!(entries[1].snapshot.is_none());
    assert_eq!(budget.usage(), snapshot_charge);
    assert_eq!(
        snapshot.backend.retained_footprint().unwrap(),
        snapshot_metadata
    );
    assert_eq!(result.retained_footprint().unwrap(), source_metadata);

    let previous_status_capacity = entries[0].status.capacity();
    entries[0].status.reserve_exact(
        previous_status_capacity
            .checked_add(ADDITIONAL_STATUS_BYTES)
            .unwrap(),
    );
    let capacity_delta = entries[0].status.capacity() - previous_status_capacity;
    assert!(capacity_delta > 0);
    assert_eq!(
        resources(&entries).unwrap().payload_bytes,
        expected.payload_bytes + capacity_delta
    );
    entries.clear();
    assert_eq!(
        resources(&entries).unwrap(),
        RetainedResources {
            payload_bytes: entries.capacity() * size_of::<Entry>(),
            ordered_entries: 0,
            shared_roots: 0,
            ..Default::default()
        }
    );
    drop(entries);
    drop(snapshot);
    assert_eq!(budget.usage(), snapshot_charge);
    assert_eq!(
        old_alias.backend.retained_footprint().unwrap(),
        snapshot_metadata
    );
    drop(old_alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}
