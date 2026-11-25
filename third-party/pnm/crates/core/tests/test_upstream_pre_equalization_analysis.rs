/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Synthetic known responses exercise analysis independently of real-capture assertions.
use pnm::{
    diagnostic::ErrorCode,
    pnm::{
        analysis::upstream_pre_equalization::PreEqualizationAnalysis,
        parser::{cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, parse_limits::ParseLimits},
    },
};

fn synthetic(pairs: &[[i16; 2]]) -> Vec<u8> {
    let mut bytes = pnm_fixtures::UPSTREAM_PRE_EQ[..34].to_vec();
    bytes[30..34].copy_from_slice(&((pairs.len() * 4) as u32).to_be_bytes());
    for pair in pairs {
        for component in pair {
            bytes.extend_from_slice(&component.to_be_bytes());
        }
    }
    bytes
}
#[test]
fn unit_magnitude_and_quarter_turn_phase_have_known_delay() {
    let bytes = synthetic(&[[8192, 0], [0, -8192], [-8192, 0], [0, 8192]]);
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    let analysis = PreEqualizationAnalysis::from_capture(&capture).unwrap();
    assert_eq!(analysis.magnitude_db, vec![Some(0.0); 4]);
    for point in analysis.group_delay {
        assert!((point.delay_s.unwrap() - 0.00001).abs() < 1e-16);
    }
}
#[test]
fn zero_splits_phase_runs_and_signed_minima_are_valid_coefficients() {
    let bytes = synthetic(&[[8192, 0], [0, 0], [-32768, -32768], [4096, 0]]);
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    let analysis = PreEqualizationAnalysis::from_capture(&capture).unwrap();
    assert_eq!(capture.iq().nth(2), Some(Some([-4.0, -4.0])));
    assert_eq!(analysis.magnitude_db[1], None);
    assert!((analysis.magnitude_db[2].unwrap() - 15.051499783199061).abs() < 1e-12);
    assert!((analysis.magnitude_db[3].unwrap() + 6.020599913279624).abs() < 1e-12);
    assert_eq!(analysis.group_delay[0].delay_s, None);
    assert_eq!(analysis.group_delay[1].delay_s, None);
    let output = serde_json::to_value(&capture).unwrap();
    assert!(output["pre_equalization_analysis"]["magnitude_db"][1].is_null());
    assert!(output["pre_equalization_analysis"]["group_delay"][0]["delay_s"].is_null());
}
#[test]
fn empty_single_zero_spacing_and_frequency_overflow_are_explicit() {
    for pairs in [&[][..], &[[8192, 0]][..]] {
        let bytes = synthetic(pairs);
        let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
        let analysis = PreEqualizationAnalysis::from_capture(&capture).unwrap();
        assert_eq!(analysis.frequencies_hz.len(), pairs.len());
        assert!(analysis
            .group_delay
            .iter()
            .all(|point| point.delay_s.is_none()));
    }
    let mut bytes = synthetic(&[[8192, 0]]);
    bytes[23..27].copy_from_slice(&u32::MAX.to_be_bytes());
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    let error = PreEqualizationAnalysis::from_capture(&capture).unwrap_err();
    assert_eq!(
        (error.code, error.offset, error.field),
        (ErrorCode::ArithmeticOverflow, 23, "zero_frequency")
    );
    assert!(capture.to_json(1_000_000).is_err());
    bytes[29] = 0;
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    let error = PreEqualizationAnalysis::from_capture(&capture).unwrap_err();
    assert_eq!(
        (error.code, error.offset, error.field),
        (ErrorCode::UnsupportedLayout, 29, "spacing_khz")
    );
}

#[test]
fn last_update_scaling_delay_gaps_and_grid_errors() {
    let mut bytes = synthetic(&[[8192, 0], [0, -8192], [-8192, 0], [0, 0], [-32768, -32768]]);
    let descriptor = pnm::pnm::parser::pnm_file_type::identify_tag("PNN7").unwrap();
    bytes[..4].copy_from_slice(&descriptor.signature);
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    let analysis = PreEqualizationAnalysis::from_capture(&capture).unwrap();
    assert_eq!(capture.iq().next(), Some(Some([0.5, 0.0])));
    for index in 0..3 {
        assert!((analysis.magnitude_db[index].unwrap() + 6.020599913279624).abs() < 1e-12);
        assert!((analysis.group_delay[index].delay_s.unwrap() - 0.00001).abs() < 1e-16);
    }
    assert_eq!(analysis.magnitude_db[3], None);
    assert_eq!(analysis.group_delay[3].delay_s, None);
    assert_eq!(analysis.group_delay[4].delay_s, None);
    assert_eq!(capture.iq().nth(4), Some(Some([-2.0, -2.0])));
    assert!((analysis.magnitude_db[4].unwrap() - 9.030899869919436).abs() < 1e-12);
    let json = serde_json::to_value(&capture).unwrap();
    assert!(json["pre_equalization_analysis"]["magnitude_db"][3].is_null());
    bytes[29] = 0;
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    assert!(capture.to_json(1_000_000).is_err());
    bytes[29] = 25;
    bytes[23..27].copy_from_slice(&u32::MAX.to_be_bytes());
    let capture = CmUsOfdmaPreEq::parse(&bytes, ParseLimits::default()).unwrap();
    assert!(capture.to_json(1_000_000).is_err());
}
