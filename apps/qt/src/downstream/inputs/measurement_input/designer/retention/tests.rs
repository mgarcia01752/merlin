/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Actual flat/imported Designer records, cached design metadata and session capacities.
use super::*;
use crate::downstream::inputs::measurement_input::document::Capture;
use merlin::downstream::{inputs::CaptureLoader, NamedCapture};
use std::sync::Arc;

// Independent requested spare mask and recipe-edit slots, not Designer limits.
const EXTRA_MASK_SLOTS: usize = 29;
const EXTRA_EDIT_SLOTS: usize = 11;

fn assert_send<T: Send>() {}

#[test]
fn flat_private_mask_and_imported_session_recipe_storage_are_passive_actual_owners() {
    assert_send::<Origin>();
    assert_send::<Session>();
    let designed = Capture::decode(
        "designed.json".into(),
        include_bytes!(
            "../../../../../../../../crates/pnm-input/tests/fixtures/designer-flat.json"
        )
        .to_vec(),
        true,
    )
    .unwrap();
    let mut origin = designed.designer.as_ref().unwrap().as_ref().clone();
    let original = origin.design.original_bytes().to_vec();
    let measured = origin_resources(&origin).unwrap();
    let Baseline::Flat { valid, .. } = &origin.record.baseline else {
        panic!("Actual flat fixture");
    };
    let design = origin.design.retained_footprint().unwrap();
    assert_eq!(
        measured.payload_bytes,
        size_of::<Origin>()
            + valid.capacity() * size_of::<bool>()
            + origin.record.recipe.edits.capacity()
                * size_of::<merlin::downstream::inputs::ValueEdit>()
            + design.payload_bytes
            - size_of::<RxMerDesign>()
    );
    assert_eq!(measured.shared_roots, design.shared_roots);
    assert_eq!(measured.ordered_entries, design.ordered_entries);
    let Baseline::Flat { valid, .. } = &mut origin.record.baseline else {
        panic!("Actual flat fixture");
    };
    let old = valid.capacity();
    let mut mask = Vec::with_capacity(old + EXTRA_MASK_SLOTS);
    mask.extend_from_slice(valid);
    let delta = mask.capacity() - old;
    *valid = mask;
    assert_eq!(
        origin_resources(&origin).unwrap().payload_bytes - measured.payload_bytes,
        delta * size_of::<bool>()
    );
    assert_eq!(origin.design.original_bytes(), original);
    let input = NamedCapture::new(
        "profile.json",
        include_bytes!(
            "../../../../../../../../examples/measurements/single-channel/modulation_profile.json"
        )
        .to_vec(),
    )
    .unwrap();
    let catalog = CaptureLoader::profile_capture(&input).unwrap();
    let profile = Arc::new(catalog.frame(catalog.profile_ids()[0]).unwrap().clone());
    let rx = Capture::decode(
        "rx.json".into(),
        include_bytes!("../../../../../../../../examples/measurements/single-channel/rxmer.json")
            .to_vec(),
        true,
    )
    .unwrap();
    let mut session = Session::begin(&[rx], profile, catalog).unwrap();
    let profile_lifetime = Arc::downgrade(&session.profile);
    assert!(matches!(
        session.origins[0].record.baseline,
        Baseline::Imported { .. }
    ));
    let measured = session_resources(&session).unwrap();
    assert!(measured.ordered_entries > 0);
    let saved = session.origins[0].design.original_bytes().to_vec();
    let old = session.request.recipe.edits.capacity();
    let mut edits = Vec::with_capacity(old + EXTRA_EDIT_SLOTS);
    edits.extend_from_slice(&session.request.recipe.edits);
    let capacity = edits.capacity();
    session.request.recipe.edits = edits;
    assert_eq!(
        session_resources(&session).unwrap().payload_bytes - measured.payload_bytes,
        (capacity - old) * size_of::<merlin::downstream::inputs::ValueEdit>()
    );
    assert_eq!(session.origins[0].design.original_bytes(), saved);
    assert_eq!(
        session_resources(&session).unwrap(),
        session_resources(&session).unwrap()
    );
    drop(session);
    assert!(profile_lifetime.upgrade().is_none());
}
