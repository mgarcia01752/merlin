/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent role-gain/label/response anchors and exact original-coordinate pairing.
mod common;
use docsis_ofdma::api::*;

const SOURCE_PRBS15_SEED: u16 = 1;
const DATA_LOADING: u8 = 4;
const BOOSTED_50KHZ_PATTERN: u8 = 5;
const BOOSTED_25KHZ_PATTERN: u8 = 12;
const GRANT_FRAMES: usize = 2;
const ACTIVE_MINISLOTS: usize = 8;
const FIRST_ACTIVE_CARRIER: usize = 128;
const REFERENCE_VARIANCE: f64 = 0.02;
const POWER_FLOOR: f64 = 1e-12;
const ITERATION_BUDGET: usize = 50;
const RETAINED_POINT_BUDGET: usize = 8192;
const NUMERICAL_TOLERANCE: f64 = 1e-10;
const NOISE_VARIANCE: f64 = 1e-4;

fn configuration(mode: Numerology, loading: u8, slots: usize) -> ExperimentConfig {
    let pattern = match mode {
        Numerology::Khz50 => BOOSTED_50KHZ_PATTERN,
        Numerology::Khz25 => BOOSTED_25KHZ_PATTERN,
    };
    ExperimentConfig::new(
        common::configuration(mode, loading, pattern, GRANT_FRAMES, slots),
        SOURCE_PRBS15_SEED,
        ReceiverConfig::new(
            Estimator::FrameFlatLs,
            REFERENCE_VARIANCE,
            POWER_FLOOR,
            ITERATION_BUDGET,
        )
        .unwrap(),
    )
    .unwrap()
}
fn session(config: ExperimentConfig) -> Simulation {
    let options = ExperimentOptions {
        retained_points: RETAINED_POINT_BUDGET,
        retained_taps: vec![Tap::TxGrid, Tap::TxPreEq, Tap::RxFft, Tap::RxEqualized],
        ..Default::default()
    };
    let mut session = Simulation::new(options).unwrap();
    session.configure(config).unwrap();
    session
}
fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < NUMERICAL_TOLERANCE,
        "{actual} != {expected}"
    );
}

