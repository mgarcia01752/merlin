/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent source envelopes, lossless capture ownership and adapter boundaries.
use pnm_input::upstream::{ApiError, CaptureFailure, PreEqCapture, RxmerCapture, UcdCapture};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const MINIMAL_UCD: &str = include_str!("../../../examples/measurements/upstream/ucd/minimal.hex");
const SCHEDULED_UCD: &str =
    include_str!("../../../examples/measurements/upstream/ucd/scheduled-50khz.hex");
const RXMER: &str = include_str!("../../../examples/measurements/upstream/rxmer/ccap-50khz.hex");
const APPLIED: &str = include_str!("../../../examples/measurements/upstream/pre-eq/applied.hex");
const UPDATE: &str = include_str!("../../../examples/measurements/upstream/pre-eq/last-update.hex");

// Independent transport-policy anchors; deliberately not imported from the codec.
const TEXT_LIMIT: usize = 1_048_576;
const SOURCE_LIMIT: usize = 65_549;

// Frozen wire octets and SHA from the independently authored minimal UCD fixture.
// This is intentionally not constructed by either source-JSON operation.
const LITERAL_UCD: &[u8] = br#"{
  "format":"merlin.upstream.capture-source",
  "schema_version":1,
  "capture_kind":"ucd",
  "original_byte_count":24,
  "original_sha256":"ccc6c4df3a909210e86062fe8bb273a6be35ab530d1c6e87b30edca0f9d9a269",
  "original_bytes":[2,0,0,0,0,1,2,0,0,0,0,2,0,10,0,0,3,5,51,0,7,254,0,9]
}"#;

type Import = fn(&[u8]) -> Result<(), ApiError>;

