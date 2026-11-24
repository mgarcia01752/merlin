/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real public error categories survive bounded projection and admission fallback.
use super::*;
use crate::shared::presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE;
use merlin::upstream::{CaptureFailure, ErrorCode};
use std::sync::mpsc;

// Fixture host policy only; detailed errors introduce bytes, not entries/roots.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: usize::MAX,
    ordered_entries: usize::MAX,
    shared_roots: usize::MAX,
};

// Exceeds the existing Qt text policy to exercise its genuine truncation path.
const LONG_IO_MESSAGE_BYTES: usize = 8192;

// One genuine terminal projection; this does not model channel storage bytes.
const QUEUE_CAPACITY: usize = 1;

#[test]
fn opaque_io_error_becomes_only_the_original_category_and_bounded_charged_text() {
    let error = ApiError::Serialization(serde_json::Error::io(io::Error::other(
        "x".repeat(LONG_IO_MESSAGE_BYTES),
    )));
    let original = error.to_string();
    let expected = crate::upstream::workbench::error_text(original.clone());
    assert!(expected.len() < original.len());
    let budget = RetentionBudget::with_limits(LIMITS);
    let failure = RunFailure::admit(error, &budget);
    assert_eq!(failure.category(), ErrorCode::Io);
    assert_eq!(failure.message(), expected);
    assert_eq!(failure.to_string(), expected);
    let charged = budget.usage();
    assert!(charged.payload_bytes > expected.len());
    assert_eq!(charged.ordered_entries, 0);
    assert_eq!(charged.shared_roots, 0);
    let (sender, receiver) = mpsc::sync_channel(QUEUE_CAPACITY);
    sender.send(failure).unwrap();
    assert_eq!(budget.usage(), charged);
    drop(receiver);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn category_survives_zero_budget_fallback_for_real_public_failure_variants() {
    let budget = RetentionBudget::with_limits(RetainedResources::default());
    for error in [
        ApiError::MissingSetting,
        ApiError::Capture {
            category: CaptureFailure::ResourceLimit,
            field: "retained_footprint",
            offset: 0,
        },
        ApiError::AdapterFailure("fixture adapter failure"),
    ] {
        let category = error.category();
        let failure = RunFailure::admit(error, &budget);
        assert_eq!(failure.category(), category);
        assert_eq!(failure.message(), WORKER_FAILURE_ADMISSION_MESSAGE);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}
