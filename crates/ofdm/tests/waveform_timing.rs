/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::{
    signal_processing::waveform::{timing::symbol_timing, WaveformGeometry},
    transmission::{
        settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES},
        SubcarrierSpacing, TransmissionSettings,
    },
};

#[test]
fn timing_budget_has_independent_units_round_trip_and_unknown_velocity() {
    let geometry = WaveformGeometry::new(&TransmissionSettings::default()).unwrap();
    let report = symbol_timing(&geometry, Some(0.82)).unwrap();
    assert_eq!(report.prefix_us, 2.5);
    assert_eq!(report.roll_off_us, 0.625);
    assert_eq!(report.guard_us, 1.875);
    assert_eq!(report.useful_us, 40.0);
    assert_eq!(report.symbol_us, 42.5);
    assert!((report.extra_path_feet.unwrap() - 1512.24025).abs() < 0.001);
    assert!((report.reflection_spacing_feet.unwrap() - 756.120125).abs() < 0.001);
    let hardline = symbol_timing(&geometry, Some(0.87)).unwrap();
    assert!((hardline.reflection_spacing_feet.unwrap() - 802.22501).abs() < 0.001);
    let unknown = symbol_timing(&geometry, None).unwrap();
    assert!(unknown.extra_path_feet.is_none());
    assert!(unknown.reflection_spacing_feet.is_none());
    assert_eq!(unknown.points.len(), report.points.len());
    for invalid in [0.0, -0.1, 1.001, f64::NAN, f64::INFINITY] {
        assert!(symbol_timing(&geometry, Some(invalid)).is_err());
    }
    assert!(symbol_timing(&geometry, Some(1.0)).is_ok());
}

#[test]
fn window_zoom_is_bounded_for_both_spacings_and_every_valid_prefix_roll_pair() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for prefix in CYCLIC_PREFIX_SAMPLES {
            for roll in ROLL_OFF_SAMPLES {
                let settings = TransmissionSettings {
                    subcarrier_spacing: spacing,
                    cyclic_prefix_samples: prefix,
                    roll_off_samples: roll,
                    ..TransmissionSettings::default()
                };
                let geometry = WaveformGeometry::new(&settings);
                if roll >= prefix {
                    assert!(geometry.is_err());
                    continue;
                }
                let timing = symbol_timing(&geometry.unwrap(), Some(0.82)).unwrap();
                assert!(timing.points.len() <= 130);
                assert_eq!(timing.points.first().unwrap().x, 0.0);
                assert_eq!(timing.points.last().unwrap().x, 1.0);
                assert!(timing.points.windows(2).all(|pair| pair[0].x < pair[1].x));
                assert!(timing
                    .points
                    .iter()
                    .all(|point| point.coefficient >= 0.0 && point.coefficient <= 1.0));
                assert_eq!(timing.points.last().unwrap().coefficient, 1.0);
                if roll == 0 {
                    assert!(timing.points.iter().all(|point| point.coefficient == 1.0));
                } else {
                    let expected =
                        0.5 * (1.0 - (std::f64::consts::PI * 0.5 / f64::from(roll)).cos());
                    assert!((timing.points[0].coefficient - expected).abs() < 1e-14);
                }
                assert!(timing.guard_us > 0.0);
            }
        }
    }
}
