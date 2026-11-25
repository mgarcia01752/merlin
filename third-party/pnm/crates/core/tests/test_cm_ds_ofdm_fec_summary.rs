/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsOfdmFecSummary.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{cm_ds_ofdm_fec_summary::CmDsOfdmFecSummary, parse_limits::ParseLimits},
};
use serde_json::json;
const FEC: &[u8] = pnm_fixtures::FEC_SUMMARY;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsOfdmFecSummary::parse(FEC, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(FEC)
    );
    assert_fixture_contract(FEC);
}
#[test]
fn golden_fixture() {
    let fec = json_capture(FEC);
    assert!(fec["pnm_header"]["capture_time"].is_null());
    let profiles = fec["fec_summary_data"].as_array().unwrap();
    assert_eq!(profiles.len(), 5);
    assert_eq!(profiles[0]["profile_id"], 255);
    assert_eq!(profiles[0]["number_of_sets"], 600);
    assert_eq!(
        profiles[0]["codeword_entries"][0],
        json!({"timestamp":1762636604,"total_codewords":44444,"corrected":0,"uncorrectable":0})
    );
    assert_eq!(profiles[4]["codeword_entries"][599]["corrected"], 1);
}
#[test]
fn malformed_lengths_and_boundaries() {
    let mut fec = FEC.to_vec();
    fec[14] += 1;
    assert_eq!(failure(&fec), ErrorCode::Truncated);
}
