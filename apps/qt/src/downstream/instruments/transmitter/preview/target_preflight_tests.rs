/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Target-budget preview preparation and guard-preserving GUI attachment fixtures.
use super::*;
use crate::shell::{ApplicationSessions, Direction, RetainedResources};
use merlin::downstream::TransmissionSettings;

// Fixture-only ceilings, independent of production and scientific limits.
const FIXTURE_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn context() -> Context {
    Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
}

fn settings() -> String {
    serde_json::to_string(&TransmissionSettings::default()).unwrap()
}

#[test]
fn target_preparation_charges_actual_roots_and_each_positive_dimension_rejects_atomically() {
    let target = context();
    let budget = target.application.budget();
    let baseline = budget.usage();
    let target_key = target.application.snapshot(Direction::Downstream).key;
    let mut model = Analyzer::default();
    let source_key = model
        .context
        .application
        .snapshot(Direction::Downstream)
        .key;
    let (source, profile) = tests::inputs();
    model
        .configure_for_budget(&settings(), &source, &profile, budget)
        .unwrap();
    model.check_prepared_target(budget).unwrap();
    let prepared = Arc::clone(model.prepared.as_ref().unwrap());
    let actual = prepared.fixture_reserved_resources();
    let usage = budget.usage();
    let revision = model.revision;
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
    assert_eq!(model.status()["status"], "complete");

    // Saturation uses actual host usage; zero-population dimensions have no
    // scientific target to reject and are deliberately excluded from this loop.
    for (quantity, pressure) in [
        (
            actual.payload_bytes,
            RetainedResources {
                payload_bytes: usize::MAX - usage.payload_bytes,
                ..Default::default()
            },
        ),
        (
            actual.ordered_entries,
            RetainedResources {
                ordered_entries: usize::MAX - usage.ordered_entries,
                ..Default::default()
            },
        ),
        (
            actual.shared_roots,
            RetainedResources {
                shared_roots: usize::MAX - usage.shared_roots,
                ..Default::default()
            },
        ),
    ] {
        if quantity == 0 {
            continue;
        }
        let guard = budget.reserve_resources(pressure).unwrap();
        let pressured = budget.usage();
        assert!(model
            .configure_for_budget(&settings(), &source, &profile, budget)
            .is_err());
        assert_eq!(budget.usage(), pressured);
        assert_eq!(model.revision, revision);
        assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &prepared));
        assert_eq!(
            target.application.snapshot(Direction::Downstream).key,
            target_key
        );
        drop(guard);
    }
    drop(model);
    assert_eq!(budget.usage(), usage);
    drop(prepared);
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn same_budget_transfer_reuses_the_exact_guard_under_full_pressure() {
    let target = context();
    let foreign = context();
    let mut model = Analyzer::default();
    let (source, profile) = tests::inputs();
    model
        .configure_for_budget(&settings(), &source, &profile, target.application.budget())
        .unwrap();
    assert!(matches!(
        model.check_prepared_target(foreign.application.budget()),
        Err(PreviewError::ForeignBudget)
    ));
    let prepared = Arc::clone(model.prepared.as_ref().unwrap());
    let budget = target.application.budget();
    let usage = budget.usage();
    let key = target.application.snapshot(Direction::Downstream).key;
    let guard = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX - usage.payload_bytes,
            ordered_entries: usize::MAX - usage.ordered_entries,
            shared_roots: usize::MAX - usage.shared_roots,
        })
        .unwrap();
    model.check_prepared_target(budget).unwrap();
    model.rebind_prepared(target.clone()).unwrap();
    assert!(Arc::ptr_eq(model.prepared.as_ref().unwrap(), &prepared));
    assert_eq!(budget.usage(), FIXTURE_LIMIT);
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        key
    );
    drop(guard);
    assert_eq!(budget.usage(), usage);
}

