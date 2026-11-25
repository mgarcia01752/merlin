/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
        pnm_header::{HasPnmHeader, PnmFormatVersion},
    },
};
const FIXTURE: &[u8] = pnm_fixtures::RXMER;
fn parse(capture_bytes: &[u8]) -> Result<CmDsOfdmRxMer<'_>, pnm::diagnostic::ParseError> {
    CmDsOfdmRxMer::parse(capture_bytes, ParseLimits::default())
}
#[test]
fn golden_fixture() {
    let rxmer = parse(FIXTURE).unwrap();
    let header_access: &dyn HasPnmHeader = &rxmer;
    assert_eq!(
        header_access.format_version(),
        PnmFormatVersion { major: 1, minor: 0 }
    );
    assert_eq!(header_access.capture_time(), Some(1380970));
    assert_eq!(rxmer.channel_id, 34);
    assert_eq!(rxmer.mac_address, [0x00, 0xca, 0x00, 0x00, 0x00, 0x01]);
    assert_eq!(rxmer.subcarrier_zero_frequency_hz, 631100000);
    assert_eq!(rxmer.first_active_subcarrier_index, 356);
    assert_eq!(rxmer.subcarrier_spacing_khz, 25);
    assert_eq!(rxmer.raw_values().len(), 7480);
    assert_eq!(
        rxmer.values_db().take(3).collect::<Vec<_>>(),
        vec![Some(42.75), Some(43.0), Some(43.0)]
    );
    assert_eq!(rxmer.frequencies_hz().unwrap().next(), Some(640000000));
    assert_eq!(rxmer.frequencies_hz().unwrap().last(), Some(826975000));
}
#[test]
fn every_truncated_prefix_fails_without_panic() {
    for end in 0..FIXTURE.len() {
        assert!(parse(&FIXTURE[..end]).is_err(), "prefix {end}");
    }
}
#[test]
fn reject_other_types_versions_and_extra_bytes() {
    let fec_summary_bytes = pnm_fixtures::FEC_SUMMARY;
    assert_eq!(
        parse(fec_summary_bytes).unwrap_err().code,
        ErrorCode::UnsupportedType
    );
    for offset in [4, 5] {
        let mut bytes = FIXTURE.to_vec();
        bytes[offset] = 2;
        assert_eq!(
            parse(&bytes).unwrap_err().code,
            ErrorCode::UnsupportedVersion
        );
    }
    let mut bytes = FIXTURE.to_vec();
    bytes.push(0);
    assert_eq!(parse(&bytes).unwrap_err().code, ErrorCode::LengthMismatch);
}
#[test]
fn enforce_limits_and_declared_length() {
    assert_eq!(
        CmDsOfdmRxMer::parse(FIXTURE, ParseLimits { max_samples: 10 })
            .unwrap_err()
            .code,
        ErrorCode::LimitExceeded
    );
    let mut bytes = FIXTURE.to_vec();
    bytes[24..28].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(parse(&bytes).unwrap_err().code, ErrorCode::LimitExceeded);
    bytes[24..28].copy_from_slice(&1_u32.to_be_bytes());
    assert_eq!(parse(&bytes).unwrap_err().code, ErrorCode::LengthMismatch);
}
#[test]
fn preserve_missing_and_boundary_values() {
    let mut bytes = FIXTURE[..28].to_vec();
    bytes[24..28].copy_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&[0, 95, 254, 255]);
    let rxmer = parse(&bytes).unwrap();
    assert_eq!(
        rxmer.values_db().collect::<Vec<_>>(),
        vec![Some(0.0), Some(23.75), Some(63.5), None]
    );
    assert_eq!(rxmer.raw_values(), &[0, 95, 254, 255]);
}

#[test]
fn frequencies_fit_u32_and_overflow_is_rejected() {
    let mut bytes = FIXTURE[..28].to_vec();
    bytes[21..23].copy_from_slice(&0_u16.to_be_bytes());
    bytes[24..28].copy_from_slice(&2_u32.to_be_bytes());
    bytes.extend_from_slice(&[100, 100]);
    bytes[17..21].copy_from_slice(&3_000_000_000_u32.to_be_bytes());
    assert_eq!(
        parse(&bytes).unwrap().frequencies_hz().unwrap().last(),
        Some(3_000_025_000)
    );
    bytes[17..21].copy_from_slice(&(u32::MAX - 25_000).to_be_bytes());
    assert_eq!(
        parse(&bytes).unwrap().frequencies_hz().unwrap().last(),
        Some(u32::MAX)
    );
    bytes[17..21].copy_from_slice(&(u32::MAX - 24_999).to_be_bytes());
    assert_eq!(
        parse(&bytes).unwrap_err().code,
        ErrorCode::ArithmeticOverflow
    );
    bytes[17..21].copy_from_slice(&0_u32.to_be_bytes());
    let mut capture = parse(&bytes).unwrap();
    capture.subcarrier_zero_frequency_hz = u32::MAX;
    assert!(capture.frequencies_hz().is_err());
}
