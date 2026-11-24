/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real accepted request/plan quantities and partial application-owner lifetimes.
use super::super::{latest, prepare_sweep, Context, Model, Options, TransmissionSettings};
use super::*;
use crate::shell::{ApplicationSessions, Direction, DispatchPhase};
use merlin::downstream::ChannelJobStatus;
use std::time::{Duration, Instant};

// Test-only ceilings; none is a production default or scientific estimate.
const TEST_MAXIMUM: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
// A short real observation reaches completed delivery; the long fixture can pause.
const COMPLETE_OBSERVATION_MS: u64 = 1;
const PAUSABLE_OBSERVATION_MS: u64 = 1000;
const CHANGED_OBSERVATION_MS: u64 = 2;
const ACTIVE_EXECUTION_LIMIT_MS: u64 = 60000;
// Host scheduling watchdogs are independent of scientific execution limits.
const WATCHDOG: Duration = Duration::from_secs(20);
const POLL_INTERVAL: Duration = Duration::from_millis(5);

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn sum(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
    }
}

fn limited(actual: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: actual.payload_bytes,
            ..TEST_MAXIMUM
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: actual.ordered_entries,
            ..TEST_MAXIMUM
        },
        Dimension::Roots => RetainedResources {
            shared_roots: actual.shared_roots,
            ..TEST_MAXIMUM
        },
    }
}

fn one_less(actual: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: actual.payload_bytes.checked_sub(1).unwrap(),
            ..actual
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: actual.ordered_entries.checked_sub(1).unwrap(),
            ..actual
        },
        Dimension::Roots => RetainedResources {
            shared_roots: actual.shared_roots.checked_sub(1).unwrap(),
            ..actual
        },
    }
}

fn rejected<T>(result: Result<T, OwnerError>, dimension: Dimension) {
    let Err(OwnerError::Retention(error)) = result else {
        panic!("A real insufficient owner dimension must reject");
    };
    assert!(matches!(
        (dimension, error),
        (Dimension::Payload, RetentionError::LimitExceeded { .. })
            | (
                Dimension::Entries,
                RetentionError::OrderedEntriesExceeded { .. }
            )
            | (Dimension::Roots, RetentionError::SharedRootsExceeded { .. })
    ));
}

fn base_resources() -> RetainedResources {
    Context::for_application(ApplicationSessions::new(TEST_MAXIMUM).unwrap())
        .unwrap()
        .application
        .budget()
        .usage()
}

fn owners(model: &Model) -> RetainedResources {
    let request = resources::<FullLoopRequest>(
        model
            .request
            .as_ref()
            .unwrap()
            .value()
            .retained_footprint()
            .unwrap(),
    )
    .unwrap();
    let plan = resources::<FullLoopSweepPlan>(
        model
            .plan
            .as_ref()
            .unwrap()
            .value()
            .retained_footprint()
            .unwrap(),
    )
    .unwrap();
    sum(request, plan)
}

fn report_allowance(request: &FullLoopRequest) -> RetainedResources {
    let budget = super::super::result_retention::fixture_budget();
    let prepared = super::super::result_retention::PreparedResult::new(request, &budget).unwrap();
    let actual = budget.usage();
    drop(prepared);
    assert_eq!(budget.usage(), RetainedResources::default());
    actual
}

fn working_allowance(request: &FullLoopRequest) -> RetainedResources {
    let budget = super::super::result_retention::fixture_budget();
    let owner =
        admit_run_request(request.clone(), request.options().max_runtime_ms, &budget).unwrap();
    let actual = budget.usage();
    drop(owner);
    assert_eq!(budget.usage(), RetainedResources::default());
    actual
}

