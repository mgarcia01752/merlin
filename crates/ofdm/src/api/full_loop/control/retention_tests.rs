/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual control layouts, unchanged lifecycle and established small-run science.
use super::*;
use std::{
    mem::size_of,
    panic::{catch_unwind, AssertUnwindSafe},
};

const PROFILE: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../examples/measurements/single-channel/modulation_profile.json"
));
const RXMER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../examples/measurements/single-channel/rxmer.json"
));

// Existing clean small-fixture residual MER, in dB; not a receiver requirement.
const CLEAN_FIXTURE_MER_DB: f64 = 80.0;

// Existing observation window, LDPC budget and separate host safety timeout.
const FIXTURE_DURATION_MS: u64 = 1;
const FIXTURE_MAX_ITERATIONS: usize = 10;
const FIXTURE_MAX_RUNTIME_MS: u64 = 60_000;

// Single-mode capture clock; this fixture never advances to another capture.
const FIXTURE_INTERVAL_SYMBOLS: u64 = 1_000;

fn expected() -> RetainedFootprint {
    RetainedFootprint {
        payload_bytes: size_of::<FullLoopControl>() + size_of::<Shared>() + size_of::<AtomicBool>(),
        ordered_entries: 0,
        shared_roots: 2,
    }
}

fn assert_passive(control: &FullLoopControl) {
    let snapshot = serde_json::to_value(control.snapshot()).unwrap();
    let notification = control.state().last_notification;
    assert_eq!(control.retained_footprint().unwrap(), expected());
    assert_eq!(serde_json::to_value(control.snapshot()).unwrap(), snapshot);
    assert_eq!(control.state().last_notification, notification);
}

#[test]
fn full_loop_layout_and_alias_lifetime_include_the_nested_common_stop_once() {
    let control = FullLoopControl::new();
    let weak = Arc::downgrade(&control.0);
    let alias = control.clone();
    let stop = control.stop();
    assert!(Arc::ptr_eq(&control.0, &alias.0));
    assert_passive(&control);
    drop(control);
    assert!(weak.upgrade().is_some());
    stop.cancel();
    assert_passive(&alias);
    assert!(alias.snapshot().cancellation_requested);
    drop(alias);
    assert!(weak.upgrade().is_none());
    assert!(stop.is_cancelled());
}

#[test]
fn requested_acknowledged_and_terminal_progress_are_not_advanced_by_metadata() {
    let control = FullLoopControl::new();
    assert_passive(&control);
    control.pause().unwrap();
    assert_passive(&control);
    assert_eq!(control.snapshot().state, FullLoopState::Idle);
    control.begin().unwrap();
    control.acknowledge_pause(true);
    control.progress(ChannelProgress {
        receiver_reliability: None,
        phase: ChannelPhase::Observation,
        completed_source_symbols: 0,
        requested_source_symbols: 0,
        recovered_payloads: 0,
        elapsed_ms: 0,
    });
    assert_passive(&control);
    assert_eq!(control.snapshot().state, FullLoopState::Paused);
    control.resume();
    control.acknowledge_pause(false);
    control.cancel();
    assert_passive(&control);
    control.finish(None);
    assert_passive(&control);
    assert_eq!(control.snapshot().state, FullLoopState::Failed);
    assert_eq!(control.begin().unwrap_err().code(), ErrorCode::Busy);
}

#[test]
fn poisoned_progress_metadata_recovers_without_clearing_poison_or_mutating_state() {
    let control = FullLoopControl::new();
    let poisoned = catch_unwind(AssertUnwindSafe(|| {
        let _guard = control.0.state.lock().unwrap();
        panic!("Progress mutex poison fixture");
    }));
    assert!(poisoned.is_err());
    assert!(control.0.state.is_poisoned());
    assert_passive(&control);
    assert!(control.0.state.is_poisoned());
    assert_eq!(control.snapshot().state, FullLoopState::Idle);
}

fn request() -> crate::api::FullLoopRequest {
    use crate::api::*;
    let profile = Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut capture: serde_json::Value = serde_json::from_slice(RXMER).unwrap();
    let source_carriers = capture["values"].as_array().unwrap().len();
    capture["values"] = serde_json::json!(vec![CLEAN_FIXTURE_MER_DB; source_carriers]);
    capture["pnm_header"]["capture_time"] =
        serde_json::json!(profile.sources()[0].original_epoch().value());
    let measured = RxMer::from_bytes(
        &NamedCapture::new("clean residual", serde_json::to_vec(&capture).unwrap()).unwrap(),
    )
    .unwrap();
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: FIXTURE_INTERVAL_SYMBOLS,
            tolerance: AssociationTolerance::ZERO,
        },
    )
    .unwrap();
    Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            measurements,
            source: SourceInput::prbs15(
                PrbsSeed::new(crate::common::constants::PRBS15_MAX_SEED).unwrap(),
            ),
            channel: ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
            response: None,
        },
        FullLoopOptions {
            duration_ms: FIXTURE_DURATION_MS,
            max_iterations: FIXTURE_MAX_ITERATIONS,
            max_runtime_ms: FIXTURE_MAX_RUNTIME_MS,
            ..Default::default()
        },
    )
    .unwrap()
}

#[test]
fn querying_control_preserves_real_complete_and_precancelled_request_behavior() {
    let request = request();
    let original = request.retained_footprint().unwrap();
    let baseline = request.run(&FullLoopControl::new()).unwrap();
    let control = FullLoopControl::new();
    assert_passive(&control);
    let measured = request.run(&control).unwrap();
    assert_eq!(measured.report().status, ChannelJobStatus::Complete);
    assert_eq!(
        serde_json::to_value(&measured.report().summary).unwrap(),
        serde_json::to_value(&baseline.report().summary).unwrap()
    );
    assert_passive(&control);
    assert_eq!(control.snapshot().state, FullLoopState::Completed);
    let cancelled = FullLoopControl::new();
    cancelled.cancel();
    assert_passive(&cancelled);
    let partial = request.run(&cancelled).unwrap();
    assert_eq!(partial.report().status, ChannelJobStatus::Cancelled);
    assert_passive(&cancelled);
    assert_eq!(cancelled.snapshot().state, FullLoopState::Cancelled);
    assert_eq!(request.retained_footprint().unwrap(), original);
}
