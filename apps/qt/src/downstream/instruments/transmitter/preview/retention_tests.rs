/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public preview owners; synthetic pressure is accounting-only, never a backend substitute.
use super::*;
use crate::shell::{
    ApplicationSessions, Direction, RetainedResources, RetentionBudget, RetentionError,
};
use merlin::downstream::{
    PhysicalLayoutCarrier, PhysicalRoleSpan, TransmissionSettings, TransmitterPreviewState,
};
use std::{
    mem::size_of,
    time::{Duration, Instant},
};

// Independent fixture ceilings; production always uses the explicit canonical host budget.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

// Deliberate capacity anchors, unrelated to physical geometry or supported scientific limits.
const EXTRA_CARRIER_SLOTS: usize = 9;
const EXTRA_ROLE_SPAN_SLOTS: usize = 5;

// Existing preview-test scheduling policies; neither is an engine/display limit.
const WATCHDOG: Duration = Duration::from_secs(30);
const POLL_CADENCE: Duration = Duration::from_millis(10);

#[derive(Clone, Copy)]
enum Dimension {
    Payload,
    Entries,
    Roots,
}

fn quantity(resources: RetainedResources, dimension: Dimension) -> usize {
    match dimension {
        Dimension::Payload => resources.payload_bytes,
        Dimension::Entries => resources.ordered_entries,
        Dimension::Roots => resources.shared_roots,
    }
}
fn only(dimension: Dimension, value: usize) -> RetainedResources {
    match dimension {
        Dimension::Payload => RetainedResources {
            payload_bytes: value,
            ..Default::default()
        },
        Dimension::Entries => RetainedResources {
            ordered_entries: value,
            ..Default::default()
        },
        Dimension::Roots => RetainedResources {
            shared_roots: value,
            ..Default::default()
        },
    }
}
fn positive(resources: RetainedResources) -> impl Iterator<Item = Dimension> {
    [Dimension::Payload, Dimension::Entries, Dimension::Roots]
        .into_iter()
        .filter(move |dimension| quantity(resources, *dimension) > 0)
}
fn plus(a: RetainedResources, b: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: a.payload_bytes.checked_add(b.payload_bytes).unwrap(),
        ordered_entries: a.ordered_entries.checked_add(b.ordered_entries).unwrap(),
        shared_roots: a.shared_roots.checked_add(b.shared_roots).unwrap(),
    }
}
fn one_less(mut resources: RetainedResources, dimension: Dimension) -> RetainedResources {
    match dimension {
        Dimension::Payload => resources.payload_bytes -= 1,
        Dimension::Entries => resources.ordered_entries -= 1,
        Dimension::Roots => resources.shared_roots -= 1,
    }
    resources
}
fn budget() -> RetentionBudget {
    RetentionBudget::with_limits(FIXTURE_LIMIT)
}
fn context() -> Context {
    Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
}
fn settings() -> String {
    serde_json::to_string(&TransmissionSettings::default()).unwrap()
}
fn candidate() -> PreparedPreview {
    let timing = TransmissionConfiguration::from_json(settings().as_bytes()).unwrap();
    let (source, profile) = tests::inputs();
    let inputs = transmitter_inputs(&source, &profile).unwrap().unwrap();
    let request = TransmitterPreviewRequest::from_capture(
        &timing,
        &inputs.capture,
        inputs.profile_id,
        &inputs.source,
    )
    .unwrap();
    let layout = timing
        .physical_layout_capture(&inputs.capture, inputs.profile_id, 0, 0, 0)
        .unwrap()
        .layout;
    PreparedPreview {
        timing,
        preview: Some(PreviewInput { request, layout }),
    }
}
fn configured() -> Analyzer {
    let mut model = Analyzer::default();
    model.context = context();
    tests::configure(&mut model, &settings(), tests::inputs());
    assert_eq!(model.status()["status"], "complete");
    model
}
fn wait_completed(model: &Analyzer) {
    let deadline = Instant::now() + WATCHDOG;
    while model.job.as_ref().unwrap().control.snapshot().state != TransmitterPreviewState::Completed
    {
        assert!(
            Instant::now() < deadline,
            "Real preview completion watchdog"
        );
        std::thread::sleep(POLL_CADENCE);
    }
}

