/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Private transport tests; nested documents deliberately receive no science admission.

use super::*;
use serde_json::{json, Value};

// Independent version-1 wire/base64 anchors, not values exported by this codec.
// AAH/ represents exactly [0, 1, 255], including non-UTF-8 project bytes.
const LITERAL: &[u8] = br#"{"application":"MERLIN","format":"merlin.qt.update-recovery","version":1,"downstream":{"document":"AAH/","path":"downstream-a.merlin.json","dirty":true},"upstream":{"document":null,"path":null,"dirty":false},"selected":"upstream","dark":true,"original_paths":[{"path":"shared-input.bin","downstream":true,"upstream":true}]}"#;
const LITERAL_DOCUMENT_BASE64_BYTES: usize = 4;
const LITERAL_DOCUMENT_BYTES: &[u8] = &[0, 1, 255];

// Independent small policy seams exercise inclusive limits without allocating
// the production 144 MiB documents or 16 MiB metadata population.
const SMALL_PROJECT_BYTES: usize = 16;
const SMALL_METADATA_BYTES: usize = 4096;
const SMALL_ORIGINAL_PATHS: usize = 2;
const PADDED_EXACT_BYTES: &[u8] = b"abcde";
const PADDED_OVER_BYTES: &[u8] = b"abcdef";
const PADDED_EXACT_BASE64: &str = "YWJjZGU=";
const PADDED_OVER_BASE64: &str = "YWJjZGVm";

fn limits() -> Limits {
    Limits {
        project_bytes: SMALL_PROJECT_BYTES,
        metadata_bytes: SMALL_METADATA_BYTES,
        original_paths: SMALL_ORIGINAL_PATHS,
    }
}

fn fixture() -> Bundle {
    Bundle {
        downstream: ProjectState {
            document: Some(LITERAL_DOCUMENT_BYTES.to_vec()),
            path: Some(PathBuf::from("downstream-a.merlin.json")),
            dirty: true,
        },
        upstream: ProjectState {
            document: None,
            path: None,
            dirty: false,
        },
        selected: ProjectDirection::Upstream,
        dark: true,
        original_paths: vec![OriginalPath {
            path: PathBuf::from("shared-input.bin"),
            downstream: true,
            upstream: true,
        }],
    }
}

fn literal_value() -> Value {
    serde_json::from_slice(LITERAL).unwrap()
}

fn decode_value(value: &Value, policy: Limits) -> Result<Bundle, String> {
    let bytes = serde_json::to_vec(value).unwrap();
    decode_with_limits(&bytes, policy)
}

#[test]
fn production_policies_keep_independent_binary_unit_anchors() {
    assert_eq!(MAX_PROJECT_BYTES, 150_994_944);
    assert_eq!(MAX_RECOVERY_METADATA_BYTES, 16_777_216);
    assert_eq!(MAX_RECOVERY_BYTES, 419_430_400);
    assert_eq!(
        Limits::production().transport_bytes().unwrap(),
        MAX_RECOVERY_BYTES
    );
    assert_eq!(
        Limits::production().original_paths,
        MAX_ORIGINAL_SOURCE_PATHS
    );
}

#[test]
fn independent_literal_decodes_exact_binary_and_directional_metadata() {
    assert_eq!(Bundle::decode(LITERAL).unwrap(), fixture());
    assert_eq!(decode_with_limits(LITERAL, limits()).unwrap(), fixture());
}

#[test]
fn encoder_matches_independent_literal_without_inner_document_reserialization() {
    let bundle = fixture();
    assert_eq!(bundle.encode().unwrap(), LITERAL);
    assert_eq!(encode_with_limits(&bundle, limits()).unwrap(), LITERAL);
    assert_eq!(bundle, fixture());
}

#[test]
fn both_documents_paths_dirty_states_tabs_and_themes_round_trip_independently() {
    for selected in [ProjectDirection::Downstream, ProjectDirection::Upstream] {
        for dark in [false, true] {
            let mut bundle = fixture();
            bundle.upstream = ProjectState {
                document: Some(b" \nopaque\0\xff\r\n".to_vec()),
                path: Some(PathBuf::from("different/upstream-b.merlin.json")),
                dirty: false,
            };
            bundle.selected = selected;
            bundle.dark = dark;
            let bytes = encode_with_limits(&bundle, limits()).unwrap();
            assert_eq!(decode_with_limits(&bytes, limits()).unwrap(), bundle);
        }
    }
}

