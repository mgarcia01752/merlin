/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual-query overlap and checked admission; no engine replacement fixtures.
use super::*;
use crate::shell::RetentionError;

#[test]
fn individual_slots_and_same_event_paging_keep_their_separate_reservations() {
    let result = super::super::tests::result();
    let source = result.retained_footprint().unwrap();
    let budget = super::super::super::result_retention::fixture_budget();
    let source_charge = RetainedResources {
        payload_bytes: source.payload_bytes + size_of::<Retained<FullLoopResult>>()
            - size_of::<FullLoopResult>(),
        ordered_entries: source.ordered_entries,
        shared_roots: source.shared_roots,
        ..Default::default()
    };
    let retained = budget
        .reserve_resources(source_charge)
        .unwrap()
        .retain(result);
    let queries = Queries::new(retained.value(), &budget).unwrap();
    let inventory = inventory_resources(source.payload_bytes).unwrap();
    let accumulator = snapshot_resources(source.payload_bytes).unwrap();
    let page = snapshot_resources(source.payload_bytes).unwrap();
    let replacement = snapshot_resources(source.payload_bytes).unwrap();
    assert_eq!(
        budget.usage().payload_bytes,
        source_charge.payload_bytes
            + inventory.payload_bytes
            + accumulator.payload_bytes
            + page.payload_bytes
            + replacement.payload_bytes
    );
    assert_eq!(budget.usage().ordered_entries, source.ordered_entries);
    assert_eq!(budget.usage().shared_roots, source.shared_roots);
    let event = queries.snapshot(Stage::PayloadCoded, 0).unwrap();
    assert!(
        event.data.len() > MAX_STAGE_QUERY_ELEMENTS,
        "Fixture exercises actual paging"
    );
    if let Data::Bits(values) = &event.data {
        assert_eq!(values.capacity(), values.len());
    }
    assert_eq!(event.context.run_id.len(), event.context.run_id.capacity());
    assert_eq!(
        event.context.configuration_id.len(),
        event.context.configuration_id.capacity()
    );
    assert_eq!(retained.value().retained_footprint().unwrap(), source);
    drop(event);
    drop(queries);
    assert_eq!(budget.usage(), source_charge);
    drop(retained);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn slot_arithmetic_and_zero_policy_reject_without_mutating_the_real_source() {
    assert!(matches!(
        inventory_resources(usize::MAX),
        Err(ObservationError::Retention(
            RetentionError::ArithmeticOverflow
        ))
    ));
    assert!(matches!(
        snapshot_resources(usize::MAX),
        Err(ObservationError::Retention(
            RetentionError::ArithmeticOverflow
        ))
    ));
    let result = super::super::tests::result();
    let source = result.retained_footprint().unwrap();
    let budget = RetentionBudget::with_limits(RetainedResources::default());
    assert!(matches!(
        Queries::new(&result, &budget),
        Err(ObservationError::Retention(_))
    ));
    assert_eq!(budget.usage(), RetainedResources::default());
    assert_eq!(result.retained_footprint().unwrap(), source);
}
