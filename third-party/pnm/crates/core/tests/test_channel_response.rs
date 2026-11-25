/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::channel_response::{group_delay_seconds, magnitude_db};

#[test]
fn magnitude_known_values_and_unavailable_samples() {
    assert_eq!(magnitude_db(Some([1.0, 0.0])), Some(0.0));
    assert!((magnitude_db(Some([3.0, 4.0])).unwrap() - 13.979400086720377).abs() < 1e-12);
    assert!((magnitude_db(Some([-0.5, 0.0])).unwrap() + 6.020599913279624).abs() < 1e-12);
    for sample in [
        None,
        Some([0.0, 0.0]),
        Some([f64::NAN, 1.0]),
        Some([1.0, f64::INFINITY]),
    ] {
        assert_eq!(magnitude_db(sample), None);
    }
}
#[test]
fn known_delays_survive_phase_wraps_and_edge_differences() {
    for delay_seconds in [-0.000002, 0.0, 0.000003] {
        let samples: Vec<_> = (0..100)
            .map(|index| {
                let phase =
                    2.8 - std::f64::consts::TAU * delay_seconds * 25_000.0 * f64::from(index);
                Some([phase.cos(), phase.sin()])
            })
            .collect();
        for result in group_delay_seconds(&samples, 25_000) {
            assert!((result.unwrap() - delay_seconds).abs() < 1e-16);
        }
    }
}
#[test]
fn missing_zero_and_nonfinite_split_runs_without_poisoning_later_carriers() {
    let samples = [
        Some([1.0, 0.0]),
        None,
        Some([0.0, 0.0]),
        Some([f64::NAN, 1.0]),
        Some([1.0, 0.0]),
        Some([0.0, -1.0]),
    ];
    let result = group_delay_seconds(&samples, 25_000);
    assert_eq!(&result[..4], &[None; 4]);
    for delay in &result[4..] {
        assert!((delay.unwrap() - 0.00001).abs() < 1e-16);
    }
    assert_eq!(group_delay_seconds(&samples, 0), vec![None; samples.len()]);
    assert!(group_delay_seconds(&[], 25_000).is_empty());
}
#[test]
fn quadratic_phase_has_second_order_edges() {
    let samples: Vec<_> = (0..5)
        .map(|index| {
            let phase = 0.01 * f64::from(index * index);
            Some([phase.cos(), phase.sin()])
        })
        .collect();
    for (index, delay) in group_delay_seconds(&samples, 25_000).iter().enumerate() {
        let expected = -(0.02 * index as f64) / (std::f64::consts::TAU * 25_000.0);
        assert!((delay.unwrap() - expected).abs() < 1e-18);
    }
}
