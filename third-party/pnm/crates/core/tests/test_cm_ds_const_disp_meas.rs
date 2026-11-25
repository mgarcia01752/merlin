/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsConstDispMeas.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{cm_ds_const_disp_meas::CmDsConstDispMeas, parse_limits::ParseLimits},
};
use serde_json::json;
const CONSTELLATION: &[u8] = pnm_fixtures::CONSTELLATION;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsConstDispMeas::parse(CONSTELLATION, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(CONSTELLATION)
    );
    assert_fixture_contract(CONSTELLATION);
}

#[test]
fn golden_constellation() {
    let output = json_capture(CONSTELLATION);
    let values = output["iq"].as_array().unwrap();
    assert_eq!(values.len(), 8192);
    assert_eq!(values[0], json!([0.376953125, -1.15625]));
    assert_eq!(values[8192 - 1], json!([1.146484375, 0.0859375]));
    assert_eq!(output["subcarrier_spacing"], 25000);
}

#[test]
fn signed_boundaries_and_iq_order() {
    let mut bytes = CONSTELLATION[..30].to_vec();
    bytes[26..30].copy_from_slice(&12_u32.to_be_bytes());
    bytes.extend_from_slice(&[0x80, 0, 0x80, 0, 0x7f, 0xff, 0, 1, 0x24, 0, 0xf8, 0]);
    assert_eq!(
        json_capture(&bytes)["iq"],
        json!([
            [-4.0, -4.0],
            [3.9998779296875, 0.0001220703125],
            [1.125, -0.25]
        ])
    );
    bytes[26..30].copy_from_slice(&11_u32.to_be_bytes());
    assert_eq!(failure(&bytes), ErrorCode::LengthMismatch);
}

#[test]
fn shared_reference_json_preserves_unscaled_received_samples() {
    let output = json_capture(CONSTELLATION);
    let analysis = &output["constellation_analysis"];
    assert_eq!(analysis["normalization"], "unit_average_symbol_power");
    assert_eq!(analysis["soft_samples_rescaled"], false);
    assert_eq!(analysis["reference"]["order"], 256);
    assert_eq!(analysis["reference"]["bit_loading"], 8);
    assert_eq!(analysis["decision_rule"], "nearest_reference_euclidean");
    assert_eq!(
        analysis["reference"]["decision_boundaries"]
            .as_array()
            .unwrap()
            .len(),
        30
    );
    assert_eq!(analysis["reference"]["iq"].as_array().unwrap().len(), 256);
    assert!(
        (analysis["reference"]["scale_factor"].as_f64().unwrap() - 0.07669649888473704).abs()
            < 1e-14
    );
    assert_eq!(output["iq"][0], json!([0.376953125, -1.15625]));
    assert_eq!(output["iq"][8191], json!([1.146484375, 0.0859375]));
    let mut unknown = CONSTELLATION.to_vec();
    unknown[21..23].copy_from_slice(&u16::MAX.to_be_bytes());
    let unknown = json_capture(&unknown);
    assert!(unknown["constellation_analysis"]["reference"].is_null());
    assert_eq!(unknown["iq"], output["iq"]);
}
