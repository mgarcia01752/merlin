/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Independent hand-authored wire fixtures and malformed/budget boundaries.
use pnm_input::upstream::{ApiError, UcdCapture};

const MINIMAL: &str = include_str!("../../../examples/measurements/upstream/ucd/minimal.hex");
const SCHEDULED: &str =
    include_str!("../../../examples/measurements/upstream/ucd/scheduled-50khz.hex");

fn hex(text: &str) -> Vec<u8> {
    let digits: String = text.chars().filter(|ch| !ch.is_whitespace()).collect();
    digits
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
fn sized(mut bytes: Vec<u8>) -> Vec<u8> {
    let count = u16::try_from(bytes.len() - 14).unwrap();
    bytes[12..14].copy_from_slice(&count.to_be_bytes());
    bytes
}
fn reject(bytes: &[u8], code: &str, field: &str, offset: usize) {
    let error = UcdCapture::from_mac_management(bytes).unwrap_err();
    assert_eq!(error.code(), code);
    match error {
        ApiError::Capture {
            field: actual,
            offset: position,
            ..
        } => {
            assert_eq!(actual, field);
            assert_eq!(position, offset);
        }
        other => panic!("missing structured capture error: {other:?}"),
    }
}

#[test]
fn literal_source_envelopes_offsets_hashes_and_owned_originals() {
    let mut bytes = hex(SCHEDULED);
    assert_eq!(bytes.len(), 92);
    let capture = UcdCapture::from_mac_management(&bytes).unwrap();
    assert_eq!(
        capture.original_sha256(),
        "7e47467fae5d8aaabbaa52f7d2dda028a3865fa5b9bf603548a38c00393c3e74"
    );
    let header = capture.header();
    assert_eq!(header.message_length, 78);
    assert_eq!(header.destination, [2, 0, 0, 0, 0, 1]);
    assert_eq!(header.source, [2, 0, 0, 0, 0, 2]);
    assert_eq!(
        (header.version, header.message_type, header.multipart),
        (5, 51, 0)
    );
    assert_eq!(
        (
            header.upstream_channel_id,
            header.configuration_change_count,
            header.minislot_size_ticks,
            header.downstream_channel_id
        ),
        (7, 254, 0, 9)
    );
    assert_eq!(
        capture
            .channel_parameters()
            .iter()
            .map(|tlv| (tlv.offset, tlv.kind))
            .collect::<Vec<_>>(),
        vec![
            (24, 24),
            (28, 25),
            (39, 26),
            (42, 27),
            (45, 28),
            (48, 29),
            (54, 30),
            (64, 31),
            (70, 32),
            (73, 33)
        ]
    );
    assert_eq!(
        capture.channel_parameters()[1].value,
        hex("01234567890abcdef0")
    );
    assert_eq!(
        capture.channel_parameters()[6].value,
        hex("0000007f021007ff")
    );
    assert_eq!(
        capture
            .bursts()
            .iter()
            .map(|burst| (burst.offset, burst.iuc))
            .collect::<Vec<_>>(),
        vec![(78, 5), (85, 13)]
    );
    assert_eq!(capture.bursts()[0].attributes[0].offset, 81);
    assert_eq!(capture.bursts()[0].attributes[0].kind, 21);
    assert_eq!(capture.bursts()[0].attributes[0].value, hex("4107"));
    assert_eq!(capture.bursts()[1].attributes[0].value, hex("2107"));
    let exported = capture.to_json().unwrap();
    let value: serde_json::Value = serde_json::from_str(&exported).unwrap();
    assert_eq!(value["format"], "merlin.upstream.ucd-inspection");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["semantics"], "structural-inspection-only");
    bytes.fill(0);
    drop(bytes);
    assert_eq!(capture.original_bytes(), hex(SCHEDULED));
    assert_eq!(capture.clone().to_json().unwrap(), exported);
    let minimal = UcdCapture::from_mac_management(&hex(MINIMAL)).unwrap();
    assert!(minimal.channel_parameters().is_empty());
    assert!(minimal.bursts().is_empty());
    assert_eq!(
        minimal.original_sha256(),
        "ccc6c4df3a909210e86062fe8bb273a6be35ab530d1c6e87b30edca0f9d9a269"
    );
}

