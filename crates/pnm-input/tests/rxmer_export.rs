/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent I27 Table68/p172 byte anchors and pinned-reader compatibility.
use docsis_ofdm_fec::{
    api::*,
    scenario::{FrequencyGrid, RxMerFrame, ScenarioKey, SourceProvenance},
};
use pnm_input::{rxmer_export::*, CaptureLoader};
use serde_json::json;
use std::sync::atomic::{AtomicBool, AtomicUsize};
fn frame(values: Vec<Option<f64>>) -> RxMerFrame {
    RxMerFrame {
        key: ScenarioKey {
            mac_address: [0, 0xca, 0, 0, 0, 1],
            channel_id: 193,
            grid: FrequencyGrid {
                zero_frequency_hz: 0x01000000,
                first_subcarrier_index: 2,
                spacing_hz: 25000,
                sample_count: values.len(),
            },
        },
        source: SourceProvenance {
            name: "synthetic fixture".into(),
            sha256: "0".repeat(64),
            byte_length: 0,
            capture_time: Some(0x01020304),
            file_kind: "synthetic".into(),
            format_major: 1,
            format_minor: 0,
        },
        capture_time: 0x01020304,
        mer_db: values,
    }
}
#[test]
fn independent_golden_header_payload_and_decoder_roundtrip() {
    let original = frame(vec![Some(0.0), Some(0.25), Some(63.5), None]);
    let encoded = encode(&original, RangePolicy::Reject).unwrap();
    // Literal bytes independently transcribed from Table68 field order and
    // original reader's BE implementation; sample anchors from I27 D.2.8 p172.
    let expected = [
        0x50, 0x4e, 0x4e, 4, 1, 0, 1, 2, 3, 4, 193, 0, 0xca, 0, 0, 0, 1, 1, 0, 0, 0, 0, 2, 25, 0,
        0, 0, 4, 0, 1, 254, 255,
    ];
    assert_eq!(encoded.bytes, expected);
    assert_eq!(encoded.summary.missing, 1);
    let decoded =
        CaptureLoader::rxmer(&NamedCapture::new("output", encoded.bytes).unwrap()).unwrap();
    assert_eq!(
        decoded
            .values()
            .iter()
            .map(|value| value.map(|value| value.value()))
            .collect::<Vec<_>>(),
        original.mer_db
    );
    for spacing in [25_000, 50_000] {
        let mut input = original.clone();
        input.key.grid.spacing_hz = spacing;
        let encoded = encode(&input, RangePolicy::Reject).unwrap();
        assert_eq!(encoded.bytes[23], (spacing / 1000) as u8);
        assert_eq!(
            CaptureLoader::rxmer(&NamedCapture::new("output", encoded.bytes).unwrap())
                .unwrap()
                .grid(),
            &input.key.grid
        );
    }
}
#[test]
fn strict_guard_clamp_quantization_and_nonfinite_are_independent_of_ui() {
    let source = frame(vec![
        Some(-0.01),
        Some(64.0),
        Some(0.125),
        Some(63.375),
        None,
    ]);
    let error = encode(&source, RangePolicy::Reject).unwrap_err();
    assert_eq!(error.code, "out_of_range");
    assert_eq!(error.subcarrier, Some(2));
    let output = encode(&source, RangePolicy::Clamp).unwrap();
    assert_eq!(&output.bytes[28..], &[0, 254, 1, 254, 255]);
    assert_eq!(output.summary.clipped, 2);
    assert_eq!(output.summary.rounded, 2);
    assert_eq!(output.summary.maximum_rounding_db, 0.125);
    assert_eq!(output.summary.maximum_change_db, 0.5);
    assert_eq!(source.mer_db[0], Some(-0.01));
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(encode(&frame(vec![Some(value)]), RangePolicy::Clamp).is_err());
    }
    let oversized = frame(vec![Some(1.0); 65_537]);
    assert_eq!(
        encode(&oversized, RangePolicy::Reject).unwrap_err().code,
        "invalid_frame"
    );
    assert!(encode_json(&oversized).is_err());
    let mut invalid = frame(vec![Some(1.0)]);
    invalid.key.grid.spacing_hz = 1;
    assert!(encode(&invalid, RangePolicy::Reject).is_err());
    invalid.key.grid.spacing_hz = 25000;
    invalid.key.grid.zero_frequency_hz = u32::MAX;
    assert!(encode(&invalid, RangePolicy::Reject).is_err());
    assert!(preflight(&[], RangePolicy::Reject).is_err());
    assert!(preflight(&vec![source.clone(); 257], RangePolicy::Clamp).is_err());
    let mut invalid = source.clone();
    invalid.mer_db.clear();
    assert!(encode(&invalid, RangePolicy::Clamp).is_err());
}
#[test]
fn create_new_files_manifest_batch_cancellation_and_failure() {
    let temporary = tempfile::tempdir().unwrap();
    let parent = temporary.path();
    let source = frame(vec![Some(40.13), None]);
    let frames = [source.clone()];
    let cancel = AtomicBool::new(false);
    let completed = AtomicUsize::new(0);
    let destination = parent.join("one.bin");
    save(
        ExportDestination::Single(&destination),
        &frames,
        RangePolicy::Reject,
        &json!({"recipe":"fixture"}),
        &cancel,
        &completed,
    )
    .unwrap();
    let original = std::fs::read(&destination).unwrap();
    let sidecar = parent.join("one.bin.merlin.json");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&sidecar).unwrap()).unwrap();
    assert_eq!(manifest["modem_measurement"], false);
    assert_eq!(manifest["captures"][0]["capture_time"], source.capture_time);
    assert!(save(
        ExportDestination::Single(&destination),
        &frames,
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed
    )
    .is_err());
    assert_eq!(std::fs::read(&destination).unwrap(), original);
    let batch = parent.join("batch");
    save(
        ExportDestination::Collection(&batch),
        &[source.clone(), source.clone()],
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed,
    )
    .unwrap();
    assert!(batch.join("merlin_rxmer_designed_0001.bin").is_file());
    assert!(batch.join("merlin-export.json").is_file());
    assert!(save(
        ExportDestination::Collection(&batch),
        &frames,
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed
    )
    .is_err());
    let empty = parent.join("existing-empty");
    std::fs::create_dir(&empty).unwrap();
    assert!(save(
        ExportDestination::Collection(&empty),
        &frames,
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed
    )
    .is_err());
    assert_eq!(std::fs::read_dir(&empty).unwrap().count(), 0);
    let taken = parent.join("taken.bin");
    std::fs::write(&taken, b"original").unwrap();
    assert!(save(
        ExportDestination::Single(&taken),
        &frames,
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed
    )
    .is_err());
    assert!(!parent.join("taken.bin.merlin.json").exists());
    assert_eq!(std::fs::read(&taken).unwrap(), b"original");
    let stopped = parent.join("cancelled");
    assert!(save(
        ExportDestination::Collection(&stopped),
        &frames,
        RangePolicy::Reject,
        &json!({}),
        &AtomicBool::new(true),
        &completed
    )
    .is_err());
    assert!(!stopped.exists());
    let invalid = parent.join("invalid");
    assert!(save(
        ExportDestination::Collection(&invalid),
        &[source.clone(), frame(vec![Some(-2.0)])],
        RangePolicy::Reject,
        &json!({}),
        &cancel,
        &completed
    )
    .is_err());
    assert!(!invalid.exists());
    assert_eq!(frames[0].mer_db, source.mer_db);
    assert!(std::fs::read_dir(parent).unwrap().all(|path| !path
        .unwrap()
        .file_name()
        .to_string_lossy()
        .starts_with(".merlin-pnm-")));
}
#[test]
fn json_preserves_full_precision_missing_values_and_metadata_without_binary_limits() {
    let source = frame(vec![Some(-2.123456789), None, Some(70.123456789)]);
    assert!(encode(&source, RangePolicy::Reject).is_err());
    let output = encode_json(&source).unwrap();
    assert_eq!(output.summary.rounded, 0);
    assert_eq!(output.summary.clipped, 0);
    assert_eq!(output.summary.missing, 1);
    let decoded =
        CaptureLoader::rxmer(&NamedCapture::new("designed.json", output.bytes).unwrap()).unwrap();
    assert_eq!(decoded.frame().mer_db, source.mer_db);
    assert_eq!(decoded.frame().key, source.key);
    assert_eq!(decoded.frame().capture_time, source.capture_time);
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(encode_json(&frame(vec![Some(value)])).is_err());
    }
    assert!(preflight_format(&[], RangePolicy::Reject, ExportFormat::Json).is_err());
    let directory = tempfile::tempdir().unwrap();
    let cancel = AtomicBool::new(false);
    let completed = AtomicUsize::new(0);
    for collection in [false, true] {
        let path = directory
            .path()
            .join(if collection { "collection" } else { "one.json" });
        let save_once = || {
            save_with_format(
                if collection {
                    ExportDestination::Collection(&path)
                } else {
                    ExportDestination::Single(&path)
                },
                std::slice::from_ref(&source),
                RangePolicy::Clamp,
                &json!({"recipe":"fixture"}),
                &cancel,
                &completed,
                ExportFormat::Json,
            )
        };
        let summary = save_once().unwrap();
        assert_eq!(summary.clipped, 0);
        assert!(save_once().is_err());
        let capture = if collection {
            path.join("merlin_rxmer_designed_0001.json")
        } else {
            path.clone()
        };
        let document: serde_json::Value =
            serde_json::from_slice(&std::fs::read(capture).unwrap()).unwrap();
        assert_eq!(document["merlin_export"]["modem_measurement"], false);
        assert_eq!(document["values"], json!(source.mer_db));
    }
}
