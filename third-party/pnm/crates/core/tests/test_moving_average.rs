/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::moving_average::reflected_window_average;

#[test]
fn independent_centered_odd_even_and_identity_results() {
    let values = [Some(1.0), Some(2.0), Some(3.0), Some(4.0)];
    assert_eq!(reflected_window_average(&values, 1).unwrap(), values);
    for (window, expected) in [
        (2, vec![1.5, 1.5, 2.5, 3.5]),
        (3, vec![5.0 / 3.0, 2.0, 3.0, 10.0 / 3.0]),
        (4, vec![2.0, 2.0, 2.5, 3.0]),
    ] {
        let actual = reflected_window_average(&values, window).unwrap();
        for (actual, expected) in actual.iter().zip(expected) {
            assert!((actual.unwrap() - expected).abs() < 1e-12);
        }
    }
    assert!(reflected_window_average(&values, 0).is_none());
    assert!(reflected_window_average(&[], 3).unwrap().is_empty());
    assert_eq!(
        reflected_window_average(&[Some(-21.5)], 99).unwrap(),
        vec![Some(-21.5)]
    );
}

#[test]
fn large_windows_remain_bounded_and_match_a_brute_force_reflection_oracle() {
    let values = [
        Some(-30.0),
        Some(-20.0),
        Some(-50.0),
        Some(-40.0),
        Some(5.0),
    ];
    for window in [1, 2, 3, 4, 11, 100] {
        let result = reflected_window_average(&values, window).unwrap();
        for (index, actual) in result.iter().enumerate() {
            let mut total = 0.0;
            for offset in 0..i64::from(window) {
                // Independently fold each virtual index until it lies in range.
                let mut position = index as i64 - i64::from(window / 2) + offset;
                while position < 0 || position >= values.len() as i64 {
                    if position < 0 {
                        position = -position;
                    } else {
                        position = 2 * (values.len() as i64 - 1) - position;
                    }
                }
                total += values[position as usize].unwrap();
            }
            assert!((actual.unwrap() - total / f64::from(window)).abs() < 1e-10);
        }
    }
    let result = reflected_window_average(&[Some(1.0), Some(3.0)], u32::MAX).unwrap();
    assert!((result[0].unwrap() - (2.0 + 1.0 / f64::from(u32::MAX))).abs() < 1e-14);
    assert!((result[1].unwrap() - (2.0 - 1.0 / f64::from(u32::MAX))).abs() < 1e-14);
}

#[test]
fn missing_and_non_finite_samples_never_create_artificial_zero_values() {
    for gap in [
        None,
        Some(f64::NAN),
        Some(f64::INFINITY),
        Some(f64::NEG_INFINITY),
    ] {
        let values = [Some(1.0), Some(2.0), gap, Some(4.0), Some(5.0)];
        let averaged = reflected_window_average(&values, 3).unwrap();
        assert_eq!(&averaged[1..4], &[None, None, None]);
        assert!((averaged[0].unwrap() - 5.0 / 3.0).abs() < 1e-12);
        assert!((averaged[4].unwrap() - 13.0 / 3.0).abs() < 1e-12);
        assert_eq!(reflected_window_average(&[gap], 100).unwrap(), vec![None]);
    }
}

#[test]
fn spectrum_uses_power_average_and_preserves_low_levels_after_large_steps() {
    use pnm::common::signal_processing::spectrum::window_average_dbmv;
    let levels = [Some(0.0), Some(20.0)];
    for level in window_average_dbmv(&levels, 2).unwrap() {
        assert!((level.unwrap() - 17.032913781186615).abs() < 1e-12);
    }
    let levels = [
        Some(300.0),
        Some(-300.0),
        Some(-300.0),
        Some(-300.0),
        Some(-300.0),
    ];
    let averaged = window_average_dbmv(&levels, 3).unwrap();
    for level in &averaged[2..] {
        assert!((level.unwrap() + 300.0).abs() < 1e-10);
    }
    for (level, actual) in levels.iter().zip(window_average_dbmv(&levels, 1).unwrap()) {
        assert!((level.unwrap() - actual.unwrap()).abs() < 1e-10);
    }
    assert!(window_average_dbmv(&levels, 0).is_none());
    assert_eq!(
        window_average_dbmv(&[Some(f64::NAN)], 3).unwrap(),
        vec![None]
    );
    assert_eq!(
        window_average_dbmv(&[Some(-10000.0)], 3).unwrap(),
        vec![None]
    );
    assert_eq!(
        window_average_dbmv(&[Some(10000.0)], 3).unwrap(),
        vec![None]
    );
}