#[test]
fn exact_envelope_llc_version_and_multipart_failures() {
    let source = hex(SCHEDULED);
    for length in 0..source.len() {
        let field = if length < 24 {
            "ucd.header"
        } else {
            "ucd.message_length"
        };
        reject(
            &source[..length],
            "InvalidInput",
            field,
            if length < 24 { length } else { 12 },
        );
    }
    for (offset, byte, code, field) in [
        (14, 1, "InvalidInput", "ucd.llc"),
        (15, 1, "InvalidInput", "ucd.llc"),
        (16, 0, "InvalidInput", "ucd.llc"),
        (17, 4, "UnsupportedCapability", "ucd.version"),
        (18, 50, "UnsupportedCapability", "ucd.message_type"),
        (19, 1, "InvalidInput", "ucd.fragment_sequence"),
        (19, 0x12, "InvalidInput", "ucd.fragment_sequence"),
        (19, 0x10, "UnsupportedCapability", "ucd.multipart"),
        (19, 0x11, "UnsupportedCapability", "ucd.multipart"),
        (19, 0xff, "UnsupportedCapability", "ucd.multipart"),
    ] {
        let mut bad = source.clone();
        bad[offset] = byte;
        reject(&bad, code, field, offset);
    }
    let mut appended = source.clone();
    appended.push(0);
    reject(&appended, "InvalidInput", "ucd.message_length", 12);
    let mut reversed = source;
    reversed.swap(12, 13);
    reject(&reversed, "InvalidInput", "ucd.message_length", 12);
}

#[test]
fn exact_outer_nested_lengths_iuc_and_channel_order_failures() {
    for (suffix, field, offset) in [
        ("c8", "ucd.tlv_header", 25),
        ("c80200", "ucd.tlv_length", 25),
        ("1700", "ucd.burst_iuc", 26),
        ("170115", "ucd.burst_iuc", 26),
        ("17020515", "ucd.tlv_header", 28),
        ("170405150200", "ucd.tlv_length", 28),
        ("0400", "ucd.burst_type", 24),
        ("0500", "ucd.burst_type", 24),
        ("170105c800", "ucd.channel_order", 27),
    ] {
        let mut bytes = hex(MINIMAL);
        bytes.extend(hex(suffix));
        reject(&sized(bytes), "InvalidInput", field, offset);
    }
}

#[test]
fn unknown_duplicate_and_zero_length_values_remain_in_source_order() {
    let mut bytes = hex(MINIMAL);
    bytes.extend(hex("c8001c01021c0101c801aa170505c8001500"));
    let capture = UcdCapture::from_mac_management(&sized(bytes)).unwrap();
    assert_eq!(
        capture
            .channel_parameters()
            .iter()
            .map(|tlv| tlv.kind)
            .collect::<Vec<_>>(),
        vec![200, 28, 28, 200]
    );
    assert!(capture.channel_parameters()[0].value.is_empty());
    assert_eq!(capture.channel_parameters()[1].value, [2]);
    assert_eq!(capture.channel_parameters()[2].value, [1]);
    assert_eq!(capture.channel_parameters()[3].value, [0xaa]);
    assert_eq!(
        capture.bursts()[0]
            .attributes
            .iter()
            .map(|tlv| tlv.kind)
            .collect::<Vec<_>>(),
        vec![200, 21]
    );
    // Structural acceptance is deliberate even when a complete channel is absent.
    assert_eq!(capture.header().upstream_channel_id, 7);
}

#[test]
fn byte_and_combined_outer_nested_record_limits_precede_unbounded_work() {
    let mut maximum = hex(MINIMAL);
    while maximum.len() + 257 <= 65549 {
        maximum.extend([200, 255]);
        maximum.extend([0; 255]);
    }
    let remainder = 65549 - maximum.len();
    assert!(remainder >= 2);
    maximum.extend([200, u8::try_from(remainder - 2).unwrap()]);
    maximum.resize(65549, 0);
    let maximum = sized(maximum);
    assert_eq!(
        UcdCapture::from_mac_management(&maximum)
            .unwrap()
            .original_bytes()
            .len(),
        65549
    );
    let mut too_large = maximum;
    too_large.push(0);
    reject(&too_large, "ResourceLimit", "ucd.bytes", 0);
    let mut at_limit = hex(MINIMAL);
    for _ in 0..1024 {
        at_limit.extend([200, 0]);
    }
    assert_eq!(
        UcdCapture::from_mac_management(&sized(at_limit.clone()))
            .unwrap()
            .channel_parameters()
            .len(),
        1024
    );
    at_limit.extend([200, 0]);
    reject(&sized(at_limit), "ResourceLimit", "ucd.tlv_count", 2072);
    let mut nested = hex(MINIMAL);
    for _ in 0..8 {
        nested.extend([23, 253, 5]);
        for _ in 0..126 {
            nested.extend([200, 0]);
        }
    }
    nested.extend([23, 17, 13]);
    for _ in 0..8 {
        nested.extend([200, 0]);
    }
    // Eight wrappers *127 records, then wrapper + seven admitted attributes.
    reject(&sized(nested), "ResourceLimit", "ucd.tlv_count", 2081);
}