#[test]
fn legacy_editor_and_current_configuration_text_are_opaque_byte_transport() {
    // These schema markers are transport anchors, not claims of public admission.
    for document in [
        &b" {\"schema_version\":1}\n"[..],
        &b"{\"schema_version\":2,\"partial\":true}"[..],
        &b"\r\n{\"schema_version\":3,\"original_document\":\" unchanged \"}"[..],
    ] {
        let mut bundle = fixture();
        bundle.downstream.document = Some(document.to_vec());
        let policy = Limits {
            project_bytes: document.len(),
            ..limits()
        };
        let bytes = encode_with_limits(&bundle, policy).unwrap();
        let decoded = decode_with_limits(&bytes, policy).unwrap();
        assert_eq!(decoded.downstream.document.as_deref(), Some(document));
    }
}

#[test]
fn present_empty_document_and_explicit_blank_upstream_remain_distinct() {
    let mut bundle = fixture();
    bundle.downstream.document = Some(Vec::new());
    bundle.upstream.dirty = true;
    let bytes = encode_with_limits(&bundle, limits()).unwrap();
    assert_eq!(decode_with_limits(&bytes, limits()).unwrap(), bundle);
    bundle.upstream.document = Some(Vec::new());
    let present = encode_with_limits(&bundle, limits()).unwrap();
    assert_ne!(present, bytes);
    assert_eq!(decode_with_limits(&present, limits()).unwrap(), bundle);
}

#[test]
fn downstream_document_is_required_in_both_codec_directions() {
    let mut bundle = fixture();
    bundle.downstream.document = None;
    assert!(encode_with_limits(&bundle, limits())
        .unwrap_err()
        .contains("requires a downstream"));
    let mut value = literal_value();
    value["downstream"]["document"] = Value::Null;
    assert!(decode_value(&value, limits())
        .unwrap_err()
        .contains("requires a downstream"));
}

#[test]
fn blank_upstream_cannot_smuggle_a_normal_save_destination() {
    let mut bundle = fixture();
    bundle.upstream.path = Some(PathBuf::from("stale.merlin.json"));
    assert!(encode_with_limits(&bundle, limits()).is_err());
    let mut value = literal_value();
    value["upstream"]["path"] = json!("stale.merlin.json");
    assert!(decode_value(&value, limits()).is_err());
}

#[test]
fn unsupported_application_format_version_and_cursor_reject() {
    for (field, replacement) in [
        ("application", json!("OTHER")),
        ("format", json!("merlin.workbench")),
        ("version", json!(0)),
        ("version", json!(2)),
        ("version", json!(1.5)),
        ("selected", json!("both")),
    ] {
        let mut value = literal_value();
        value[field] = replacement;
        assert!(decode_value(&value, limits()).is_err(), "{field}");
    }
}

#[test]
fn unknown_fields_reject_at_each_fixed_envelope_level() {
    for target in ["root", "downstream", "upstream", "original"] {
        let mut value = literal_value();
        let object = match target {
            "root" => &mut value,
            "original" => &mut value["original_paths"][0],
            direction => &mut value[direction],
        };
        object["unexpected"] = json!(true);
        assert!(decode_value(&value, limits()).is_err(), "{target}");
    }
}

#[test]
fn duplicate_fields_including_nullable_fields_reject() {
    let literal = std::str::from_utf8(LITERAL).unwrap();
    for duplicate in [
        literal.replacen("\"version\":1", "\"version\":1,\"version\":1", 1),
        literal.replacen(
            "\"document\":null",
            "\"document\":null,\"document\":null",
            1,
        ),
        literal.replacen("\"path\":null", "\"path\":null,\"path\":null", 1),
        literal.replacen(
            "\"downstream\":true",
            "\"downstream\":true,\"downstream\":false",
            1,
        ),
    ] {
        assert!(decode_with_limits(duplicate.as_bytes(), limits()).is_err());
    }
}

#[test]
fn every_required_field_including_explicit_nulls_must_be_present() {
    for name in [
        "application",
        "format",
        "version",
        "downstream",
        "upstream",
        "selected",
        "dark",
        "original_paths",
    ] {
        let mut value = literal_value();
        value.as_object_mut().unwrap().remove(name);
        assert!(decode_value(&value, limits()).is_err(), "root {name}");
    }
    for direction in ["downstream", "upstream"] {
        for name in ["document", "path", "dirty"] {
            let mut value = literal_value();
            value[direction].as_object_mut().unwrap().remove(name);
            assert!(
                decode_value(&value, limits()).is_err(),
                "{direction}.{name}"
            );
        }
    }
}

