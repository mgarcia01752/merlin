/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Synthetic latency coverage; no real capture fixture is available.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_latency_rpt::CmLatencyRpt,
        parse_limits::ParseLimits,
        pnm_file::PnmFile,
        pnm_file_type::{identify_tag, HeaderLayout},
    },
};
use serde_json::json;
fn header(tag: &str) -> Vec<u8> {
    let descriptor = identify_tag(tag).unwrap();
    let mut bytes = descriptor.signature.to_vec();
    bytes.extend_from_slice(&[1, 0]);
    if descriptor.layout == Some(HeaderLayout::WithCaptureTime) {
        bytes.extend_from_slice(&0_u32.to_be_bytes());
    }
    bytes
}
fn latency_fixture() -> Vec<u8> {
    // Synthetic LLD1 v1.0 per I27 Tables 83/84 and I13 Tables 116/117.
    let mut bytes = header("LLD01");
    bytes.extend_from_slice(&[1, 2, 3, 4, 5, 6, 1]); // MAC and one flow.
    bytes.extend_from_slice(&1050_u32.to_be_bytes());
    bytes.extend_from_slice(b"LLService1\0\0\0\0\0\0");
    bytes.push(1); // One edge, two histogram bins.
    bytes.extend_from_slice(&100_u16.to_be_bytes());
    bytes.extend_from_slice(&1_u16.to_be_bytes()); // One snapshot.
    bytes.extend_from_slice(&1456252718_u32.to_be_bytes());
    bytes.extend_from_slice(&1456252719_u32.to_be_bytes());
    bytes.extend_from_slice(&u64::MAX.to_be_bytes());
    bytes.extend_from_slice(&10_u64.to_be_bytes());
    for value in [57_u32, 2575, 9, 3, 4, 0, 5, 6] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes
}
#[test]
fn synthetic_layout_counters_units_and_truncations() {
    let bytes = latency_fixture();
    let output = json_capture(&bytes);
    assert!(output["pnm_header"]["capture_time"].is_null());
    let flow = &output["latency_data"][0];
    assert_eq!(flow["sf_id"], 1050);
    assert_eq!(flow["bin_edges_hundredths_ms"], json!([100]));
    assert_eq!(flow["snapshots"][0]["bin_counts"], json!([u64::MAX, 10]));
    assert_eq!(flow["snapshots"][0]["max_latency_us"], 57);
    assert_eq!(flow["snapshots"][0]["ce_marked_ect1_pkts"], 6);
    for end in 0..bytes.len() {
        assert!(PnmFile::from_bytes(&bytes[..end])
            .decode(ParseLimits::default())
            .is_err());
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert_eq!(failure(&extra), ErrorCode::LengthMismatch);
    assert_eq!(
        PnmFile::from_bytes(&bytes)
            .decode(ParseLimits { max_samples: 4 })
            .unwrap_err()
            .code,
        ErrorCode::LimitExceeded
    );

    assert_fixture_contract(&bytes);
    let capture = CmLatencyRpt::parse(&bytes, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        output
    );
}
