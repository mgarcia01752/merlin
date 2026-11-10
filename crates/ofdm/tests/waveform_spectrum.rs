/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::{
    signal_processing::waveform::spectrum::{density_db, estimate, MAX_SPECTRUM_AVERAGES},
    transmission::SubcarrierSpacing,
};
use std::f64::consts::TAU;

#[test]
fn hann_psd_has_independent_tone_frequency_integral_and_bandwidth_at_both_spacings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let length = spacing.fft_samples() as usize;
        let count = length + (MAX_SPECTRUM_AVERAGES - 1) * length / 2;
        let samples: Vec<_> = (0..count)
            .map(|index| {
                let (imaginary, real) = (TAU * 17.0 * index as f64 / length as f64).sin_cos();
                [real, imaginary]
            })
            .collect();
        let psd = estimate(spacing, 100_000_000, &samples).unwrap();
        assert_eq!(psd.averages, 32);
        let peak = psd
            .density
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .unwrap()
            .0;
        assert_eq!(peak, length / 2 + 17);
        let integrated: f64 = psd.density.iter().sum::<f64>() * f64::from(spacing.frequency_hz());
        assert!((integrated - 1.0).abs() < 1e-11);
        assert!(
            (psd.equivalent_noise_bandwidth_hz - 1.5 * f64::from(spacing.frequency_hz())).abs()
                < 1e-7
        );
        let expected_peak = 2.0 / (3.0 * f64::from(spacing.frequency_hz()));
        assert!((psd.density[peak] - expected_peak).abs() < 1e-13);
        assert_eq!(
            psd.buckets(peak, 1).unwrap()[0].maximum_db,
            density_db(psd.density[peak])
        );
        let buckets = psd.buckets(0, length).unwrap();
        assert!(buckets.len() <= 2048);
        assert_eq!(buckets.first().unwrap().first_bin, 0);
        assert_eq!(buckets.last().unwrap().last_bin, length - 1);
        assert!(psd.buckets(length, 1).is_err());
        assert!(estimate(spacing, 0, &samples[..length - 1]).is_err());
    }
}

#[test]
fn silence_and_nonfinite_inputs_remain_explicit() {
    let mut samples = vec![[0.0, 0.0]; 4096];
    let psd = estimate(SubcarrierSpacing::Khz50, 0, &samples).unwrap();
    assert!(psd
        .buckets(0, 4096)
        .unwrap()
        .iter()
        .all(|bucket| bucket.maximum_db.is_none()));
    samples[0][0] = f64::NAN;
    assert!(estimate(SubcarrierSpacing::Khz50, 0, &samples).is_err());
}
