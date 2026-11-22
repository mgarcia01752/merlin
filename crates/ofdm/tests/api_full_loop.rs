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

#[test]
fn independent_response_tolerance_preserves_measurement_policy_and_sweep_admission() {
    // Literal 17-second H/profile distance is independent of the zero-second RxMER policy.
    const RESPONSE_EPOCH_DISTANCE_SECONDS: u32 = 17;
    let mut original = inputs();
    let mut response: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/measurements/single-channel/channel_estimation.json"
    ))
    .unwrap();
    response["pnm_header"]["capture_time"] = json!(
        original.profile.sources()[0].original_epoch().value() + RESPONSE_EPOCH_DISTANCE_SECONDS
    );
    original.response = Some(
        ChannelEstimate::from_bytes(
            &NamedCapture::new("independent H", serde_json::to_vec(&response).unwrap()).unwrap(),
        )
        .unwrap(),
    );
    assert_eq!(
        Simulation::prepare_full_loop(original.clone(), options())
            .err()
            .unwrap()
            .code(),
        ErrorCode::AssociationFailed
    );
    let tolerance = AssociationTolerance::new(RESPONSE_EPOCH_DISTANCE_SECONDS).unwrap();
    let request = Simulation::prepare_full_loop_with_response_tolerance(
        original.clone(),
        options(),
        tolerance,
    )
    .unwrap();
    assert_eq!(
        request.response_tolerance().value(),
        RESPONSE_EPOCH_DISTANCE_SECONDS
    );
    assert_eq!(
        request
            .settings()
            .playback
            .as_ref()
            .unwrap()
            .tolerance_seconds,
        0
    );
    let catalog = sweep_catalog(&[0, 3]);
    let plan =
        FullLoopSweepPlan::new(&request, &catalog, FullLoopProfileSelection::AllCaptured).unwrap();
    for index in 0..plan.info().selected_profile_ids.len() {
        assert_eq!(
            plan.request(index).unwrap().response_tolerance().value(),
            RESPONSE_EPOCH_DISTANCE_SECONDS
        );
        assert_eq!(
            plan.request(index)
                .unwrap()
                .settings()
                .playback
                .as_ref()
                .unwrap()
                .tolerance_seconds,
            0
        );
    }
    assert_eq!(
        Simulation::prepare_full_loop_with_response_tolerance(
            original,
            options(),
            AssociationTolerance::new(RESPONSE_EPOCH_DISTANCE_SECONDS - 1).unwrap(),
        )
        .err()
        .unwrap()
        .code(),
        ErrorCode::AssociationFailed
    );
}

