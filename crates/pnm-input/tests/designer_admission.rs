/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable public Designer operations preserve saved envelopes and independent baselines.
use docsis_ofdm_fec::api::*;
use pnm_input::{CaptureScope, RxMerDesign, RxMerDesignPosition, RxmerRecipe, Tilt};
use serde_json::{json, Value};

const FIXTURE: &[u8] = include_bytes!("fixtures/designer-flat.json");
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
const CHANNEL: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");

fn profile() -> Dpd {
    Dpd::from_bytes(
        &NamedCapture::new("profile", PROFILE.to_vec()).unwrap(),
        ProfileId::new(0).unwrap(),
    )
    .unwrap()
}
fn position() -> RxMerDesignPosition {
    RxMerDesignPosition::new(1, 1).unwrap()
}
fn tilt(db: f64) -> RxmerRecipe {
    RxmerRecipe {
        tilt: Some(Tilt {
            scope: CaptureScope::All,
            tilt_db: db,
        }),
        ..Default::default()
    }
}
fn baseline() -> NamedCapture {
    let mut value: Value = serde_json::from_slice(RXMER).unwrap();
    value["values"] = json!(vec![48.0; 7480]);
    NamedCapture::new(
        "exact ordinary baseline",
        serde_json::to_vec(&value).unwrap(),
    )
    .unwrap()
}

#[test]
fn saved_envelope_load_retains_exact_bytes_epochs_mask_and_independent_result() {
    let original = NamedCapture::new("exact saved design", FIXTURE.to_vec()).unwrap();
    let design = RxMerDesign::from_bytes(&original).unwrap();
    assert_eq!(design.original_bytes(), FIXTURE);
    assert_eq!(design.position().index, 1);
    assert_eq!(design.position().count, 1);
    assert!(design.is_synthetic());
    let output = design.materialize();
    assert_eq!(output.source_kind(), "merlin_synthetic_rxmer");
    assert_eq!(output.provenance().original_epoch().value(), 1700000000);
    assert_eq!(output.provenance().original_bytes(), FIXTURE);
    let expected: Value = serde_json::from_slice(FIXTURE).unwrap();
    let mask = expected["baseline"]["valid"].as_array().unwrap();
    for (value, valid) in output.values().iter().zip(mask) {
        assert_eq!(value.is_some(), valid.as_bool().unwrap());
    }
    assert!(design.check_profile(&profile()).is_err());
    drop(original);
    drop(design);
    assert_eq!(output.provenance().original_bytes(), FIXTURE);
}

#[test]
fn typed_flat_creation_uses_exact_profile_identity_and_backend_source_kind() {
    let profile = profile();
    let design = RxMerDesign::flat(
        &profile,
        MerDb::new(48.0).unwrap(),
        CaptureTime::new(7).unwrap(),
        RxmerRecipe::default(),
        position(),
    )
    .unwrap();
    design.check_profile(&profile).unwrap();
    let output = design.materialize();
    assert_eq!(&output.frame().key, profile.key());
    assert_eq!(output.provenance().original_epoch().value(), 7);
    assert_eq!(output.source_kind(), "merlin_synthetic_rxmer");
    assert!(output.values().iter().flatten().all(|v| v.value() == 48.0));
    let plain = RxMer::from_bytes(&NamedCapture::new("raw", RXMER.to_vec()).unwrap()).unwrap();
    assert_eq!(plain.source_kind(), "decoded_json_rxmer");
    let channel =
        ChannelEstimate::from_bytes(&NamedCapture::new("raw H", CHANNEL.to_vec()).unwrap())
            .unwrap();
    assert_eq!(channel.source_kind(), "decoded_json_channel_response");
    let reloaded = RxMerDesign::from_bytes(
        &NamedCapture::new("reload", design.original_bytes().to_vec()).unwrap(),
    )
    .unwrap();
    assert_eq!(reloaded.materialize().values(), output.values());
}

