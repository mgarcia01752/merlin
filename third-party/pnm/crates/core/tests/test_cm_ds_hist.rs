/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsHist.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{cm_ds_hist::CmDsHist, parse_limits::ParseLimits},
};
use serde_json::json;
const HISTOGRAM: &[u8] = pnm_fixtures::HISTOGRAM;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsHist::parse(HISTOGRAM, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(HISTOGRAM)
    );
    assert_fixture_contract(HISTOGRAM);
}
#[test]
fn golden_fixture() {
    let histogram = json_capture(HISTOGRAM);
    assert_eq!(histogram["symmetry"], 2); // Preserve observed value rather than coercing to bool.
    assert_eq!(histogram["dwell_count_values"], json!([16777216]));
    assert_eq!(histogram["hit_count_values"].as_array().unwrap().len(), 256);
}
#[test]
fn malformed_lengths_and_boundaries() {
    let mut histogram = HISTOGRAM.to_vec();
    histogram[17..21].copy_from_slice(&3_u32.to_be_bytes());
    assert_eq!(failure(&histogram), ErrorCode::LengthMismatch);
}
