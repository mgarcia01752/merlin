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

#[test]
fn strict_target_copy_rejection_keeps_original_help_and_normal_replacement_falls_back() {
    let source = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    });
    let original = WorkerFailure::try_admit(
        AdmissionError {
            diagnostic: Some(Box::new(
                help::catalog_diagnostic("QamRunUnavailable").unwrap(),
            )),
            ..plain()
        },
        &source,
    )
    .unwrap();
    let before = json!({"message":original.message(),"help":original.diagnostic()});
    let used = source.usage();
    let blocked = RetentionBudget::with_limits(RetainedResources::default());
    assert!(original.try_copy_for_budget(&blocked).is_err());
    assert_eq!(
        json!({"message":original.message(),"help":original.diagnostic()}),
        before
    );
    assert_eq!(source.usage(), used);
    assert_eq!(blocked.usage(), RetainedResources::default());
    let replacement = original.replace_message(MESSAGE.to_owned(), &blocked, FALLBACK);
    assert_eq!(replacement.message(), FALLBACK);
    assert!(replacement.diagnostic().is_none());
    assert_eq!(source.usage(), used);
    drop(original);
    assert_eq!(source.usage(), RetainedResources::default());
}

#[test]
fn message_only_replacement_admits_actual_help_clone_while_original_stays_charged() {
    let budget = RetentionBudget::with_limits(RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    });
    let original = WorkerFailure::try_admit(
        AdmissionError {
            diagnostic: Some(Box::new(
                help::catalog_diagnostic("QamRunUnavailable").unwrap(),
            )),
            ..plain()
        },
        &budget,
    )
    .unwrap();
    let original_charge = budget.usage();
    let help = serde_json::to_value(original.diagnostic()).unwrap();
    let replacement = original.replace_static_message(MESSAGE, &budget, FALLBACK);
    let WorkerFailure::Detailed(retained) = &replacement else {
        panic!("Real guidance replacement must admit its own pointee");
    };
    let replacement_charge = retained.fixture_reserved_resources();
    let mut combined = original_charge;
    crate::downstream::inputs::retention::append(&mut combined, replacement_charge).unwrap();
    assert_eq!(budget.usage(), combined);
    assert_eq!(
        serde_json::to_value(replacement.diagnostic()).unwrap(),
        help
    );
    assert_eq!(replacement.message(), MESSAGE);
    drop(original);
    assert_eq!(budget.usage(), replacement_charge);
    drop(replacement);
    assert_eq!(budget.usage(), RetainedResources::default());
    assert_eq!(WorkerFailure::default().message(), "");
    assert!(WorkerFailure::default().diagnostic().is_none());
}
