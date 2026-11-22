/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual idle TX compatibility plus independent timing/count and lifecycle anchors.
#![cfg(all(feature = "simulation", feature = "pnm-input"))]
use docsis_ofdm_fec::{
    api::*,
    scenario::pnm_json::{import_profiles, JsonCapture},
    simulation::{source::PayloadSourceConfig, transmitter_preview as engine},
};
use std::{
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
fn inputs() -> (TransmissionConfiguration, Dpd, SourceInput) {
    (
        TransmissionConfiguration::new(TransmissionSettings::default()).unwrap(),
        Dpd::from_bytes(
            &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
            ProfileId::new(0).unwrap(),
        )
        .unwrap(),
        SourceInput::prbs15(PrbsSeed::new(32767).unwrap()),
    )
}

#[test]
fn actual_tx_samples_psd_and_owned_originals_match_existing_owner() {
    let (transmission, profile, source) = inputs();
    let request = TransmitterPreviewRequest::new(&transmission, &profile, &source).unwrap();
    let control = TransmitterPreviewControl::new();
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Idle);
    let output = request.generate(&control).unwrap();
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Completed);
    assert_eq!(control.snapshot().completed_symbols, 31);
    assert_eq!(control.snapshot().total_symbols, 31);
    let info = output.info();
    assert_eq!(info.sample_count, 135168);
    assert_eq!(info.first_sample, 130560);
    assert_eq!(info.warmup_symbols, 15);
    assert!(info.generated_source_bits > 0);
    assert!(!info.source_exhausted);
    let captured = import_profiles(JsonCapture {
        name: "old",
        bytes: PROFILE,
    })
    .unwrap();
    let old = engine::TransmitterPreviewRequest::new(
        transmission.settings(),
        captured
            .iter()
            .find(|profile| profile.profile_id == 0)
            .unwrap(),
        PayloadSourceConfig::Prbs15,
        32767,
    )
    .unwrap()
    .generate(&AtomicBool::new(false), |_, _| {})
    .unwrap();
    assert_eq!(
        output.sample_view(0, 4096).unwrap().samples,
        old.samples[..4096]
    );
    assert_eq!(
        output.sample_view(135160, 0).unwrap().samples,
        old.samples[135160..]
    );
    let spectrum = output.spectrum(0, 0).unwrap();
    assert_eq!(spectrum.averages, 32);
    assert_eq!(spectrum.fft_samples, 8192);
    assert_eq!(spectrum.spacing_hz, 25000);
    assert!((spectrum.equivalent_noise_bandwidth_hz - 37500.0).abs() < 1e-8);
    let buckets = old.spectrum.buckets(0, 8192).unwrap();
    assert_eq!(spectrum.buckets.len(), 2048);
    for (api, old) in spectrum.buckets.iter().zip(buckets) {
        assert_eq!(api.minimum_db, old.minimum_db);
        assert_eq!(api.maximum_db, old.maximum_db);
        assert_eq!((api.first_bin, api.last_bin), (old.first_bin, old.last_bin));
    }
    let repeated = request.generate(&TransmitterPreviewControl::new()).unwrap();
    assert_eq!(
        repeated.sample_view(0, 64).unwrap().samples,
        output.sample_view(0, 64).unwrap().samples
    );
    assert_eq!(source.prbs_seed().unwrap().value(), 32767);
    drop(request);
    drop(profile);
    drop(source);
    drop(output);
    assert_eq!(repeated.info().sources[0].original_bytes(), PROFILE);
    assert_eq!(
        repeated.sample_view(135160, 0).unwrap().first_sample,
        265720
    );
}

