/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Counter-only logical presentation fixtures; no Qt allocation or footprint claims.
use super::*;
use crate::shell::{
    constants::{
        MAX_SESSION_OPEN_VIEWS, MAX_SESSION_PRESENTATION_ENTRIES,
        MAX_SESSION_PRESENTATION_UTF16_BYTES, MAX_SESSION_RETAINED_VIEWS,
    },
    ApplicationSessions,
};
use std::sync::atomic::{AtomicBool, Ordering};

// Independent synthetic ceilings, deliberately unrelated to desktop policies.
const PRESENTATION: RetainedResources = RetainedResources {
    presentation_utf16_bytes: 26,
    presentation_entries: 7,
    retained_views: 5,
    open_views: 3,
    payload_bytes: 0,
    ordered_entries: 0,
    shared_roots: 0,
    channel_count: 0,
    slot_count: 0,
};

// A second independently retained logical candidate for replacement overlap.
const CANDIDATE: RetainedResources = RetainedResources {
    presentation_utf16_bytes: 14,
    presentation_entries: 3,
    retained_views: 2,
    open_views: 1,
    payload_bytes: 0,
    ordered_entries: 0,
    shared_roots: 0,
    channel_count: 0,
    slot_count: 0,
};

// One counter unit checks an exact adjacent boundary, not a measured text value.
const COUNTER_UNIT: usize = 1;
const PAYLOAD_ONLY_QUOTA: usize = 11;

fn units() -> [RetainedResources; 4] {
    [
        RetainedResources {
            presentation_utf16_bytes: COUNTER_UNIT,
            ..Default::default()
        },
        RetainedResources {
            presentation_entries: COUNTER_UNIT,
            ..Default::default()
        },
        RetainedResources {
            retained_views: COUNTER_UNIT,
            ..Default::default()
        },
        RetainedResources {
            open_views: COUNTER_UNIT,
            ..Default::default()
        },
    ]
}