#[test]
fn prepared_layout_uses_actual_spare_capacities_and_complete_option_wrapper_storage() {
    let mut prepared = candidate();
    let before = retention::prepared_resources(&prepared).unwrap();
    let layout = &mut prepared.preview.as_mut().unwrap().layout;
    let carrier_slots = layout.carriers.capacity();
    let span_slots = layout.role_spans.capacity();
    layout.carriers.reserve(carrier_slots + EXTRA_CARRIER_SLOTS);
    layout
        .role_spans
        .reserve(span_slots + EXTRA_ROLE_SPAN_SLOTS);
    let added = (layout.carriers.capacity() - carrier_slots) * size_of::<PhysicalLayoutCarrier>()
        + (layout.role_spans.capacity() - span_slots) * size_of::<PhysicalRoleSpan>();
    let after = retention::prepared_resources(&prepared).unwrap();
    assert_eq!(after.payload_bytes - before.payload_bytes, added);
    assert_eq!(after.ordered_entries, before.ordered_entries);
    assert_eq!(after.shared_roots, before.shared_roots);
    let timing_only = PreparedPreview {
        timing: prepared.timing,
        preview: None,
    };
    assert_eq!(
        retention::prepared_resources(&timing_only).unwrap(),
        RetainedResources {
            payload_bytes: size_of::<PreparedPreview>()
                + size_of::<Retained<Arc<PreparedPreview>>>(),
            ordered_entries: 0,
            shared_roots: 2, // Actual inner preparation and retained outer wrapper.
        }
    );
    assert!(matches!(
        retention::product_resources(merlin::common::RetainedFootprint::default()),
        Err(RetentionError::ArithmeticOverflow)
    ));
    assert!(matches!(
        retention::product_resources(merlin::common::RetainedFootprint {
            payload_bytes: usize::MAX,
            ..Default::default()
        }),
        Err(RetentionError::ArithmeticOverflow)
    ));
}

#[test]
fn actual_prepared_candidate_rejects_each_positive_dimension_and_aliases_keep_old_charge() {
    let prepared = candidate();
    let resources = retention::prepared_resources(&prepared).unwrap();
    let exact = RetentionBudget::with_limits(resources);
    let owner = retention::admit(prepared, &exact).unwrap();
    assert_eq!(exact.usage(), resources);
    let worker_alias = Arc::clone(&owner);
    drop(owner);
    assert_eq!(exact.usage(), resources);
    assert_eq!(Arc::strong_count(&worker_alias), 1);
    drop(worker_alias);
    assert_eq!(exact.usage(), RetainedResources::default());
    for dimension in positive(resources) {
        let candidate = candidate();
        let actual = retention::prepared_resources(&candidate).unwrap();
        let limited = RetentionBudget::with_limits(one_less(actual, dimension));
        assert!(matches!(
            retention::admit(candidate, &limited),
            Err(PreviewError::Retention(_))
        ));
        assert_eq!(limited.usage(), RetainedResources::default());
    }
}

#[test]
fn prepared_rebind_precedes_aliasing_and_preserves_source_and_target_on_each_rejection() {
    let source_budget = budget();
    let source = retention::admit(candidate(), &source_budget).unwrap();
    let resources = source_budget.usage();
    for dimension in positive(resources) {
        let target = RetentionBudget::with_limits(one_less(resources, dimension));
        assert!(matches!(
            retention::rebind(&source, &target),
            Err(PreviewError::Retention(_))
        ));
        assert_eq!(target.usage(), RetainedResources::default());
        assert_eq!(source_budget.usage(), resources);
    }
    let target_budget = RetentionBudget::with_limits(resources);
    let rebound = retention::rebind(&source, &target_budget).unwrap();
    assert!(Arc::ptr_eq(source.value(), rebound.value()));
    assert_eq!(source_budget.usage(), resources);
    assert_eq!(target_budget.usage(), resources);
    let old = Arc::clone(&source);
    drop(source);
    assert_eq!(source_budget.usage(), resources);
    drop(old);
    assert_eq!(source_budget.usage(), RetainedResources::default());
    assert_eq!(target_budget.usage(), resources);
    drop(rebound);
    assert_eq!(target_budget.usage(), RetainedResources::default());
}