#[test]
fn malformed_truncated_non_utf8_and_trailing_documents_reject() {
    for end in 0..LITERAL.len() {
        assert!(
            decode_with_limits(&LITERAL[..end], limits()).is_err(),
            "prefix {end}"
        );
    }
    for input in [&b"null"[..], &b"[]"[..], &b"\xff"[..], &b"{} {}"[..]] {
        assert!(decode_with_limits(input, limits()).is_err());
    }
    let mut trailing = LITERAL.to_vec();
    trailing.extend_from_slice(b"{}");
    assert!(decode_with_limits(&trailing, limits()).is_err());
}

#[test]
fn field_types_and_canonical_base64_are_not_silently_coerced() {
    for field in ["dark", "version", "original_paths"] {
        let mut value = literal_value();
        value[field] = json!("not-the-required-type");
        assert!(decode_value(&value, limits()).is_err());
    }
    for document in [
        json!([0, 1, 255]),
        json!(true),
        json!("AAH"),
        json!("AAH?"),
        json!("AB=="),
        json!("A==="),
        json!("===="),
        json!("AA H/"),
    ] {
        let mut value = literal_value();
        value["downstream"]["document"] = document;
        assert!(decode_value(&value, limits()).is_err());
    }
}

#[test]
fn each_project_encode_bound_accepts_exact_and_rejects_one_extra_byte() {
    let policy = Limits {
        project_bytes: PADDED_EXACT_BYTES.len(),
        ..limits()
    };
    for direction in [ProjectDirection::Downstream, ProjectDirection::Upstream] {
        let mut bundle = fixture();
        let project = match direction {
            ProjectDirection::Downstream => &mut bundle.downstream,
            ProjectDirection::Upstream => &mut bundle.upstream,
        };
        project.document = Some(PADDED_EXACT_BYTES.to_vec());
        assert!(encode_with_limits(&bundle, policy).is_ok());
        let project = match direction {
            ProjectDirection::Downstream => &mut bundle.downstream,
            ProjectDirection::Upstream => &mut bundle.upstream,
        };
        project.document = Some(PADDED_OVER_BYTES.to_vec());
        assert_eq!(
            encode_with_limits(&bundle, policy).unwrap_err(),
            PROJECT_LIMIT_MESSAGE
        );
    }
}

#[test]
fn decoded_byte_bounds_handle_padding_even_when_encoded_lengths_match() {
    assert_eq!(PADDED_EXACT_BASE64.len(), PADDED_OVER_BASE64.len());
    let policy = Limits {
        project_bytes: PADDED_EXACT_BYTES.len(),
        ..limits()
    };
    for direction in ["downstream", "upstream"] {
        let mut value = literal_value();
        value[direction]["document"] = json!(PADDED_EXACT_BASE64);
        let exact = decode_value(&value, policy).unwrap();
        let document = if direction == "downstream" {
            exact.downstream.document
        } else {
            exact.upstream.document
        };
        assert_eq!(document.as_deref(), Some(PADDED_EXACT_BYTES));
        value[direction]["document"] = json!(PADDED_OVER_BASE64);
        assert_eq!(
            decode_value(&value, policy).unwrap_err(),
            PROJECT_LIMIT_MESSAGE
        );
    }
    assert_eq!(
        decoded_length("AAAAAAAAA", PADDED_EXACT_BYTES.len()).unwrap_err(),
        PROJECT_LIMIT_MESSAGE
    );
}

#[test]
fn metadata_counter_has_an_independent_exact_and_one_byte_beyond_anchor() {
    let exact_metadata = LITERAL.len() - LITERAL_DOCUMENT_BASE64_BYTES;
    let exact = Limits {
        metadata_bytes: exact_metadata,
        ..limits()
    };
    assert_eq!(encode_with_limits(&fixture(), exact).unwrap(), LITERAL);
    assert_eq!(decode_with_limits(LITERAL, exact).unwrap(), fixture());
    let over = Limits {
        metadata_bytes: exact_metadata - 1,
        ..limits()
    };
    assert!(encode_with_limits(&fixture(), over)
        .unwrap_err()
        .contains(METADATA_LIMIT_MESSAGE));
    assert!(decode_with_limits(LITERAL, over)
        .unwrap_err()
        .contains(METADATA_LIMIT_MESSAGE));
}