#[test]
fn four_inclusive_ceilings_reject_one_more_without_changing_other_dimensions() {
    // Each old dimension can still fit this request. A late logical rejection
    // must leave all nine counters unchanged, not publish a successful prefix.
    let original = RetainedResources {
        payload_bytes: COUNTER_UNIT,
        ordered_entries: COUNTER_UNIT,
        shared_roots: COUNTER_UNIT,
        channel_count: COUNTER_UNIT,
        slot_count: COUNTER_UNIT,
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    };
    let maximum = original.checked_add(PRESENTATION).unwrap();
    let budget = RetentionBudget::with_limits(maximum);
    let owner = budget.reserve_resources(PRESENTATION).unwrap();
    assert_eq!(budget.usage(), PRESENTATION);
    let errors = [
        RetentionError::PresentationUtf16BytesExceeded {
            maximum: PRESENTATION.presentation_utf16_bytes,
            requested: PRESENTATION.presentation_utf16_bytes + COUNTER_UNIT,
        },
        RetentionError::PresentationEntriesExceeded {
            maximum: PRESENTATION.presentation_entries,
            requested: PRESENTATION.presentation_entries + COUNTER_UNIT,
        },
        RetentionError::RetainedViewsExceeded {
            maximum: PRESENTATION.retained_views,
            requested: PRESENTATION.retained_views + COUNTER_UNIT,
        },
        RetentionError::OpenViewsExceeded {
            maximum: PRESENTATION.open_views,
            requested: PRESENTATION.open_views + COUNTER_UNIT,
        },
    ];
    for (unit, error) in units().into_iter().zip(errors) {
        let candidate = original.checked_add(unit).unwrap();
        assert_eq!(budget.reserve_resources(candidate).unwrap_err(), error);
        assert_eq!(budget.usage(), PRESENTATION);
    }
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn overflow_in_each_logical_quantity_rolls_back_before_any_counter_changes() {
    let maximum = RetainedResources {
        presentation_utf16_bytes: usize::MAX,
        presentation_entries: usize::MAX,
        retained_views: usize::MAX,
        open_views: usize::MAX,
        ..Default::default()
    };
    for unit in units() {
        let budget = RetentionBudget::with_limits(maximum);
        let owner = budget.reserve_resources(maximum).unwrap();
        assert_eq!(
            maximum.checked_add(unit),
            Err(RetentionError::ArithmeticOverflow)
        );
        assert_eq!(
            budget.reserve_resources(unit).unwrap_err(),
            RetentionError::ArithmeticOverflow
        );
        assert_eq!(budget.usage(), maximum);
        drop(owner);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn replacement_requires_actual_overlap_and_rejection_preserves_the_old_owner() {
    let overlap = PRESENTATION.checked_add(CANDIDATE).unwrap();
    for maximum in [
        RetainedResources {
            presentation_utf16_bytes: overlap.presentation_utf16_bytes - COUNTER_UNIT,
            ..overlap
        },
        RetainedResources {
            presentation_entries: overlap.presentation_entries - COUNTER_UNIT,
            ..overlap
        },
        RetainedResources {
            retained_views: overlap.retained_views - COUNTER_UNIT,
            ..overlap
        },
        RetainedResources {
            open_views: overlap.open_views - COUNTER_UNIT,
            ..overlap
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        let old = budget
            .reserve_resources(PRESENTATION)
            .unwrap()
            .retain("old");
        assert!(budget.reserve_resources(CANDIDATE).is_err());
        assert_eq!(*old.value(), "old");
        assert_eq!(budget.usage(), PRESENTATION);
        drop(old);
        let candidate = budget.reserve_resources(CANDIDATE).unwrap().retain("new");
        assert_eq!(budget.usage(), CANDIDATE);
        drop(candidate);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    let budget = RetentionBudget::with_limits(overlap);
    let old = budget
        .reserve_resources(PRESENTATION)
        .unwrap()
        .retain("old");
    let candidate = budget.reserve_resources(CANDIDATE).unwrap().retain("new");
    assert_eq!(budget.usage(), overlap);
    assert_eq!(*old.value(), "old");
    assert_eq!(*candidate.value(), "new");
    drop(old);
    assert_eq!(budget.usage(), CANDIDATE);
    drop(candidate);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn final_alias_releases_logical_quantities_after_payload_destruction() {
    // Test-only refcount/Drop instrumentation; this exercises logical reservations,
    // not complete Rust allocation metadata for this fixture's observer handles.
    struct Probe {
        budget: RetentionBudget,
        destroyed: Arc<AtomicBool>,
    }
    impl Drop for Probe {
        /// Observes all logical charges while the retained payload is destroyed.
        ///
        /// The reservation must release only after this destructor and its data.
        fn drop(&mut self) {
            assert_eq!(self.budget.usage(), PRESENTATION);
            self.destroyed.store(true, Ordering::SeqCst);
        }
    }
    let budget = RetentionBudget::with_limits(PRESENTATION);
    let weak = Arc::downgrade(&budget.0);
    let observer = budget.clone();
    let destroyed = Arc::new(AtomicBool::new(false));
    let owner = Arc::new(
        budget
            .reserve_resources(PRESENTATION)
            .unwrap()
            .retain(Probe {
                budget: budget.clone(),
                destroyed: Arc::clone(&destroyed),
            }),
    );
    let hidden = Arc::clone(&owner);
    drop(budget);
    drop(owner);
    assert!(!destroyed.load(Ordering::SeqCst));
    assert_eq!(observer.usage(), PRESENTATION);
    drop(hidden);
    assert!(destroyed.load(Ordering::SeqCst));
    assert_eq!(observer.usage(), RetainedResources::default());
    drop(observer);
    assert!(weak.upgrade().is_none());
}

#[test]
fn bytes_only_constructor_keeps_all_other_eight_ceilings_zero() {
    let budget = RetentionBudget::new(RetainedBytes(PAYLOAD_ONLY_QUOTA));
    assert_eq!(
        lock(&budget.0).maximum,
        RetainedResources {
            payload_bytes: PAYLOAD_ONLY_QUOTA,
            ..Default::default()
        }
    );
    assert_eq!(budget.usage(), RetainedResources::default());
    let bytes = budget.reserve(RetainedBytes(PAYLOAD_ONLY_QUOTA)).unwrap();
    let held = budget.usage();
    for unit in units() {
        assert!(budget.reserve_resources(unit).is_err());
        assert_eq!(budget.usage(), held);
    }
    assert_eq!(budget.used(), RetainedBytes(PAYLOAD_ONLY_QUOTA));
    drop(bytes);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn desktop_uses_authoritative_logical_policies_without_changing_rust_usage() {
    let application = ApplicationSessions::desktop().unwrap();
    let budget = application.budget();
    let maximum = lock(&budget.0).maximum;
    assert_eq!(
        maximum.presentation_utf16_bytes,
        MAX_SESSION_PRESENTATION_UTF16_BYTES
    );
    assert_eq!(
        maximum.presentation_entries,
        MAX_SESSION_PRESENTATION_ENTRIES
    );
    assert_eq!(maximum.retained_views, MAX_SESSION_RETAINED_VIEWS);
    assert_eq!(maximum.open_views, MAX_SESSION_OPEN_VIEWS);
    let baseline = budget.usage();
    assert_eq!(baseline.presentation_utf16_bytes, 0);
    assert_eq!(baseline.presentation_entries, 0);
    assert_eq!(baseline.retained_views, 0);
    assert_eq!(baseline.open_views, 0);
    let presentation = RetainedResources {
        presentation_utf16_bytes: maximum.presentation_utf16_bytes,
        presentation_entries: maximum.presentation_entries,
        retained_views: maximum.retained_views,
        open_views: maximum.open_views,
        ..Default::default()
    };
    let owner = budget.reserve_resources(presentation).unwrap();
    assert_eq!(budget.usage(), baseline.checked_add(presentation).unwrap());
    drop(owner);
    assert_eq!(budget.usage(), baseline);
}