#[test]
fn target_rejection_and_precancel_happen_without_replacing_current_science() {
    let prepared = candidate();
    let preview = &prepared.preview.as_ref().unwrap().request;
    let target = retention::product_resources(preview.preview_retained_bound().unwrap()).unwrap();
    let source = retention::prepared_resources(&prepared).unwrap();
    for dimension in positive(target) {
        let prepared = candidate();
        let source = retention::prepared_resources(&prepared).unwrap();
        let preview = &prepared.preview.as_ref().unwrap().request;
        let target =
            retention::product_resources(preview.preview_retained_bound().unwrap()).unwrap();
        let limits = one_less(plus(source, target), dimension);
        let budget = RetentionBudget::with_limits(limits);
        let owner = retention::admit(prepared, &budget).unwrap();
        let control = TransmitterPreviewControl::new();
        assert!(matches!(
            retention::generate(Arc::clone(&owner), &control, &budget),
            Err(PreviewError::Retention(_))
        ));
        assert_eq!(control.snapshot().state, TransmitterPreviewState::Idle);
        assert_eq!(control.snapshot().completed_symbols, 0);
        assert_eq!(budget.usage(), source);
    }
    let budget = RetentionBudget::with_limits(plus(source, target));
    let owner = retention::admit(prepared, &budget).unwrap();
    let control = TransmitterPreviewControl::new();
    control.cancel();
    assert!(matches!(
        retention::generate(Arc::clone(&owner), &control, &budget),
        Err(PreviewError::Metadata(_))
    ));
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Cancelled);
    assert_eq!(budget.usage(), source);
}

#[test]
fn configuration_and_new_product_pressure_keep_the_same_accepted_source_result_and_revision() {
    let mut model = configured();
    model.generate();
    tests::wait(&mut model);
    let accepted = Arc::clone(model.prepared.as_ref().unwrap());
    let old_result = model.result.as_ref().unwrap().value() as *const TransmitterPreview;
    let revision = model.revision;
    let token = model.result_token;
    let budget = model.context.application.budget().clone();
    let baseline = budget.usage();
    let prepared = retention::prepared_resources(accepted.value()).unwrap();
    for dimension in positive(prepared) {
        // Explicit synthetic pressure fills this counter dimension only. It is not
        // a scientific byte estimate or a weakened backend target allowance.
        let pressure = budget
            .reserve_resources(only(
                dimension,
                quantity(FIXTURE_LIMIT, dimension) - quantity(baseline, dimension),
            ))
            .unwrap();
        tests::configure(&mut model, &settings(), tests::inputs());
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &accepted));
        assert_eq!(model.revision, revision);
        assert_eq!(model.result_token, token);
        assert_eq!(
            model.result.as_ref().unwrap().value() as *const _,
            old_result
        );
        assert_eq!(model.status()["status"], "error");
        drop(pressure);
        assert_eq!(budget.usage(), baseline);
    }
    let product = retention::product_resources(
        accepted
            .value()
            .preview
            .as_ref()
            .unwrap()
            .request
            .preview_retained_bound()
            .unwrap(),
    )
    .unwrap();
    for dimension in positive(product) {
        let pressure = budget
            .reserve_resources(only(
                dimension,
                quantity(FIXTURE_LIMIT, dimension) - quantity(baseline, dimension),
            ))
            .unwrap();
        model.generate();
        tests::wait(&mut model);
        assert_eq!(model.result_token, token);
        assert_eq!(
            model.result.as_ref().unwrap().value() as *const _,
            old_result
        );
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &accepted));
        drop(pressure);
        assert_eq!(budget.usage(), baseline);
    }
}

