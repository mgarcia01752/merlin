/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::{
    signal_processing::receiver::diagnostics::{llr_statistics, response_diagnostics},
    transmission::SubcarrierSpacing,
};
#[test]
fn impulse_delay_and_gain_use_independent_direct_tap_vector() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let count = spacing.fft_samples() as usize;
        let response: Vec<_> = (0..count)
            .map(|bin| {
                let angle = -2.0 * std::f64::consts::PI * (bin as f64 - count as f64 / 2.0) * 16.0
                    / count as f64;
                Some([1.0 + 0.5 * angle.cos(), 0.5 * angle.sin()])
            })
            .collect();
        let view = response_diagnostics(spacing, &response, 0.87).unwrap();
        assert!(view.peak_delay_microseconds.abs() < 1e-12);
        assert!(view.impulse_db_relative_peak[count / 2].abs() < 1e-12);
        assert!((view.impulse_db_relative_peak[count / 2 + 16] + 6.020599913279624).abs() < 1e-9);
        assert!((view.delay_microseconds[count / 2 + 16] - 16.0 / 204.8).abs() < 1e-12);
        assert!((view.gain_db[count / 2].unwrap() - 20.0 * 1.5_f64.log10()).abs() < 1e-12);
        assert!(response_diagnostics(spacing, &response, 0.0).is_err());
        assert!(response_diagnostics(spacing, &response[..4], 0.87).is_err());
    }
}
#[test]
fn llr_statistics_preserve_actual_values_and_empty_observations() {
    let result = llr_statistics(&[-8.0, -2.0, 0.0, 2.0, 8.0]).unwrap();
    assert_eq!(result.count, 5);
    assert_eq!(result.minimum, Some(-8.0));
    assert_eq!(result.maximum, Some(8.0));
    assert_eq!(result.mean, Some(0.0));
    assert_eq!(result.histogram.iter().sum::<u64>(), 5);
    assert_eq!(llr_statistics(&[]).unwrap().mean, None);
    assert!(llr_statistics(&[f64::NAN]).is_err());
    assert!(llr_statistics(&vec![0.0; 32769]).is_err());
}
