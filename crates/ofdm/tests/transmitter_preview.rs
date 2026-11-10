/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    scenario::pnm_json::{import_profiles, JsonCapture},
    simulation::{
        source::{file::FilePayload, PayloadSourceConfig},
        transmitter_preview::TransmitterPreviewRequest,
    },
    transmission::{settings::SAMPLE_RATE_HZ, TransmissionSettings},
};
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn actual_demo_transmission_uses_selected_source_and_cooperative_cancellation() {
    let mut profiles = import_profiles(JsonCapture {
        name: "demo.json",
        bytes: include_bytes!(
            "../../../examples/measurements/single-channel/modulation_profile.json"
        ),
    })
    .unwrap();
    let profile = profiles
        .iter()
        .find(|profile| profile.profile_id == 0)
        .unwrap();
    let settings = TransmissionSettings::default();
    let request =
        TransmitterPreviewRequest::new(&settings, profile, PayloadSourceConfig::Prbs15, 32767)
            .unwrap();
    let cancel = AtomicBool::new(false);
    let result = request.generate(&cancel, |_, _| {}).unwrap();
    assert_eq!(result.spectrum.averages, 32);
    assert_eq!(result.samples.len(), 8192 + 31 * 4096);
    assert_eq!(result.first_sample, 15 * (8192 + 512));
    assert!(result.generated_source_bits > 0);
    assert!(!result.source_exhausted);
    assert!(result.spectrum.density.iter().sum::<f64>() * f64::from(SAMPLE_RATE_HZ) / 8192.0 > 0.1);
    let before = result.spectrum.density.clone();
    result.spectrum.buckets(100, 500).unwrap();
    assert_eq!(result.spectrum.density, before);
    let file = FilePayload::new(b"exact source\r\n", None).unwrap();
    let request = TransmitterPreviewRequest::new(
        &settings,
        profile,
        PayloadSourceConfig::File {
            payload: file.clone(),
        },
        32767,
    )
    .unwrap();
    let finite = request.generate(&cancel, |_, _| {}).unwrap();
    assert!(finite.source_exhausted);
    assert_eq!(finite.generated_source_bits, file.target_bits());
    assert_eq!(file.bytes(), b"exact source\r\n");
    assert_ne!(result.samples, finite.samples);
    let canceled = request.generate(&cancel, |_, _| cancel.store(true, Ordering::Relaxed));
    assert!(canceled.is_err());
    profiles[0].profile_id = 1;
    assert!(TransmitterPreviewRequest::new(
        &settings,
        &profiles[0],
        PayloadSourceConfig::Prbs15,
        32767
    )
    .is_err());
}
