/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Passive Designer metadata preserves original inputs, recipes and cached observations.
use docsis_ofdm_fec::api::{CaptureTime, Dpd, MerDb, NamedCapture, ProfileId};
use pnm_input::{RxMerDesign, RxMerDesignPosition, RxmerRecipe};
use std::mem::size_of;

const SAVED: &[u8] = include_bytes!("fixtures/designer-flat.json");
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
/// Original fixture profile identity, independent of generated/transmitted profile admission.
const PROFILE_ID: u8 = 0;
/// Explicit synthetic baseline dB for this metadata-only input fixture.
const FLAT_BASELINE_DB: f64 = 48.0;
/// Original anchor seconds; not a simulation duration or generated timestamp.
const ANCHOR_EPOCH: u32 = 7;
/// Extra original whitespace exercises both original-byte Arc occurrences.
const ORIGINAL_PADDING_BYTES: usize = 4096;
/// One-based first original baseline position.
const ORIGINAL_CAPTURE_INDEX: usize = 1;
/// Complete original collection contains one baseline.
const ORIGINAL_CAPTURE_COUNT: usize = 1;

fn profile() -> Dpd {
    Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(PROFILE_ID).unwrap(),
    )
    .unwrap()
}

fn position() -> RxMerDesignPosition {
    RxMerDesignPosition::new(ORIGINAL_CAPTURE_INDEX, ORIGINAL_CAPTURE_COUNT).unwrap()
}

#[test]
fn flat_metadata_includes_typed_record_original_and_cached_rxmer_without_changes() {
    let design = RxMerDesign::flat(
        &profile(),
        MerDb::new(FLAT_BASELINE_DB).unwrap(),
        CaptureTime::new(ANCHOR_EPOCH).unwrap(),
        RxmerRecipe::default(),
        position(),
    )
    .unwrap();
    let original = design.original_bytes().to_vec();
    let recipe = design.recipe().clone();
    let observation = design.materialize();
    let values = observation.values();
    let decoded = observation.provenance().decoded_bytes().to_vec();
    let footprint = design.retained_footprint().unwrap();
    assert!(footprint.payload_bytes > size_of::<RxMerDesign>() + original.len() + decoded.len());
    // Original bytes, record, cached RxMER and cached original/decoded provenance.
    assert_eq!(footprint.shared_roots, 5);
    assert_eq!(footprint.ordered_entries, 0);
    assert_eq!(design.retained_footprint().unwrap(), footprint);
    assert_eq!(design.original_bytes(), original);
    assert_eq!(design.recipe(), &recipe);
    assert_eq!(design.materialize().values(), values);
    assert_eq!(design.position().index, ORIGINAL_CAPTURE_INDEX);
    assert_eq!(design.position().count, ORIGINAL_CAPTURE_COUNT);
    assert_eq!(
        observation.provenance().original_epoch().value(),
        ANCHOR_EPOCH
    );
    assert_eq!(observation.provenance().original_bytes(), original);
    assert_eq!(observation.provenance().decoded_bytes(), decoded);
    assert_eq!(observation.source_kind(), "merlin_synthetic_rxmer");
}

#[test]
fn imported_metadata_preserves_embedded_baseline_and_original_source_provenance() {
    let input = NamedCapture::new("ordinary baseline", RXMER.to_vec()).unwrap();
    let design =
        RxMerDesign::imported(&profile(), &input, RxmerRecipe::default(), position()).unwrap();
    let original = design.original_bytes().to_vec();
    let observation = design.materialize();
    let source = observation.frame().source.clone();
    let original_digest = observation.provenance().original_sha256().to_owned();
    let footprint = design.retained_footprint().unwrap();
    assert!(!design.is_synthetic());
    assert_eq!(footprint.shared_roots, 5);
    assert_eq!(footprint.ordered_entries, 0);
    assert!(footprint.payload_bytes > original.len() + RXMER.len());
    assert_eq!(design.original_bytes(), original);
    assert_eq!(input.bytes(), RXMER);
    assert_eq!(design.materialize().frame().source, source);
    assert_eq!(
        design.materialize().provenance().original_sha256(),
        original_digest
    );
    assert_eq!(design.materialize().source_kind(), "merlin_designed_rxmer");
    assert_eq!(design.retained_footprint().unwrap(), footprint);
}

#[test]
fn exact_original_whitespace_is_charged_on_each_retained_original_arc_edge() {
    let ordinary = NamedCapture::new("saved design", SAVED.to_vec()).unwrap();
    let mut padded = SAVED.to_vec();
    padded.extend(std::iter::repeat_n(b' ', ORIGINAL_PADDING_BYTES));
    let padded = NamedCapture::new("saved design", padded).unwrap();
    let small = RxMerDesign::from_bytes(&ordinary).unwrap();
    let large = RxMerDesign::from_bytes(&padded).unwrap();
    let small_footprint = small.retained_footprint().unwrap();
    let large_footprint = large.retained_footprint().unwrap();
    assert!(
        large_footprint.payload_bytes - small_footprint.payload_bytes >= ORIGINAL_PADDING_BYTES * 2
    );
    assert_eq!(large_footprint.shared_roots, small_footprint.shared_roots);
    assert_eq!(large.recipe(), small.recipe());
    assert_eq!(large.materialize().values(), small.materialize().values());
    assert_eq!(large.original_bytes(), padded.bytes());
    assert_eq!(
        large.materialize().provenance().original_bytes(),
        padded.bytes()
    );
}

#[test]
fn cloned_design_metadata_survives_original_release_and_keeps_cached_result_independent() {
    let original = NamedCapture::new("saved design", SAVED.to_vec()).unwrap();
    let design = RxMerDesign::from_bytes(&original).unwrap();
    let footprint = design.retained_footprint().unwrap();
    let alias = design.clone();
    let observed = design.materialize();
    let recipe = design.recipe().clone();
    assert_eq!(alias.retained_footprint().unwrap(), footprint);
    drop(original);
    drop(design);
    assert_eq!(alias.retained_footprint().unwrap(), footprint);
    assert_eq!(alias.original_bytes(), SAVED);
    assert_eq!(alias.recipe(), &recipe);
    assert_eq!(alias.materialize().values(), observed.values());
    assert_eq!(
        alias.materialize().provenance().decoded_bytes(),
        observed.provenance().decoded_bytes()
    );
}
