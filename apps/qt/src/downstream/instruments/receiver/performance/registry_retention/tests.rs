/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Spare-slot fixture over an actual admitted performance Entry.
use super::super::retention::PreparedEntry;
use super::*;
use crate::shell::RetentionBudget;

// Requested registry capacity is an independent fixture anchor. Every expected
// quantity uses the actual capacity and Entry layout rather than a growth rule.
const REGISTRY_SLOT_CAPACITY: usize = 11;

// Independent captured identities, with no scientific or accounting meaning.
const FIXTURE_PROFILE_ID: u8 = 1;
const FIXTURE_CHANNEL_ID: u8 = 2;

// Actual error provenance is preserved; it belongs to the live Entry's guard.
const FIXTURE_ERROR: &str = "Original finite setup diagnostic.";

// Accounting-only fixture policy; production uses its canonical host policy.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

#[test]
fn spare_slots_exclude_live_layout_and_children_while_the_entry_guard_survives_moves() {
    let budget = RetentionBudget::with_limits(FIXTURE_LIMIT);
    let entry = PreparedEntry::failed(FIXTURE_PROFILE_ID, FIXTURE_CHANNEL_ID, FIXTURE_ERROR.into())
        .admit(&budget)
        .unwrap();
    let entry_charge = budget.usage();
    assert!(entry_charge.payload_bytes > size_of::<Entry>());
    let mut entries = Vec::with_capacity(REGISTRY_SLOT_CAPACITY);
    entries.push(entry);
    let expected = RetainedResources {
        payload_bytes: (entries.capacity() - entries.len()) * size_of::<Entry>(),
        ordered_entries: 0,
        shared_roots: 0,
    };
    assert_eq!(resources(&entries).unwrap(), expected);
    assert_eq!(resources(&entries).unwrap(), expected);
    assert_eq!(budget.usage(), entry_charge);
    assert_eq!(entries[0].id, FIXTURE_PROFILE_ID);
    assert_eq!(entries[0].channel, FIXTURE_CHANNEL_ID);
    assert_eq!(entries[0].status, "failed");
    assert_eq!(entries[0].error.as_deref(), Some(FIXTURE_ERROR));

    // Moving the live Entry out leaves its formerly live slot spare. Its own
    // charge must survive that move independently of the registry's metadata.
    let entry = entries.pop().unwrap();
    let empty = resources(&entries).unwrap();
    assert_eq!(empty.payload_bytes, entries.capacity() * size_of::<Entry>());
    assert_eq!(
        empty.payload_bytes - expected.payload_bytes,
        size_of::<Entry>()
    );
    assert_eq!(empty.ordered_entries, 0);
    assert_eq!(empty.shared_roots, 0);
    assert_eq!(budget.usage(), entry_charge);
    drop(entries);
    assert_eq!(budget.usage(), entry_charge);
    assert_eq!(entry.error.as_deref(), Some(FIXTURE_ERROR));
    drop(entry);
    assert_eq!(budget.usage(), RetainedResources::default());
}
