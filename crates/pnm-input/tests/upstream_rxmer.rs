/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Independently frozen CCAP major2 metadata, quarter-dB/nulls and hostile extents.
use pnm_input::upstream::{ApiError, RxmerCapture};

const KHZ50: &str = include_str!("../../../examples/measurements/upstream/rxmer/ccap-50khz.hex");
const KHZ25: &str = include_str!("../../../examples/measurements/upstream/rxmer/ccap-25khz.hex");
fn hex(text: &str) -> Vec<u8> {
    let digits: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    digits
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}
fn reject(bytes: &[u8], code: &str, field: &str, offset: usize) {
    let error = RxmerCapture::from_pnm(bytes).unwrap_err();
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
        error => panic!("lost capture diagnostic: {error:?}"),
    }
}
fn with_data(mut bytes: Vec<u8>, data: &[u8]) -> Vec<u8> {
    bytes.truncate(297);
    bytes[293..297].copy_from_slice(&u32::try_from(data.len()).unwrap().to_be_bytes());
    bytes.extend_from_slice(data);
    bytes
}

#[test]
fn frozen_50khz_metadata_offsets_values_nulls_and_hash() {
    let bytes = hex(KHZ50);
    assert_eq!(bytes.len(), 305);
    let capture = RxmerCapture::from_pnm(&bytes).unwrap();
    assert_eq!(
        capture.original_sha256(),
        "ffeadba3dcf58825fad71aa949a61dff0c24adad08f8b656d4a9c8cc552de03c"
    );
    let header = capture.header();
    assert_eq!((header.major_version, header.minor_version), (2, 0));
    assert_eq!(header.capture_time_epoch_seconds, 0x01234567);
    assert_eq!(header.if_index, 0x10203040);
    assert_eq!(header.ccap_id_raw.len(), 256);
    assert_eq!(&header.ccap_id_raw[..13], b"ccap.example\0");
    assert_eq!(header.rpd_id, [2, 0, 0, 0, 0, 10]);
    assert_eq!(header.rpd_port, 3);
    assert_eq!(header.cm_mac, [2, 0, 0, 0, 0, 1]);
    assert_eq!(header.number_of_averages, 291);
    assert_eq!(header.pre_eq_on_off, 1);
    assert_eq!(header.subcarrier_zero_hz, 12_800_000);
    assert_eq!(header.first_active_subcarrier, 128);
    assert_eq!(header.subcarrier_spacing_khz, 50);
    assert_eq!(header.data_length_bytes, 8);
    assert_eq!(capture.ccap_id_text(), Some("ccap.example"));
    assert_eq!(capture.pre_eq_enabled(), Some(true));
    assert_eq!(
        capture.raw_readings(),
        [0, 95, 127, 128, 160, 253, 254, 255]
    );
    assert_eq!(
        capture.readings_db(),
        [
            Some(0.0),
            Some(23.75),
            Some(31.75),
            Some(32.0),
            Some(40.0),
            Some(63.25),
            Some(63.5),
            None
        ]
    );
    assert_eq!(capture.original_bytes(), bytes);
    let value: serde_json::Value = serde_json::from_str(&capture.to_json().unwrap()).unwrap();
    assert_eq!(value["format"], "merlin.upstream.rxmer-capture");
    assert_eq!(value["schema_version"], 1);
    assert_eq!(value["semantics"], "decoded-observation-only");
    assert_eq!(value["readings_db"][7], serde_json::Value::Null);
    assert_eq!(value["raw_readings"][7], 255);
}

#[test]
fn frozen_25khz_vendor_minor_integrated_ccap_and_exact_subgrid() {
    let capture = RxmerCapture::from_pnm(&hex(KHZ25)).unwrap();
    assert_eq!(
        capture.original_sha256(),
        "1ee74407762af69674b866bfb5fde97eef2f45af3fa990252b64813cdd1deb84"
    );
    let header = capture.header();
    assert_eq!(header.minor_version, 165);
    assert_eq!(header.capture_time_epoch_seconds, 0xfedcba98);
    assert_eq!(header.if_index, 7);
    assert_eq!(header.rpd_id, [0; 6]);
    assert_eq!(header.rpd_port, 0);
    assert_eq!(header.cm_mac, [2, 0, 0, 0, 0, 2]);
    assert_eq!(header.number_of_averages, 1);
    assert_eq!(capture.ccap_id_text(), Some("2001:db8::1"));
    assert_eq!(capture.pre_eq_enabled(), Some(false));
    assert_eq!(header.subcarrier_zero_hz, 100_000_000);
    assert_eq!(header.first_active_subcarrier, 3800);
    assert_eq!(header.subcarrier_spacing_khz, 25);
    assert_eq!(capture.raw_readings(), [255, 0, 40, 254]);
    assert_eq!(
        capture.readings_db(),
        [None, Some(0.0), Some(10.0), Some(63.5)]
    );
}

#[test]
fn every_raw_reading_encoding_and_interior_missing_positions() {
    let data: Vec<u8> = (0..=255).collect();
    let bytes = with_data(hex(KHZ50), &data);
    let capture = RxmerCapture::from_pnm(&bytes).unwrap();
    assert_eq!(capture.raw_readings(), data);
    for (index, decoded) in capture.readings_db().iter().enumerate() {
        let expected = if index == 255 {
            None
        } else {
            Some((index * 25) as f64 / 100.0)
        };
        assert_eq!(*decoded, expected);
    }
    let bytes = with_data(hex(KHZ50), &[95, 255, 160]);
    let capture = RxmerCapture::from_pnm(&bytes).unwrap();
    assert_eq!(capture.header().first_active_subcarrier, 128);
    assert_eq!(capture.readings_db(), [Some(23.75), None, Some(40.0)]);
}