#[test]
fn ordinary_readiness_survives_and_tracked_real_preview_cannot_transfer() {
    let target = context();
    let budget = target.application.budget();
    let (source, profile) = tests::inputs();
    let mut candidate = Analyzer::default();
    let mut legacy = Analyzer::default();
    for (document, source, profile) in [
        ("{}".to_owned(), source.clone(), profile.clone()),
        (settings(), Value::Null, Value::Null),
    ] {
        candidate
            .configure_for_budget(&document, &source, &profile, budget)
            .unwrap();
        legacy.configure(&document, &source, &profile);
        assert_eq!(candidate.status(), legacy.status());
        candidate.check_prepared_target(budget).unwrap();
    }
    candidate
        .configure_for_budget(&settings(), &source, &profile, budget)
        .unwrap();
    candidate.rebind_prepared(target.clone()).unwrap();
    candidate.generate();
    assert!(candidate.job.is_some());
    let prepared = Arc::clone(candidate.prepared.as_ref().unwrap());
    let revision = candidate.revision;
    assert!(matches!(
        candidate.check_prepared_target(budget),
        Err(PreviewError::TransferNotIdle)
    ));
    assert!(candidate
        .configure_for_budget(&settings(), &source, &profile, budget)
        .is_err());
    assert!(candidate.rebind_prepared(target.clone()).is_err());
    assert!(Arc::ptr_eq(candidate.prepared.as_ref().unwrap(), &prepared));
    assert_eq!(candidate.revision, revision);
    tests::wait(&mut candidate);
    assert!(candidate.result.is_some(), "{}", candidate.status());
    assert!(matches!(
        candidate.check_prepared_target(budget),
        Err(PreviewError::TransferNotIdle)
    ));
    assert!(candidate
        .configure_for_budget(&settings(), &source, &profile, budget)
        .is_err());
}

#[test]
fn invalid_target_metadata_is_strict_and_persistent_error_rebind_preserves_source_on_rejection() {
    const INVALID_DOCUMENT: &str = "not a transmission document";
    let target = context();
    let mut model = Analyzer::for_context(context());
    let source_key = model
        .context
        .application
        .snapshot(Direction::Downstream)
        .key;
    model.configure(INVALID_DOCUMENT, &Value::Null, &Value::Null);
    assert!(!model.error.message().is_empty());
    let original = model.error.message().to_owned();
    let revision = model.revision;
    let source_usage = model.context.application.budget().usage();
    assert!(matches!(
        model.check_prepared_target(target.application.budget()),
        Err(PreviewError::ForeignBudget)
    ));
    let usage = target.application.budget().usage();
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: FIXTURE_LIMIT.payload_bytes - usage.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert!(matches!(
        model.rebind_prepared(target.clone()),
        Err(PreviewError::PersistentMetadata(_))
    ));
    assert!(model
        .configure_for_budget(
            INVALID_DOCUMENT,
            &Value::Null,
            &Value::Null,
            target.application.budget()
        )
        .is_err());
    assert_eq!(
        model
            .context
            .application
            .snapshot(Direction::Downstream)
            .key,
        source_key
    );
    assert_eq!(model.context.application.budget().usage(), source_usage);
    assert_eq!(model.error.message(), original);
    assert_eq!(model.revision, revision);
    drop(pressure);
    model.rebind_prepared(target.clone()).unwrap();
    assert!(model.error.owns(target.application.budget()));
    assert_eq!(model.error.message(), original);
    let usage = target.application.budget().usage();
    let pressure = target
        .application
        .budget()
        .reserve_resources(RetainedResources {
            payload_bytes: FIXTURE_LIMIT.payload_bytes - usage.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    model.rebind_prepared(target.clone()).unwrap();
    assert_eq!(
        target.application.budget().usage().payload_bytes,
        FIXTURE_LIMIT.payload_bytes
    );
    drop(pressure);
    assert_eq!(target.application.budget().usage(), usage);
}
