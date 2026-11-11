/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent electrical anchors and fixed-noise gain budgets.
use docsis_ofdm_fec::signal_processing::{
    channel::power::{channel_power, PowerConditioning},
    waveform::spectrum::PowerSpectrum,
};
fn calibrated() -> PowerConditioning {
    PowerConditioning {
        input_data_level_dbmv_per_6mhz: Some(0.0),
        ..Default::default()
    }
}
#[test]
fn unitary_data_density_calibrates_to_explicit_six_mhz_reference() {
    let power = calibrated();
    // 204.8 MHz sample clock, unit mean-square data FFT cells: PSD=1/Fs.
    let spectrum = PowerSpectrum {
        zero_frequency_hz: 100_000_000,
        spacing_hz: 25_000,
        fft_samples: 8192,
        averages: 1,
        equivalent_noise_bandwidth_hz: 37_500.0,
        density: vec![1.0 / 204_800_000.0; 8192],
    };
    let reading = channel_power(&spectrum, 100, 339, &power).unwrap();
    assert_eq!(reading.bandwidth_hz, 6_000_000.0);
    assert!(reading.dbmv.unwrap().abs() < 1e-10);
    let full = channel_power(&spectrum, 0, 8191, &power).unwrap();
    assert!((full.dbmv.unwrap() - 15.331787).abs() < 0.00001);
    assert!(full.equivalent_dbmv_per_6mhz.unwrap().abs() < 1e-10);
    assert!(channel_power(&spectrum, 0, 9000, &power).is_err());
    let mut malformed = spectrum.clone();
    malformed.density.pop();
    assert!(channel_power(&malformed, 0, 100, &power).is_err());
    assert!(channel_power(&spectrum, 0, 8191, &Default::default())
        .unwrap()
        .dbmv
        .is_none());
}
#[test]
fn gain_target_attenuation_and_independent_noise_have_distinct_references() {
    let mut power = calibrated();
    power.amplifier_gain_db = 20.0;
    power.attenuation_db = 6.0;
    assert!((power.net_amplitude() - 10.0_f64.powf(14.0 / 20.0)).abs() < 1e-12);
    power.amplifier_target_dbmv_per_6mhz = Some(30.0);
    assert_eq!(power.gain_db(), 30.0);
    power.receiver_noise_dbmv_hz = Some(-100.0);
    // -100 dBmV/Hz over6 MHz, referenced to0 dBmV data over6 MHz.
    assert!((power.receiver_noise_power() - 0.0006).abs() < 1e-12);
    power.attenuation_db = 40.0;
    assert!((power.receiver_noise_power() - 0.0006).abs() < 1e-12);
    power.validate().unwrap();
}
#[test]
fn malformed_bounds_and_unreferenced_electrical_choices_reject() {
    for gain in [-1.0, 60.1, f64::NAN] {
        assert!(PowerConditioning {
            amplifier_gain_db: gain,
            ..Default::default()
        }
        .validate()
        .is_err());
    }
    assert!(PowerConditioning {
        receiver_noise_dbmv_hz: Some(-120.0),
        ..Default::default()
    }
    .validate()
    .is_err());
    assert!(PowerConditioning {
        amplifier_target_dbmv_per_6mhz: Some(10.0),
        ..Default::default()
    }
    .validate()
    .is_err());
    assert!(serde_json::from_str::<PowerConditioning>(r#"{"unknown":2}"#).is_err());
}