#[test]
fn literal_upstream_labels_and_recorded_boosted_pilot_gain_normalize_both_modes() {
    const EXPECTED_16QAM_ENERGY: f64 = 10.0;
    const EXPECTED_BOOSTED_PILOT_POWER: f64 = 3.0;
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let mut simulation = session(configuration(mode, DATA_LOADING, ACTIVE_MINISLOTS));
        let report = simulation.run().unwrap();
        assert!(report.reference_recovered);
        let snapshot = report.qam_constellation(Default::default()).unwrap();
        assert_eq!(
            snapshot.normalization,
            QamNormalization::ReceiverUnitReference
        );
        assert_eq!(snapshot.references.len(), 16);
        for (label, real, imaginary) in [(0, 3.0, 3.0), (7, 1.0, -1.0), (10, -1.0, 3.0)] {
            let point = &snapshot.references[label];
            assert_eq!(usize::from(point.label), label);
            close(point.value.re * EXPECTED_16QAM_ENERGY.sqrt(), real);
            close(point.value.im * EXPECTED_16QAM_ENERGY.sqrt(), imaginary);
        }
        let original = report
            .qam(Tap::RxEqualized, Some(DATA_LOADING), MAX_QAM_QUERY_POINTS)
            .unwrap();
        for (view, raw) in snapshot.points.iter().zip(original.points) {
            assert_eq!(view.observation, raw);
            assert_eq!(view.unit_value, raw.value);
            assert_eq!(view.normalization_divisor, 1.0);
        }
        let pilots = report
            .qam_constellation(QamConstellationRequest {
                tap: Tap::TxGrid,
                loading: 0,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(pilots.payload_loading, 0);
        assert_eq!(pilots.reference_bits, 1);
        assert_eq!(pilots.references.len(), 2);
        assert_eq!(pilots.normalization, QamNormalization::TxRoleGainRemoved);
        for point in &pilots.points {
            assert_eq!(point.observation.role, CellRole::KnownPilot);
            assert_eq!(point.observation.loading, 0);
            close(
                point.observation.value.power(),
                EXPECTED_BOOSTED_PILOT_POWER,
            );
            close(
                point.normalization_divisor,
                EXPECTED_BOOSTED_PILOT_POWER.sqrt(),
            );
            close(point.unit_value.power(), 1.0);
        }
        assert_eq!(pilots.references[0].value, Iq { re: 1.0, im: 0.0 });
        assert_eq!(pilots.references[1].value, Iq { re: -1.0, im: 0.0 });
        simulation
            .configure(configuration(mode, DATA_LOADING, ACTIVE_MINISLOTS))
            .unwrap();
        assert_eq!(
            report
                .qam_constellation(QamConstellationRequest {
                    tap: Tap::TxGrid,
                    loading: 0,
                    ..Default::default()
                })
                .unwrap(),
            pilots
        );
    }
}

#[test]
fn exact_pairing_keeps_all_original_coordinates_counts_and_capped_left_order() {
    const DISPLAY_PAIR_LIMIT: usize = 3;
    const EXPECTED_50KHZ_MATCHED: usize = 112;
    const EXPECTED_25KHZ_MATCHED: usize = 240;
    const EXPECTED_TX_PILOTS: usize = 16;
    for (mode, expected_matches) in [
        (Numerology::Khz50, EXPECTED_50KHZ_MATCHED),
        (Numerology::Khz25, EXPECTED_25KHZ_MATCHED),
    ] {
        let report = session(configuration(mode, DATA_LOADING, ACTIVE_MINISLOTS))
            .run()
            .unwrap();
        let pairs = report.qam_pairing(Default::default()).unwrap();
        assert_eq!(pairs.matched_total, expected_matches);
        assert_eq!(pairs.unpaired_left, EXPECTED_TX_PILOTS);
        assert_eq!(
            pairs.matched_total + pairs.unpaired_right,
            pairs.right_selected
        );
        assert!(!pairs.capped);
        for pair in &pairs.pairs {
            assert_eq!(pair.left.identity, pair.right.identity);
            assert_eq!(pair.left.frame, pair.right.frame);
            assert_eq!(pair.left.symbol, pair.right.symbol);
            assert_eq!(pair.left.carrier, pair.right.carrier);
            assert_eq!(pair.left.loading, pair.right.loading);
            close(pair.left.value.re, pair.right.value.re);
            close(pair.left.value.im, pair.right.value.im);
        }
        let capped = report
            .qam_pairing(QamPairRequest {
                maximum_pairs: DISPLAY_PAIR_LIMIT,
                ..Default::default()
            })
            .unwrap();
        assert_eq!(capped.matched_total, pairs.matched_total);
        assert_eq!(capped.pairs, pairs.pairs[..DISPLAY_PAIR_LIMIT]);
        assert!(capped.capped);
        let missing_pilots = report
            .qam_pairing(QamPairRequest {
                loading: Some(0),
                ..Default::default()
            })
            .unwrap();
        assert!(missing_pilots.pairs.is_empty());
        assert_eq!(missing_pilots.unpaired_left, EXPECTED_TX_PILOTS);
        assert_eq!(missing_pilots.right_selected, 0);
    }
}

#[test]
fn reference_geometry_and_actual_display_use_independent_budgets_and_keep_supported_maximum() {
    const MAXIMUM_UPSTREAM_LOADING: u8 = 12;
    const ONE_ACTIVE_MINISLOT: usize = 1;
    const EXPECTED_REFERENCE_POINTS: usize = 4096;
    const DISPLAY_POINT_LIMIT: usize = 1;
    const OVER_POINT_LIMIT: usize = 4097;
    let report = session(configuration(
        Numerology::Khz50,
        MAXIMUM_UPSTREAM_LOADING,
        ONE_ACTIVE_MINISLOT,
    ))
    .run()
    .unwrap();
    let request = QamConstellationRequest {
        loading: MAXIMUM_UPSTREAM_LOADING,
        maximum_points: DISPLAY_POINT_LIMIT,
        ..Default::default()
    };
    let view = report.qam_constellation(request).unwrap();
    assert_eq!(view.points.len(), DISPLAY_POINT_LIMIT);
    assert_eq!(view.references.len(), EXPECTED_REFERENCE_POINTS);
    assert!(view.capped);
    for bad in [
        QamConstellationRequest {
            maximum_points: 0,
            ..request
        },
        QamConstellationRequest {
            maximum_points: OVER_POINT_LIMIT,
            ..request
        },
        QamConstellationRequest {
            maximum_references: OVER_POINT_LIMIT,
            ..request
        },
        QamConstellationRequest {
            maximum_references: DISPLAY_POINT_LIMIT,
            ..request
        },
    ] {
        assert!(report.qam_constellation(bad).is_err());
    }
    assert_eq!(report.qam_constellation(request).unwrap(), view);
}

#[test]
fn unsupported_planes_raw_availability_and_actual_failed_or_never_started_histories_are_explicit() {
    const OVER_PAIR_LIMIT: usize = 4097;
    let mut config = configuration(Numerology::Khz50, DATA_LOADING, ACTIVE_MINISLOTS);
    config.channel.taps = vec![Iq::default()];
    let mut simulation = session(config);
    let report = simulation.run().unwrap();
    assert!(!report.reference_recovered);
    for tap in [Tap::TxPreEq, Tap::RxFft] {
        assert!(!report
            .qam(tap, None, MAX_QAM_QUERY_POINTS)
            .unwrap()
            .points
            .is_empty());
        assert!(matches!(
            report.qam_constellation(QamConstellationRequest {
                tap,
                ..Default::default()
            }),
            Err(PhyError::Unsupported { .. })
        ));
    }
    assert!(report.qam_constellation(Default::default()).is_err());
    let empty = report.qam_pairing(Default::default()).unwrap();
    assert_eq!(empty.matched_total, 0);
    assert!(empty.left_selected > 0);
    assert_eq!(empty.right_selected, 0);
    assert_eq!(empty.unpaired_left, empty.left_selected);
    for maximum_pairs in [0, OVER_PAIR_LIMIT] {
        assert!(report
            .qam_pairing(QamPairRequest {
                maximum_pairs,
                ..Default::default()
            })
            .is_err());
    }
    assert!(report
        .qam_pairing(QamPairRequest {
            left: Tap::TxOutput,
            ..Default::default()
        })
        .is_err());
    let control = RunControl::new();
    control.cancel();
    let never_started = simulation.run_with_control(&control).unwrap();
    assert!(never_started
        .qam_pairing(Default::default())
        .unwrap()
        .pairs
        .is_empty());
    assert!(never_started.qam_constellation(Default::default()).is_err());
}

#[test]
fn passive_display_queries_preserve_noisy_science_and_original_report_json_shape() {
    let mut config = configuration(Numerology::Khz50, DATA_LOADING, ACTIVE_MINISLOTS);
    config.channel.noise_variance = NOISE_VARIANCE;
    let mut simulation = session(config);
    let first = simulation.run().unwrap();
    first.qam_constellation(Default::default()).unwrap();
    first.qam_pairing(Default::default()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&first.to_json().unwrap()).unwrap();
    for trace in json["observations"]["traces"].as_object().unwrap().values() {
        assert!(trace.get("role_gains").is_none());
    }
    let second = simulation.run().unwrap();
    assert_eq!(first.grants[0].words, second.grants[0].words);
    assert_eq!(
        first.grants[0].matched_data_mer,
        second.grants[0].matched_data_mer
    );
    assert_eq!(first.bit_errors, second.bit_errors);
    assert_eq!(first.ber, second.ber);
}

#[test]
fn actual_received_response_has_literal_db_phase_and_explicit_missing_zero_readings() {
    const ESTIMATE_FRAME_BUDGET: usize = 1;
    const ESTIMATE_SCALAR_BUDGET: usize = 8192;
    // Literal -1+2j has magnitude sqrt(5) and principal phase atan2(2,-1).
    const EXPECTED_AMPLITUDE_DB: f64 = 6.989_700_043_360_188;
    const EXPECTED_PHASE_RADIANS: f64 = 2.034_443_935_795_702_7;
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let mut config = configuration(mode, DATA_LOADING, ACTIVE_MINISLOTS);
        config.channel.taps = vec![Iq { re: -1.0, im: 2.0 }];
        let mut simulation = session(config);
        simulation
            .set_receiver_observation_policy(
                ReceiverObservationPolicy::new(ESTIMATE_FRAME_BUDGET, ESTIMATE_SCALAR_BUDGET)
                    .unwrap(),
            )
            .unwrap();
        let report = simulation.run().unwrap();
        let response = report.receiver_estimate(Default::default()).unwrap();
        let active = &response.points[FIRST_ACTIVE_CARRIER];
        close(active.amplitude_db.unwrap(), EXPECTED_AMPLITUDE_DB);
        close(active.phase_radians.unwrap(), EXPECTED_PHASE_RADIANS);
        assert_eq!(response.points[0].value, None);
        assert_eq!(response.points[0].amplitude_db, None);
        assert_eq!(response.points[0].phase_radians, None);
        let mut zero = configuration(mode, DATA_LOADING, ACTIVE_MINISLOTS);
        zero.channel.taps = vec![Iq::default()];
        simulation.configure(zero).unwrap();
        let response = simulation
            .run()
            .unwrap()
            .receiver_estimate(Default::default())
            .unwrap();
        assert_eq!(
            response.points[FIRST_ACTIVE_CARRIER].value,
            Some(Iq::default())
        );
        assert_eq!(response.points[FIRST_ACTIVE_CARRIER].amplitude_db, None);
        assert_eq!(response.points[FIRST_ACTIVE_CARRIER].phase_radians, None);
    }
}