#[test]
fn escaped_unicode_paths_remain_exact_and_counted_as_json_metadata() {
    let mut bundle = fixture();
    bundle.downstream.path = Some(PathBuf::from("line\n\"back\\🛰.merlin.json"));
    bundle.original_paths[0].path = PathBuf::from("source\t\"\\🛰.bin");
    let encoded = encode_with_limits(&bundle, limits()).unwrap();
    let text = std::str::from_utf8(&encoded).unwrap();
    assert!(text.contains("line\\n\\\"back\\\\🛰.merlin.json"));
    assert_eq!(decode_with_limits(&encoded, limits()).unwrap(), bundle);
    let metadata = encoded.len() - LITERAL_DOCUMENT_BASE64_BYTES;
    assert!(encode_with_limits(
        &bundle,
        Limits {
            metadata_bytes: metadata - 1,
            ..limits()
        }
    )
    .is_err());
}

#[test]
fn complete_input_limit_is_inclusive_and_precedes_json_decoding() {
    let policy = limits();
    let mut exact = LITERAL.to_vec();
    exact.resize(policy.transport_bytes().unwrap(), b' ');
    assert_eq!(decode_with_limits(&exact, policy).unwrap(), fixture());
    exact.push(b' ');
    assert_eq!(
        decode_with_limits(&exact, policy).unwrap_err(),
        TRANSPORT_LIMIT_MESSAGE
    );
}

#[test]
fn original_path_count_accepts_exact_and_rejects_one_more() {
    let mut bundle = fixture();
    bundle.original_paths.push(OriginalPath {
        path: PathBuf::from("second.bin"),
        downstream: false,
        upstream: true,
    });
    let exact = encode_with_limits(&bundle, limits()).unwrap();
    assert_eq!(decode_with_limits(&exact, limits()).unwrap(), bundle);
    bundle.original_paths.push(OriginalPath {
        path: PathBuf::from("third.bin"),
        downstream: true,
        upstream: false,
    });
    assert!(encode_with_limits(&bundle, limits()).is_err());
    let mut value = literal_value();
    value["original_paths"] = serde_json::to_value(&bundle.original_paths).unwrap();
    assert!(decode_value(&value, limits()).is_err());
}

#[test]
fn duplicate_paths_and_records_without_an_owner_reject_on_both_operations() {
    for duplicate in [false, true] {
        let mut bundle = fixture();
        if duplicate {
            bundle.original_paths.push(OriginalPath {
                path: bundle.original_paths[0].path.clone(),
                downstream: true,
                upstream: false,
            });
        } else {
            bundle.original_paths[0].downstream = false;
            bundle.original_paths[0].upstream = false;
        }
        assert!(encode_with_limits(&bundle, limits()).is_err());
        let mut value = literal_value();
        value["original_paths"] = serde_json::to_value(&bundle.original_paths).unwrap();
        assert!(decode_value(&value, limits()).is_err());
    }
}

#[test]
fn checked_length_overflow_and_bounded_writer_reject_before_mutation() {
    assert!(padded_length(usize::MAX).is_err());
    assert!(Limits {
        project_bytes: usize::MAX,
        ..limits()
    }
    .transport_bytes()
    .is_err());
    assert!(Limits {
        metadata_bytes: usize::MAX,
        ..limits()
    }
    .transport_bytes()
    .is_err());
    let mut output = Output::buffer(LITERAL_DOCUMENT_BYTES.len());
    assert_eq!(
        output.write(LITERAL_DOCUMENT_BYTES).unwrap(),
        LITERAL_DOCUMENT_BYTES.len()
    );
    assert!(output.write(b"x").is_err());
    assert_eq!(output.bytes.as_deref(), Some(LITERAL_DOCUMENT_BYTES));
    output.length = usize::MAX;
    output.maximum = usize::MAX;
    assert!(output.write(b"x").is_err());
    assert_eq!(output.bytes.as_deref(), Some(LITERAL_DOCUMENT_BYTES));
}

#[cfg(unix)]
#[test]
fn non_utf8_paths_fail_without_altering_exact_binary_documents() {
    use std::os::unix::ffi::OsStringExt;
    let mut bundle = fixture();
    bundle.downstream.path = Some(PathBuf::from(std::ffi::OsString::from_vec(vec![b'/', 255])));
    assert!(encode_with_limits(&bundle, limits()).is_err());
    assert_eq!(
        bundle.downstream.document.as_deref(),
        Some(LITERAL_DOCUMENT_BYTES)
    );
}
