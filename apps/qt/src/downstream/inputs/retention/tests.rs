/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Independent JSON/storage arithmetic and actual public DTO spare-capacity fixtures.
use super::*;
use merlin::downstream::{inputs::CaptureLoader, NamedCapture};
use serde_json::json;

// Independent requested spare storage; no production or scientific policy is inferred.
const SPARE_BYTES: usize = 31;
const SPARE_SLOTS: usize = 13;
fn text(value: &str) -> String {
    let mut owned = String::with_capacity(value.len() + SPARE_BYTES);
    owned.push_str(value);
    owned
}

#[test]
fn actual_json_capacity_map_population_and_all_dimension_arithmetic_are_passive() {
    let key = text("items");
    let label = text("unchanged");
    let mut values = Vec::with_capacity(SPARE_SLOTS);
    values.push(Value::String(label));
    let value = Value::Object([(key, Value::Array(values))].into_iter().collect());
    let Value::Object(map) = &value else {
        panic!("Object fixture");
    };
    let (key, array) = map.iter().next().unwrap();
    let Value::Array(values) = array else {
        panic!("Array fixture");
    };
    let Value::String(label) = &values[0] else {
        panic!("Text fixture");
    };
    let expected = Resources {
        payload_bytes: size_of::<(String, Value)>()
            + key.capacity()
            + values.capacity() * size_of::<Value>()
            + label.capacity(),
        ordered_entries: 1,
        shared_roots: 0,
    };
    assert_eq!(json_children(&value).unwrap(), expected);
    assert_eq!(json_children(&value).unwrap(), expected);
    assert_eq!(value, json!({"items":["unchanged"]}));
    assert!(product(usize::MAX, size_of::<Value>()).is_err());
    assert!(public_children(RetainedFootprint::default(), size_of::<String>()).is_err());
    for dimension in 0..3 {
        // Independent payload/entry/root overflow cases.
        let mut all = Resources::default();
        let mut one = Resources::default();
        match dimension {
            0 => {
                all.payload_bytes = usize::MAX;
                one.payload_bytes = 1;
            }
            1 => {
                all.ordered_entries = usize::MAX;
                one.ordered_entries = 1;
            }
            _ => {
                all.shared_roots = usize::MAX;
                one.shared_roots = 1;
            }
        }
        let before = all;
        assert!(append(&mut all, one).is_err());
        assert_eq!(all, before);
    }
}

#[test]
fn inferred_public_frame_and_profile_storage_keep_actual_spare_capacity() {
    let original = NamedCapture::new(
        "rx.json",
        include_bytes!("../../../../../../examples/measurements/single-channel/rxmer.json")
            .to_vec(),
    )
    .unwrap();
    let admitted = CaptureLoader::rxmer(&original).unwrap();
    let mut frame = admitted.frame().clone();
    let before = rxmer_frame_children(&frame).unwrap();
    let old = frame.mer_db.capacity();
    let mut values = Vec::with_capacity(old + SPARE_SLOTS);
    values.extend_from_slice(&frame.mer_db);
    let delta = (values.capacity() - old) * size_of::<Option<f64>>();
    frame.mer_db = values;
    let after = rxmer_frame_children(&frame).unwrap();
    assert_eq!(after.payload_bytes - before.payload_bytes, delta);
    assert_eq!(frame.mer_db, admitted.frame().mer_db);
    let old = frame.source.name.capacity();
    let mut name = String::with_capacity(old + SPARE_BYTES);
    name.push_str(&frame.source.name);
    frame.source.name = name;
    assert_eq!(
        rxmer_frame_children(&frame).unwrap().payload_bytes - after.payload_bytes,
        frame.source.name.capacity() - old
    );
    let original = NamedCapture::new(
        "profile.json",
        include_bytes!(
            "../../../../../../examples/measurements/single-channel/modulation_profile.json"
        )
        .to_vec(),
    )
    .unwrap();
    let catalog = CaptureLoader::profile_capture(&original).unwrap();
    let mut profile = catalog.frame(catalog.profile_ids()[0]).unwrap().clone();
    let before = profile_children(&profile).unwrap();
    let old = profile.assignments.capacity();
    let mut assignments = Vec::with_capacity(old + SPARE_SLOTS);
    assignments.extend_from_slice(&profile.assignments);
    let expected = count_buffer(&assignments) - count_buffer(&profile.assignments);
    profile.assignments = assignments;
    assert!(profile.assignments.capacity() > old);
    assert_eq!(
        profile_children(&profile).unwrap().payload_bytes - before.payload_bytes,
        expected
    );
}
fn count_buffer<T>(values: &Vec<T>) -> usize {
    values.capacity() * size_of::<T>()
}
