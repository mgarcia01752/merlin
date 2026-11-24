/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Admitted detailed completions and payload-free fallback under host pressure.
use super::*;
use crate::{
    downstream::{
        common::constants::DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE,
        help::{self, AdmissionError},
    },
    shell::{RetainedResources, RetentionBudget},
};
use serde_json::json;
use std::sync::mpsc;

// Independent test counter ceiling, not application or scientific policy.
const TEST_LIMIT: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
    channel_count: usize::MAX,
    slot_count: usize::MAX,
};
// Exactly one terminal fixture message, matching either real Designer queue.
const TERMINAL_CAPACITY: usize = 1;

fn error() -> AdmissionError {
    AdmissionError {
        message: "Original worker error with preserved diagnostic.".into(),
        diagnostic: Some(Box::new(
            help::catalog_diagnostic("QamRunUnavailable").unwrap(),
        )),
    }
}

#[test]
fn detailed_queued_error_keeps_original_rendering_and_charge_through_final_drop() {
    let budget = RetentionBudget::with_limits(TEST_LIMIT);
    let error = error();
    let expected = json!({"error":error.message,"diagnostic":error.diagnostic});
    let failure = WorkerFailure::admit(error, &budget);
    let WorkerFailure::Detailed(retained) = &failure else {
        panic!("Actual diagnostic fits the fixture host");
    };
    let charge = retained.fixture_reserved_resources();
    assert!(charge.payload_bytes > 0);
    assert_eq!(charge.ordered_entries, 0);
    assert_eq!(charge.shared_roots, 0);
    let (sender, receiver) = mpsc::sync_channel(TERMINAL_CAPACITY);
    sender.send(failure).unwrap();
    drop(sender);
    assert_eq!(budget.usage(), charge);
    let failure = receiver.recv().unwrap();
    assert_eq!(
        json!({"error":failure.message(),"diagnostic":failure.diagnostic()}),
        expected
    );
    assert_eq!(budget.usage(), charge);
    drop(failure);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn rejected_error_queues_only_static_fallback_without_changing_the_full_counter() {
    let budget = RetentionBudget::with_limits(TEST_LIMIT);
    let pressure = budget
        .reserve_resources(RetainedResources {
            payload_bytes: usize::MAX,
            ..Default::default()
        })
        .unwrap();
    let before = budget.usage();
    let failure = WorkerFailure::admit(error(), &budget);
    assert!(matches!(failure, WorkerFailure::AdmissionRejected(_)));
    let (sender, receiver) = mpsc::sync_channel(TERMINAL_CAPACITY);
    sender.send(failure).unwrap();
    drop(sender);
    let failure = receiver.recv().unwrap();
    assert_eq!(failure.message(), DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE);
    assert!(failure.diagnostic().is_none());
    assert_eq!(budget.usage(), before);
    drop(failure);
    drop(pressure);
    assert_eq!(budget.usage(), RetainedResources::default());
}
