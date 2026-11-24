/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual fixed Context layouts, canonical host sharing and construction rollback.
use super::*;
use crate::downstream::{
    instruments::receiver::{performance::Entry, qam_history::Registry},
    simulation::closed_loop::ResultData,
    workbench::project_load::PreparedModels,
};
use crate::shell::{Direction, Retained, RetainedResources};
use std::{mem::size_of, sync::Mutex};

// A probe measures the existing host baseline; it is not a production policy.
const BASELINE_PROBE_LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

fn fixed_resources() -> RetainedResources {
    let layouts = [
        size_of::<Retained<Mutex<Option<AcceptedState>>>>(),
        size_of::<Retained<Mutex<Option<Arc<ResultData>>>>>(),
        size_of::<Retained<Mutex<Registry>>>(),
        size_of::<Retained<Mutex<Vec<Entry>>>>(),
        size_of::<Retained<Mutex<PreparedModels>>>(),
    ];
    RetainedResources {
        payload_bytes: layouts
            .into_iter()
            .try_fold(0usize, usize::checked_add)
            .unwrap(),
        ordered_entries: 0,
        shared_roots: layouts.len(),
    }
}

fn add(left: RetainedResources, right: RetainedResources) -> RetainedResources {
    RetainedResources {
        payload_bytes: left.payload_bytes.checked_add(right.payload_bytes).unwrap(),
        ordered_entries: left
            .ordered_entries
            .checked_add(right.ordered_entries)
            .unwrap(),
        shared_roots: left.shared_roots.checked_add(right.shared_roots).unwrap(),
    }
}

fn application_with_space(space: RetainedResources) -> ApplicationSessions {
    let probe = ApplicationSessions::new(BASELINE_PROBE_LIMITS).unwrap();
    let baseline = probe.budget().usage();
    drop(probe);
    ApplicationSessions::new(add(baseline, space)).unwrap()
}

#[test]
fn context_admits_exact_fixed_bytes_and_roots_without_changing_readiness() {
    let required = fixed_resources();
    let application = application_with_space(required);
    let baseline = application.budget().usage();
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let context = Context::for_application(application.clone()).unwrap();
    assert_eq!(application.budget().usage(), add(baseline, required));
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(application.snapshot(Direction::Upstream), upstream);
    assert!(application.active().is_none());
    assert!(context.accepted.borrow().is_none());
    assert!(context.result.borrow().is_none());
    assert!(context.qam.borrow().is_empty());
    assert!(context.performance.borrow().is_empty());
    let prepared = context.prepared.borrow();
    assert!(prepared.source.is_none());
    assert!(prepared.profile.is_none());
    assert!(prepared.measurements.is_none());
    assert!(prepared.receiver.is_none());
    assert!(prepared.transmitter.is_none());
    drop(prepared);
    drop(context);
    assert_eq!(application.budget().usage(), baseline);
}

#[test]
fn partial_context_construction_rolls_back_every_prior_store_on_pressure() {
    let required = fixed_resources();
    for available_roots in 0..required.shared_roots {
        let application = application_with_space(required);
        let baseline = application.budget().usage();
        let downstream = application.snapshot(Direction::Downstream);
        let pressure = application
            .budget()
            .reserve_resources(RetainedResources {
                shared_roots: required.shared_roots - available_roots,
                ..Default::default()
            })
            .unwrap();
        let pressured = application.budget().usage();
        assert!(matches!(
            Context::for_application(application.clone()),
            Err(RetentionError::SharedRootsExceeded { .. })
        ));
        assert_eq!(application.budget().usage(), pressured);
        assert_eq!(application.snapshot(Direction::Downstream), downstream);
        drop(pressure);
        assert_eq!(application.budget().usage(), baseline);
    }
    let application = application_with_space(RetainedResources {
        payload_bytes: required.payload_bytes - 1,
        ..required
    });
    let baseline = application.budget().usage();
    assert!(matches!(
        Context::for_application(application.clone()),
        Err(RetentionError::LimitExceeded { .. })
    ));
    assert_eq!(application.budget().usage(), baseline);
}

fn assert_store_identity<T>(original: &Store<T>, alias: &Store<T>, independent: &Store<T>) {
    assert!(original.downgrade().ptr_eq(&alias.downgrade()));
    assert!(!original.downgrade().ptr_eq(&independent.downgrade()));
}

#[test]
fn independent_contexts_share_the_host_and_clones_share_all_five_stores() {
    let required = fixed_resources();
    let application = application_with_space(add(required, required));
    application.configure_downstream(true).unwrap();
    let baseline = application.budget().usage();
    let downstream = application.snapshot(Direction::Downstream);
    let upstream = application.snapshot(Direction::Upstream);
    let original = Context::for_application(application.clone()).unwrap();
    let alias = original.clone();
    let independent = Context::for_application(application.clone()).unwrap();
    assert_store_identity(&original.accepted, &alias.accepted, &independent.accepted);
    assert_store_identity(&original.result, &alias.result, &independent.result);
    assert_store_identity(&original.qam, &alias.qam, &independent.qam);
    assert_store_identity(
        &original.performance,
        &alias.performance,
        &independent.performance,
    );
    assert_store_identity(&original.prepared, &alias.prepared, &independent.prepared);
    for context in [&original, &alias, &independent] {
        assert_eq!(
            context.application.snapshot(Direction::Downstream),
            downstream
        );
        assert_eq!(context.application.snapshot(Direction::Upstream), upstream);
    }
    assert!(application.active().is_none());
    assert_eq!(
        application.budget().usage(),
        add(baseline, add(required, required))
    );
    let released = original.accepted.downgrade();
    drop(original);
    assert!(released.upgrade().is_some());
    assert_eq!(
        application.budget().usage(),
        add(baseline, add(required, required))
    );
    drop(alias);
    assert!(released.upgrade().is_none());
    assert_eq!(application.budget().usage(), add(baseline, required));
    drop(independent);
    assert_eq!(application.budget().usage(), baseline);
}
