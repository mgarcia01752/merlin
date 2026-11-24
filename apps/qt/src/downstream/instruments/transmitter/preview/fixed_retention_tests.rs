/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete analyzer layout and real separately boxed preview product ownership.
use super::*;
use crate::shell::{ApplicationSessions, Direction};
use merlin::downstream::TransmissionSettings;
use std::{
    mem::size_of,
    time::{Duration, Instant},
};

// Independent actual-layout pressure and worker-disposal watchdog, not DSP limits.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const WATCHDOG: Duration = Duration::from_secs(30);

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap()
}

#[test]
fn analyzer_fixed_box_exact_and_one_byte_below_leave_existing_host_unchanged() {
    let measured_baseline = context().application.budget().usage();
    let fixed = size_of::<Analyzer>();
    for shortage in [false, true] {
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
        let candidate = Analyzer::for_context(context.clone());
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
            drop(candidate);
            assert_eq!(budget.usage(), baseline);
        }
    }
}

#[test]
fn analyzer_foreign_target_rollback_and_real_product_keep_fixed_pointees_separate_until_drop() {
    let source = context();
    let target = context();
    let source_baseline = source.application.budget().usage();
    let target_baseline = target.application.budget().usage();
    let mut model = Analyzer::for_context(source.clone()).unwrap();
    let settings = serde_json::to_string(&TransmissionSettings::default()).unwrap();
    tests::configure(&mut model, &settings, tests::inputs());
    let source_prepared = Arc::clone(model.prepared.as_ref().unwrap());
    let source_charge = source_prepared.fixture_reserved_resources();
    let key = source.application.snapshot(Direction::Downstream).key;
    let revision = model.revision;
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - target_baseline.payload_bytes,
            ..Default::default()
        })
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
        key
    );
    assert_eq!(model.revision, revision);
    assert!(Arc::ptr_eq(
        model.prepared.as_ref().unwrap(),
        &source_prepared
    ));
    assert!(source.application.budget().owns(&model.fixed_guard));
    drop(pressure);
    model.rebind_prepared(target.clone()).unwrap();
    assert!(target.application.budget().owns(&model.fixed_guard));
    let mut source_remaining = source_baseline;
    crate::downstream::inputs::retention::append(&mut source_remaining, source_charge).unwrap();
    assert_eq!(source.application.budget().usage(), source_remaining);
    let before_product = target.application.budget().usage();
    model.generate();
    let control = Arc::downgrade(&model.job.as_ref().unwrap().control);
    tests::wait(&mut model);
    assert_eq!(model.status()["has_result"], true);
    let product = model.result.as_ref().unwrap();
    assert!(product.value().info().sample_count > 0);
    let mut expected = before_product;
    crate::downstream::inputs::retention::append(
        &mut expected,
        product.fixture_reserved_resources(),
    )
    .unwrap();
    let deadline = Instant::now() + WATCHDOG;
    while control.upgrade().is_some() || target.application.budget().usage() != expected {
        assert!(
            Instant::now() < deadline,
            "Actual preview control cleanup watchdog"
        );
        std::thread::yield_now();
    }
    assert_eq!(target.application.budget().usage(), expected);
    drop(model);
    assert_eq!(target.application.budget().usage(), target_baseline);
    assert_eq!(source.application.budget().usage(), source_remaining);
    drop(source_prepared);
    assert_eq!(source.application.budget().usage(), source_baseline);
}
