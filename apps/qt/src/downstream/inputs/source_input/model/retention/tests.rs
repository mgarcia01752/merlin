/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual source modes, repeated accepted aliases and requested option capacity.
use super::*;
use serde_json::json;

// Requested spare UTF-8 bytes for the existing source-mode String only.
const EXTRA_MODE_BYTES: usize = 29;

fn assert_send<T: Send>() {}

#[test]
fn file_draft_accepted_config_and_mode_spare_capacity_are_passive_independent_occurrences() {
    assert_send::<SourceModel>();
    let mut model = SourceModel::default();
    assert_eq!(
        resources(&model).unwrap().payload_bytes,
        size_of::<SourceModel>()
    );
    model
        .stage("exact.bin".into(), vec![0xef, 0xbb, 0xbf, 0, 0xff, 13, 10])
        .unwrap(); // Original BOM/null/newline byte anchors.
    let options =
        json!({"mode":"file","seed":32767,"append":false,"target_bits":0,"fill_seed":32767}); // Existing valid independent seeds; no source advances.
    model.accept(&options.to_string()).unwrap();
    model.begin();
    let before = model.export();
    let accepted = model.accepted.as_ref().unwrap();
    let dynamic = |value: &SourceInput| {
        value.retained_footprint().unwrap().payload_bytes - size_of::<SourceInput>()
    };
    let expected = size_of::<SourceModel>()
        + dynamic(&model.file.as_ref().unwrap().payload)
        + dynamic(&accepted.file.as_ref().unwrap().payload)
        + dynamic(&accepted.config)
        + accepted.options.mode.capacity();
    let measured = resources(&model).unwrap();
    assert_eq!(measured.payload_bytes, expected);
    let roots = model
        .file
        .as_ref()
        .unwrap()
        .payload
        .retained_footprint()
        .unwrap()
        .shared_roots
        + accepted
            .file
            .as_ref()
            .unwrap()
            .payload
            .retained_footprint()
            .unwrap()
            .shared_roots
        + accepted.config.retained_footprint().unwrap().shared_roots;
    assert_eq!(measured.shared_roots, roots);
    let accepted = model.accepted.as_mut().unwrap();
    let old = accepted.options.mode.capacity();
    let mut mode = String::with_capacity(old + EXTRA_MODE_BYTES);
    mode.push_str(&accepted.options.mode);
    accepted.options.mode = mode;
    let delta = accepted.options.mode.capacity() - old;
    assert_eq!(
        resources(&model).unwrap().payload_bytes - measured.payload_bytes,
        delta
    );
    assert_eq!(model.export(), before);
    let removed = model
        .file
        .as_ref()
        .unwrap()
        .payload
        .retained_footprint()
        .unwrap();
    model.file = None;
    assert_eq!(
        resources(&model).unwrap().payload_bytes,
        measured.payload_bytes + delta - (removed.payload_bytes - size_of::<SourceInput>())
    );
    assert_eq!(resources(&model).unwrap(), resources(&model).unwrap());
    let alias = model.snapshot().unwrap();
    let original = alias.config.original_bytes().unwrap().to_vec();
    drop(model);
    assert_eq!(alias.config.original_bytes().unwrap(), original);
}
