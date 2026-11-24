/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete receiver Box admission and target transfer, without duplicate child charges.
use super::*;
use crate::shell::ApplicationSessions;
use std::mem::size_of;

// Synthetic host ceilings permit exact actual-layout pressure; no DSP estimate.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap()
}

#[test]
fn receiver_fixed_box_exact_and_one_byte_below_preserve_host_and_release_on_final_move() {
    let measured_baseline = context().application.budget().usage();
    let fixed = size_of::<Model>();
    for shortage in [false, true] {
        // This finite inclusive policy is measured from actual empty owners, not
        // MAX-counter overflow or a guessed bootstrap/layout allowance.
        let exact = measured_baseline.payload_bytes.checked_add(fixed).unwrap();
        let maximum = if shortage { exact - 1 } else { exact };
        let context = Context::for_application(
            ApplicationSessions::new(RetainedResources {
                payload_bytes: maximum,
                ..LIMITS
            })
            .unwrap(),
        )
        .unwrap();
        let budget = context.application.budget();
        let baseline = budget.usage();
        assert_eq!(baseline, measured_baseline);
        let snapshot = context.application.snapshot(Direction::Downstream);
        let candidate = Model::for_context(context.clone());
        assert_eq!(
            context.application.snapshot(Direction::Downstream),
            snapshot
        );
        if shortage {
            assert!(matches!(
                candidate,
                Err(RetentionError::LimitExceeded { .. })
            ));
            assert_eq!(budget.usage(), baseline);
        } else {
            let candidate = candidate.unwrap();
            assert!(budget.owns(&candidate.fixed_guard));
            assert_eq!(
                candidate.fixed_guard.fixture_reserved_resources(),
                RetainedResources {
                    payload_bytes: fixed,
                    ..Default::default()
                }
            );
            assert_eq!(budget.usage().payload_bytes, maximum);
            assert_eq!(budget.usage().ordered_entries, baseline.ordered_entries);
            assert_eq!(budget.usage().shared_roots, baseline.shared_roots);

            // The same charged Box moves through a queue-shaped owner; it is not cloned.
            let mut queued = Some(candidate);
            let adopted = queued.take().unwrap();
            assert_eq!(budget.usage().payload_bytes, maximum);
            drop(adopted);
            assert_eq!(budget.usage(), baseline);
        }
    }
}

#[test]
fn receiver_foreign_fixed_target_rejects_before_mutation_and_same_host_attachment_reuses() {
    let source = context();
    let target = context();
    let source_baseline = source.application.budget().usage();
    let target_baseline = target.application.budget().usage();
    let mut model = Model::for_context(source.clone()).unwrap();
    let source_key = source.application.snapshot(Direction::Downstream).key;
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - target_baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(model
        .prepare_inputs_for_budget(
            "",
            &Inputs::default(),
            &Sections::default(),
            target.application.budget()
        )
        .is_err());
    assert!(source.application.budget().owns(&model.fixed_guard));
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        source_key
    );
    assert_eq!(model.generation, None);
    assert!(model.options.is_none());
    assert!(model.request.is_none());
    assert!(model.plan.is_none());
    drop(pressure);
    model
        .prepare_inputs_for_budget(
            "",
            &Inputs::default(),
            &Sections::default(),
            target.application.budget(),
        )
        .unwrap();
    assert_eq!(
        model.check_prepared_target(target.application.budget()),
        Ok(false)
    );
    assert!(model
        .check_prepared_target(source.application.budget())
        .is_err());
    assert_eq!(source.application.budget().usage(), source_baseline);
    let key = target.application.configure_downstream(false).unwrap();
    model.commit_prepared_generation(key);
    let accepted = target.application.budget().usage();
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - accepted.payload_bytes,
            ordered_entries: LIMITS.ordered_entries - accepted.ordered_entries,
            shared_roots: LIMITS.shared_roots - accepted.shared_roots,
        })
        .unwrap();
    model.attach_prepared(target.clone()).unwrap();
    assert_eq!(target.application.budget().usage(), LIMITS);
    assert_eq!(model.generation, Some(key));
    drop(pressure);
    drop(model);
    assert_eq!(target.application.budget().usage(), target_baseline);
}
