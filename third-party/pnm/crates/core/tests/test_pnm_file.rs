/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
        model::DEFAULT_MAX_JSON_BYTES,
        pnm_file::{DecodedPnm, PnmFile},
        pnm_file_type::{identify, identify_tag, PnmDecoderKind, FILE_TYPES},
        pnm_header::{HasPnmHeader, PnmFormatVersion, PnmHeader},
        reader::Reader,
    },
};
const RXMER: &[u8] = pnm_fixtures::RXMER;
const FEC: &[u8] = pnm_fixtures::FEC_SUMMARY;

#[test]
fn header_retains_original_payload_and_shared_access() {
    for (bytes, offset, capture_time) in [(RXMER, 10, Some(1380970)), (FEC, 6, None)] {
        let header = PnmHeader::from_bytes(bytes).unwrap();
        let access: &dyn HasPnmHeader = &header;
        assert_eq!(access.capture_time(), capture_time);
        assert_eq!(
            access.format_version(),
            PnmFormatVersion { major: 1, minor: 0 }
        );
        assert_eq!(header.payload_offset(), offset);
        assert_eq!(header.pnm_data(), &bytes[offset..]);
        assert_eq!(header.pnm_data().as_ptr(), bytes[offset..].as_ptr());
        for end in 0..offset {
            let failure = PnmHeader::from_bytes(&bytes[..end]).unwrap_err();
            assert_eq!(failure.code, ErrorCode::Truncated);
        }
    }
}

#[test]
fn dispatch_reuses_header_and_matches_direct_parser() {
    let file = PnmFile::from_bytes(RXMER);
    let decoded = file.decode(ParseLimits::default()).unwrap();
    let DecodedPnm::RxMer(capture) = decoded else {
        panic!("wrong decoder")
    };
    assert_eq!(
        capture,
        CmDsOfdmRxMer::parse(RXMER, ParseLimits::default()).unwrap()
    );
    assert_eq!(capture.pnm_header(), file.header().unwrap());
    assert_eq!(capture.raw_values().as_ptr(), RXMER[28..].as_ptr());
    assert_eq!(file.bytes().as_ptr(), RXMER.as_ptr());
    assert_eq!(
        file.decode(ParseLimits { max_samples: 1 })
            .unwrap_err()
            .code,
        ErrorCode::LimitExceeded
    );
    assert!(matches!(
        PnmFile::from_bytes(FEC)
            .decode(ParseLimits::default())
            .unwrap(),
        DecodedPnm::FecSummary(_)
    ));
}

#[test]
fn unsupported_inputs_remain_inspectable() {
    let mut unknown = RXMER.to_vec();
    unknown[3] = 255;
    let mut version = RXMER.to_vec();
    version[4] = 2;
    let mut unverified = RXMER.to_vec();
    unverified[3] = 1;
    let mut false_fec = FEC.to_vec();
    false_fec[0] = b'X';
    for (bytes, code) in [
        (&unknown, ErrorCode::UnsupportedType),
        (&version, ErrorCode::UnsupportedVersion),
        (&false_fec, ErrorCode::UnsupportedType),
    ] {
        let file = PnmFile::from_bytes(bytes);
        assert_eq!(file.bytes(), bytes);
        assert_eq!(file.header().unwrap_err().code, code);
        assert_eq!(file.decode(ParseLimits::default()).unwrap_err().code, code);
    }
    assert!(PnmFile::from_bytes(&unverified).header().is_ok());
    assert_eq!(
        PnmFile::from_bytes(&unverified)
            .decode(ParseLimits::default())
            .unwrap_err()
            .code,
        ErrorCode::UnsupportedDecoder
    );
    assert_eq!(
        PnmFile::from_bytes(&RXMER[..3]).header().unwrap_err().code,
        ErrorCode::Truncated
    );
}

#[test]
fn payload_errors_keep_original_file_offsets() {
    for (end, offset, field) in [
        (10, 10, "channel_id"),
        (16, 11, "mac_address"),
        (20, 17, "zero_frequency"),
        (22, 21, "first_subcarrier"),
        (23, 23, "spacing_khz"),
        (27, 24, "data_length"),
    ] {
        let file = PnmFile::from_bytes(&RXMER[..end]);
        let error = file.decode(ParseLimits::default()).unwrap_err();
        assert_eq!(error.code, ErrorCode::Truncated);
        assert_eq!(error.offset, offset);
        assert_eq!(error.field, field);
    }
    assert!(Reader::with_origin(&[0], usize::MAX).is_err());
    let mut reader = Reader::with_origin(&[1], 10).unwrap();
    assert_eq!(reader.read_u16_be("short").unwrap_err().offset, 10);
    assert_eq!(reader.read_u8("last").unwrap(), 1);
    assert_eq!(reader.position(), 11);
    let mut reader = Reader::with_origin(&[1, 2, 3], 10).unwrap();
    assert_eq!(
        reader
            .read_slice(usize::MAX, "overflow")
            .unwrap_err()
            .offset,
        10
    );
    assert_eq!(reader.position(), 10);
    assert_eq!(reader.read_slice(2, "slice").unwrap(), &[1, 2]);
    assert_eq!(reader.read_slice(2, "short").unwrap_err().offset, 12);
    assert_eq!(reader.read_u8("last").unwrap(), 3);
}

