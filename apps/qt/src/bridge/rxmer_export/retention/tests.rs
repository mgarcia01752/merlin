/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual decoded frame and independent product-storage fixtures; no export acceptance.
use super::*;
use merlin::downstream::{
    inputs::{rxmer_export, CaptureLoader},
    NamedCapture, RxMerFrame,
};
use serde_json::{json, Value};

// Independent spare-capacity anchors, not scientific/export/application limits.
const FRAME_SLOT_CAPACITY: usize = 3;
const JSON_SLOT_CAPACITY: usize = 7;
const EXTRA_SLOTS: usize = 11;
const EXTRA_TEXT_BYTES: usize = 37;

// Explicit original-grid missing/present anchors, matching the existing decoded
// RxMER fixture convention. No visitor reconstructs or interpolates these values.
const MISSING_READING_INDEX: usize = 0;
const PRESENT_READING_INDEX: usize = 1;
const PRESENT_READING_DB: f64 = 42.75;

fn text(value: &str) -> String {
    let mut owned = String::with_capacity(value.len() + EXTRA_TEXT_BYTES);
    owned.push_str(value);
    owned
}

fn fixture() -> Prepared {
    let mut original: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../examples/measurements/single-channel/rxmer.json"
    ))
    .unwrap();
    original["values"][MISSING_READING_INDEX] = Value::Null;
    original["values"][PRESENT_READING_INDEX] = json!(PRESENT_READING_DB);
    let input = NamedCapture::new("rxmer.json", serde_json::to_vec(&original).unwrap()).unwrap();
    let capture = CaptureLoader::rxmer(&input).unwrap();
    let mut frames = Vec::with_capacity(FRAME_SLOT_CAPACITY);
    frames.push(capture.frame().clone());
    let mut context_values = Vec::with_capacity(JSON_SLOT_CAPACITY);
    context_values.push(Value::Null);
    context_values.push(Value::String(text("unchanged context")));
    let context = Value::Object(
        [(text("captures"), Value::Array(context_values))]
            .into_iter()
            .collect(),
    );
    let summary =
        serde_json::to_value(rxmer_export::preflight(&frames, RangePolicy::Clamp).unwrap())
            .unwrap();
    let json_summary = serde_json::to_value(
        rxmer_export::preflight_format(
            &frames,
            RangePolicy::Clamp,
            rxmer_export::ExportFormat::Json,
        )
        .unwrap(),
    )
    .unwrap();
    Prepared {
        frames,
        policy: RangePolicy::Clamp,
        context,
        summary,
        json_summary,
        // Independent display metadata exercises optional diagnostic storage;
        // this fixture makes no Ready/PNM availability or save-lifecycle claim.
        pnm_error: Some(text("Original display diagnostic.")),
        diagnostic: Value::Object(
            [(text("message"), Value::String(text("Existing diagnostic")))]
                .into_iter()
                .collect(),
        ),
        collection: false,
    }
}

fn numeric_summary_bytes(value: &Value) -> usize {
    let fields = value.as_object().unwrap();
    assert!(fields.values().all(Value::is_number));
    fields.len() * size_of::<(String, Value)>() + fields.keys().map(String::capacity).sum::<usize>()
}

fn grow_text(value: &mut String) -> usize {
    let old = value.capacity();
    value.reserve_exact(old + EXTRA_TEXT_BYTES);
    value.capacity() - old
}

fn grow_first_key(value: &mut Value) -> usize {
    let fields = value.as_object_mut().unwrap();
    let mut original = std::mem::take(fields).into_iter();
    let (mut key, value) = original.next().unwrap();
    fields.extend(original);
    let delta = grow_text(&mut key);
    fields.insert(key, value);
    delta
}

