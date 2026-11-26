/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual original/catalog storage and repeated shared presentation occurrences.
use super::*;
use std::sync::Arc;

// Requested spare UTF-8 bytes in the draft display name, independent of file bytes.
const EXTRA_NAME_BYTES: usize = 23;

fn assert_send<T: Send>() {}

#[test]
fn draft_and_accepted_binary_original_catalog_json_and_spare_name_are_counted_passively() {
    assert_send::<ProfileModel>();
    let capture = Capture::decode(
        "original.bin".into(),
        include_bytes!(
            "../../../../../../../examples/measurements/single-channel/modulation_profile.bin"
        )
        .to_vec(),
    )
    .unwrap();
    let id = capture.catalog.profile_ids()[0];
    let mut model = ProfileModel {
        context: Default::default(),
        draft: Some(capture.clone()),
        accepted: Some((capture, id)),
    };
    let draft = model.draft.as_ref().unwrap();
    let accepted = &model.accepted.as_ref().unwrap().0;
    assert!(Arc::ptr_eq(&draft.document, &accepted.document));
    let document_lifetime = Arc::downgrade(&draft.document);
    let original = draft.original.bytes().to_vec();
    let document = draft.document.as_ref().clone();
    let measured = resources(&model).unwrap();
    let expected_roots = [draft, accepted]
        .into_iter()
        .map(|capture| {
            capture.original.retained_footprint().unwrap().shared_roots
                + capture.catalog.retained_footprint().unwrap().shared_roots
                + 1
        })
        .sum::<usize>();
    assert_eq!(measured.shared_roots, expected_roots);
    assert!(measured.ordered_entries > 0);
    let draft = model.draft.as_mut().unwrap();
    let old = draft.name.capacity();
    let mut name = String::with_capacity(old + EXTRA_NAME_BYTES);
    name.push_str(&draft.name);
    draft.name = name;
    let delta = draft.name.capacity() - old;
    assert_eq!(
        resources(&model).unwrap().payload_bytes - measured.payload_bytes,
        delta
    );
    assert_eq!(model.draft.as_ref().unwrap().original.bytes(), original);
    assert_eq!(model.draft.as_ref().unwrap().document.as_ref(), &document);
    assert_eq!(model.accepted.as_ref().unwrap().1, id);
    let repeated = resources(&model).unwrap();
    assert_eq!(resources(&model).unwrap(), repeated);
    model.draft = None;
    assert!(resources(&model).unwrap().payload_bytes < repeated.payload_bytes);
    assert_eq!(
        model.accepted.as_ref().unwrap().0.original.bytes(),
        original
    );
    assert!(document_lifetime.upgrade().is_some());
    drop(model);
    assert!(document_lifetime.upgrade().is_none());
}
