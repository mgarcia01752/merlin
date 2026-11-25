/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared assertions; each parser owns and invokes its own test cases.
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        parse_limits::ParseLimits,
        pnm_file::{DecodedPnm, PnmFile},
    },
};
use serde_json::Value;
const JSON_BUDGET: usize = 4_000_000;
fn decode(bytes: &[u8]) -> DecodedPnm<'_> {
    PnmFile::from_bytes(bytes)
        .decode(ParseLimits::default())
        .unwrap()
}
/// Exports a capture through dispatch for comparison with parser assertions.
/// # Arguments
/// * `bytes` - Complete test capture.
/// # Returns
/// Parsed JSON; panics on a test failure.
pub fn json_capture(bytes: &[u8]) -> Value {
    serde_json::from_str(&decode(bytes).to_json(JSON_BUDGET).unwrap()).unwrap()
}
/// Reads the dispatch diagnostic for a deliberately invalid capture.
/// # Arguments
/// * `bytes` - Invalid test capture.
/// # Returns
/// Error code; panics if accepted.
pub fn failure(bytes: &[u8]) -> ErrorCode {
    PnmFile::from_bytes(bytes)
        .decode(ParseLimits::default())
        .unwrap_err()
        .code
}
/// Checks shared malformed-input and JSON-budget behavior for one fixture.
/// # Arguments
/// * `bytes` - Valid capture, real or explicitly synthetic.
/// # Returns
/// Nothing; assertions panic on regression.
pub fn assert_fixture_contract(bytes: &[u8]) {
    // Every fixed-header boundary plus representative nested-record boundaries.
    for end in (0..40.min(bytes.len())).chain([bytes.len() / 2, bytes.len() - 1]) {
        assert!(
            PnmFile::from_bytes(&bytes[..end])
                .decode(ParseLimits::default())
                .is_err(),
            "accepted truncation at {end}"
        );
    }
    let mut extra = bytes.to_vec();
    extra.push(0);
    assert_eq!(failure(&extra), ErrorCode::LengthMismatch);
    let mut version = bytes.to_vec();
    version[5] = 1;
    assert_eq!(failure(&version), ErrorCode::UnsupportedVersion);
    assert_eq!(
        PnmFile::from_bytes(bytes)
            .decode(ParseLimits { max_samples: 0 })
            .unwrap_err()
            .code,
        ErrorCode::LimitExceeded
    );
    let capture = decode(bytes);
    assert!(capture.to_json(1).is_err());
    let output = capture.to_json(JSON_BUDGET).unwrap();
    assert!(capture.to_json(output.len() - 1).is_err());
    assert_eq!(capture.to_json(output.len()).unwrap(), output);
}
