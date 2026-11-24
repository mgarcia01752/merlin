/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Fixed Store allocation boundaries, alias lifetime and payload destruction order.
use super::*;

// Distinct payload values exercise the existing shared read/write interface.
const ORIGINAL_VALUE: u16 = 17;
const REPLACEMENT_VALUE: u16 = 31;

fn fixed_resources<T>() -> RetainedResources {
    RetainedResources {
        payload_bytes: size_of::<Retained<Mutex<T>>>(),
        ordered_entries: 0,
        shared_roots: 1,
    }
}

#[test]
fn fixed_store_admits_exact_bytes_and_root_without_ordered_entries() {
    let required = fixed_resources::<u16>();
    for maximum in [
        RetainedResources {
            payload_bytes: required.payload_bytes - 1,
            ..required
        },
        RetainedResources {
            shared_roots: 0,
            ..required
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        let rejected = Store::new(ORIGINAL_VALUE, &budget);
        if maximum.shared_roots == 0 {
            assert!(matches!(
                rejected,
                Err(RetentionError::SharedRootsExceeded { .. })
            ));
        } else {
            assert!(matches!(
                rejected,
                Err(RetentionError::LimitExceeded { .. })
            ));
        }
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    let budget = RetentionBudget::with_limits(required);
    let store = Store::new(ORIGINAL_VALUE, &budget).unwrap();
    assert_eq!(budget.usage(), required);
    assert_eq!(*store.borrow(), ORIGINAL_VALUE);
    store.with(|owner| *owner.borrow_mut() = REPLACEMENT_VALUE);
    let alias = store.clone();
    assert!(store.downgrade().ptr_eq(&alias.downgrade()));
    assert_eq!(*alias.borrow(), REPLACEMENT_VALUE);
    assert_eq!(budget.usage(), required);
    drop(store);
    assert_eq!(budget.usage(), required);
    drop(alias);
    assert_eq!(budget.usage(), RetainedResources::default());
}

struct DropWitness {
    budget: RetentionBudget,
    observed: Arc<Mutex<Option<RetainedResources>>>,
}

impl Drop for DropWitness {
    /// Records the live Store charge while the original payload is destroyed.
    fn drop(&mut self) {
        *self.observed.lock().unwrap() = Some(self.budget.usage());
    }
}

#[test]
fn final_alias_drops_payload_before_releasing_its_store_reservation() {
    let required = fixed_resources::<DropWitness>();
    let budget = RetentionBudget::with_limits(required);

    // The observation handle is external test instrumentation, not retained science.
    let observed = Arc::new(Mutex::new(None));
    let store = Store::new(
        DropWitness {
            budget: budget.clone(),
            observed: Arc::clone(&observed),
        },
        &budget,
    )
    .unwrap();
    let weak = store.downgrade();
    let alias = store.clone();
    drop(store);
    assert_eq!(*observed.lock().unwrap(), None);
    assert!(weak.upgrade().is_some());
    assert_eq!(budget.usage(), required);
    drop(alias);
    assert_eq!(*observed.lock().unwrap(), Some(required));
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}