#[test]
fn recipe_replacement_uses_original_imported_baseline_and_never_compounds_edits() {
    let original = baseline();
    let design = RxMerDesign::imported(&profile(), &original, tilt(-6.0), position()).unwrap();
    let values = design.materialize().values();
    assert_eq!(values[0].unwrap().value(), 48.0);
    assert_eq!(values[7479].unwrap().value(), 42.0);
    assert!(!design.is_synthetic());
    assert_eq!(design.materialize().source_kind(), "merlin_designed_rxmer");
    let replaced = design.with_recipe(tilt(-3.0)).unwrap();
    assert_eq!(replaced.materialize().values()[7479].unwrap().value(), 45.0);
    assert_eq!(design.materialize().values()[7479].unwrap().value(), 42.0);
    let restored = design.with_recipe(RxmerRecipe::default()).unwrap();
    assert!(restored
        .materialize()
        .values()
        .iter()
        .all(|v| v.unwrap().value() == 48.0));
    let record: Value = serde_json::from_slice(replaced.original_bytes()).unwrap();
    assert_eq!(
        record["baseline"]["original"]["byte_length"],
        original.bytes().len()
    );
    let reload = RxMerDesign::from_bytes(
        &NamedCapture::new("reload", replaced.original_bytes().to_vec()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        reload.materialize().values(),
        replaced.materialize().values()
    );
    assert_eq!(original.bytes(), baseline().bytes());
}

#[test]
fn invalid_recipe_version_position_nested_and_random_import_reject_without_mutation() {
    for (index, count) in [(0, 1), (2, 1), (1, 0), (1, 257)] {
        assert!(RxMerDesignPosition::new(index, count).is_err());
    }
    for bytes in [b"{bad".as_slice(), b"{\"unrecognized\":1}".as_slice()] {
        assert!(RxMerDesign::recipe_from_json(bytes).is_err());
    }
    assert!(RxMerDesign::recipe_from_json(&vec![b' '; 65537]).is_err());
    assert!(RxMerDesign::from_bytes(&baseline()).is_err());
    let original = NamedCapture::new("saved", FIXTURE.to_vec()).unwrap();
    assert!(
        RxMerDesign::imported(&profile(), &original, RxmerRecipe::default(), position()).is_err()
    );
    let mut bad: Value = serde_json::from_slice(FIXTURE).unwrap();
    bad["merlin_rxmer_designer"] = json!(2);
    assert!(RxMerDesign::from_bytes(
        &NamedCapture::new("bad", serde_json::to_vec(&bad).unwrap()).unwrap()
    )
    .is_err());
    let mut excessive: Value = serde_json::from_slice(FIXTURE).unwrap();
    excessive["baseline"]["key"]["grid"]["sample_count"] = json!(65536);
    excessive["baseline"]["valid"] = json!(vec![false; 65536]);
    excessive["recipe"] = json!({});
    excessive["capture_count"] = json!(65);
    assert!(RxMerDesign::from_bytes(
        &NamedCapture::new("too many values", serde_json::to_vec(&excessive).unwrap()).unwrap()
    )
    .is_err());
    let design = RxMerDesign::from_bytes(&original).unwrap();
    let before = design.original_bytes().to_vec();
    let invalid =
        RxMerDesign::recipe_from_json(b"{\"tilt\":{\"scope\":{\"mode\":\"all\"},\"tilt_db\":61}}")
            .unwrap();
    assert!(design.with_recipe(invalid).is_err());
    assert_eq!(design.original_bytes(), before);
    let random = RxMerDesign::recipe_from_json(
        b"{\"random_variation\":{\"standard_deviation_db\":1,\"seed\":41}}",
    )
    .unwrap();
    assert!(RxMerDesign::imported(&profile(), &baseline(), random, position()).is_err());
}

#[test]
fn source_catalog_designs_preserve_raw_ids_and_existing_baseline_semantics() {
    let mut document: Value = serde_json::from_slice(PROFILE).unwrap();

    // Independent encoded ID 255 must remain usable for passive design only.
    document["profiles"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|profile| profile["profile_id"] == 0)
        .unwrap()["profile_id"] = json!(255);
    let original =
        NamedCapture::new("raw profile", serde_json::to_vec(&document).unwrap()).unwrap();
    let capture = ProfileCapture::from_bytes(&original).unwrap();
    let flat = RxMerDesign::flat_capture(
        &capture,
        255,
        MerDb::new(48.0).unwrap(),
        CaptureTime::new(7).unwrap(),
        tilt(-6.0),
        position(),
    )
    .unwrap();
    let old = RxMerDesign::flat(
        &profile(),
        MerDb::new(48.0).unwrap(),
        CaptureTime::new(7).unwrap(),
        tilt(-6.0),
        position(),
    )
    .unwrap();
    assert_eq!(flat.original_bytes(), old.original_bytes());
    flat.check_capture(&capture, 255).unwrap();
    assert_eq!(
        flat.check_capture(&capture, 254).unwrap_err().code(),
        ErrorCode::InvalidIdentifier
    );
    assert!(ProfileId::new(255).is_err());
    let baseline = baseline();
    let imported =
        RxMerDesign::imported_capture(&capture, 255, &baseline, tilt(-6.0), position()).unwrap();
    let old = RxMerDesign::imported(&profile(), &baseline, tilt(-6.0), position()).unwrap();
    assert_eq!(imported.original_bytes(), old.original_bytes());
    assert_eq!(
        imported
            .materialize()
            .values()
            .last()
            .unwrap()
            .unwrap()
            .value(),
        42.0
    );
    assert!(RxMerDesign::imported_capture(
        &capture,
        255,
        &NamedCapture::new("nested", flat.original_bytes().to_vec()).unwrap(),
        RxmerRecipe::default(),
        position()
    )
    .is_err());
    assert!(RxMerDesign::flat_capture(
        &capture,
        254,
        MerDb::new(48.0).unwrap(),
        CaptureTime::new(7).unwrap(),
        RxmerRecipe::default(),
        position()
    )
    .is_err());
    document["channel_id"] = json!(99);
    let other = ProfileCapture::from_bytes(
        &NamedCapture::new("other", serde_json::to_vec(&document).unwrap()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        flat.check_capture(&other, 255).unwrap_err().code(),
        ErrorCode::IncompatibleGrid
    );
    assert!(RxMerDesign::imported_capture(
        &other,
        255,
        &baseline,
        RxmerRecipe::default(),
        position()
    )
    .is_err());
    assert_eq!(capture.provenance().original_bytes(), original.bytes());
}

#[test]
fn admitted_designer_limits_match_independent_existing_domains_and_collection_guard() {
    let limits = RxMerDesign::limits();

    // Independent established input domains; expected values never read production constants.
    assert_eq!(
        (limits.minimum_mer_db, limits.maximum_mer_db),
        (-30.0, 120.0)
    );
    assert_eq!(
        (
            limits.maximum_captures,
            limits.maximum_values,
            limits.maximum_value_edits
        ),
        (256, 4_000_000, 32)
    );
    assert_eq!(limits.maximum_random_standard_deviation_db, 20.0);
    assert_eq!(
        (
            limits.maximum_absolute_tilt_db,
            limits.maximum_ripple_peak_to_peak_db,
            limits.maximum_notch_depth_db
        ),
        (60.0, 60.0, 60.0)
    );
    assert_eq!(
        (
            limits.minimum_echo_distance_metres,
            limits.maximum_echo_distance_metres
        ),
        (0.001, 10_000.0)
    );
    let original = NamedCapture::new("original-design", FIXTURE.to_vec()).unwrap();
    let before = RxMerDesign::from_bytes(&original).unwrap();
    let mut excessive: Value = serde_json::from_slice(FIXTURE).unwrap();
    excessive["capture_count"] = json!(256);
    // 15,625 values times 256 captures equals the independent four-million guard.
    excessive["baseline"]["valid"] = json!(vec![true; 15625]);
    excessive["baseline"]["key"]["grid"]["sample_count"] = json!(15625);
    let boundary = NamedCapture::new("at-budget", serde_json::to_vec(&excessive).unwrap()).unwrap();
    assert_eq!(
        RxMerDesign::from_bytes(&boundary).unwrap().position().count,
        256
    );
    excessive["baseline"]["valid"] = json!(vec![true; 15626]);
    excessive["baseline"]["key"]["grid"]["sample_count"] = json!(15626);
    let excessive =
        NamedCapture::new("over-budget", serde_json::to_vec(&excessive).unwrap()).unwrap();
    assert!(RxMerDesign::from_bytes(&excessive)
        .err()
        .unwrap()
        .message()
        .contains("4,000,000"));
    assert_eq!(before.original_bytes(), FIXTURE);
    assert_eq!(before.position().count, 1);
}
