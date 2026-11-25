/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmSpectrumAnalysis.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{cm_spectrum_analysis::CmSpectrumAnalysis, parse_limits::ParseLimits},
};
use serde_json::json;
const SPECTRUM: &[u8] = pnm_fixtures::SPECTRUM;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmSpectrumAnalysis::parse(SPECTRUM, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(SPECTRUM)
    );
    assert_fixture_contract(SPECTRUM);
}
#[test]
fn golden_fixture() {
    let spectrum = json_capture(SPECTRUM);
    let segments = spectrum["amplitude_bin_segments_float"].as_array().unwrap();
    assert_eq!(segments.len(), 81);
    assert_eq!(segments[0].as_array().unwrap().len(), 256);
    assert_eq!(segments[0][0], -22.8);
    assert_eq!(segments[80][255], -77.3);
}
#[test]
fn malformed_lengths_and_boundaries() {
    let mut spectrum = SPECTRUM[..39].to_vec();
    spectrum[35..39].copy_from_slice(&6_u32.to_be_bytes());
    spectrum.extend_from_slice(&[0x80, 0, 0, 0, 0x7f, 0xff]);
    assert_eq!(
        json_capture(&spectrum)["amplitude_bin_segments_float"],
        json!([[-327.68, 0.0, 327.67]])
    );
    spectrum[29..31].copy_from_slice(&0_u16.to_be_bytes());
    assert_eq!(failure(&spectrum), ErrorCode::LengthMismatch);
}
