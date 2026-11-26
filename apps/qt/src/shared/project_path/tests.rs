/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact path capacities, inclusive byte admission and actual move/disposal.
use super::*;

// Independent spare-capacity fixture anchor, unrelated to filesystem limits.
const SPARE_PATH_BYTES: usize = 37;

fn fixture_path() -> PathBuf {
    let mut path = PathBuf::from("a project/α.merlin.json");
    path.reserve(SPARE_PATH_BYTES);
    path
}

#[test]
fn actual_path_capacity_and_wrapper_admit_exactly_without_population_charges() {
    let path = fixture_path();
    let capacity = path.capacity();
    assert!(capacity > path.as_os_str().len());
    let expected = RetainedResources {
        payload_bytes: size_of::<Retained<PathBuf>>() + capacity,
        ..Default::default()
    };
    let budget = RetentionBudget::with_limits(expected);
    let owner = admit(path, &budget).unwrap();
    assert_eq!(owner.value(), &PathBuf::from("a project/α.merlin.json"));
    assert_eq!(owner.value().capacity(), capacity);
    assert_eq!(owner.fixture_reserved_resources(), expected);
    assert_eq!(budget.usage(), expected);
    let address = owner.value() as *const PathBuf;
    let moved = owner;
    assert_eq!(moved.value() as *const PathBuf, address);
    assert_eq!(budget.usage(), expected);
    drop(moved);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn one_byte_short_or_old_candidate_overlap_rejects_without_losing_previous_path() {
    let path = fixture_path();
    let bytes = size_of::<Retained<PathBuf>>() + path.capacity();
    let short = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: bytes - 1,
        ..Default::default()
    });
    assert!(admit(path, &short).is_err());
    assert_eq!(short.usage(), RetainedResources::default());

    let previous = fixture_path();
    let budget = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: size_of::<Retained<PathBuf>>() + previous.capacity(),
        ..Default::default()
    });
    let previous = admit(previous, &budget).unwrap();
    let baseline = budget.usage();
    let address = previous.value() as *const PathBuf;
    assert!(admit(PathBuf::from("another.merlin.json"), &budget).is_err());
    assert_eq!(budget.usage(), baseline);
    assert_eq!(previous.value() as *const PathBuf, address);
    drop(previous);
    assert_eq!(budget.usage(), RetainedResources::default());
}