#[test]
fn centralized_identities_are_unique_and_routes_preserve_identity() {
    for (index, descriptor) in FILE_TYPES.iter().enumerate() {
        assert_eq!(identify(descriptor.signature), Some(descriptor));
        assert_eq!(identify_tag(descriptor.tag), Some(descriptor));
        assert_eq!(
            descriptor.header_version.is_some(),
            descriptor.layout.is_some()
        );
        for other in &FILE_TYPES[index + 1..] {
            assert_ne!(descriptor.signature, other.signature);
            assert_ne!(descriptor.tag, other.tag);
            assert_ne!(descriptor.kind, other.kind);
        }
    }
    let first = identify_tag("PNN6").unwrap();
    let second = identify_tag("PNN7").unwrap();
    assert_ne!(first.kind, second.kind);
    assert_eq!(first.decoder, second.decoder);
    assert_eq!(first.decoder, PnmDecoderKind::UpstreamPreEqualization);
    assert_eq!(identify_tag(" lld1 ").unwrap().tag, "LLD01");
    assert_eq!(identify_tag("pnn010").unwrap().signature[3], 10);
    assert!(identify(*b"PNN4").is_none());
    for tag in ["PXX9", "PNN256", "PNN+4", "LLD", "PNN", "PNNé"] {
        assert!(identify_tag(tag).is_none());
    }
}

#[test]
fn json_matches_output_contract_and_enforces_limits() {
    let header = PnmHeader::from_bytes(RXMER).unwrap();
    let header_json: serde_json::Value =
        serde_json::from_str(&header.to_json(false, DEFAULT_MAX_JSON_BYTES).unwrap()).unwrap();
    assert_eq!(header_json["pnm_header"]["file_type"], "PNN");
    assert_eq!(header_json["pnm_header"]["file_type_version"], 4);
    assert_eq!(header_json["pnm_header"]["capture_time"], 1380970);
    assert!(header_json.get("data").is_none());
    let fec_json: serde_json::Value = serde_json::from_str(
        &PnmHeader::from_bytes(FEC)
            .unwrap()
            .to_json(false, 1024)
            .unwrap(),
    )
    .unwrap();
    assert!(fec_json["pnm_header"]["capture_time"].is_null());
    let hex_json: serde_json::Value =
        serde_json::from_str(&header.to_json(true, DEFAULT_MAX_JSON_BYTES).unwrap()).unwrap();
    let hex = hex_json["data"].as_str().unwrap();
    assert_eq!(hex.len(), header.pnm_data().len() * 2);
    for (index, &byte) in header.pnm_data().iter().enumerate() {
        assert_eq!(
            u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).unwrap(),
            byte
        );
    }
    assert!(header.to_json(true, 10).is_err());
    assert!(header.to_json(false, 1).is_err());
    let mut bytes = RXMER[..28].to_vec();
    bytes[24..28].copy_from_slice(&4_u32.to_be_bytes());
    bytes.extend_from_slice(&[0, 95, 254, 255]);
    let capture = CmDsOfdmRxMer::parse(&bytes, ParseLimits::default()).unwrap();
    let json = capture.to_json(DEFAULT_MAX_JSON_BYTES).unwrap();
    let model: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_value(&capture).unwrap(), model);
    assert_eq!(model.as_object().unwrap().len(), 9);
    assert_eq!(model["values"], serde_json::json!([0.0, 23.75, 63.5, null]));
    assert_eq!(model["mac_address"], "00:ca:00:00:00:01");
    assert_eq!(model["subcarrier_spacing"], 25000);
    assert_eq!(model["data_length"], 4);
    assert_eq!(model["value_units"], "dB");
    assert!(capture.to_json(json.len() - 1).is_err());
    assert_eq!(capture.to_json(json.len()).unwrap(), json);
    assert_eq!(capture.raw_values(), &[0, 95, 254, 255]);
}

#[test]
fn header_rejection_order_and_fec_version_policy_are_explicit() {
    let unknown = PnmHeader::from_bytes(b"BAD!").unwrap_err();
    assert_eq!(unknown.code, ErrorCode::UnsupportedType);
    assert_eq!(unknown.offset, 0);
    for bytes in [&b"PNN\x04\x02\x00"[..], &b"PNN\x08\x01\x02"[..]] {
        let failure = PnmHeader::from_bytes(bytes).unwrap_err();
        assert_eq!(failure.code, ErrorCode::UnsupportedVersion);
        assert_eq!(failure.offset, 4);
    }
    let truncated_fec = CmDsOfdmRxMer::parse(&FEC[..5], ParseLimits::default()).unwrap_err();
    assert_eq!(truncated_fec.code, ErrorCode::Truncated);
    assert_eq!(truncated_fec.field, "minor_version");
}