#[test]
fn truncated_trailing_wrong_type_version_and_declared_lengths() {
    let original = hex(KHZ50);
    for length in 0..original.len() {
        reject(
            &original[..length],
            "InvalidInput",
            if length < 297 {
                "rxmer.header"
            } else {
                "rxmer.data_length"
            },
            if length < 297 { length } else { 293 },
        );
    }
    let mut bytes = original.clone();
    bytes.push(0);
    reject(&bytes, "InvalidInput", "rxmer.data_length", 293);
    for (offset, byte, field, expected_offset) in [
        (0, 0, "rxmer.file_type", 0),
        (2, 0x4d, "rxmer.file_type", 0),
        (3, 10, "rxmer.file_type", 0),
        (4, 1, "rxmer.major_version", 4),
        (4, 3, "rxmer.major_version", 4),
        (292, 100, "rxmer.subcarrier_spacing", 292),
    ] {
        let mut bytes = original.clone();
        bytes[offset] = byte;
        reject(&bytes, "UnsupportedCapability", field, expected_offset);
    }
    for count in [4097, u32::MAX, 0x08000000] {
        let mut bytes = original.clone();
        bytes[293..297].copy_from_slice(&count.to_be_bytes());
        reject(&bytes, "ResourceLimit", "rxmer.data_length", 293);
    }
    let mut bytes = original.clone();
    bytes[293..297].copy_from_slice(&7u32.to_be_bytes());
    reject(&bytes, "InvalidInput", "rxmer.data_length", 293);
    let mut bytes = original;
    bytes[14..270].fill(1);
    reject(&bytes, "InvalidInput", "rxmer.ccap_id_terminator", 14);
}

#[test]
fn exact_file_grid_and_empty_observation_boundaries() {
    let mut bytes = hex(KHZ25);
    bytes[290..292].copy_from_slice(&0u16.to_be_bytes());
    let maximum = with_data(bytes, &vec![255; 4096]);
    assert_eq!(maximum.len(), 4393);
    let capture = RxmerCapture::from_pnm(&maximum).unwrap();
    assert_eq!(capture.readings_db().len(), 4096);
    assert!(capture.readings_db().iter().all(Option::is_none));
    assert!(capture.to_json().unwrap().len() < 1024 * 1024);
    let mut oversized = maximum;
    oversized.push(0);
    reject(&oversized, "ResourceLimit", "rxmer.bytes", 0);
    for (fixture, last, limit) in [(KHZ25, 4095u16, 4096u16), (KHZ50, 2047, 2048)] {
        let mut bytes = with_data(hex(fixture), &[255]);
        bytes[290..292].copy_from_slice(&last.to_be_bytes());
        assert_eq!(
            RxmerCapture::from_pnm(&bytes)
                .unwrap()
                .header()
                .first_active_subcarrier,
            last
        );
        bytes[290..292].copy_from_slice(&limit.to_be_bytes());
        reject(&bytes, "InvalidInput", "rxmer.subcarrier_grid", 290);
        bytes[290..292].copy_from_slice(&last.to_be_bytes());
        reject(
            &with_data(bytes, &[255, 255]),
            "InvalidInput",
            "rxmer.subcarrier_grid",
            290,
        );
    }
    let empty = with_data(hex(KHZ50), &[]);
    let capture = RxmerCapture::from_pnm(&empty).unwrap();
    assert!(capture.readings_db().is_empty());
    assert!(capture.raw_readings().is_empty());
}

#[test]
fn raw_minor_flag_identifier_padding_and_original_lifetimes() {
    for raw in 0..=255 {
        let mut bytes = hex(KHZ50);
        bytes[5] = raw;
        bytes[285] = raw;
        let capture = RxmerCapture::from_pnm(&bytes).unwrap();
        assert_eq!(capture.header().minor_version, raw);
        assert_eq!(capture.header().pre_eq_on_off, raw);
        assert_eq!(
            capture.pre_eq_enabled(),
            match raw {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            }
        );
        assert_eq!(capture.original_bytes(), bytes);
    }
    let mut bytes = hex(KHZ50);
    bytes[14] = 255;
    bytes[269] = 0xa5;
    bytes[6..14].fill(0);
    let capture = RxmerCapture::from_pnm(&bytes).unwrap();
    assert_eq!(capture.ccap_id_text(), None);
    assert_eq!(capture.header().ccap_id_raw[255], 0xa5);
    assert_eq!(capture.header().capture_time_epoch_seconds, 0);
    assert_eq!(capture.header().if_index, 0);
    let exported = capture.to_json().unwrap();
    let clone = capture.clone();
    bytes.fill(0);
    drop(bytes);
    drop(capture);
    assert_eq!(clone.to_json().unwrap(), exported);
    assert_eq!(clone.original_bytes()[14], 255);
    let mut bad = clone.original_bytes().to_vec();
    bad[4] = 0;
    assert!(RxmerCapture::from_pnm(&bad).is_err());
    assert_eq!(clone.to_json().unwrap(), exported);
}