fn sweep_catalog(ids: &[u8]) -> ProfileCapture {
    let mut document: Value = serde_json::from_slice(PROFILE).unwrap();
    let template = document["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|profile| profile["profile_id"] == 0)
        .unwrap()
        .clone();
    document["profiles"] = Value::Array(
        ids.iter()
            .map(|id| {
                let mut profile = template.clone();
                profile["profile_id"] = json!(id);
                profile
            })
            .collect(),
    );
    document["num_profiles"] = json!(ids.len());
    ProfileCapture::from_bytes(
        &NamedCapture::new("sweep-catalog", serde_json::to_vec(&document).unwrap()).unwrap(),
    )
    .unwrap()
}

#[test]
fn mixed_source_catalog_admits_supported_loading_without_losing_raw_ids_or_provenance() {
    const INSPECTION_ONLY_SOURCE_ID: u8 = 200;
    const SELECTED_PROFILE_ID: u8 = 7;
    const ABSENT_PROFILE_ID: u8 = 5;
    let capture = sweep_catalog(&[INSPECTION_ONLY_SOURCE_ID, SELECTED_PROFILE_ID, 0, 2]);
    let selected = capture
        .simulation_profile(ProfileId::new(SELECTED_PROFILE_ID).unwrap())
        .unwrap();
    assert_eq!(
        capture.profile_ids(),
        &[0, 2, SELECTED_PROFILE_ID, INSPECTION_ONLY_SOURCE_ID]
    );
    assert_eq!(
        selected
            .profile_ids()
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![0, 2, SELECTED_PROFILE_ID]
    );
    assert_eq!(selected.profile_id().value(), SELECTED_PROFILE_ID);
    assert_eq!(selected.origin(), ProfileOrigin::Captured);
    assert_eq!(
        selected.sources()[0].original_sha256(),
        capture.provenance().original_sha256()
    );
    assert_eq!(
        selected.sources()[0].original_bytes(),
        capture.provenance().original_bytes()
    );
    assert_eq!(selected.carriers(), inputs().profile.carriers());
    assert!(ProfileId::new(INSPECTION_ONLY_SOURCE_ID).is_err());
    assert!(capture
        .simulation_profile(ProfileId::new(ABSENT_PROFILE_ID).unwrap())
        .is_err());
    let old = NamedCapture::new("legacy", capture.provenance().original_bytes().to_vec()).unwrap();
    assert!(Dpd::from_bytes(&old, ProfileId::new(0).unwrap()).is_err());
}

#[test]
fn sweep_preflights_sorted_originals_and_preserves_independent_controls_partial_reports_and_retention(
) {
    const SECOND_PROFILE_ID: u8 = 7;
    const EXPECTED_ASSIGNMENTS: usize = 14_960;
    const EXPECTED_TOTAL_DURATION_MS: u64 = 2;
    const EXPECTED_TOTAL_RUNTIME_MS: u64 = 120_000;
    const EXPECTED_TOTAL_WORD_DETAILS: usize = 128;
    let capture = sweep_catalog(&[SECOND_PROFILE_ID, 2, 0]);
    let request = Simulation::prepare_full_loop(inputs(), options())
        .unwrap()
        .with_observations(
            FullLoopObservationPolicy::new(FullLoopObservationInput::default()).unwrap(),
        )
        .unwrap();
    let plan = FullLoopSweepPlan::new(
        &request,
        &capture,
        FullLoopProfileSelection::Selected(vec![
            ProfileId::new(SECOND_PROFILE_ID).unwrap(),
            ProfileId::new(0).unwrap(),
        ]),
    )
    .unwrap();
    let info = plan.info();
    assert_eq!(info.source_profile_ids, vec![0, 2, SECOND_PROFILE_ID]);
    assert_eq!(
        info.selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![0, SECOND_PROFILE_ID]
    );
    assert_eq!(info.total_profile_assignments, EXPECTED_ASSIGNMENTS);
    assert_eq!(
        info.requested_observation_duration_ms,
        EXPECTED_TOTAL_DURATION_MS
    );
    assert_eq!(
        info.maximum_sequential_runtime_ms,
        EXPECTED_TOTAL_RUNTIME_MS
    );
    assert_eq!(
        info.maximum_retained_word_details,
        EXPECTED_TOTAL_WORD_DETAILS
    );
    let first = plan.request(0).unwrap();
    let second = plan.request(1).unwrap();
    assert_eq!(
        serde_json::to_value(first.observation_policy()).unwrap(),
        serde_json::to_value(second.observation_policy()).unwrap()
    );
    assert_eq!(
        serde_json::to_value(first.options()).unwrap(),
        serde_json::to_value(second.options()).unwrap()
    );
    let control = FullLoopControl::new();
    let complete = first.run(&control).unwrap();
    assert_eq!(complete.report().status, ChannelJobStatus::Complete);
    assert_eq!(complete.report().summary.comparison.bit_errors, 0);
    assert!(complete
        .stage_inventory()
        .iter()
        .any(|stage| stage.snapshots > 0));
    assert_eq!(second.run(&control).err().unwrap().code(), ErrorCode::Busy);
    let cancelled = FullLoopControl::new();
    cancelled.cancel();
    let partial = second.run(&cancelled).unwrap();
    assert_eq!(partial.report().status, ChannelJobStatus::Cancelled);
    assert_eq!(
        partial.inputs().profile.profile_id().value(),
        SECOND_PROFILE_ID
    );
    assert_eq!(complete.report().status, ChannelJobStatus::Complete);
    let completed_second = second.run(&FullLoopControl::new()).unwrap();
    assert_eq!(completed_second.report().status, ChannelJobStatus::Complete);
    assert_eq!(completed_second.report().summary.comparison.bit_errors, 0);
    assert_eq!(
        completed_second.inputs().profile.sources()[0].original_sha256(),
        capture.provenance().original_sha256()
    );
    assert!(plan.request(2).is_err());
    let all =
        FullLoopSweepPlan::new(&request, &capture, FullLoopProfileSelection::AllCaptured).unwrap();
    assert_eq!(
        all.info()
            .selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![0, 2, SECOND_PROFILE_ID]
    );
}

#[test]
fn sweep_rejects_complete_invalid_selection_atomically_and_keeps_current_edited_request_exact() {
    const UNSUPPORTED_SOURCE_ID: u8 = 200;
    const ABSENT_PROFILE_ID: u8 = 5;
    const EXPECTED_PROFILE_BUDGET: usize = 32;
    const EXPECTED_ASSIGNMENT_BUDGET: usize = 262_144;
    const OVER_PROFILE_BUDGET_COUNT: usize = 33;
    assert_eq!(MAX_FULL_LOOP_SWEEP_PROFILES, EXPECTED_PROFILE_BUDGET);
    assert_eq!(MAX_FULL_LOOP_SWEEP_ASSIGNMENTS, EXPECTED_ASSIGNMENT_BUDGET);
    let capture = sweep_catalog(&[0, 7]);
    let mut accepted = inputs();
    accepted.profile = accepted
        .profile
        .with_modulation(
            CarrierRange::new(
                SubcarrierIndex::new(400).unwrap(),
                SubcarrierIndex::new(420).unwrap(),
            )
            .unwrap(),
            DataModulation::Qam16,
        )
        .unwrap();
    let request = Simulation::prepare_full_loop(accepted, options()).unwrap();
    let original = serde_json::to_value(request.settings()).unwrap();
    let current =
        FullLoopSweepPlan::new(&request, &capture, FullLoopProfileSelection::Current).unwrap();
    assert_eq!(
        serde_json::to_value(current.request(0).unwrap().settings()).unwrap(),
        original
    );
    for ids in [
        vec![],
        vec![ProfileId::new(0).unwrap(); 2],
        vec![ProfileId::new(ABSENT_PROFILE_ID).unwrap()],
        vec![ProfileId::new(0).unwrap(); OVER_PROFILE_BUDGET_COUNT],
    ] {
        assert!(FullLoopSweepPlan::new(
            &request,
            &capture,
            FullLoopProfileSelection::Selected(ids)
        )
        .is_err());
    }
    let mixed = sweep_catalog(&[0, UNSUPPORTED_SOURCE_ID]);
    let error = FullLoopSweepPlan::new(&request, &mixed, FullLoopProfileSelection::AllCaptured)
        .err()
        .unwrap();
    assert_eq!(error.code(), ErrorCode::UnsupportedCapability);
    assert!(error.message().contains("200"));
    FullLoopSweepPlan::new(
        &request,
        &mixed,
        FullLoopProfileSelection::Selected(vec![ProfileId::new(0).unwrap()]),
    )
    .unwrap();
    let mut malformed: Value =
        serde_json::from_slice(capture.provenance().original_bytes()).unwrap();
    malformed["profiles"][1]["schemes"][0]["modulation_order_code"] = json!(2);
    malformed["profiles"][1]["schemes"][0]["modulation_order"] = json!("qpsk");
    malformed["profiles"][1]["schemes"][0]["bit_loading"] = json!(2);
    let unsupported = ProfileCapture::from_bytes(
        &NamedCapture::new(
            "unsupported-loading",
            serde_json::to_vec(&malformed).unwrap(),
        )
        .unwrap(),
    )
    .unwrap();
    let error = FullLoopSweepPlan::new(
        &request,
        &unsupported,
        FullLoopProfileSelection::AllCaptured,
    )
    .err()
    .unwrap();
    assert!(error.message().contains("Captured profile 7"));
    malformed = serde_json::from_slice(capture.provenance().original_bytes()).unwrap();
    malformed["channel_id"] = json!(99);
    let different = ProfileCapture::from_bytes(
        &NamedCapture::new("different-channel", serde_json::to_vec(&malformed).unwrap()).unwrap(),
    )
    .unwrap();
    assert!(
        FullLoopSweepPlan::new(&request, &different, FullLoopProfileSelection::Current).is_err()
    );
    assert_eq!(serde_json::to_value(request.settings()).unwrap(), original);
}