fn prepared_model(duration_ms: u64, tight: Option<Dimension>) -> (Model, String) {
    let (inputs, transmission, sections) = super::super::tests::accepted_inputs();
    let options = Options {
        duration_ms,
        max_runtime_ms: ACTIVE_EXECUTION_LIMIT_MS,
        ..Default::default()
    };
    let plan = prepare_sweep(&transmission, &inputs, &options, &sections).unwrap();
    let request = plan.request(0).unwrap();
    let initial = crate::downstream::instruments::receiver::performance::fixture_initial_resources(
        inputs.profiles.iter().filter(|profile| {
            plan.info()
                .selected_profile_ids
                .iter()
                .any(|id| id.value() == profile.profile_id)
        }),
    );
    let actual = sum(
        resources::<FullLoopRequest>(request.retained_footprint().unwrap()).unwrap(),
        resources::<FullLoopSweepPlan>(plan.retained_footprint().unwrap()).unwrap(),
    );
    assert!(actual.payload_bytes > 0 && actual.ordered_entries > 0 && actual.shared_roots > 0);
    let input_charge =
        crate::downstream::workbench::session::fixture_candidate_resources(&inputs, &sections);
    let maximum = tight.map_or(TEST_MAXIMUM, |dimension| {
        // The test first completes real science. Keep its independently preflighted
        // result and active working-request charges before pressuring a new candidate.
        limited(
            sum(
                sum(sum(base_resources(), input_charge), initial),
                sum(
                    actual,
                    sum(working_allowance(&request), report_allowance(&request)),
                ),
            ),
            dimension,
        )
    });
    let context = Context::for_application(ApplicationSessions::new(maximum).unwrap()).unwrap();
    crate::downstream::workbench::session::restore(&context, inputs, sections).unwrap();
    // Admit canonical metadata while the actual prepared scientific candidates
    // still belong to their finite preparation builders, before final wrappers.
    let key = context.application.configure_downstream(true).unwrap();
    let request = admit_request(request, context.application.budget()).unwrap();
    let plan = admit_plan(plan, context.application.budget()).unwrap();
    let mut model = Model::default();
    model.context = context;
    model.options = Some(options);
    model.verify = true;
    model.request = Some(request);
    model.plan = Some(plan);
    model.generation = Some(key);
    (model, serde_json::to_string(&transmission).unwrap())
}

