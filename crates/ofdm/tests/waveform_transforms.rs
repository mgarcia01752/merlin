/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent PHY centered/unitary vectors, not only reversible transform tests.
use docsis_ofdm_fec::{
    signal_processing::waveform::{fft::ForwardTransform, ifft::InverseTransform},
    transmission::SubcarrierSpacing,
};
fn close(actual: [f64; 2], expected: [f64; 2]) {
    for axis in 0..2 {
        assert!(
            (actual[axis] - expected[axis]).abs() < 2e-10,
            "{actual:?} != {expected:?}"
        );
    }
}
#[test]
fn independent_center_dc_quarter_tone_and_low_edge_vectors_match_phy() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let length = spacing.fft_samples() as usize;
        let scale = 1.0 / (length as f64).sqrt();
        let plan = InverseTransform::new(spacing);
        for (bin, period) in [
            (length / 2, vec![[1.0, 0.0]]),
            (
                3 * length / 4,
                vec![[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]],
            ),
            (0, vec![[1.0, 0.0], [-1.0, 0.0]]),
        ] {
            let mut bins = vec![[0.0, 0.0]; length];
            bins[bin] = [1.0, 0.0];
            let samples = plan.apply(&bins).unwrap();
            for (index, sample) in samples.iter().enumerate() {
                close(
                    *sample,
                    period[index % period.len()].map(|value| value * scale),
                );
            }
        }
    }
}
#[test]
fn independent_fft_impulse_and_quarter_tone_have_expected_scale_and_bin() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let length = spacing.fft_samples() as usize;
        let plan = ForwardTransform::new(spacing);
        let mut impulse = vec![[0.0, 0.0]; length];
        impulse[0] = [1.0, 0.0];
        for bin in plan.apply(&impulse).unwrap() {
            close(bin, [1.0 / (length as f64).sqrt(), 0.0]);
        }
        let period = [[1.0, 0.0], [0.0, 1.0], [-1.0, 0.0], [0.0, -1.0]];
        let samples: Vec<_> = (0..length).map(|index| period[index % 4]).collect();
        for (index, bin) in plan.apply(&samples).unwrap().iter().enumerate() {
            close(
                *bin,
                if index == 3 * length / 4 {
                    [(length as f64).sqrt(), 0.0]
                } else {
                    [0.0, 0.0]
                },
            );
        }
    }
}
#[test]
fn dense_complex_grid_preserves_energy_and_matches_direct_idft_samples() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let length = spacing.fft_samples() as usize;
        let bins: Vec<_> = (0..length)
            .map(|index| [(index % 11) as f64 - 5.0, (index % 7) as f64 - 3.0])
            .collect();
        let samples = InverseTransform::new(spacing).apply(&bins).unwrap();
        let energy = |values: &[[f64; 2]]| {
            values
                .iter()
                .flatten()
                .map(|value| value * value)
                .sum::<f64>()
        };
        assert!((energy(&bins) - energy(&samples)).abs() < 1e-7);
        for time in [0, 1, 7, 31] {
            let mut expected = [0.0, 0.0];
            for (bin, iq) in bins.iter().enumerate() {
                let angle =
                    std::f64::consts::TAU * time as f64 * (bin as f64 - length as f64 / 2.0)
                        / length as f64;
                expected[0] += iq[0] * angle.cos() - iq[1] * angle.sin();
                expected[1] += iq[0] * angle.sin() + iq[1] * angle.cos();
            }
            close(
                samples[time],
                expected.map(|value| value / (length as f64).sqrt()),
            );
        }
        for (actual, expected) in ForwardTransform::new(spacing)
            .apply(&samples)
            .unwrap()
            .iter()
            .zip(&bins)
        {
            close(*actual, *expected);
        }
    }
}
#[test]
fn transforms_reject_shape_nan_and_overflow() {
    let spacing = SubcarrierSpacing::Khz50;
    let length = 4096;
    let inverse = InverseTransform::new(spacing);
    let forward = ForwardTransform::new(spacing);
    for samples in [
        vec![],
        vec![[0.0, 0.0]; length - 1],
        vec![[f64::NAN, 0.0]; length],
        vec![[f64::MAX, f64::MAX]; length],
    ] {
        assert!(inverse.apply(&samples).is_err());
        assert!(forward.apply(&samples).is_err());
    }
}
