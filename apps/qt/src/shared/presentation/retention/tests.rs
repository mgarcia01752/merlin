/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete presentation owners on the real checked canonical resource primitive.
use super::*;

// Independent logical-policy fixtures; no production limit is repeated.
const LOGICAL_BYTES: usize = 18;
const LOGICAL_ENTRIES: usize = 7;
const RETAINED_VIEWS: usize = 2;
const OPEN_VIEWS: usize = 1;

fn maximum() -> RetainedResources {
    RetainedResources {
        payload_bytes: RETAINED_VIEWS * size_of::<View>(),
        presentation_utf16_bytes: LOGICAL_BYTES,
        presentation_entries: LOGICAL_ENTRIES,
        retained_views: RETAINED_VIEWS,
        open_views: OPEN_VIEWS,
        ..Default::default()
    }
}
fn current_resources() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<View>(),
        retained_views: 1,
        ..Default::default()
    }
}

#[test]
fn actual_fixed_layout_and_retained_view_population_admit_before_allocation() {
    let exact = current_resources();
    let budget = RetentionBudget::with_limits(exact);
    let owner = View::new(&budget).unwrap();
    assert_eq!(budget.usage(), exact);
    assert_eq!(owner._fixed.fixture_reserved_resources(), exact);
    assert!(owner.is_empty());
    assert!(View::new(&budget).is_err());
    assert_eq!(budget.usage(), exact);
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());

    for limits in [
        RetainedResources {
            payload_bytes: size_of::<View>() - 1,
            retained_views: 1,
            ..Default::default()
        },
        RetainedResources {
            payload_bytes: size_of::<View>(),
            retained_views: 0,
            ..Default::default()
        },
    ] {
        let budget = RetentionBudget::with_limits(limits);
        assert!(View::new(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn logical_candidate_overlap_rollback_and_commit_preserve_exact_old_ownership() {
    let budget = RetentionBudget::with_limits(maximum());
    let mut owner = View::new(&budget).unwrap();
    // Independent complete candidate occupies the inclusive logical policy.
    assert!(owner.prepare(LOGICAL_BYTES as f64, LOGICAL_ENTRIES as f64));
    assert!(owner.commit());
    let accepted = budget.usage();
    assert_eq!(accepted.presentation_utf16_bytes, LOGICAL_BYTES);
    assert_eq!(accepted.presentation_entries, LOGICAL_ENTRIES);
    assert!(!owner.is_empty());
    assert!(!owner.prepare(1.0, 0.0));
    assert!(!owner.prepare(0.0, 1.0));
    assert!(!owner.commit());
    assert_eq!(budget.usage(), accepted);

    owner.clear();
    assert_eq!(budget.usage(), current_resources());
    // A two-code-unit label and one wrapper entry; unrelated numeric purposes.
    assert!(owner.prepare(4.0, 1.0));
    assert!(owner.commit());
    let old = budget.usage();
    assert!(owner.prepare(6.0, 2.0));
    let overlap = budget.usage();
    assert_eq!(overlap.presentation_utf16_bytes, 10);
    assert_eq!(overlap.presentation_entries, 3);
    assert!(!owner.prepare(0.0, 0.0)); // A pending transaction cannot be stolen.
    assert_eq!(budget.usage(), overlap);
    owner.cancel();
    assert_eq!(budget.usage(), old);
    assert!(owner.prepare(6.0, 2.0));
    assert!(owner.commit());
    assert_eq!(budget.usage().presentation_utf16_bytes, 6);
    assert_eq!(budget.usage().presentation_entries, 2);
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn closing_keeps_hidden_payload_while_clear_and_final_disposal_release_their_owners() {
    let budget = RetentionBudget::with_limits(maximum());
    let mut owner = View::new(&budget).unwrap();
    let mut other = View::new(&budget).unwrap();
    assert!(owner.prepare(8.0, 2.0));
    assert!(owner.commit());
    let hidden = budget.usage();
    assert!(owner.begin_open());
    assert!(owner.begin_open()); // Repeated open shares one actual slot.
    assert_eq!(budget.usage().open_views, OPEN_VIEWS);
    let full = budget.usage();
    assert!(!other.begin_open());
    assert_eq!(budget.usage(), full);
    owner.end_open();
    assert_eq!(budget.usage(), hidden);
    assert!(!owner.is_empty());
    assert!(other.begin_open());
    assert_eq!(budget.usage().open_views, OPEN_VIEWS);
    owner.clear();
    assert!(owner.is_empty());
    assert_eq!(budget.usage().presentation_utf16_bytes, 0);
    assert_eq!(budget.usage().presentation_entries, 0);
    assert_eq!(budget.usage().retained_views, RETAINED_VIEWS);
    drop(owner);
    assert_eq!(budget.usage().retained_views, 1);
    assert_eq!(budget.usage().open_views, OPEN_VIEWS);
    drop(other);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn malformed_javascript_counts_reject_without_pending_or_counter_mutation() {
    let budget = RetentionBudget::with_limits(maximum());
    let mut owner = View::new(&budget).unwrap();
    let before = budget.usage();
    for malformed in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -1.0,
        0.5,
        JAVASCRIPT_MAX_SAFE_INTEGER + 1.0,
    ] {
        assert!(!owner.prepare(malformed, 0.0));
        assert!(!owner.prepare(0.0, malformed));
        assert!(!owner.commit());
        assert_eq!(budget.usage(), before);
    }
    assert!(owner.prepare(0.0, 0.0));
    assert!(owner.commit());
    assert!(!owner.is_empty());
    assert_eq!(budget.usage(), before);
    owner.clear();
    assert!(owner.is_empty());
}
