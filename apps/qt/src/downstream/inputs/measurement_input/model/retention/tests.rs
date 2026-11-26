/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Ordinary RX/H, Designer originals and actual collection/option capacities.
use super::*;
use std::sync::Arc;

// Independent requested spare capacities, not scientific or collection limits.
const EXTRA_CAPTURE_SLOTS: usize = 17;
const SELECTED_HASH_SLOTS: usize = 7;
const EXTRA_SELECTED_HASH_BYTES: usize = 19;
// Every Capture has one original-byte Arc and one typed-measurement Arc.
const BASE_SHARED_ROOTS_PER_CAPTURE: usize = 2;

fn assert_send<T: Send>() {}

#[test]
fn independent_rx_response_designer_aliases_and_option_slots_keep_originals_and_nulls() {
    assert_send::<Model>();
    let rx = Capture::decode(
        "rx.json".into(),
        include_bytes!("../../../../../../../../examples/measurements/single-channel/rxmer.json")
            .to_vec(),
        true,
    )
    .unwrap();
    let response = Capture::decode(
        "response.bin".into(),
        include_bytes!(
            "../../../../../../../../examples/workbench/downstream/single/ds-channel-estimate.bin"
        )
        .to_vec(),
        false,
    )
    .unwrap();
    let designed = Capture::decode(
        "designed.json".into(),
        include_bytes!(
            "../../../../../../../../crates/pnm-input/tests/fixtures/designer-flat.json"
        )
        .to_vec(),
        true,
    )
    .unwrap();
    assert!(designed.designer.is_some());
    let mut model = Model {
        draft_rx: vec![rx.clone(), designed.clone()],
        accepted_rx: vec![rx, designed],
        draft_response: Some(response.clone()),
        accepted_response: Some(response),
        ..Default::default()
    };
    assert!(Arc::ptr_eq(
        &model.draft_rx[0].original,
        &model.accepted_rx[0].original
    ));
    let original_lifetime = Arc::downgrade(&model.accepted_rx[0].original);
    let original = model.export();
    let measured = resources(&model).unwrap();
    let capture_occurrences = model.draft_rx.len()
        + model.accepted_rx.len()
        + usize::from(model.draft_response.is_some())
        + usize::from(model.accepted_response.is_some());
    assert!(measured.shared_roots >= capture_occurrences * BASE_SHARED_ROOTS_PER_CAPTURE);
    let old = model.draft_rx.capacity();
    let mut draft = Vec::with_capacity(old + EXTRA_CAPTURE_SLOTS);
    draft.append(&mut model.draft_rx);
    let capacity = draft.capacity();
    model.draft_rx = draft;
    assert_eq!(
        resources(&model).unwrap().payload_bytes - measured.payload_bytes,
        (capacity - old) * size_of::<Capture>()
    );
    let before = resources(&model).unwrap();
    let mut selected = Vec::with_capacity(SELECTED_HASH_SLOTS);
    let mut hash =
        String::with_capacity(model.accepted_rx[0].hash.len() + EXTRA_SELECTED_HASH_BYTES);
    hash.push_str(&model.accepted_rx[0].hash);
    let expected = selected.capacity() * size_of::<String>() + hash.capacity();
    selected.push(hash);
    model.options.selected_captures = Some(selected);
    assert_eq!(
        resources(&model).unwrap().payload_bytes - before.payload_bytes,
        expected
    );
    assert_eq!(model.export()["rxmer"], original["rxmer"]);
    assert_eq!(model.export()["response"], original["response"]);
    assert_eq!(resources(&model).unwrap(), resources(&model).unwrap());
    model.draft_rx.clear();
    assert!(original_lifetime.upgrade().is_some());
    assert_eq!(model.export()["rxmer"], original["rxmer"]);
    drop(model);
    assert!(original_lifetime.upgrade().is_none());
}
