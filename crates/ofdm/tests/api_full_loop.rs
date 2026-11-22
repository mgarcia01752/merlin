/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed admission, real closed-loop behavior and safe-boundary controls through public API.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::{
    api::*,
    simulation::{
        channel::{run_channel, ChannelJobLimits},
        test_points::{NoopObserver, TestPointProbe},
    },
};
use serde_json::{json, Value};
use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
fn inputs() -> FullLoopInputs {
    let profile = Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap();
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default()).unwrap();
    let mut capture: Value = serde_json::from_slice(RXMER).unwrap();
    capture["values"] = json!(vec![80.0; 7480]);
    capture["pnm_header"]["capture_time"] = json!(1466967);
    let measured = RxMer::from_bytes(
        &NamedCapture::new("clean", serde_json::to_vec(&capture).unwrap()).unwrap(),
    )
    .unwrap();
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: 1000,
            tolerance: AssociationTolerance::ZERO,
        },
    )
    .unwrap();
    FullLoopInputs {
        transmission,
        profile,
        source: SourceInput::prbs15(PrbsSeed::new(32767).unwrap()),
        measurements,
        channel: ChannelConfiguration::new(ChannelConfigurationInput::default()).unwrap(),
        response: None,
    }
}
fn options() -> FullLoopOptions {
    FullLoopOptions {
        duration_ms: 1,
        max_iterations: 10,
        max_runtime_ms: 60000,
        ..Default::default()
    }
}
fn until(control: &FullLoopControl, predicate: impl Fn(&FullLoopProgress) -> bool) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while !predicate(&control.snapshot()) {
        assert!(
            Instant::now() < deadline,
            "Worker failed to reach requested boundary"
        );
        std::thread::sleep(Duration::from_millis(2));
    }
}
#[test]
fn complete_request_matches_existing_job_and_preserves_originals_and_fec() {
    let original = inputs();
    let request = Simulation::prepare_full_loop(original.clone(), options()).unwrap();
    assert_eq!(
        request.settings().transmission,
        *original.transmission.settings()
    );
    assert_eq!(request.settings().duration, Duration::from_millis(1));
    assert_eq!(request.settings().impairments.seed, 41);
    assert_eq!(
        request
            .settings()
            .playback
            .as_ref()
            .unwrap()
            .interval_symbols,
        1000
    );
    assert_eq!(
        request
            .settings()
            .constellation
            .as_ref()
            .unwrap()
            .requested_pairs,
        65536
    );
    let control = FullLoopControl::new();
    let result = request.run(&control).unwrap();
    let report = result.report();
    assert_eq!(control.snapshot().state, FullLoopState::Completed);
    assert_eq!(report.status, ChannelJobStatus::Complete);
    assert!(report.summary.comparison.bits_tested > 0);
    assert_eq!(report.summary.comparison.bit_errors, 0);
    assert!(report.summary.recovered_payloads > 0);
    assert_eq!(report.summary.payload_ldpc_failures, 0);
    assert_eq!(report.summary.payload_bch_failures, 0);
    assert_eq!(report.summary.timing.complete_symbols, 23);
    let old = run_channel(
        request.settings(),
        &ChannelJobLimits {
            max_runtime_ms: 60000,
            max_retained_words: 64,
        },
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(old.summary.comparison, report.summary.comparison);
    assert_eq!(old.retained_words, report.retained_words);
    assert_eq!(
        old.summary.recovered_payloads,
        report.summary.recovered_payloads
    );
    assert_eq!(request.run(&control).err().unwrap().code(), ErrorCode::Busy);
    assert_eq!(
        result.inputs().summary().unwrap().profile_sources[0].original_bytes(),
        PROFILE
    );
    drop(request);
    drop(original);
    assert_eq!(result.inputs().source.prbs_seed().unwrap().value(), 32767);
    assert!(result.recovered_bytes().is_none());
}
#[test]
fn acknowledged_pause_resume_cancel_retains_completed_comparisons() {
    let request = Simulation::prepare_full_loop(
        inputs(),
        FullLoopOptions {
            duration_ms: 1000,
            max_retained_words: 2,
            ..options()
        },
    )
    .unwrap();
    let control = FullLoopControl::new();
    let token = control.clone();
    let worker_request = request.clone();
    let worker = std::thread::spawn(move || worker_request.run(&token));
    until(&control, |status| {
        status
            .progress
            .as_ref()
            .is_some_and(|value| value.recovered_payloads > 0)
    });
    control.pause().unwrap();
    until(&control, |status| status.state == FullLoopState::Paused);
    let before = control.snapshot().progress.unwrap();
    std::thread::sleep(Duration::from_millis(30));
    let held = control.snapshot().progress.unwrap();
    assert_eq!(
        held.completed_source_symbols,
        before.completed_source_symbols
    );
    assert_eq!(held.recovered_payloads, before.recovered_payloads);
    assert_eq!(request.run(&control).err().unwrap().code(), ErrorCode::Busy);
    control.resume();
    until(&control, |status| {
        status
            .progress
            .as_ref()
            .is_some_and(|value| value.completed_source_symbols > before.completed_source_symbols)
    });
    control.pause().unwrap();
    until(&control, |status| status.state == FullLoopState::Paused);
    control.cancel();
    let result = worker.join().unwrap().unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
    assert_eq!(control.snapshot().state, FullLoopState::Cancelled);
    assert!(!control.snapshot().pause_requested);
    assert!(result.report().summary.comparison.bits_tested > 0);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    assert!(result.report().retained_words.len() <= 2);
    assert!(!result.report().retained_words.is_empty());
    assert!(result.report().summary.recovered_payloads >= before.recovered_payloads);
    assert!(control.pause().is_err());
}
#[test]
fn initial_pause_and_precancel_preserve_zero_completed_trials_and_file_output_is_exact() {
    let request = Simulation::prepare_full_loop(inputs(), options()).unwrap();
    let control = FullLoopControl::new();
    control.pause().unwrap();
    let token = control.clone();
    let worker = std::thread::spawn(move || request.run(&token));
    until(&control, |status| status.state == FullLoopState::Paused);
    assert!(control.snapshot().progress.is_none());
    control.cancel();
    let stopped = worker.join().unwrap().unwrap();
    assert_eq!(stopped.report().summary.comparison.bits_tested, 0);
    assert!(stopped.report().retained_words.is_empty());
    let mut finite = inputs();
    finite.source = SourceInput::file(
        NamedCapture::new("file", b"exact source\r\n".to_vec()).unwrap(),
        None,
    )
    .unwrap();
    let request = Simulation::prepare_full_loop(finite, options()).unwrap();
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(result.recovered_bytes().unwrap(), b"exact source\r\n");
    assert_eq!(result.report().summary.comparison.bits_tested, 112);
    assert!(result.report().recovered_file.as_ref().unwrap().complete);
    let stop = FullLoopControl::new();
    stop.cancel();
    let result = request.run(&stop).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Cancelled);
    assert_eq!(result.report().summary.comparison.bits_tested, 0);
}
#[test]
fn options_timing_and_identity_failures_leave_accepted_inputs_unchanged() {
    for bytes in [
        b"{".as_slice(),
        b"{\"unknown\":1}".as_slice(),
        b"{\"duration_ms\":1001}".as_slice(),
    ] {
        assert!(FullLoopOptions::from_json(bytes).is_err());
    }
    assert_eq!(
        FullLoopOptions::from_json(&vec![b' '; 65537])
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    let original = inputs();
    let before = original.profile.sources()[0].original_sha256().to_owned();
    for invalid in [
        FullLoopOptions {
            max_iterations: 0,
            ..options()
        },
        FullLoopOptions {
            max_runtime_ms: 0,
            ..options()
        },
        FullLoopOptions {
            max_retained_words: 10001,
            ..options()
        },
        FullLoopOptions {
            constellation: Some(ConstellationSelection {
                requested_pairs: 0,
                ..Default::default()
            }),
            ..options()
        },
    ] {
        assert!(Simulation::prepare_full_loop(original.clone(), invalid).is_err());
    }
    let mut changed = original.clone();
    changed.transmission = TransmissionConfiguration::new(TransmissionSettings {
        cyclic_prefix_samples: 1024,
        ..Default::default()
    })
    .unwrap();
    assert_eq!(
        Simulation::prepare_full_loop(changed, options())
            .err()
            .unwrap()
            .code(),
        ErrorCode::IncompatibleGrid
    );
    assert_eq!(original.profile.sources()[0].original_sha256(), before);
    let mut wrong = original.clone();
    wrong.response = Some(
        ChannelEstimate::from_bytes(
            &NamedCapture::new(
                "H",
                include_bytes!(
                    "../../../examples/measurements/single-channel/channel_estimation.json"
                )
                .to_vec(),
            )
            .unwrap(),
        )
        .unwrap(),
    );
    assert_eq!(
        Simulation::prepare_full_loop(wrong, options())
            .err()
            .unwrap()
            .code(),
        ErrorCode::AssociationFailed
    );
    Simulation::prepare_full_loop(original, options()).unwrap();
}
