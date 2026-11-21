/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent category spelling/order and shared cancellation identity checks.
use merlin_api::{Cancellation, ErrorCode, RunControl};
use std::sync::atomic::AtomicBool;

#[test]
fn every_category_keeps_legacy_spelling_and_variant_order() {
    let cases = [
        (ErrorCode::InvalidInput, "InvalidInput"),
        (ErrorCode::UnsupportedFormat, "UnsupportedFormat"),
        (
            ErrorCode::UnsupportedDecoderContract,
            "UnsupportedDecoderContract",
        ),
        (ErrorCode::UnsupportedCapability, "UnsupportedCapability"),
        (ErrorCode::MissingSetting, "MissingSetting"),
        (ErrorCode::IncompatibleGrid, "IncompatibleGrid"),
        (ErrorCode::IncompatibleRoles, "IncompatibleRoles"),
        (ErrorCode::InvalidIdentifier, "InvalidIdentifier"),
        (ErrorCode::AssociationFailed, "AssociationFailed"),
        (ErrorCode::AmbiguousTimestamp, "AmbiguousTimestamp"),
        (ErrorCode::MissingCoverage, "MissingCoverage"),
        (ErrorCode::ResourceLimit, "ResourceLimit"),
        (ErrorCode::Io, "Io"),
        (ErrorCode::Busy, "Busy"),
        (ErrorCode::Engine, "Engine"),
    ];
    for (index, (category, spelling)) in cases.into_iter().enumerate() {
        assert_eq!(category as usize, index);
        assert_eq!(
            serde_json::to_string(&category).unwrap(),
            format!("\"{spelling}\"")
        );
    }
}

#[test]
fn clones_threads_and_legacy_atomic_queries_share_one_way_cancellation() {
    fn query(value: &impl Cancellation) -> bool {
        value.is_cancelled()
    }
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<RunControl>();
    let control = RunControl::new();
    let clone = control.clone();
    assert!(!query(&control));
    std::thread::spawn(move || clone.cancel()).join().unwrap();
    assert!(query(&control));
    control.cancel();
    assert!(control.is_cancelled());
    assert!(!RunControl::default().is_cancelled());
    assert!(!query(&AtomicBool::new(false)));
    assert!(query(&AtomicBool::new(true)));
}
