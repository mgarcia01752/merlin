/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual detailed pointee admission, borrowed fallbacks and queue disposal.
use super::*;
use crate::{
    downstream::help,
    shell::{RetainedResources, RetentionReservation},
};
use serde_json::json;
use std::{mem::size_of, sync::mpsc};

// Independent fixture slack to expose actual requested String capacity.
const MESSAGE_SPARE_BYTES: usize = 19;
const MESSAGE: &str = "Original queued worker failure.";
const FALLBACK: &str = "Fixture failure details unavailable.";

// One actual completion payload, independent of scientific execution limits.
const QUEUE_CAPACITY: usize = 1;

fn plain() -> AdmissionError {
    let mut message = String::with_capacity(MESSAGE.len() + MESSAGE_SPARE_BYTES);
    message.push_str(MESSAGE);
    AdmissionError {
        message,
        diagnostic: None,
    }
}

fn fixed_layout() -> usize {
    size_of::<Retained<AdmissionError>>()
        .max(size_of::<(AdmissionError, Option<RetentionReservation>)>())
}

#[test]
fn exact_spare_capacity_and_one_byte_below_use_actual_retained_layout() {
    let error = plain();
    let expected = RetainedResources {
        payload_bytes: fixed_layout() + error.message.capacity(),
        ..Default::default()
    };
    let budget = RetentionBudget::with_limits(expected);
    let failure = WorkerFailure::admit_with_fallback(error, &budget, FALLBACK);
    assert!(matches!(&failure, WorkerFailure::Detailed(_)));
    assert_eq!(failure.message(), MESSAGE);
    assert!(failure.diagnostic().is_none());
    assert_eq!(budget.usage(), expected);
    drop(failure);
    assert_eq!(budget.usage(), RetainedResources::default());

    // Reuse this actual candidate's capacity; Clone capacity is not an assumption.
    let error = plain();
    let below = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: fixed_layout() + error.message.capacity() - 1,
        ..Default::default()
    });
    let failure = WorkerFailure::admit_with_fallback(error, &below, FALLBACK);
    assert!(matches!(&failure, WorkerFailure::AdmissionRejected(_)));
    assert_eq!(failure.message(), FALLBACK);
    assert_eq!(below.usage(), RetainedResources::default());
}

#[test]
fn diagnostic_queue_and_disconnected_publication_release_the_complete_pointee() {
    let diagnostic = help::catalog_diagnostic("QamRunUnavailable").unwrap();
    let error = AdmissionError {
        diagnostic: Some(Box::new(diagnostic)),
        ..plain()
    };
    let original = json!({"error":error.message,"diagnostic":error.diagnostic});
    let metadata = error.retained_resources().unwrap();
    let expected = RetainedResources {
        payload_bytes: metadata.payload_bytes - size_of::<AdmissionError>() + fixed_layout(),
        ..metadata
    };
    let budget = RetentionBudget::with_limits(expected);
    let failure = WorkerFailure::admit_with_fallback(error, &budget, FALLBACK);
    let (sender, receiver) = mpsc::sync_channel(QUEUE_CAPACITY);
    sender.send(failure).unwrap();
    assert_eq!(budget.usage(), expected);
    let failure = receiver.recv().unwrap();
    assert_eq!(
        json!({"error":failure.message(),"diagnostic":failure.diagnostic()}),
        original
    );
    assert_eq!(budget.usage(), expected);
    drop(receiver);

    // A disconnected send returns the same charged owner until its actual drop.
    let rejected = sender.send(failure).unwrap_err().0;
    assert_eq!(rejected.message(), MESSAGE);
    assert_eq!(budget.usage(), expected);
    drop(rejected);
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn static_payload_and_selected_fallback_need_no_owned_text_reservation() {
    let budget = RetentionBudget::with_limits(RetainedResources::default());
    let failure = WorkerFailure::admit_with_fallback(plain(), &budget, FALLBACK);
    assert_eq!(failure.message(), FALLBACK);
    assert!(failure.diagnostic().is_none());
    let borrowed = WorkerFailure::Static(MESSAGE);
    assert_eq!(borrowed.message().as_ptr(), MESSAGE.as_ptr());
    assert!(borrowed.diagnostic().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}
