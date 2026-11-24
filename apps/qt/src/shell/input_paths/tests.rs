/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact registry/record quantities, duplicate flags, cap and final aliases.
use super::*;

// Independent accounting ceilings, never a filesystem/scientific policy.
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

// Deliberate spare requested path storage, independent of path-count policy.
const SPARE_PATH_BYTES: usize = 23;

fn fixed() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<Retained<Mutex<BTreeSet<Record>>>>(),
        ordered_entries: 0,
        shared_roots: 1,
        ..Default::default()
    }
}
fn add(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
        ..Default::default()
    }
}
fn fixture_path() -> PathBuf {
    // Collection-only fixture keys; real filesystem canonicalization is tested
    // independently by the existing platform reader adapter.
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixture-original.bin");
    path.reserve(SPARE_PATH_BYTES);
    path
}
fn record(path: &PathBuf) -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<Record>() + path.capacity(),
        ordered_entries: 1,
        shared_roots: 0,
        ..Default::default()
    }
}

#[test]
fn exact_registry_layout_and_below_byte_or_root_limits_leave_no_partial_owner() {
    let expected = fixed();
    let budget = RetentionBudget::with_limits(expected);
    let owner = InputPaths::new(&budget).unwrap();
    assert_eq!(budget.usage(), expected);
    assert_eq!(owner.records().len(), 0);
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
    for maximum in [
        RetainedResources {
            payload_bytes: expected.payload_bytes - 1,
            ..expected
        },
        RetainedResources {
            shared_roots: 0,
            ..expected
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(InputPaths::new(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn actual_path_capacity_and_entry_reject_one_below_without_losing_prior_records() {
    let path = fixture_path();
    assert!(path.capacity() > path.as_os_str().len());
    let key = path.clone();
    let expected = add(fixed(), record(&path));
    let budget = RetentionBudget::with_limits(expected);
    let owner = InputPaths::new(&budget).unwrap();
    owner
        .register(&budget, Direction::Downstream, path)
        .unwrap();
    assert!(owner.contains(&key));
    assert_eq!(budget.usage(), expected);
    owner
        .register(&budget, Direction::Upstream, key.clone())
        .unwrap();
    assert_eq!(budget.usage(), expected);
    owner.clear(Direction::Downstream);
    assert!(owner.contains(&key));
    assert_eq!(budget.usage(), expected);
    owner.clear(Direction::Upstream);
    assert!(!owner.contains(&key));
    assert_eq!(budget.usage(), fixed());

    for bytes in [true, false] {
        let budget = RetentionBudget::with_limits(LIMITS);
        let owner = InputPaths::new(&budget).unwrap();
        let original = fixture_path();
        let original_key = original.clone();
        owner
            .register(&budget, Direction::Downstream, original)
            .unwrap();
        let before = budget.usage();
        let candidate = original_key.with_file_name("new-original.bin");
        let candidate_key = candidate.clone();
        let demand = record(&candidate);
        let pressure = budget
            .reserve_resources(if bytes {
                RetainedResources {
                    payload_bytes: LIMITS.payload_bytes
                        - before.payload_bytes
                        - demand.payload_bytes
                        + 1,
                    ..Default::default()
                }
            } else {
                RetainedResources {
                    ordered_entries: LIMITS.ordered_entries - before.ordered_entries,
                    ..Default::default()
                }
            })
            .unwrap();
        let pressured = budget.usage();
        assert!(owner
            .register(&budget, Direction::Upstream, candidate)
            .is_err());
        assert!(owner.contains(&original_key));
        assert!(!owner.contains(&candidate_key));
        assert_eq!(budget.usage(), pressured);
        drop(pressure);
        assert_eq!(budget.usage(), before);
    }
}

#[test]
fn distinct_cap_duplicate_and_direction_union_keep_one_record_until_final_owner() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let owner = InputPaths::new(&budget).unwrap();
    let selected = fixture_path();
    let selected_key = selected.clone();
    let selected_charge = record(&selected);
    owner
        .register(&budget, Direction::Downstream, selected)
        .unwrap();
    for index in 1..MAX_ORIGINAL_SOURCE_PATHS {
        let path = selected_key.with_file_name(format!("original-{index}.bin"));
        owner
            .register(&budget, Direction::Downstream, path)
            .unwrap();
    }
    let full = budget.usage();
    assert_eq!(owner.records().len(), MAX_ORIGINAL_SOURCE_PATHS);
    owner
        .register(&budget, Direction::Upstream, selected_key.clone())
        .unwrap();
    assert_eq!(budget.usage(), full);
    let rejected = selected_key.with_file_name("one-extra-original.bin");
    assert!(owner
        .register(&budget, Direction::Downstream, rejected)
        .unwrap_err()
        .contains("Too many original source paths"));
    assert_eq!(budget.usage(), full);
    owner.clear(Direction::Downstream);
    assert_eq!(owner.records().len(), 1);
    assert!(owner.contains(&selected_key));
    assert_eq!(budget.usage(), add(fixed(), selected_charge));
    owner.clear(Direction::Upstream);
    assert_eq!(owner.records().len(), 0);
    assert_eq!(budget.usage(), fixed());
}

#[test]
fn foreign_budget_rejects_before_flags_and_final_alias_releases_root_and_records() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let foreign = RetentionBudget::with_limits(LIMITS);
    let owner = InputPaths::new(&budget).unwrap();
    let path = fixture_path();
    let key = path.clone();
    owner
        .register(&budget, Direction::Downstream, path)
        .unwrap();
    let before = budget.usage();
    assert!(owner
        .register(&foreign, Direction::Upstream, key.clone())
        .is_err());
    assert_eq!(budget.usage(), before);
    assert_eq!(foreign.usage(), RetainedResources::default());
    let released = Arc::downgrade(&owner.storage);
    let alias = owner.clone();
    assert!(Arc::ptr_eq(&owner.storage, &alias.storage));
    drop(owner);
    assert!(released.upgrade().is_some());
    assert_eq!(budget.usage(), before);
    alias.clear(Direction::Downstream);
    assert!(!alias.contains(&key));
    assert_eq!(budget.usage(), fixed());
    alias.register(&budget, Direction::Upstream, key).unwrap();
    assert!(budget.usage().payload_bytes > fixed().payload_bytes);
    drop(alias);
    assert!(released.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}