fn importers() -> [(&'static str, Import); 3] {
    [
        ("ucd", |bytes| {
            UcdCapture::from_source_json(bytes).map(|_| ())
        }),
        ("rxmer", |bytes| {
            RxmerCapture::from_source_json(bytes).map(|_| ())
        }),
        ("pre_eq", |bytes| {
            PreEqCapture::from_source_json(bytes).map(|_| ())
        }),
    ]
}

fn hex(text: &str) -> Vec<u8> {
    let digits: String = text.split_whitespace().collect();
    digits
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn envelope(kind: &str, bytes: &[u8]) -> Value {
    json!({
        "format":"merlin.upstream.capture-source",
        "schema_version":1,
        "capture_kind":kind,
        "original_byte_count":bytes.len(),
        "original_sha256":format!("{:x}", Sha256::digest(bytes)),
        "original_bytes":bytes,
    })
}

fn reject(error: ApiError, category: CaptureFailure, field: &str) {
    match error {
        ApiError::Capture {
            category: actual,
            field: actual_field,
            offset,
        } => {
            assert_eq!(actual, category);
            assert_eq!(actual_field, field);
            assert_eq!(offset, 0);
        }
        other => panic!("Expected structured capture failure: {other:?}"),
    }
}

fn assert_export(document: &str, kind: &str, original: &[u8]) {
    assert!(document.len() <= TEXT_LIMIT);
    let value: Value = serde_json::from_str(document).unwrap();
    assert_eq!(value.as_object().unwrap().len(), 6);
    assert_eq!(value, envelope(kind, original));
}

#[test]
fn independent_literal_envelope_and_known_digest_admit_original_ucd() {
    let capture = UcdCapture::from_source_json(LITERAL_UCD).unwrap();
    assert_eq!(capture.original_bytes(), hex(MINIMAL_UCD));
    assert_eq!(
        capture.original_sha256(),
        "ccc6c4df3a909210e86062fe8bb273a6be35ab530d1c6e87b30edca0f9d9a269"
    );
    assert_eq!(capture.header().configuration_change_count, 254);
    assert!(capture.channel_parameters().is_empty());
    let exported: Value = serde_json::from_str(&capture.to_source_json().unwrap()).unwrap();
    assert_eq!(
        exported,
        serde_json::from_slice::<Value>(LITERAL_UCD).unwrap()
    );
}

#[test]
fn ucd_source_round_trip_preserves_opaque_records_and_old_inspection() {
    let mut original = hex(SCHEDULED_UCD);
    let capture = UcdCapture::from_mac_management(&original).unwrap();
    let inspection = capture.to_json().unwrap();
    let mut document = capture.to_source_json().unwrap().into_bytes();
    assert_export(std::str::from_utf8(&document).unwrap(), "ucd", &original);
    let restored = UcdCapture::from_source_json(&document).unwrap();
    assert_eq!(restored, capture);
    assert_eq!(restored.to_json().unwrap(), inspection);
    assert_eq!(
        restored.channel_parameters()[1].value,
        [1, 35, 69, 103, 137, 10, 188, 222, 240]
    );
    original.fill(0);
    document.fill(0);
    drop(capture);
    assert_eq!(restored.original_bytes(), hex(SCHEDULED_UCD));
    assert_eq!(restored.to_json().unwrap(), inspection);
}

#[test]
fn rxmer_source_round_trip_preserves_null_grid_header_and_old_inspection() {
    let original = hex(RXMER);
    let capture = RxmerCapture::from_pnm(&original).unwrap();
    let inspection = capture.to_json().unwrap();
    let document = capture.to_source_json().unwrap();
    assert_export(&document, "rxmer", &original);
    let restored = RxmerCapture::from_source_json(document.as_bytes()).unwrap();
    assert_eq!(restored, capture);
    assert_eq!(restored.to_json().unwrap(), inspection);
    assert_eq!(restored.header().capture_time_epoch_seconds, 0x0123_4567);
    assert_eq!(restored.header().first_active_subcarrier, 128);
    assert_eq!(restored.header().ccap_id_raw.len(), 256);
    assert_eq!(restored.readings_db()[7], None);
    assert_eq!(restored.raw_readings()[7], 255);
    drop(document);
    drop(original);
    drop(capture);
    assert_eq!(restored.to_json().unwrap(), inspection);
}

#[test]
fn both_pre_eq_source_kinds_preserve_signed_coefficients_and_old_inspection() {
    for fixture in [APPLIED, UPDATE] {
        let original = hex(fixture);
        let capture = PreEqCapture::from_pnm(&original).unwrap();
        let inspection = capture.to_json().unwrap();
        let document = capture.to_source_json().unwrap();
        assert_export(&document, "pre_eq", &original);
        let restored = PreEqCapture::from_source_json(document.as_bytes()).unwrap();
        assert_eq!(restored, capture);
        assert_eq!(restored.to_json().unwrap(), inspection);
        assert_eq!(restored.coefficients()[1].re, 1.125);
        assert_eq!(restored.coefficients()[1].im, -0.25);
        drop(document);
        drop(original);
        drop(capture);
        assert_eq!(restored.to_json().unwrap(), inspection);
    }
}

#[test]
fn wrong_format_version_kind_count_and_exact_lowercase_hash_reject() {
    let valid: Value = serde_json::from_slice(LITERAL_UCD).unwrap();
    for (field, value, diagnostic) in [
        ("format", json!("merlin.upstream.ucd-inspection"), "format"),
        ("schema_version", json!(0), "schema_version"),
        ("schema_version", json!(2), "schema_version"),
        ("capture_kind", json!("unknown"), "capture_kind"),
        ("original_byte_count", json!(23), "original_byte_count"),
        ("original_byte_count", json!(25), "original_byte_count"),
        ("original_sha256", json!("0".repeat(64)), "original_sha256"),
        (
            "original_sha256",
            json!(valid["original_sha256"].as_str().unwrap().to_uppercase()),
            "original_sha256",
        ),
        ("original_sha256", json!("c".repeat(63)), "original_sha256"),
        ("original_sha256", json!("g".repeat(64)), "original_sha256"),
    ] {
        let mut changed = valid.clone();
        changed[field] = value;
        reject(
            UcdCapture::from_source_json(&serde_json::to_vec(&changed).unwrap()).unwrap_err(),
            CaptureFailure::InvalidInput,
            &format!("capture_source_json.{diagnostic}"),
        );
    }
}

#[test]
fn every_typed_import_rejects_another_valid_capture_family_before_decoding() {
    for (source_kind, source) in [("ucd", MINIMAL_UCD), ("rxmer", RXMER), ("pre_eq", APPLIED)] {
        let document = serde_json::to_vec(&envelope(source_kind, &hex(source))).unwrap();
        for (target_kind, import) in importers() {
            if source_kind != target_kind {
                reject(
                    import(&document).unwrap_err(),
                    CaptureFailure::InvalidInput,
                    "capture_source_json.capture_kind",
                );
            }
        }
    }
}

#[test]
fn each_missing_duplicate_and_unknown_envelope_field_is_document_error() {
    let valid: Value = serde_json::from_slice(LITERAL_UCD).unwrap();
    for key in valid.as_object().unwrap().keys() {
        let mut missing = valid.clone();
        missing.as_object_mut().unwrap().remove(key);
        reject(
            UcdCapture::from_source_json(&serde_json::to_vec(&missing).unwrap()).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
        let duplicate = format!(
            "{{{}:{},{}",
            serde_json::to_string(key).unwrap(),
            valid[key],
            std::str::from_utf8(LITERAL_UCD).unwrap().trim_start()[1..].trim_start(),
        );
        reject(
            UcdCapture::from_source_json(duplicate.as_bytes()).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
    }
    let mut unknown = valid;
    unknown["inspection"] = json!({});
    reject(
        UcdCapture::from_source_json(&serde_json::to_vec(&unknown).unwrap()).unwrap_err(),
        CaptureFailure::InvalidInput,
        "capture_source_json.document",
    );
}

#[test]
fn invalid_octets_wrong_types_and_malformed_documents_never_reach_binary_decoder() {
    let valid: Value = serde_json::from_slice(LITERAL_UCD).unwrap();
    for byte in [json!(-1), json!(256), json!(0.5), json!("2"), Value::Null] {
        let mut changed = valid.clone();
        changed["original_bytes"][0] = byte;
        reject(
            UcdCapture::from_source_json(&serde_json::to_vec(&changed).unwrap()).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
    }
    for (field, wrong_type) in [
        ("format", json!(1)),
        ("schema_version", json!("1")),
        ("capture_kind", Value::Null),
        ("original_byte_count", json!(1.5)),
        ("original_sha256", json!([])),
        ("original_bytes", json!("0200")),
    ] {
        let mut changed = valid.clone();
        changed[field] = wrong_type;
        reject(
            UcdCapture::from_source_json(&serde_json::to_vec(&changed).unwrap()).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
    }
    for bytes in [b"".as_slice(), b"[]", b"null", b"{", b"{}{}", &[0xff]] {
        reject(
            UcdCapture::from_source_json(bytes).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
    }
}

#[test]
fn legacy_inspection_documents_are_not_silently_upgraded_to_sources() {
    let documents = [
        UcdCapture::from_mac_management(&hex(MINIMAL_UCD))
            .unwrap()
            .to_json()
            .unwrap(),
        RxmerCapture::from_pnm(&hex(RXMER))
            .unwrap()
            .to_json()
            .unwrap(),
        PreEqCapture::from_pnm(&hex(APPLIED))
            .unwrap()
            .to_json()
            .unwrap(),
    ];
    for ((_, import), document) in importers().into_iter().zip(documents) {
        reject(
            import(document.as_bytes()).unwrap_err(),
            CaptureFailure::InvalidInput,
            "capture_source_json.document",
        );
    }
}

#[test]
fn exact_text_ceiling_accepts_valid_padding_and_next_byte_rejects_before_parsing() {
    let mut padded = LITERAL_UCD.to_vec();
    padded.resize(TEXT_LIMIT, b' ');
    assert_eq!(
        UcdCapture::from_source_json(&padded)
            .unwrap()
            .original_bytes(),
        hex(MINIMAL_UCD)
    );
    padded.push(b' ');
    for (_, import) in importers() {
        reject(
            import(&padded).unwrap_err(),
            CaptureFailure::ResourceLimit,
            "capture_source_json.bytes",
        );
    }
}

#[test]
fn source_octet_ceiling_accepts_largest_ucd_and_rejects_next_before_binary_decode() {
    let mut maximum = hex(MINIMAL_UCD);
    while maximum.len() + 257 <= SOURCE_LIMIT {
        maximum.extend([200, 255]);
        maximum.extend([0; 255]);
    }
    let remaining = SOURCE_LIMIT - maximum.len();
    maximum.extend([200, u8::try_from(remaining - 2).unwrap()]);
    maximum.resize(SOURCE_LIMIT, 0);
    maximum[12..14].copy_from_slice(&u16::try_from(SOURCE_LIMIT - 14).unwrap().to_be_bytes());
    let native = UcdCapture::from_mac_management(&maximum).unwrap();
    let document = native.to_source_json().unwrap();
    assert_export(&document, "ucd", &maximum);
    assert_eq!(
        UcdCapture::from_source_json(document.as_bytes()).unwrap(),
        native
    );

    maximum.push(0);
    for (kind, import) in importers() {
        let document = serde_json::to_vec(&envelope(kind, &maximum)).unwrap();
        assert!(document.len() < TEXT_LIMIT);
        reject(
            import(&document).unwrap_err(),
            CaptureFailure::ResourceLimit,
            "capture_source_json.original_bytes",
        );
    }
}

#[test]
fn original_binary_failure_category_field_and_nonzero_offset_are_unchanged() {
    let mut ucd = hex(MINIMAL_UCD);
    ucd[17] = 4;
    let mut rxmer = hex(RXMER);
    rxmer[4] = 1;
    let mut pre_eq = hex(APPLIED);
    pre_eq[4] = 2;
    let truncated_ucd = hex(MINIMAL_UCD)[..23].to_vec();
    let mut excessive_rxmer_data = hex(RXMER);
    excessive_rxmer_data[293..297].copy_from_slice(&4097u32.to_be_bytes());
    let mut invalid_pre_eq_grid = hex(APPLIED);
    invalid_pre_eq_grid[27..29].copy_from_slice(&u16::MAX.to_be_bytes());
    let cases = [
        (
            "ucd",
            ucd.clone(),
            UcdCapture::from_mac_management(&ucd).unwrap_err(),
        ),
        (
            "rxmer",
            rxmer.clone(),
            RxmerCapture::from_pnm(&rxmer).unwrap_err(),
        ),
        (
            "pre_eq",
            pre_eq.clone(),
            PreEqCapture::from_pnm(&pre_eq).unwrap_err(),
        ),
        (
            "ucd",
            truncated_ucd.clone(),
            UcdCapture::from_mac_management(&truncated_ucd).unwrap_err(),
        ),
        (
            "rxmer",
            excessive_rxmer_data.clone(),
            RxmerCapture::from_pnm(&excessive_rxmer_data).unwrap_err(),
        ),
        (
            "pre_eq",
            invalid_pre_eq_grid.clone(),
            PreEqCapture::from_pnm(&invalid_pre_eq_grid).unwrap_err(),
        ),
    ];
    for (kind, bytes, original) in cases {
        let (_, import) = importers()
            .into_iter()
            .find(|(target, _)| *target == kind)
            .unwrap();
        let document = serde_json::to_vec(&envelope(kind, &bytes)).unwrap();
        let imported = import(&document).unwrap_err();
        match (original, imported) {
            (
                ApiError::Capture {
                    category,
                    field,
                    offset,
                },
                ApiError::Capture {
                    category: actual_category,
                    field: actual_field,
                    offset: actual_offset,
                },
            ) => {
                assert_ne!(offset, 0);
                assert_eq!(
                    (actual_category, actual_field, actual_offset),
                    (category, field, offset)
                );
            }
            errors => panic!("Lost original binary diagnostics: {errors:?}"),
        }
    }
}