#[test]
fn full_product_counts_containing_fixed_layout_once_and_preserves_real_frame_nulls_and_values() {
    let prepared = fixture();
    let frame = &prepared.frames[0];
    let context = prepared.context.as_object().unwrap();
    let (context_key, values) = context.iter().next().unwrap();
    let values = values.as_array().unwrap();
    let Value::String(context_text) = &values[1] else {
        panic!("Owned context text fixture");
    };
    let diagnostic = prepared.diagnostic.as_object().unwrap();
    let (diagnostic_key, diagnostic_value) = diagnostic.iter().next().unwrap();
    let Value::String(diagnostic_text) = diagnostic_value else {
        panic!("Owned diagnostic text fixture");
    };
    let expected = RetainedResources {
        payload_bytes: size_of::<Prepared>()
            + prepared.frames.capacity() * size_of::<RxMerFrame>()
            + frame.mer_db.capacity() * size_of::<Option<f64>>()
            + frame.source.name.capacity()
            + frame.source.sha256.capacity()
            + frame.source.file_kind.capacity()
            + size_of::<(String, Value)>()
            + context_key.capacity()
            + values.capacity() * size_of::<Value>()
            + context_text.capacity()
            + numeric_summary_bytes(&prepared.summary)
            + numeric_summary_bytes(&prepared.json_summary)
            + size_of::<(String, Value)>()
            + diagnostic_key.capacity()
            + diagnostic_text.capacity()
            + prepared.pnm_error.as_ref().unwrap().capacity(),
        ordered_entries: context.len()
            + prepared.summary.as_object().unwrap().len()
            + prepared.json_summary.as_object().unwrap().len()
            + diagnostic.len(),
        shared_roots: 0,
        ..Default::default()
    };
    let frames = serde_json::to_value(&prepared.frames).unwrap();
    let key = frame.key.clone();
    let frame_pointer = prepared.frames.as_ptr();
    let readings_pointer = frame.mer_db.as_ptr();
    assert_eq!(resources(&prepared).unwrap(), expected);
    assert_eq!(resources(&prepared).unwrap(), expected);
    assert_eq!(serde_json::to_value(&prepared.frames).unwrap(), frames);
    assert_eq!(prepared.frames[0].key, key);
    assert_eq!(prepared.frames[0].mer_db[MISSING_READING_INDEX], None);
    assert_eq!(
        prepared.frames[0].mer_db[PRESENT_READING_INDEX],
        Some(PRESENT_READING_DB)
    );
    assert_eq!(prepared.frames.as_ptr(), frame_pointer);
    assert_eq!(prepared.frames[0].mer_db.as_ptr(), readings_pointer);
}

#[test]
fn actual_spare_capacity_deltas_are_independent_and_metadata_queries_never_mutate_products() {
    let mut prepared = fixture();
    let frames_before = serde_json::to_value(&prepared.frames).unwrap();
    let json_before = [
        prepared.context.clone(),
        prepared.summary.clone(),
        prepared.json_summary.clone(),
        prepared.diagnostic.clone(),
    ];
    let error_before = prepared.pnm_error.clone();
    let mut expected = resources(&prepared).unwrap();
    let old = prepared.frames.capacity();
    let mut frames = Vec::with_capacity(old + EXTRA_SLOTS);
    frames.append(&mut prepared.frames);
    expected.payload_bytes += (frames.capacity() - old) * size_of::<RxMerFrame>();
    prepared.frames = frames;
    assert_eq!(resources(&prepared).unwrap(), expected);

    let frame = &mut prepared.frames[0];
    let old = frame.mer_db.capacity();
    let mut readings = Vec::with_capacity(old + EXTRA_SLOTS);
    readings.append(&mut frame.mer_db);
    expected.payload_bytes += (readings.capacity() - old) * size_of::<Option<f64>>();
    frame.mer_db = readings;
    assert_eq!(resources(&prepared).unwrap(), expected);
    for field in ["name", "sha256", "file_kind"] {
        let source = &mut prepared.frames[0].source;
        let text = match field {
            "name" => &mut source.name,
            "sha256" => &mut source.sha256,
            _ => &mut source.file_kind,
        };
        expected.payload_bytes += grow_text(text);
        assert_eq!(resources(&prepared).unwrap(), expected);
    }

    let values = prepared.context["captures"].as_array_mut().unwrap();
    let old = values.capacity();
    let mut larger = Vec::with_capacity(old + EXTRA_SLOTS);
    larger.append(values);
    expected.payload_bytes += (larger.capacity() - old) * size_of::<Value>();
    *values = larger;
    assert_eq!(resources(&prepared).unwrap(), expected);
    let Value::String(value) = &mut prepared.context["captures"][1] else {
        panic!("Owned context text fixture");
    };
    expected.payload_bytes += grow_text(value);
    assert_eq!(resources(&prepared).unwrap(), expected);
    for json in [
        &mut prepared.context,
        &mut prepared.summary,
        &mut prepared.json_summary,
        &mut prepared.diagnostic,
    ] {
        expected.payload_bytes += grow_first_key(json);
    }
    assert_eq!(resources(&prepared).unwrap(), expected);
    let Value::String(value) = &mut prepared.diagnostic["message"] else {
        panic!("Owned diagnostic text fixture");
    };
    expected.payload_bytes += grow_text(value);
    expected.payload_bytes += grow_text(prepared.pnm_error.as_mut().unwrap());
    assert_eq!(resources(&prepared).unwrap(), expected);

    let error = prepared.pnm_error.take().unwrap();
    let mut absent = expected;
    absent.payload_bytes -= error.capacity();
    assert_eq!(resources(&prepared).unwrap(), absent);
    prepared.pnm_error = Some(error);
    prepared.policy = RangePolicy::Reject;
    prepared.collection = true;
    assert_eq!(resources(&prepared).unwrap(), expected);
    assert_eq!(resources(&prepared).unwrap(), expected);
    assert_eq!(
        serde_json::to_value(&prepared.frames).unwrap(),
        frames_before
    );
    assert_eq!(
        [
            prepared.context,
            prepared.summary,
            prepared.json_summary,
            prepared.diagnostic,
        ],
        json_before
    );
    assert_eq!(prepared.pnm_error, error_before);
}
