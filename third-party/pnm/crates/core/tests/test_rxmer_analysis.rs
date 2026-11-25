/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Full-resolution RxMER estimates, coverage and ideal gross rates.
use pnm::{
    common::signal_processing::bit_loading::BitLoadingModel,
    pnm::{
        analysis::rxmer::{RxMerAnalysisRequest, RxMerBitLoadingAnalysis},
        parser::cm_ds_ofdm_rx_mer::CmDsOfdmRxMer,
    },
};
fn synthetic(values: &[u8], spacing: u8) -> Vec<u8> {
    let mut bytes = pnm_fixtures::RXMER[..28].to_vec();
    bytes[23] = spacing;
    bytes[24..28].copy_from_slice(&(values.len() as u32).to_be_bytes());
    bytes.extend_from_slice(values);
    bytes
}
fn analyze(bytes: &[u8], request: RxMerAnalysisRequest) -> RxMerBitLoadingAnalysis {
    let capture = CmDsOfdmRxMer::parse(bytes, Default::default()).unwrap();
    RxMerBitLoadingAnalysis::calculate(&capture, request).unwrap()
}
#[test]
fn fixture_zoom_matches_the_approved_proof_and_all_scenarios() {
    let request = RxMerAnalysisRequest {
        start_frequency_hz: Some(650_000_000),
        stop_frequency_hz: Some(675_000_000),
        ..Default::default()
    };
    let result = analyze(pnm_fixtures::RXMER, request);
    assert_eq!(result.selection.sample_count, 1001);
    assert_eq!(result.selection.first_sample_index, 400);
    assert_eq!(result.summary.minimum_rxmer_db, Some(39.25));
    assert_eq!(result.summary.uniform_bit_loading, Some(10));
    assert_eq!(result.summary.uniform_modulation, Some("qam_1024"));
    assert_eq!(result.summary.remaining_margin_db, Some(0.25));
    assert_eq!(result.rate.estimated_gross_rate_bps, Some(250_250_000));
    assert!(!result.coverage.carrier_roles_verified);
    let highest = result.uniform_scenarios.last().unwrap();
    assert_eq!(highest.bit_loading, 13);
    assert_eq!(highest.status, "not_covered_by_model");
    assert_eq!(highest.estimated_gross_rate_bps, Some(325_325_000));
    let ossi = analyze(
        pnm_fixtures::RXMER,
        RxMerAnalysisRequest {
            model: BitLoadingModel::OssiCandidateDefaults,
            ..request
        },
    );
    assert_eq!(ossi.summary.uniform_bit_loading, Some(11));
    assert_eq!(ossi.rate.estimated_gross_rate_bps, Some(275_275_000));
    let json: serde_json::Value =
        serde_json::from_str(&result.to_json(16 * 1024 * 1024).unwrap()).unwrap();
    assert_eq!(json["subcarriers"].as_array().unwrap().len(), 1001);
    assert_eq!(json["subcarriers"][0]["rxmer_db"], 41.75);
    assert!(result.to_json(50).is_err());
}
#[test]
fn weakest_carrier_and_missing_values_are_not_hidden_by_averages() {
    let mut values = vec![180; 4097];
    values[2049] = 132;
    values[100] = 255;
    let result = analyze(&synthetic(&values, 25), Default::default());
    assert_eq!(result.summary.uniform_bit_loading, Some(8));
    assert_eq!(result.summary.minimum_rxmer_db, Some(33.0));
    assert_eq!(result.coverage.measured_count, 4096);
    assert_eq!(result.coverage.unavailable_count, 1);
    assert_eq!(result.coverage.status, "measured_carriers_only");
    assert_eq!(result.rate.estimated_gross_rate_bps, Some(819_200_000));
    assert_eq!(result.subcarriers[100].rxmer_db, None);
    assert_eq!(result.subcarriers[100].bit_loading, None);
    let low = analyze(&synthetic(&[180, 0, 180], 25), Default::default());
    assert_eq!(low.summary.uniform_bit_loading, None);
    assert_eq!(low.rate.estimated_gross_rate_bps, None);
    assert_eq!(low.summary.status, "below_model_minimum");
    assert_eq!(low.coverage.measured_count, 3);
    for values in [&[][..], &[255, 255][..]] {
        let empty = analyze(&synthetic(values, 25), Default::default());
        assert_eq!(empty.summary.status, "no_measurements");
        assert_eq!(empty.rate.estimated_gross_rate_bps, None);
    }
    let doubled = analyze(&synthetic(&[156], 50), Default::default());
    assert_eq!(doubled.rate.estimated_gross_rate_bps, Some(500_000));
}
#[test]
fn ranges_and_controls_are_checked_before_analysis() {
    let bytes = synthetic(&[156, 156], 25);
    let capture = CmDsOfdmRxMer::parse(&bytes, Default::default()).unwrap();
    let base = RxMerAnalysisRequest::default();
    for request in [
        RxMerAnalysisRequest {
            extra_margin_db: f64::NAN,
            ..base
        },
        RxMerAnalysisRequest {
            extra_margin_db: -1.0,
            ..base
        },
        RxMerAnalysisRequest {
            maximum_bit_loading: 5,
            ..base
        },
        RxMerAnalysisRequest {
            start_frequency_hz: Some(640000000),
            ..base
        },
        RxMerAnalysisRequest {
            start_frequency_hz: Some(640000001),
            stop_frequency_hz: Some(640000002),
            ..base
        },
        RxMerAnalysisRequest {
            start_frequency_hz: Some(1),
            stop_frequency_hz: Some(640000000),
            ..base
        },
    ] {
        assert!(RxMerBitLoadingAnalysis::calculate(&capture, request).is_err());
    }
    let zero = synthetic(&[156], 0);
    let capture = CmDsOfdmRxMer::parse(&zero, Default::default()).unwrap();
    assert!(RxMerBitLoadingAnalysis::calculate(&capture, base).is_err());
}
