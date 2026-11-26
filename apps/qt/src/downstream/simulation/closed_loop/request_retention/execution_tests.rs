/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual finite preparation, independent accepted source and charged execution input.
use super::*;
use crate::downstream::simulation::closed_loop::worker;
use merlin::downstream::{FullLoopControl, FullLoopState};

// Independent remaining-runtime fixture, below the accepted active execution budget.
const REMAINING_RUNTIME_MS: u64 = 500;

#[test]
fn narrowed_execution_owner_keeps_exact_public_charge_and_independent_source_until_final_drop() {
    let (model, _) = prepared_model(COMPLETE_OBSERVATION_MS, None);
    let source = model.request.as_ref().unwrap();
    let budget = model.context.application.budget();
    let baseline = budget.usage();
    let original_options = serde_json::to_value(source.value().options()).unwrap();
    let original_settings = serde_json::to_value(source.value().settings()).unwrap();
    let candidate = source
        .value()
        .with_runtime_limit(REMAINING_RUNTIME_MS)
        .unwrap();
    let scientific = candidate.retained_footprint().unwrap();
    // This request owns Vec/Arc storage and no ordered-container entries.
    assert_eq!(scientific.ordered_entries, 0);
    let expected = resources::<FullLoopRequest>(scientific).unwrap();
    let execution = admit_run_request(candidate, REMAINING_RUNTIME_MS, budget).unwrap();
    assert_eq!(budget.usage(), sum(baseline, expected));
    assert_eq!(
        execution.value().options().max_runtime_ms,
        REMAINING_RUNTIME_MS
    );
    assert!(std::ptr::eq(
        execution.value().settings(),
        source.value().settings()
    ));
    assert_eq!(
        serde_json::to_value(execution.value().settings()).unwrap(),
        original_settings
    );
    assert_eq!(
        serde_json::to_value(source.value().options()).unwrap(),
        original_options
    );
    let old = Arc::clone(&execution);
    let weak = Arc::downgrade(&execution);
    drop(execution);
    assert_eq!(budget.usage(), sum(baseline, expected));
    drop(old);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), baseline);
    assert_eq!(
        serde_json::to_value(source.value().options()).unwrap(),
        original_options
    );
    let zero_entry_budget = RetentionBudget::with_limits(RetainedResources {
        ordered_entries: 0,
        ..TEST_MAXIMUM
    });
    let exact_zero = admit_run_request(
        source.value().as_ref().clone(),
        REMAINING_RUNTIME_MS,
        &zero_entry_budget,
    )
    .unwrap();
    assert_eq!(zero_entry_budget.usage(), expected);
    drop(exact_zero);
    assert_eq!(zero_entry_budget.usage(), RetainedResources::default());
}

#[test]
fn insufficient_execution_input_rejects_its_owned_dimensions_before_control_and_preserves_source() {
    let (model, _) = prepared_model(COMPLETE_OBSERVATION_MS, None);
    let source = model.request.as_ref().unwrap();
    let source_budget = model.context.application.budget();
    let source_charge = source_budget.usage();
    let original = serde_json::to_value(source.value().settings()).unwrap();
    let candidate = source
        .value()
        .with_runtime_limit(REMAINING_RUNTIME_MS)
        .unwrap();
    let required = resources::<FullLoopRequest>(candidate.retained_footprint().unwrap()).unwrap();
    assert_eq!(required.ordered_entries, 0);
    drop(candidate);
    // Plan/result tests reject ordered-entry pressure. This zero-entry root
    // instead admits at an exact zero-entry ceiling in the preceding test.
    for dimension in [Dimension::Payload, Dimension::Roots] {
        let budget = RetentionBudget::with_limits(one_less(required, dimension));
        let control = FullLoopControl::new();
        assert!(worker::run_one_admitted(
            source.value().as_ref().clone(),
            REMAINING_RUNTIME_MS,
            &control,
            &budget,
        )
        .is_err());
        assert_eq!(control.snapshot().state, FullLoopState::Idle);
        assert!(control.snapshot().progress.is_none());
        assert_eq!(budget.usage(), RetainedResources::default());
        assert_eq!(source_budget.usage(), source_charge);
        assert_eq!(
            serde_json::to_value(source.value().settings()).unwrap(),
            original
        );
    }
}