fn wait_for(mut ready: impl FnMut() -> bool) {
    let deadline = Instant::now() + WATCHDOG;
    while !ready() {
        assert!(Instant::now() < deadline, "Actual owner lifecycle watchdog");
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn exact_public_quantities_and_cross_budget_aliases_cover_both_real_roots() {
    let (mut model, _) = prepared_model(COMPLETE_OBSERVATION_MS, None);
    let source = model.context.application.clone();
    let request = model.request.take().unwrap();
    let plan = model.plan.take().unwrap();
    let request_charge =
        resources::<FullLoopRequest>(request.value().retained_footprint().unwrap()).unwrap();
    let plan_charge =
        resources::<FullLoopSweepPlan>(plan.value().retained_footprint().unwrap()).unwrap();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let count = Arc::strong_count(request.value());
        if matches!(dimension, Dimension::Entries) && request_charge.ordered_entries == 0 {
            // A root without ordered entries must fit an exact zero-entry policy;
            // the real plan's catalog still exercises entry-limit rejection below.
            let budget = RetentionBudget::with_limits(request_charge);
            let alias = rebind_request(&request, &budget).unwrap();
            assert_eq!(budget.usage().ordered_entries, 0);
            drop(alias);
            assert_eq!(budget.usage(), RetainedResources::default());
        } else {
            let budget = RetentionBudget::with_limits(one_less(request_charge, dimension));
            rejected(rebind_request(&request, &budget), dimension);
            assert_eq!(budget.usage(), RetainedResources::default());
        }
        assert_eq!(Arc::strong_count(request.value()), count);
        let budget = RetentionBudget::with_limits(one_less(plan_charge, dimension));
        let count = Arc::strong_count(plan.value());
        rejected(rebind_plan(&plan, &budget), dimension);
        assert_eq!(Arc::strong_count(plan.value()), count);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
    let target = RetentionBudget::with_limits(sum(request_charge, plan_charge));
    let rebound_request = rebind_request(&request, &target).unwrap();
    let rebound_plan = rebind_plan(&plan, &target).unwrap();
    assert!(Arc::ptr_eq(request.value(), rebound_request.value()));
    assert!(Arc::ptr_eq(plan.value(), rebound_plan.value()));
    assert_eq!(target.usage(), sum(request_charge, plan_charge));
    let alias = Arc::clone(&rebound_request);
    drop(request);
    drop(plan);
    assert_eq!(
        source.budget().usage(),
        sum(
            base_resources(),
            crate::downstream::workbench::session::fixture_reserved_resources(&model.context)
        )
    );
    drop(rebound_request);
    drop(rebound_plan);
    assert_eq!(target.usage(), request_charge);
    drop(alias);
    assert_eq!(target.usage(), RetainedResources::default());
}

#[test]
fn every_real_candidate_dimension_preserves_accepted_science_generation_and_report() {
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let (mut model, settings) = prepared_model(COMPLETE_OBSERVATION_MS, Some(dimension));
        let context = model.context.clone();
        model.start();
        wait_for(|| model.poll()["busy"] == false);
        let report = latest(&context).expect("Accepted real science must complete");
        assert_eq!(report.job.status, ChannelJobStatus::Complete);
        let request = Arc::clone(model.request.as_ref().unwrap());
        let plan = Arc::clone(model.plan.as_ref().unwrap());
        let key = model.generation;
        let before = context.application.budget().usage();
        let accepted_options = serde_json::to_value(&model.options).unwrap();
        let mut changed = model.options.clone().unwrap();
        changed.duration_ms = CHANGED_OBSERVATION_MS;
        let inputs = crate::downstream::workbench::session::snapshot(&context);
        let sections = crate::downstream::workbench::session::saved_sections(&context);
        let transmission: TransmissionSettings = serde_json::from_str(&settings).unwrap();
        prepare_sweep(&transmission, &inputs, &changed, &sections).unwrap();
        assert!(model
            .accept_options(Some(changed), false, &settings)
            .is_err());
        assert_eq!(model.generation, key);
        assert_eq!(
            serde_json::to_value(&model.options).unwrap(),
            accepted_options
        );
        assert!(model.verify);
        assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
        assert!(Arc::ptr_eq(model.plan.as_ref().unwrap(), &plan));
        assert!(Arc::ptr_eq(&latest(&context).unwrap(), &report));
        assert_eq!(context.application.budget().usage(), before);
        assert!(context.application.active().is_none());
        assert!(model.error.contains("retained-data limit"));
    }
}

#[test]
fn actual_worker_retains_old_owners_while_a_new_candidate_is_accepted() {
    let (mut model, settings) = prepared_model(PAUSABLE_OBSERVATION_MS, None);
    let application = model.context.application.clone();
    let base = sum(
        base_resources(),
        crate::downstream::workbench::session::fixture_reserved_resources(&model.context),
    );
    let old_charge = owners(&model);
    let old_result_charge = report_allowance(model.request.as_ref().unwrap().value());
    let old_working_charge = working_allowance(model.request.as_ref().unwrap().value());
    let old_request = Arc::downgrade(model.request.as_ref().unwrap());
    let old_plan = Arc::downgrade(model.plan.as_ref().unwrap());
    model.start();
    model.pause();
    wait_for(|| {
        model.poll();
        application
            .active()
            .is_some_and(|run| run.phase == DispatchPhase::Paused)
    });
    assert!(old_request.strong_count() >= 2 && old_plan.strong_count() >= 2);
    // Actual acknowledged pause keeps the execution input and target alive.
    assert_eq!(
        application.budget().usage(),
        sum(
            sum(
                base,
                crate::downstream::instruments::receiver::performance::fixture_reserved_resources(
                    &model.context
                )
            ),
            sum(old_charge, sum(old_working_charge, old_result_charge))
        )
    );
    let mut changed = model.options.clone().unwrap();
    changed.duration_ms = CHANGED_OBSERVATION_MS;
    model
        .accept_options(Some(changed), true, &settings)
        .unwrap();
    let new_charge = owners(&model);
    // Profile plus Finished delivery cannot both pass the unchanged one-slot
    // backpressure before GUI polling; the old closure still owns both aliases.
    assert!(old_request.upgrade().is_some() && old_plan.upgrade().is_some());
    let stable = sum(base, sum(old_result_charge, sum(old_charge, new_charge)));
    let actual = application.budget().usage();
    // Cancellation can finish the numerical call before Profile delivery. Only
    // that call's execution-input charge may already have dropped; queued science
    // and accepted source aliases remain charged until the original terminal gate.
    // Display query/target copies may now coexist after cancellation. The stable
    // source/result floor remains; only the execution input adds scientific roots.
    assert!(actual.payload_bytes >= stable.payload_bytes);
    assert!(actual.ordered_entries >= stable.ordered_entries);
    assert!(actual.shared_roots >= stable.shared_roots);
    assert!(actual.shared_roots <= sum(stable, old_working_charge).shared_roots);
    assert_eq!(
        application.active().unwrap().phase,
        DispatchPhase::Cancelling
    );
    wait_for(|| model.poll()["busy"] == false);
    assert!(old_request.upgrade().is_none() && old_plan.upgrade().is_none());
    assert_eq!(application.budget().usage(), sum(base, new_charge));
    assert!(latest(&model.context).is_none());
    assert_eq!(
        model.options.as_ref().unwrap().duration_ms,
        CHANGED_OBSERVATION_MS
    );
    assert!(model.invalidate());
    assert_eq!(application.budget().usage(), base);
}

#[test]
fn rebind_reserves_both_target_roots_before_replacing_source_ownership() {
    let (mut model, _) = prepared_model(COMPLETE_OBSERVATION_MS, None);
    // Keep the accepted source Context, not just its host metadata, alive while
    // testing transfer rollback. Its newly admitted Inputs have their own lifetime.
    let source_context = model.context.clone();
    let source = model.context.application.clone();
    let request = Arc::clone(model.request.as_ref().unwrap());
    let plan = Arc::clone(model.plan.as_ref().unwrap());
    let charge = owners(&model);
    let key = model.generation;
    let source_usage = source.budget().usage();
    for dimension in [Dimension::Payload, Dimension::Entries, Dimension::Roots] {
        let target = Context::for_application(
            ApplicationSessions::new(limited(
                one_less(sum(base_resources(), charge), dimension),
                dimension,
            ))
            .unwrap(),
        )
        .unwrap();
        let snapshot = target.application.snapshot(Direction::Downstream);
        assert!(model.rebind(target.clone()).is_err());
        assert_eq!(target.application.snapshot(Direction::Downstream), snapshot);
        assert_eq!(target.application.budget().usage(), base_resources());
        assert_eq!(model.generation, key);
        assert_eq!(
            model.context.application.snapshot(Direction::Downstream),
            source.snapshot(Direction::Downstream)
        );
        assert_eq!(source.budget().usage(), source_usage);
        assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
        assert!(Arc::ptr_eq(model.plan.as_ref().unwrap(), &plan));
    }
    let target = Context::isolated();
    let target_application = target.application.clone();
    model.rebind(target.clone()).unwrap();
    assert!(!Arc::ptr_eq(model.request.as_ref().unwrap(), &request));
    assert!(!Arc::ptr_eq(model.plan.as_ref().unwrap(), &plan));
    assert!(Arc::ptr_eq(
        model.request.as_ref().unwrap().value(),
        request.value()
    ));
    assert!(Arc::ptr_eq(
        model.plan.as_ref().unwrap().value(),
        plan.value()
    ));
    assert_eq!(source.budget().usage(), source_usage);
    assert_eq!(
        target_application.budget().usage(),
        sum(base_resources(), charge)
    );
    let rebound = Arc::clone(model.request.as_ref().unwrap());
    model.rebind(target).unwrap();
    assert!(Arc::ptr_eq(model.request.as_ref().unwrap(), &rebound));
    assert_eq!(
        target_application.budget().usage(),
        sum(base_resources(), charge)
    );
    drop(request);
    drop(plan);
    drop(source_context);
    // The source Context is gone; only its independently retained host remains.
    assert_eq!(
        source.budget().usage(),
        ApplicationSessions::new(TEST_MAXIMUM)
            .unwrap()
            .budget()
            .usage()
    );
    assert!(model.invalidate());
    assert_eq!(
        target_application.budget().usage(),
        sum(
            base_resources(),
            resources::<FullLoopRequest>(rebound.value().retained_footprint().unwrap()).unwrap()
        )
    );
    drop(rebound);
    assert_eq!(target_application.budget().usage(), base_resources());
}

#[test]
fn wrapper_layout_adds_scientific_root_once_and_checks_every_arithmetic_boundary() {
    let scientific = RetainedFootprint {
        payload_bytes: size_of::<FullLoopRequest>(),
        ..Default::default()
    };
    let actual = resources::<FullLoopRequest>(scientific).unwrap();
    assert_eq!(
        actual.payload_bytes,
        size_of::<FullLoopRequest>() + size_of::<Retained<Arc<FullLoopRequest>>>()
    );
    assert_eq!(actual.ordered_entries, 0);
    assert_eq!(actual.shared_roots, OWNER_SHARED_ROOTS);
    assert_eq!(
        resources::<FullLoopRequest>(RetainedFootprint {
            payload_bytes: usize::MAX,
            ..scientific
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
    assert_eq!(
        resources::<FullLoopRequest>(RetainedFootprint {
            shared_roots: usize::MAX,
            ..scientific
        }),
        Err(RetentionError::ArithmeticOverflow)
    );
}

#[path = "execution_tests.rs"]
mod execution_tests;
