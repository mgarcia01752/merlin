/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    scenario::pnm_json::{import_channel, import_profiles, import_rxmer, JsonCapture},
    simulation::channel::measurement_playback::{
        validate_response, MeasurementPlayback, PlaybackMode,
    },
    transmission::TransmissionSettings,
};
fn fixtures() -> (
    docsis_ofdm_fec::profile::candidate::CapturedProfile,
    docsis_ofdm_fec::scenario::RxMerFrame,
) {
    let profile = import_profiles(JsonCapture {
        name: "dpd",
        bytes: include_bytes!(
            "../../../examples/measurements/single-channel/modulation_profile.json"
        ),
    })
    .unwrap()
    .remove(0);
    let frame = import_rxmer(JsonCapture {
        name: "rxmer",
        bytes: include_bytes!("../../../examples/measurements/single-channel/rxmer.json"),
    })
    .unwrap();
    (profile, frame)
}
#[test]
fn power_average_interpolation_hold_and_source_epochs_are_independent_and_immutable() {
    let (profile, mut first) = fixtures();
    for value in first.mer_db.iter_mut().flatten() {
        *value = 20.0;
    }
    let mut second = first.clone();
    second.capture_time += 1;
    second.source.capture_time = Some(second.capture_time);
    for value in second.mer_db.iter_mut().flatten() {
        *value = 40.0;
    }
    let originals = serde_json::to_vec(&vec![first.clone(), second.clone()]).unwrap();
    let settings = TransmissionSettings::default();
    let make = |mode| {
        MeasurementPlayback::new(
            &settings,
            &profile,
            vec![first.clone(), second.clone()],
            mode,
            1000,
            90000,
        )
        .unwrap()
    };
    let average = make(PlaybackMode::Average);
    let index = first.mer_db.iter().position(Option::is_some).unwrap();
    let power = average.at(0).power[index].unwrap();
    assert!((power - 0.00505).abs() < 1e-15);
    assert!((-10.0 * power.log10() - 22.9670862188).abs() < 1e-9);
    let interpolation = make(PlaybackMode::Interpolate);
    assert!((interpolation.at(0).power[index].unwrap() - 0.01).abs() < 1e-15);
    assert!((interpolation.at(500).power[index].unwrap() - 0.00505).abs() < 1e-15);
    assert_eq!(interpolation.at(500).second_weight, 0.5);
    assert!((interpolation.at(1000).power[index].unwrap() - 0.0001).abs() < 1e-15);
    assert_eq!(interpolation.at(u64::MAX).first_source, 1);
    assert_eq!(interpolation.at(u64::MAX).second_source, 1);
    assert_eq!(interpolation.span(), (1000, 42.5));
    assert_eq!(interpolation.interval_milliseconds(), 42.5);
    let hold = make(PlaybackMode::Hold);
    assert_eq!(hold.at(999).power[index], Some(0.01));
    assert_eq!(hold.at(1000).power[index], Some(0.0001));
    assert_eq!(
        serde_json::to_vec(interpolation.frames()).unwrap(),
        originals
    );
    assert_eq!(average.at(0).power, average.at(999999).power);
}
#[test]
fn invalid_identity_missing_data_schedule_and_response_are_rejected() {
    let (profile, first) = fixtures();
    let settings = TransmissionSettings::default();
    validate_response(&settings, &profile, None).unwrap();
    let mut response = import_channel(JsonCapture {
        name: "channel",
        bytes: include_bytes!(
            "../../../examples/measurements/single-channel/channel_estimation.json"
        ),
    })
    .unwrap();
    validate_response(&settings, &profile, Some(&response)).unwrap();
    for sample in response.iq.iter_mut().flatten() {
        *sample = [0.0, 0.0];
    }
    assert!(validate_response(&settings, &profile, Some(&response)).is_err());
    let new = |frames, mode, interval| {
        MeasurementPlayback::new(&settings, &profile, frames, mode, interval, 90000)
    };
    assert!(new(vec![], PlaybackMode::Single, 1).is_err());
    assert!(new(vec![first.clone()], PlaybackMode::Hold, 1).is_err());
    assert!(new(vec![first.clone(), first.clone()], PlaybackMode::Hold, 0).is_err());
    assert!(new(
        vec![first.clone(), first.clone(), first.clone()],
        PlaybackMode::Interpolate,
        u64::MAX
    )
    .is_err());
    let mut bad = first.clone();
    bad.key.grid.spacing_hz = 50000;
    assert!(new(vec![bad], PlaybackMode::Single, 1).is_err());
    let mut missing = first.clone();
    missing.mer_db.fill(None);
    assert!(new(vec![missing], PlaybackMode::Single, 1).is_err());
    let mut stale = first.clone();
    stale.capture_time = 0;
    stale.source.capture_time = Some(0);
    assert!(new(vec![stale], PlaybackMode::Single, 1).is_err());
    let mut invalid = first.clone();
    invalid.mer_db[0] = Some(f64::NAN);
    assert!(new(vec![invalid], PlaybackMode::Single, 1).is_err());
    assert!(new(vec![first; 65], PlaybackMode::Average, 1).is_err());
}