#[test]
fn genuine_box_queue_and_stale_discard_release_only_the_final_product_owner() {
    let mut model = configured();
    let budget = model.context.application.budget().clone();
    let product = retention::product_resources(
        model
            .prepared
            .as_ref()
            .unwrap()
            .value()
            .preview
            .as_ref()
            .unwrap()
            .request
            .preview_retained_bound()
            .unwrap(),
    )
    .unwrap();
    let before = budget.usage();
    model.generate();
    let queued = model
        .job
        .as_ref()
        .unwrap()
        .receiver
        .recv_timeout(WATCHDOG)
        .unwrap()
        .unwrap();
    assert_eq!(budget.usage(), plus(before, product));
    assert!(queued.value().info().sample_count > 0);
    drop(model);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
            .checked_add(product)
            .unwrap()
    );
    drop(queued);
    assert_eq!(
        budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );

    let mut stale = configured();
    let budget = stale.context.application.budget().clone();
    let before = budget.usage();
    stale.generate();
    wait_completed(&stale);
    assert!(budget.usage().payload_bytes > before.payload_bytes);
    tests::configure(&mut stale, &settings(), tests::inputs());
    tests::wait(&mut stale);
    assert!(stale.result.is_none());
    let prepared = retention::prepared_resources(stale.prepared.as_ref().unwrap().value()).unwrap();
    let metadata = context().application.budget().usage();
    assert_eq!(budget.usage(), plus(metadata, prepared));
}

#[test]
fn analyzer_rebind_is_transactional_and_rejects_active_or_result_bearing_transfer() {
    let mut model = configured();

    // Keep the source host alive independently of the transferred analyzer.
    // Its metadata and the old prepared alias have separate final owners.
    let source_context = model.context.clone();
    let old = Arc::clone(model.prepared.as_ref().unwrap());
    let source_budget = model.context.application.budget().clone();
    let original_host = model
        .context
        .application
        .snapshot(Direction::Downstream)
        .key;
    let target = context();
    let target_budget = target.application.budget().clone();
    let resources = retention::prepared_resources(old.value()).unwrap();
    let target_baseline = target_budget.usage();
    for dimension in positive(resources) {
        let pressure = target_budget
            .reserve_resources(only(
                dimension,
                quantity(FIXTURE_LIMIT, dimension) - quantity(target_baseline, dimension),
            ))
            .unwrap();
        assert!(matches!(
            model.rebind_prepared(target.clone()),
            Err(PreviewError::Retention(_))
        ));
        assert_eq!(
            model
                .context
                .application
                .snapshot(Direction::Downstream)
                .key,
            original_host
        );
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &old));
        drop(pressure);
        assert_eq!(target_budget.usage(), target_baseline);
    }
    let source_before = source_budget.usage();
    model.rebind_prepared(target.clone()).unwrap();
    assert!(Arc::ptr_eq(
        model.prepared.as_ref().unwrap().value(),
        old.value()
    ));
    assert_eq!(source_budget.usage(), source_before);
    assert_eq!(target_budget.usage(), plus(target_baseline, resources));
    model.generate();
    let untouched = context();
    let destination = untouched.application.budget().usage();
    assert!(matches!(
        model.rebind_prepared(untouched.clone()),
        Err(PreviewError::TransferNotIdle)
    ));
    assert_eq!(untouched.application.budget().usage(), destination);
    tests::wait(&mut model);
    assert!(model.result.is_some());
    let before = target_budget.usage();
    assert!(matches!(
        model.rebind_prepared(untouched.clone()),
        Err(PreviewError::TransferNotIdle)
    ));
    assert_eq!(target_budget.usage(), before);
    assert_eq!(untouched.application.budget().usage(), destination);
    assert!(Arc::ptr_eq(
        model.prepared.as_ref().unwrap().value(),
        old.value()
    ));
    drop(old);
    let source_metadata = context().application.budget().usage();
    assert_eq!(source_budget.usage(), source_metadata);
    drop(source_context);
    assert_eq!(
        source_budget.usage(),
        crate::shell::ApplicationSessions::fixture_counter_resources()
    );
}
