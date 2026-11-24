/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real receiver candidate ownership without target Context or generation adoption.
use super::*;
use crate::downstream::workbench::session;
use crate::shell::{ApplicationSessions, Direction, RetainedResources};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

// Fixture-only ceilings permit actual-usage pressure without estimating science.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
// The supported short observation and active execution limit are separate fixtures.
const OBSERVATION_MS: u64 = 1;
const ACTIVE_EXECUTION_LIMIT_MS: u64 = 60000;
// Host scheduling limits only; neither constrains the scientific result.
const WATCHDOG: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(5);

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
}

fn candidate() -> Box<Model> {
    let mut model = Model::for_context(Context::isolated()).unwrap();
    model.options = Some(Options {
        duration_ms: OBSERVATION_MS,
        max_runtime_ms: ACTIVE_EXECUTION_LIMIT_MS,
        ..Default::default()
    });
    model.verify = true;
    model
}

#[test]
fn target_preparation_preserves_context_and_all_dimension_rejection_preserves_roots() {
    let (inputs, settings, sections) = super::super::tests::accepted_inputs();
    let settings = serde_json::to_string(&settings).unwrap();
    let target = context();
    let budget = target.application.budget();
    let baseline = budget.usage();
    let target_key = target.application.snapshot(Direction::Downstream).key;
    let mut model = candidate();
    let source_key = model
        .context
        .application
        .snapshot(Direction::Downstream)
        .key;
    model
        .prepare_inputs_for_budget(&settings, &inputs, &sections, budget)
        .unwrap();
    assert_eq!(model.check_prepared_target(budget), Ok(true));
    assert_eq!(model.generation, None);
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        source_key
    );
    assert_eq!(
        target.application.snapshot(Direction::Downstream).key,
        target_key
    );
    assert!(session::snapshot(&target).source.is_none());
    let request = Arc::clone(model.request.as_ref().unwrap());
    let plan = Arc::clone(model.plan.as_ref().unwrap());
    let usage = budget.usage();
    assert!(usage.payload_bytes > baseline.payload_bytes);
    assert!(usage.ordered_entries > baseline.ordered_entries);
    assert!(usage.shared_roots > baseline.shared_roots);

    // Fill one actual remaining host quantity at a time. This pressure has no
    // scientific payload estimate and cannot change the finite input builder.
    for pressure in [
        RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ..Default::default()
        },
        RetainedResources {
            ordered_entries: usize::MAX - usage.ordered_entries,
            ..Default::default()
        },
        RetainedResources {
            shared_roots: usize::MAX - usage.shared_roots,
            ..Default::default()
        },
    ] {
        let guard = budget.reserve_resources(pressure).unwrap();
        let pressured = budget.usage();
        assert!(model
            .prepare_inputs_for_budget(&settings, &inputs, &sections, budget)
            .is_err());
        assert_eq!(budget.usage(), pressured);
        assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
        assert!(Arc::ptr_eq(model.plan.as_ref().unwrap(), &plan));
        assert_eq!(model.generation, None);
        assert_eq!(
            target.application.snapshot(Direction::Downstream).key,
            target_key
        );
        drop(guard);
    }
    let fixed = model.fixed_guard.fixture_reserved_resources();
    drop(model);
    assert_eq!(
        budget.usage(),
        RetainedResources {
            payload_bytes: usage
                .payload_bytes
                .checked_sub(fixed.payload_bytes)
                .unwrap(),
            ..usage
        }
    );
    drop(request);
    drop(plan);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn target_check_and_attachment_require_consistent_owners_and_the_stamped_key() {
    let (inputs, settings, sections) = super::super::tests::accepted_inputs();
    let settings = serde_json::to_string(&settings).unwrap();
    let target = context();
    let foreign = context();
    let mut model = candidate();
    model
        .prepare_inputs_for_budget(&settings, &inputs, &sections, target.application.budget())
        .unwrap();
    assert!(model
        .check_prepared_target(foreign.application.budget())
        .is_err());
    let plan = model.plan.take();
    assert!(model
        .check_prepared_target(target.application.budget())
        .is_err());
    model.plan = plan;
    let source_key = model
        .context
        .application
        .snapshot(Direction::Downstream)
        .key;
    assert!(model.attach_prepared(target.clone()).is_err());
    model.generation = Some(foreign.application.snapshot(Direction::Downstream).key);
    assert!(model.attach_prepared(target.clone()).is_err());
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        source_key
    );

    let key = target.application.configure_downstream(true).unwrap();
    model.generation = Some(key);
    let request = Arc::clone(model.request.as_ref().unwrap());
    let plan = Arc::clone(model.plan.as_ref().unwrap());
    let usage = target.application.budget().usage();
    model.attach_prepared(target.clone()).unwrap();
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        key
    );
    assert_eq!(target.application.budget().usage(), usage);
    assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
    assert!(Arc::ptr_eq(model.plan.as_ref().unwrap(), &plan));

    let mut incomplete = candidate();
    incomplete
        .prepare_inputs_for_budget(
            &settings,
            &Inputs::default(),
            &Sections::default(),
            target.application.budget(),
        )
        .unwrap();
    assert_eq!(
        incomplete.check_prepared_target(target.application.budget()),
        Ok(false)
    );
    assert!(!incomplete.error.message().is_empty());
    assert_eq!(incomplete.generation, None);
}

#[test]
fn tracked_real_execution_rejects_candidate_preparation_and_attachment() {
    let (inputs, settings, sections) = super::super::tests::accepted_inputs();
    let settings = serde_json::to_string(&settings).unwrap();
    let target = context();
    session::restore(&target, inputs.clone(), sections.clone()).unwrap();
    let mut model = candidate();
    model
        .prepare_inputs_for_budget(&settings, &inputs, &sections, target.application.budget())
        .unwrap();
    model.generation = Some(target.application.configure_downstream(true).unwrap());
    model.attach_prepared(target.clone()).unwrap();
    model.start();
    assert!(model.worker.is_some(), "{}", model.state());
    let request = Arc::clone(model.request.as_ref().unwrap());
    let key = model.generation;
    assert!(model
        .check_prepared_target(target.application.budget())
        .is_err());
    assert!(model
        .prepare_inputs_for_budget(&settings, &inputs, &sections, target.application.budget())
        .is_err());
    assert!(model.attach_prepared(target.clone()).is_err());
    assert_eq!(model.generation, key);
    assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
    model.stop();
    let deadline = Instant::now() + WATCHDOG;
    while model.worker.is_some() {
        model.poll();
        assert!(Instant::now() < deadline, "Receiver termination watchdog");
        std::thread::sleep(POLL_INTERVAL);
    }
    assert!(target.application.active().is_none());
}
