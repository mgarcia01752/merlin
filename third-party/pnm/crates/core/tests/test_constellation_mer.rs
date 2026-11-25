/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent power references and full-capture decision-directed analysis.
use pnm::{
    common::signal_processing::qam::reference_for_modulation_type,
    pnm::{
        analysis::constellation::mer::{calculate_mer, MerStatus},
        parser::{cm_ds_const_disp_meas::CmDsConstDispMeas, parse_limits::ParseLimits},
    },
};

#[test]
fn known_30_and_40_db_and_correct_complex_error_vectors() {
    for (quadrature_error, expected_mer, expected_error, expected_evm) in [
        (0.03162277660168379, 30.0, 0.001, 3.1622776601683795),
        (0.01, 40.0, 0.0001, 1.0),
    ] {
        for count in [1, 3, 8192] {
            let soft = vec![[1.0, quadrature_error]; count];
            let hard = vec![[1.0, 0.0]; count];
            let result = calculate_mer(&soft, &hard);
            assert_eq!(result.calculation_status, MerStatus::Available);
            assert_eq!(result.sample_count, count);
            assert!((result.mer_db.unwrap() - expected_mer).abs() < 1e-9);
            assert!((result.signal_power.unwrap() - count as f64).abs() < 1e-10);
            assert!((result.error_power.unwrap() - expected_error * count as f64).abs() < 1e-9);
            assert!((result.rms_evm_percent.unwrap() - expected_evm).abs() < 1e-9);
        }
    }
    let result = calculate_mer(&[[4.0, 6.0]], &[[3.0, 4.0]]);
    assert_eq!(result.signal_power, Some(25.0));
    assert_eq!(result.error_power, Some(5.0));
    assert!((result.mer_db.unwrap() - 6.989700043360188).abs() < 1e-12);
    // Pairing is per sample, not an aggregate/order-independent constellation.
    let reversed = calculate_mer(&[[1.0, 0.0], [-1.0, 0.0]], &[[-1.0, 0.0], [1.0, 0.0]]);
    assert_eq!(reversed.signal_power, Some(2.0));
    assert_eq!(reversed.error_power, Some(8.0));
    assert!((reversed.mer_db.unwrap() + 6.020599913279624).abs() < 1e-12);
}

#[test]
fn unavailable_and_perfect_results_never_encode_nan_or_infinity() {
    let perfect = calculate_mer(&[[1.0, -1.0], [-1.0, 1.0]], &[[1.0, -1.0], [-1.0, 1.0]]);
    assert_eq!(perfect.calculation_status, MerStatus::PerfectReference);
    assert_eq!(perfect.mer_db, None);
    assert_eq!(perfect.error_power, Some(0.0));
    assert_eq!(perfect.rms_evm_percent, Some(0.0));
    assert_eq!(
        calculate_mer(&[], &[]).calculation_status,
        MerStatus::NoSamples
    );
    for (soft, hard) in [(&[[1.0, 0.0]][..], &[][..]), (&[][..], &[[1.0, 0.0]][..])] {
        assert_eq!(
            calculate_mer(soft, hard).calculation_status,
            MerStatus::SampleCountMismatch
        );
    }
    let zero = calculate_mer(&[[1.0, 0.0]], &[[0.0, 0.0]]);
    assert_eq!(zero.calculation_status, MerStatus::ZeroSignalPower);
    assert_eq!(zero.signal_power, Some(0.0));
    assert_eq!(zero.error_power, Some(1.0));
    assert_eq!(zero.mer_db, None);
    assert_eq!(zero.rms_evm_percent, None);
    assert_eq!(
        calculate_mer(&[[0.0, 0.0]], &[[0.0, 0.0]]).calculation_status,
        MerStatus::ZeroSignalPower
    );
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        for axis in 0..2 {
            let mut invalid = [1.0, 1.0];
            invalid[axis] = value;
            for result in [
                calculate_mer(&[invalid], &[[1.0, 1.0]]),
                calculate_mer(&[[1.0, 1.0]], &[invalid]),
            ] {
                assert_eq!(result.calculation_status, MerStatus::InvalidSamples);
                assert_eq!(result.sample_count, 1);
                assert_eq!(result.signal_power, None);
                assert_eq!(result.mer_db, None);
                let json = serde_json::to_value(result).unwrap();
                assert!(json["mer_db"].is_null());
            }
        }
    }
    let overflow = calculate_mer(&[[f64::MAX, 0.0]], &[[1.0, 0.0]]);
    assert_eq!(overflow.calculation_status, MerStatus::NonFiniteResult);
    assert_eq!(overflow.mer_db, None);
}

