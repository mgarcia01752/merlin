/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Independent DC/tone/time-impulse/direct-sum/energy acceptance, both modes.
use docsis_ofdma::{configuration::Numerology, signal_processing::Transform, types::Iq};
use std::f64::consts::TAU;

#[test]
fn centered_unitary_tones_and_direct_sums() {
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let n = mode.fft_len();
        let transform = Transform::new(mode);
        for k in [0, n / 2, n / 2 + 1, n - 1] {
            let mut bins = vec![Iq::default(); n];
            bins[k] = Iq { re: 1.0, im: 0.0 };
            let time = transform.inverse(&bins).unwrap();
            for i in [0, 1, 17, n / 2, n - 1] {
                let angle = TAU * i as f64 * (k as f64 - n as f64 / 2.0) / n as f64;
                let (im, re) = angle.sin_cos();
                let expected = Iq { re, im } * (n as f64).sqrt().recip();
                assert!((time[i] - expected).power() < 1e-22);
            }
            assert!((time.iter().map(|z| z.power()).sum::<f64>() - 1.0).abs() < 1e-11);
            let recovered = transform.forward(&time).unwrap();
            assert!(recovered
                .iter()
                .zip(&bins)
                .all(|(a, b)| (*a - *b).power() < 1e-24));
        }
        let mut impulse = vec![Iq::default(); n];
        impulse[0] = Iq { re: 1.0, im: 0.0 };
        assert!(transform.forward(&impulse).unwrap().iter().all(|z| (z.re
            - (n as f64).sqrt().recip())
        .abs()
            < 1e-13
            && z.im.abs() < 1e-13));
        assert!(transform.forward(&[]).is_err());
        impulse[1].re = f64::NAN;
        assert!(transform.forward(&impulse).is_err());
    }
}
