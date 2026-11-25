/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent mean-square-voltage references; synthetic, not device captures.
use pnm::common::signal_processing::spectrum_power::{integrated_level_dbmv, SpectrumRange};

fn integrate(levels: &[Option<f64>], start_hz: f64, stop_hz: f64, enbw: f64) -> Option<f64> {
    integrated_level_dbmv(
        levels,
        100.0,
        10.0,
        enbw,
        SpectrumRange { start_hz, stop_hz },
    )
}
#[test]
fn power_sum_uses_ten_log_not_coherent_voltage_sum() {
    // Three 0-dBmV bins, endpoint half cells: total mean-square voltage = 2 mV².
    let level = integrate(&[Some(0.0); 3], 100.0, 120.0, 10.0).unwrap();
    assert!((level - 3.010_299_956_639_812).abs() < 1e-12);
    // 0 dBmV + 20 dBmV + 0 dBmV with half endpoints = 101 mV².
    let level = integrate(&[Some(0.0), Some(20.0), Some(0.0)], 100.0, 120.0, 10.0).unwrap();
    assert!((level - 20.043_213_737_826_427).abs() < 1e-12);
}
#[test]
fn fractional_edges_and_enbw_are_applied_in_linear_power() {
    assert!(
        (integrate(&[Some(0.0); 3], 100.0, 115.0, 10.0).unwrap() - 1.760_912_590_556_812_4).abs()
            < 1e-12
    );
    assert!((integrate(&[Some(0.0); 3], 100.0, 120.0, 20.0).unwrap()).abs() < 1e-12);
    // Sub-bin selection covers 6 Hz of a 10-Hz cell: 0.6 mV².
    assert!(
        (integrate(&[Some(0.0); 3], 107.0, 113.0, 10.0).unwrap() + 2.218_487_496_163_563_7).abs()
            < 1e-12
    );
}
#[test]
fn extreme_levels_do_not_overflow_linear_intermediates() {
    assert!(
        (integrate(&[Some(4000.0); 3], 100.0, 120.0, 10.0).unwrap() - 4_003.010_299_956_64).abs()
            < 1e-9
    );
    assert!(
        (integrate(&[Some(-4000.0); 3], 100.0, 120.0, 10.0).unwrap() + 3_996.989_700_043_36).abs()
            < 1e-9
    );
}
#[test]
fn missing_bins_only_invalidate_the_selected_band() {
    for missing in [None, Some(f64::NAN), Some(f64::INFINITY)] {
        let levels = [Some(0.0), Some(0.0), missing];
        assert!(integrate(&levels, 100.0, 120.0, 10.0).is_none());
        assert!(integrate(&levels, 100.0, 115.0, 10.0).is_some());
    }
}
#[test]
fn invalid_grid_or_frequency_requests_are_rejected() {
    let levels = [Some(0.0); 3];
    for (start, stop) in [
        (94.0, 110.0),
        (110.0, 126.0),
        (110.0, 110.0),
        (120.0, 100.0),
        (f64::NAN, 120.0),
        (100.0, f64::INFINITY),
    ] {
        assert!(integrate(&levels, start, stop, 10.0).is_none());
    }
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(integrate(&levels, 100.0, 120.0, invalid).is_none());
        assert!(integrated_level_dbmv(
            &levels,
            100.0,
            invalid,
            10.0,
            SpectrumRange {
                start_hz: 100.0,
                stop_hz: 120.0
            }
        )
        .is_none());
    }
    assert!(integrate(&[], 100.0, 120.0, 10.0).is_none());
    assert!(integrate(&[Some(0.0)], 100.0, 120.0, 10.0).is_none());
}