fn bytes(code: u16, points: &[[i16; 2]], declared: u16) -> Vec<u8> {
    let mut bytes = pnm_fixtures::CONSTELLATION[..30].to_vec();
    bytes[21..23].copy_from_slice(&code.to_be_bytes());
    bytes[23..25].copy_from_slice(&declared.to_be_bytes());
    bytes[26..30].copy_from_slice(&(points.len() as u32 * 4).to_be_bytes());
    for point in points {
        bytes.extend_from_slice(&point[0].to_be_bytes());
        bytes.extend_from_slice(&point[1].to_be_bytes());
    }
    bytes
}

#[test]
fn all_viewer_modulations_match_an_independent_brute_force_slicer() {
    for code in 3..=13 {
        let reference = reference_for_modulation_type(code).unwrap();
        let points = [[1234, -4567], [24000, 24000], [-24000, 20000], [0, 0]];
        let bytes = bytes(code, &points, points.len() as u16);
        let capture = CmDsConstDispMeas::parse(&bytes, ParseLimits::default()).unwrap();
        let soft: Vec<_> = points
            .iter()
            .map(|point| [f64::from(point[0]) / 8192.0, f64::from(point[1]) / 8192.0])
            .collect();
        let hard: Vec<_> = soft
            .iter()
            .map(|sample| {
                // Independent full-table oracle. Ties use lower Q, then lower I.
                *reference
                    .iq
                    .iter()
                    .min_by(|first, second| {
                        let distance = |point: &&[f64; 2]| {
                            (sample[0] - point[0]).powi(2) + (sample[1] - point[1]).powi(2)
                        };
                        distance(first)
                            .total_cmp(&distance(second))
                            .then(first[1].total_cmp(&second[1]))
                            .then(first[0].total_cmp(&second[0]))
                    })
                    .unwrap()
            })
            .collect();
        assert_eq!(
            capture.constellation_analysis.mer,
            calculate_mer(&soft, &hard),
            "{}-QAM",
            reference.order
        );
        assert_eq!(
            capture.constellation_analysis.mer.calculation_status,
            MerStatus::Available
        );
    }
}

#[test]
fn parser_preserves_all_samples_and_reports_count_and_modulation_failures() {
    for (code, points, declared, status) in [
        (7, vec![], 0, MerStatus::NoSamples),
        (7, vec![[8192, 0]], 2, MerStatus::SampleCountMismatch),
        (
            u16::MAX,
            vec![[8192, 0]],
            1,
            MerStatus::UnsupportedModulation,
        ),
    ] {
        let bytes = bytes(code, &points, declared);
        let capture = CmDsConstDispMeas::parse(&bytes, ParseLimits::default()).unwrap();
        let result = capture.constellation_analysis.mer;
        assert_eq!(result.calculation_status, status);
        assert_eq!(result.sample_count, points.len());
        assert_eq!(result.mer_db, None);
        let json: serde_json::Value =
            serde_json::from_str(&capture.to_json(4_000_000).unwrap()).unwrap();
        assert_eq!(json["iq"].as_array().unwrap().len(), points.len());
        assert!(json["constellation_analysis"]["mer"]["mer_db"].is_null());
    }
    // 0x8000 is valid constellation data, not a missing-channel-estimate marker.
    let bytes = bytes(7, &[[i16::MIN, i16::MIN]], 1);
    let capture = CmDsConstDispMeas::parse(&bytes, ParseLimits::default()).unwrap();
    assert_eq!(capture.iq().next(), Some(Some([-4.0, -4.0])));
    assert_eq!(capture.constellation_analysis.mer.sample_count, 1);
    assert_eq!(
        capture.constellation_analysis.mer.calculation_status,
        MerStatus::Available
    );
}

#[test]
fn invalid_replacement_reference_is_unavailable_without_panicking() {
    use pnm::{
        common::signal_processing::qam::QamReference,
        pnm::analysis::constellation::mer::from_capture,
    };
    static EMPTY: QamReference = QamReference {
        order: 4,
        bit_loading: 2,
        scale_factor: 1.0,
        iq: &[],
        decision_boundaries: &[],
    };
    static INVALID: QamReference = QamReference {
        order: 4,
        bit_loading: 2,
        scale_factor: 1.0,
        iq: &[[f64::NAN, 0.0]],
        decision_boundaries: &[],
    };
    let bytes = bytes(7, &[[8192, 0]], 1);
    let mut capture = CmDsConstDispMeas::parse(&bytes, ParseLimits::default()).unwrap();
    for reference in [&EMPTY, &INVALID] {
        capture.constellation_analysis.reference = Some(reference);
        let result = from_capture(&capture);
        assert_eq!(result.calculation_status, MerStatus::InvalidSamples);
        assert_eq!(result.sample_count, 1);
        assert_eq!(result.mer_db, None);
    }
}