#[test]
fn finite_file_previews_exhaust_without_mutating_source_and_queries_reject_bounds() {
    let (transmission, profile, _) = inputs();
    let source = SourceInput::file(
        NamedCapture::new("source", b"exact source\r\n".to_vec()).unwrap(),
        None,
    )
    .unwrap();
    let request = TransmitterPreviewRequest::new(&transmission, &profile, &source).unwrap();
    let control = TransmitterPreviewControl::new();
    let output = request.generate(&control).unwrap();
    assert!(output.info().source_exhausted);
    assert_eq!(output.info().generated_source_bits, 112);
    assert_eq!(source.original_bytes().unwrap(), b"exact source\r\n");
    assert_eq!(
        output.info().source_sha256.as_deref(),
        source.original_sha256()
    );
    assert_eq!(
        request.generate(&control).err().unwrap().code(),
        ErrorCode::Busy
    );
    assert_eq!(
        output.sample_view(0, 4097).err().unwrap().code(),
        ErrorCode::ResourceLimit
    );
    assert!(output.sample_view(usize::MAX, 0).is_err());
    assert!(output.sample_view(135168, 0).is_err());
    assert!(output.sample_view(135167, 2).is_err());
    assert!(output.spectrum(8192, 0).is_err());
    assert!(output.spectrum(usize::MAX, 1).is_err());
    assert!(output.spectrum(1, usize::MAX).is_err());
    assert_eq!(output.spectrum(100, 20).unwrap().buckets.len(), 20);
}

#[test]
fn cross_thread_cancel_progress_busy_and_precancel_are_one_shot() {
    let (transmission, profile, source) = inputs();
    let request = TransmitterPreviewRequest::new(&transmission, &profile, &source).unwrap();
    let control = TransmitterPreviewControl::new();
    let worker_control = control.clone();
    let worker_request = request.clone();
    let worker = std::thread::spawn(move || worker_request.generate(&worker_control));
    let deadline = Instant::now() + Duration::from_secs(10);
    while control.snapshot().state == TransmitterPreviewState::Idle {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Running);
    assert_eq!(
        request.generate(&control).err().unwrap().code(),
        ErrorCode::Busy
    );
    control.cancel();
    assert!(worker.join().unwrap().is_err());
    assert_eq!(control.snapshot().state, TransmitterPreviewState::Cancelled);
    assert!(control.snapshot().completed_symbols <= control.snapshot().total_symbols);
    let precancel = TransmitterPreviewControl::new();
    precancel.cancel();
    assert_eq!(precancel.snapshot().state, TransmitterPreviewState::Idle);
    assert!(request.generate(&precancel).is_err());
    assert_eq!(
        precancel.snapshot().state,
        TransmitterPreviewState::Cancelled
    );
    assert_eq!(precancel.snapshot().completed_symbols, 0);
}

#[test]
fn idle_timing_window_and_encoded_id_admission_keep_existing_support() {
    let (transmission, _, source) = inputs();
    let timing = transmission.symbol_timing(None).unwrap();
    assert_eq!(timing.fft_samples, 8192);
    assert_eq!(timing.prefix_samples, 512);
    assert_eq!(timing.useful_us, 40.0);
    assert_eq!(timing.symbol_us, 42.5);
    assert_eq!(timing.guard_us, 1.875);
    assert_eq!(timing.extra_path_feet, None);
    assert!(timing.points.len() <= 130);
    let cable = transmission.symbol_timing(Some(0.82)).unwrap();
    assert!((cable.reflection_spacing_feet.unwrap() - 756.1202496309055).abs() < 1e-6);
    for value in [0.0, -1.0, 1.01, f64::NAN, f64::INFINITY] {
        assert!(transmission.symbol_timing(Some(value)).is_err());
    }
    let window = transmission.window().unwrap();
    assert_eq!(window.stride_samples, 8704);
    assert_eq!(window.coefficients.len(), 8832);
    assert_eq!(window.sample_rate_hz, 204800000);
    for index in 0..128 {
        assert!(
            (window.coefficients[index] + window.coefficients[8704 + index] - 1.0).abs() < 1e-12
        );
    }
    let mut document: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    document["profiles"][0]["profile_id"] = serde_json::json!(200);
    let capture = ProfileCapture::from_bytes(
        &NamedCapture::new("raw", serde_json::to_vec(&document).unwrap()).unwrap(),
    )
    .unwrap();
    assert!(
        TransmitterPreviewRequest::from_capture(&transmission, &capture, 200, &source).is_err()
    );
    assert!(TransmitterPreviewRequest::from_capture(&transmission, &capture, 99, &source).is_err());
    let capture =
        ProfileCapture::from_bytes(&NamedCapture::new("raw", PROFILE.to_vec()).unwrap()).unwrap();
    TransmitterPreviewRequest::from_capture(&transmission, &capture, 0, &source).unwrap();
}
