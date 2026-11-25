/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Header-only stub coverage; semantic decoding remains unsupported.
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_symbol_capture::CmSymbolCapture,
        parse_limits::ParseLimits,
        pnm_file::PnmFile,
        pnm_file_type::{identify_tag, HeaderLayout},
        pnm_header::HasPnmHeader,
    },
};
const CHANNEL: &[u8] = pnm_fixtures::CHANNEL_ESTIMATE;
fn header(tag: &str) -> Vec<u8> {
    let descriptor = identify_tag(tag).unwrap();
    let mut bytes = descriptor.signature.to_vec();
    bytes.extend_from_slice(&[1, 0]);
    if descriptor.layout == Some(HeaderLayout::WithCaptureTime) {
        bytes.extend_from_slice(&0_u32.to_be_bytes());
    }
    bytes
}
#[test]
fn symbol_capture_is_only_a_header_stub() {
    let mut bytes = header("PNN1");
    bytes.extend_from_slice(&[42, 43]);
    let stub = CmSymbolCapture::parse(&bytes).unwrap();
    assert_eq!(stub.pnm_header().pnm_data(), &[42, 43]);
    assert_eq!(
        PnmFile::from_bytes(&bytes)
            .decode(ParseLimits::default())
            .unwrap_err()
            .code,
        ErrorCode::UnsupportedDecoder
    );
    assert_eq!(
        CmSymbolCapture::parse(CHANNEL).unwrap_err().code,
        ErrorCode::UnsupportedType
    );
}
