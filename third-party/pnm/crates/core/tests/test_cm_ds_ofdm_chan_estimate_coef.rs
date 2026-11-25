/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsOfdmChanEstimateCoef.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef, parse_limits::ParseLimits,
    },
};
use serde_json::json;
const CHANNEL: &[u8] = pnm_fixtures::CHANNEL_ESTIMATE;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsOfdmChanEstimateCoef::parse(CHANNEL, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(CHANNEL)
    );
    assert_fixture_contract(CHANNEL);
}

#[test]
fn golden_channel() {
    let output = json_capture(CHANNEL);
    let values = output["iq"].as_array().unwrap();
    assert_eq!(values.len(), 7480);
    assert_eq!(values[0], json!([-0.216552734375, -1.1671142578125]));
    assert_eq!(values[7480 - 1], json!([-0.587890625, 0.593994140625]));
    assert_eq!(output["subcarrier_spacing"], 25000);
}
use pnm::pnm::parser::pnm_header::HasPnmHeader;
const UPSTREAM: &[u8] = pnm_fixtures::UPSTREAM_PRE_EQ;
#[test]
fn borrowed_samples_and_wrong_type() {
    let channel = CmDsOfdmChanEstimateCoef::parse(CHANNEL, ParseLimits::default()).unwrap();
    assert_eq!(channel.raw_iq().as_ptr(), CHANNEL[28..].as_ptr());
    assert_eq!(
        channel.pnm_header().pnm_data().as_ptr(),
        CHANNEL[10..].as_ptr()
    );
    assert_eq!(
        CmDsOfdmChanEstimateCoef::parse(UPSTREAM, ParseLimits::default())
            .unwrap_err()
            .code,
        ErrorCode::UnsupportedType
    );
}
#[test]
fn signed_boundaries_and_exclusion() {
    let mut channel = CHANNEL[..28].to_vec();
    channel[24..28].copy_from_slice(&12_u32.to_be_bytes());
    channel.extend_from_slice(&[0x80, 0, 0x80, 0, 0x80, 0, 0x7f, 0xff, 0x24, 0, 0xf8, 0]);
    assert_eq!(
        json_capture(&channel)["iq"],
        json!([null, [-4.0, 3.9998779296875], [1.125, -0.25]])
    );
    channel[24..28].copy_from_slice(&11_u32.to_be_bytes());
    assert_eq!(failure(&channel), ErrorCode::LengthMismatch);
}
