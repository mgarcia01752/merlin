/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Exact application atomic pointees and final shared-owner disposal.
use super::*;

fn verify<Control>(
    construct: impl Fn(&RetentionBudget) -> Result<Arc<Retained<Control>>, RetentionError>,
) {
    let exact = RetainedResources {
        payload_bytes: size_of::<Retained<Control>>(),
        shared_roots: CONTROL_SHARED_ROOTS,
        ..Default::default()
    };
    let budget = RetentionBudget::with_limits(exact);
    let control = construct(&budget).unwrap();
    let alias = Arc::clone(&control);
    let weak = Arc::downgrade(&control);
    assert_eq!(budget.usage(), exact);
    drop(control);
    assert!(weak.upgrade().is_some());
    assert_eq!(budget.usage(), exact);
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());

    for maximum in [
        RetainedResources {
            payload_bytes: exact.payload_bytes - 1,
            ..exact
        },
        RetainedResources {
            shared_roots: 0,
            ..exact
        },
    ] {
        let budget = RetentionBudget::with_limits(maximum);
        assert!(construct(&budget).is_err());
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}

#[test]
fn cancellation_capture_and_stage_pointees_have_exact_independent_final_owner_coverage() {
    verify(cancellation);
    verify(completed);
    verify(load_stage);
}
