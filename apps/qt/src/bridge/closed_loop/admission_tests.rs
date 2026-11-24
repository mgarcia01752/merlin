/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual Apply help ownership, independently retained from receiver status.
use super::*;
use crate::{
    downstream::{help, workbench::session::Context},
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::{ApplicationSessions, RetainedResources},
};
use serde_json::json;

// Independent accounting ceilings; these are not scientific/configuration limits.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};
const MESSAGE: &str = "Original receiver Apply diagnostic.";

fn owner() -> (Context, ClosedLoopRust, RetainedResources) {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let empty = context.application.budget().usage();
    let owner = ClosedLoopRust::for_context(context.clone());
    assert!(owner.model.is_some());
    (context, owner, empty)
}

fn error() -> help::AdmissionError {
    help::AdmissionError {
        message: MESSAGE.to_owned(),
        diagnostic: Some(Box::new(
            help::catalog_diagnostic("QamRunUnavailable").unwrap(),
        )),
    }
}

#[test]
fn actual_apply_help_survives_independently_without_changing_receiver_state() {
    let (context, mut owner, empty) = owner();
    let budget = context.application.budget();
    let baseline = budget.usage();
    let previous_state = owner.model.as_deref().unwrap().state();
    let candidate = error();
    let original_help = json!(candidate.diagnostic);
    assert_eq!(owner.admit_failure(candidate), MESSAGE);
    assert_eq!(json!(owner.admission_failure.diagnostic()), original_help);
    assert_eq!(owner.model.as_deref().unwrap().state(), previous_state);
    assert!(budget.usage().payload_bytes > baseline.payload_bytes);
    assert_eq!(budget.usage().ordered_entries, baseline.ordered_entries);
    assert_eq!(budget.usage().shared_roots, baseline.shared_roots);
    drop(owner);
    assert_eq!(budget.usage(), empty);
}

#[test]
fn actual_apply_detail_rejection_releases_old_help_and_keeps_receiver_state() {
    let (context, mut owner, _) = owner();
    let budget = context.application.budget();
    let baseline = budget.usage();
    let previous_state = owner.model.as_deref().unwrap().state();
    owner.admit_failure(error());
    let WorkerFailure::Detailed(retained) = &owner.admission_failure else {
        panic!("The original help must fit the independent fixture host");
    };
    let old_charge = retained.fixture_reserved_resources();
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - budget.usage().payload_bytes,
            ..Default::default()
        })
        .unwrap();
    assert_eq!(
        owner.admit_failure(error()),
        WORKER_FAILURE_ADMISSION_MESSAGE
    );
    assert!(owner.admission_failure.diagnostic().is_none());
    assert_eq!(owner.model.as_deref().unwrap().state(), previous_state);
    assert_eq!(
        budget.usage().payload_bytes,
        LIMITS.payload_bytes - old_charge.payload_bytes
    );
    assert_eq!(budget.usage().ordered_entries, baseline.ordered_entries);
    assert_eq!(budget.usage().shared_roots, baseline.shared_roots);
    drop(pressure);
    assert_eq!(budget.usage(), baseline);
    owner.admission_failure = WorkerFailure::Static("");
    assert_eq!(budget.usage(), baseline);
}

#[test]
fn fixed_receiver_rejection_keeps_only_host_handles_and_retries_without_host_changes() {
    let context = Context::for_application(ApplicationSessions::new(LIMITS).unwrap()).unwrap();
    let budget = context.application.budget();
    let baseline = budget.usage();
    let before = context
        .application
        .snapshot(crate::shell::Direction::Downstream);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: LIMITS.payload_bytes - baseline.payload_bytes,
            ..Default::default()
        })
        .unwrap();
    let mut owner = ClosedLoopRust::for_context(context.clone());
    assert!(owner.model.is_none());
    assert!(owner.ensure_model().is_none());
    assert_eq!(ClosedLoopRust::unavailable_state()["can_start"], false);
    assert_eq!(
        context
            .application
            .snapshot(crate::shell::Direction::Downstream),
        before
    );
    assert!(owner.admission_failure.diagnostic().is_none());
    drop(pressure);
    assert!(owner.ensure_model().is_some());
    assert_eq!(
        context
            .application
            .snapshot(crate::shell::Direction::Downstream),
        before
    );
    assert!(budget.usage().payload_bytes > baseline.payload_bytes);
    drop(owner);
    assert_eq!(budget.usage(), baseline);
}
