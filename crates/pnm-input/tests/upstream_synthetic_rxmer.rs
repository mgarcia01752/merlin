/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent CCAP-OSSI I18 Table 410 fields and section 7.3.2.5 octet anchors.
use docsis_ofdma::api::ErrorCode;
use pnm_input::{
    rxmer_export::RangePolicy,
    upstream::{SyntheticRxmerCapture, SyntheticRxmerRequest, SyntheticRxmerValues},
};
use serde_json::{json, Value};

const REQUEST: &str =
    include_str!("../../../examples/measurements/upstream/synthetic-rxmer/us-50khz.json");
const REQUEST_25: &str =
    include_str!("../../../examples/measurements/upstream/synthetic-rxmer/us-25khz.json");
// Independent Table 410 byte count and physical FFT extents; no production constants.
const HEADER_BYTES: usize = 297;
const FFT_25_CARRIERS: usize = 4096;
const FFT_50_CARRIERS: usize = 2048;

fn request() -> SyntheticRxmerRequest {
    serde_json::from_str(REQUEST).unwrap()
}

#[test]
fn literal_header_and_quarter_db_octets_match_ossi() {
    let mut request = request();
    request.values = SyntheticRxmerValues::Samples {
        readings_db: vec![
            Some(0.0),
            Some(23.75),
            Some(31.75),
            Some(32.0),
            Some(40.0),
            Some(63.25),
            Some(63.5),
            None,
        ],
    };
    let generated = SyntheticRxmerCapture::generate(&request).unwrap();
    let bytes = generated.capture().original_bytes();
    assert_eq!(
        &bytes[..14],
        &[0x50, 0x4e, 0x4e, 0x69, 2, 0, 0x65, 0x53, 0xf1, 0x00, 0, 0, 0, 101]
    );
    assert_eq!(&bytes[14..43], b"synthetic-cmts.merlin.invalid");
    assert!(bytes[43..270].iter().all(|&byte| byte == 0));
    assert_eq!(&bytes[270..277], &[0; 7]);
    assert_eq!(&bytes[277..283], &[2, 0, 0, 0, 0, 1]);
    assert_eq!(
        &bytes[283..],
        &[
            0, 1, 1, 0, 0x4c, 0x4b, 0x40, 0, 128, 50, 0, 0, 0, 8, 0, 95, 127, 128, 160, 253, 254,
            255
        ]
    );
    assert_eq!(bytes.len(), HEADER_BYTES + 8);
    assert_eq!(
        generated.capture().readings_db(),
        &[
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
}

#[test]
fn both_spacings_preserve_physical_missing_positions_and_originals() {
    for (input, count, missing_offset) in [(REQUEST, 512, 32), (REQUEST_25, 1024, 64)] {
        let generated = SyntheticRxmerCapture::from_json(input).unwrap();
        assert_eq!(generated.capture().raw_readings().len(), count);
        for (index, value) in generated.capture().raw_readings().iter().enumerate() {
            assert_eq!(
                *value,
                if (missing_offset..missing_offset + 2).contains(&index) {
                    255
                } else {
                    140
                }
            );
        }
        let artifact: Value = serde_json::from_str(&generated.to_json().unwrap()).unwrap();
        assert_eq!(artifact["origin"], "synthetic");
        assert_eq!(artifact["measurement_performed"], false);
        assert_eq!(
            artifact["request"],
            serde_json::from_str::<Value>(input).unwrap()
        );
        assert_eq!(
            artifact["binary_sha256"],
            generated.capture().original_sha256()
        );
        assert_eq!(artifact["conversion"]["missing"], 2);
    }
}

#[test]
fn shipped_synthetic_examples_match_requests_and_provenance() {
    // These generated examples are additional regression fixtures. Independent
    // normative octet expectations remain in literal_header_and_quarter_db_octets_match_ossi.
    let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/measurements/upstream/synthetic-rxmer");
    for name in ["us-25khz", "us-50khz"] {
        let generated = SyntheticRxmerCapture::from_json(
            &std::fs::read_to_string(directory.join(format!("{name}.json"))).unwrap(),
        )
        .unwrap();
        assert_eq!(
            generated.capture().original_bytes(),
            std::fs::read(directory.join(name).join("synthetic-cmts-rxmer.bin")).unwrap()
        );
        let provenance: Value = serde_json::from_str(
            &std::fs::read_to_string(
                directory
                    .join(name)
                    .join("synthetic-cmts-rxmer.bin.merlin.json"),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(
            provenance,
            serde_json::from_str::<Value>(&generated.to_json().unwrap()).unwrap()
        );
    }
}

#[test]
fn explicit_clamping_rounding_and_source_immutability() {
    let mut request = request();
    request.values = SyntheticRxmerValues::Samples {
        readings_db: vec![Some(-1.0), Some(23.625), Some(64.0), None],
    };
    assert_eq!(
        SyntheticRxmerCapture::generate(&request)
            .unwrap_err()
            .category(),
        ErrorCode::InvalidInput
    );
    request.range_policy = RangePolicy::Clamp;
    let artifact = SyntheticRxmerCapture::generate(&request).unwrap();
    let before = artifact.to_json().unwrap();
    request.ccap_id.clear();
    assert_eq!(artifact.to_json().unwrap(), before);
    assert_eq!(artifact.capture().raw_readings(), &[0, 95, 254, 255]);
    let value: Value = serde_json::from_str(&before).unwrap();
    assert_eq!(value["conversion"]["clipped"], 2);
    assert_eq!(value["conversion"]["rounded"], 1);
    assert_eq!(value["conversion"]["maximum_rounding_db"], 0.125);
    assert_eq!(value["conversion"]["maximum_change_db"], 1.0);
}

#[test]
fn invalid_metadata_unknown_fields_and_nonfinite_values_reject() {
    for (key, value) in [
        ("number_of_averages", json!(0)),
        ("number_of_averages", json!(256)),
        ("ccap_id", json!("")),
        ("ccap_id", json!("a\u{0000}b")),
        ("rpd_port", json!(1)),
        ("unexpected", json!(true)),
        ("capture_time_epoch_seconds", json!(-1)),
        ("subcarrier_spacing_khz", json!(100)),
        ("schema_version", json!(2)),
        ("format", json!("merlin.upstream.captured-rxmer")),
    ] {
        let mut input: Value = serde_json::from_str(REQUEST).unwrap();
        input[key] = value;
        assert!(
            SyntheticRxmerCapture::from_json(&input.to_string()).is_err(),
            "{key}"
        );
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut input = request();
        input.values = SyntheticRxmerValues::Samples {
            readings_db: vec![Some(value)],
        };
        assert_eq!(
            SyntheticRxmerCapture::generate(&input)
                .unwrap_err()
                .category(),
            ErrorCode::InvalidInput
        );
    }
}

#[test]
fn malformed_missing_extents_and_capacity_bounds_reject() {
    for (count, missing) in [
        (0, vec![]),
        (512, vec![127]),
        (512, vec![640]),
        (512, vec![160, 160]),
        (usize::MAX, vec![]),
    ] {
        let mut input = request();
        input.values = SyntheticRxmerValues::Flat {
            mer_db: 35.0,
            count,
            missing_subcarriers: missing,
        };
        assert!(SyntheticRxmerCapture::generate(&input).is_err());
    }
    for (spacing, fft) in [(25, FFT_25_CARRIERS), (50, FFT_50_CARRIERS)] {
        let mut input = request();
        input.first_active_subcarrier = 0;
        input.subcarrier_spacing_khz = spacing;
        input.values = SyntheticRxmerValues::Flat {
            mer_db: 35.0,
            count: fft,
            missing_subcarriers: vec![],
        };
        assert_eq!(
            SyntheticRxmerCapture::generate(&input)
                .unwrap()
                .capture()
                .raw_readings()
                .len(),
            fft
        );
        input.first_active_subcarrier = 1;
        assert!(SyntheticRxmerCapture::generate(&input).is_err());
    }
    let mut input = request();
    input.ccap_id = "s".repeat(255);
    assert!(SyntheticRxmerCapture::generate(&input).is_ok());
    input.ccap_id.push('s');
    assert_eq!(
        SyntheticRxmerCapture::generate(&input)
            .unwrap_err()
            .category(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(
        SyntheticRxmerCapture::from_json(&" ".repeat(256 * 1024 + 1))
            .unwrap_err()
            .category(),
        ErrorCode::ResourceLimit
    );
    let mut input: Value = serde_json::from_str(REQUEST).unwrap();
    input["values"] = json!({"kind":"samples","readings_db":vec![Some(35.0);FFT_25_CARRIERS+1]});
    assert!(SyntheticRxmerCapture::from_json(&input.to_string()).is_err());
}
