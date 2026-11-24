/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application counter baseline boundaries and final shared-owner destruction.
use super::*;

// A scalar payload tests a real retained wrapper alongside the counter baseline.
const RETAINED_VALUE: usize = 17;

fn baseline() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<Mutex<Accounting>>(),
        ordered_entries: 0,
        shared_roots: 1,
    }
}

#[test]
fn application_counter_admits_exact_layout_and_rejects_each_smaller_dimension() {
    let required = baseline();
    let budget = RetentionBudget::for_application(required).unwrap();
    assert_eq!(budget.usage(), required);
    assert_eq!(budget.used(), RetainedBytes(required.payload_bytes));
    let alias = budget.clone();
    assert!(Arc::ptr_eq(&budget.0, &alias.0));
    assert_eq!(alias.usage(), required);
    assert!(matches!(
        RetentionBudget::for_application(RetainedResources {
            payload_bytes: required.payload_bytes - 1,
            ..required
        }),
        Err(RetentionError::LimitExceeded { .. })
    ));
    assert!(matches!(
        RetentionBudget::for_application(RetainedResources {
            shared_roots: 0,
            ..required
        }),
        Err(RetentionError::SharedRootsExceeded { .. })
    ));
    assert_eq!(budget.usage(), required);
}

#[test]
fn application_baseline_survives_reservations_and_dies_with_final_counter_alias() {
    let required = baseline();
    let payload = RetainedResources {
        payload_bytes: size_of::<Retained<usize>>(),
        ..Default::default()
    };
    let maximum = required.checked_add(payload).unwrap();
    let budget = RetentionBudget::for_application(maximum).unwrap();
    let weak = Arc::downgrade(&budget.0);
    let alias = budget.clone();
    let owner = budget
        .reserve_resources(payload)
        .unwrap()
        .retain(RETAINED_VALUE);
    assert_eq!(budget.usage(), maximum);
    drop(budget);
    assert_eq!(*owner.value(), RETAINED_VALUE);
    assert_eq!(alias.usage(), maximum);
    drop(owner);
    assert_eq!(alias.usage(), required);

    // Even a zero-resource reservation retains the same live counter allocation.
    let reservation = alias.reserve_resources(Default::default()).unwrap();
    drop(alias);
    let observed = weak.upgrade().unwrap();
    assert_eq!(lock(&observed).used, required);
    drop(observed);
    drop(reservation);
    assert!(weak.upgrade().is_none());
}

#[test]
fn generic_constructors_keep_their_original_zero_baselines() {
    let bytes_only = RetentionBudget::new(RetainedBytes(0));
    let multidimensional = RetentionBudget::with_limits(RetainedResources::default());
    assert_eq!(bytes_only.usage(), RetainedResources::default());
    assert_eq!(multidimensional.usage(), RetainedResources::default());
    for budget in [bytes_only, multidimensional] {
        let empty = budget.reserve_resources(Default::default()).unwrap();
        assert_eq!(budget.usage(), RetainedResources::default());
        drop(empty);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}
