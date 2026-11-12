/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent power-ratio arithmetic and indexed decision-reference checks.
use docsis_ofdm_fec::signal_processing::modulation::{
    mer::{calculate_mer, MerCalculator, MerStatus},
    reference_for_order,
};

#[test]
fn known_complex_errors_use_power_sums_and_not_average_db() {
    let result = calculate_mer(&[[1.1, 0.0], [0.0, 1.2]], &[[1.0, 0.0], [0.0, 1.0]]);
    assert!((result.signal_power.unwrap() - 2.0).abs() < 1e-12);
    assert!((result.error_power.unwrap() - 0.05).abs() < 1e-12);
    assert!((result.mer_db.unwrap() - 16.020599913279625).abs() < 1e-10);
    assert!((result.rms_evm_percent.unwrap() - 15.811388300841896).abs() < 1e-10);
    assert_eq!(result.sample_count, 2);
}

#[test]
fn exceptional_readings_remain_explicit_and_json_safe() {
    let perfect = calculate_mer(&[[1.0, 0.0]], &[[1.0, 0.0]]);
    assert_eq!(perfect.calculation_status, MerStatus::PerfectReference);
    assert_eq!(perfect.mer_db, None);
    assert_eq!(perfect.rms_evm_percent, Some(0.0));
    assert_eq!(
        calculate_mer(&[], &[]).calculation_status,
        MerStatus::NoSamples
    );
    assert_eq!(
        calculate_mer(&[[0.0, 0.0]], &[]).calculation_status,
        MerStatus::SampleCountMismatch
    );
    assert_eq!(
        calculate_mer(&[[1.0, 0.0]], &[[0.0, 0.0]]).calculation_status,
        MerStatus::ZeroSignalPower
    );
    assert_eq!(
        calculate_mer(&[[f64::NAN, 0.0]], &[[1.0, 0.0]]).calculation_status,
        MerStatus::InvalidSamples
    );
    assert_eq!(
        calculate_mer(&[[f64::MAX, 0.0]], &[[1.0, 0.0]]).calculation_status,
        MerStatus::NonFiniteResult
    );
    let calculator = MerCalculator::new(2, 1.0).unwrap();
    let invalid = calculator.calculate([[1.0, 0.0], [f64::INFINITY, 0.0]]);
    assert_eq!(invalid.calculation_status, MerStatus::InvalidSamples);
    assert_eq!(invalid.sample_count, 2);
    assert!(serde_json::to_string(&invalid).unwrap().contains("null"));
    assert!(MerCalculator::new(32, 1.0).is_err());
    for amplitude in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(MerCalculator::new(2, amplitude).is_err());
    }
}

#[test]
fn row_index_matches_exhaustive_distances_for_every_supported_order() {
    for order in [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        let reference = reference_for_order(order).unwrap();
        let samples = [
            [0.0123, -0.4567],
            [1.37, -1.13],
            [-0.789, 0.231],
            [0.0, 0.0],
        ];
        let mut signal = 0.0;
        let mut error = 0.0;
        for sample in samples {
            let nearest = reference
                .iq
                .iter()
                .min_by(|first, second| {
                    let distance = |point: &[f64; 2]| {
                        (point[0] - sample[0]).powi(2) + (point[1] - sample[1]).powi(2)
                    };
                    distance(first).total_cmp(&distance(second))
                })
                .unwrap();
            signal += nearest[0].powi(2) + nearest[1].powi(2);
            error += (sample[0] - nearest[0]).powi(2) + (sample[1] - nearest[1]).powi(2);
        }
        let result = MerCalculator::new(order as u16, 1.0)
            .unwrap()
            .calculate(samples);
        assert!(
            (result.signal_power.unwrap() - signal).abs() < 1e-12,
            "order {order}"
        );
        assert!(
            (result.error_power.unwrap() - error).abs() < 1e-12,
            "order {order}"
        );
    }
}

#[test]
fn boosted_pilots_and_full_selection_are_not_rescaled_or_display_sampled() {
    let calculator = MerCalculator::new(2, 2.0).unwrap();
    let reading = calculator.calculate([[2.2, 0.0], [-2.0, 0.2]]);
    assert!((reading.mer_db.unwrap() - 20.0).abs() < 1e-10);
    let samples = (0..4096).map(|index| if index < 2048 { [2.0, 0.0] } else { [2.0, 0.2] });
    let reading = calculator.calculate(samples);
    assert_eq!(reading.sample_count, 4096);
    assert!((reading.error_power.unwrap() - 81.92).abs() < 1e-9);
    assert!((reading.mer_db.unwrap() - 23.010299956639813).abs() < 1e-9);
}

#[test]
fn tx_reference_does_not_hide_wrong_decisions() {
    let actual = docsis_ofdm_fec::signal_processing::modulation::mer::calculate_mer_pairs([(
        [-0.99, 0.0],
        [1.0, 0.0],
    )]);
    let nearest = MerCalculator::new(2, 1.0)
        .unwrap()
        .calculate([[-0.99, 0.0]]);
    assert!((actual.mer_db.unwrap() + 5.977061528194134).abs() < 1e-10);
    assert!((nearest.mer_db.unwrap() - 40.0).abs() < 1e-10);
}
