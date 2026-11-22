/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native input consumers use only facade paths and preserve accepted contracts.
#![cfg(feature = "native-input")]
use merlin::{downstream, upstream};

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.bin");
const DECODED_PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const WORKBENCH: &[u8] = include_bytes!("../../../examples/workbench/single/workbench.merlin.json");

fn hex(text: &str) -> Vec<u8> {
    text.split_whitespace()
        .flat_map(|word| {
            word.as_bytes()
                .chunks_exact(2)
                .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn facade_native_profiles_preserve_catalog_roles_grids_and_original_octets() {
    let binary = downstream::NamedCapture::new("wrong.json", PROFILE.to_vec()).unwrap();
    let decoded = downstream::NamedCapture::new("wrong.bin", DECODED_PROFILE.to_vec()).unwrap();
    let ids = downstream::inputs::CaptureLoader::profile_ids(&binary).unwrap();
    assert_eq!(
        ids.iter().map(|id| id.value()).collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
    assert_eq!(
        ids,
        downstream::inputs::CaptureLoader::profile_ids(&decoded).unwrap()
    );
    for id in ids {
        let first = downstream::inputs::CaptureLoader::profile(&binary, id).unwrap();
        let second = downstream::inputs::CaptureLoader::profile(&decoded, id).unwrap();
        assert_eq!(first.grid(), second.grid());
        assert_eq!(first.carriers(), second.carriers());
        assert_eq!(first.sources()[0].original_bytes(), PROFILE);
        assert_eq!(second.sources()[0].original_bytes(), DECODED_PROFILE);
        assert_eq!(first.profile_ids(), second.profile_ids());
    }
    assert_eq!(binary.bytes(), PROFILE);
    assert_eq!(decoded.bytes(), DECODED_PROFILE);
}

#[test]
fn facade_workbench_load_is_idle_owned_and_keeps_legacy_validation() {
    let workbench = downstream::inputs::Workbench::from_bytes(WORKBENCH).unwrap();
    assert_eq!(workbench.original_document().as_ref(), WORKBENCH);
    assert!(!workbench.controllers().is_empty());
    for controller in workbench.controllers() {
        assert!(controller.profile().is_some());
        assert!(!controller.prepare().unwrap().is_empty());
    }
    assert!(downstream::inputs::Workbench::from_bytes(b"{invalid").is_err());
    let input = downstream::NamedCapture::new("not a profile", b"{}".to_vec()).unwrap();
    assert!(downstream::inputs::CaptureLoader::profile_ids(&input).is_err());
    assert_eq!(input.bytes(), b"{}");
}

#[test]
fn facade_upstream_ucd_keeps_literal_header_and_source_identity() {
    let original = hex(include_str!(
        "../../../examples/measurements/upstream/ucd/scheduled-50khz.hex"
    ));
    let capture = upstream::inputs::UcdCapture::from_mac_management(&original).unwrap();
    assert_eq!(capture.original_bytes(), original);
    assert_eq!(capture.header().message_type, 51);
    assert_eq!(capture.header().upstream_channel_id, 7);
    assert_eq!(capture.header().configuration_change_count, 254);
    assert_eq!(
        capture.original_sha256(),
        "7e47467fae5d8aaabbaa52f7d2dda028a3865fa5b9bf603548a38c00393c3e74"
    );
    assert_eq!(capture.bursts()[0].iuc, 5);
    assert_eq!(capture.bursts()[1].iuc, 13);
    assert!(upstream::inputs::UcdCapture::from_mac_management(&original[..14]).is_err());
}

#[test]
fn facade_upstream_rxmer_retains_quarter_db_and_missing_readings() {
    let original = hex(include_str!(
        "../../../examples/measurements/upstream/rxmer/ccap-50khz.hex"
    ));
    let capture = upstream::inputs::RxmerCapture::from_pnm(&original).unwrap();
    assert_eq!(capture.original_bytes(), original);
    assert_eq!(
        capture.raw_readings(),
        [0, 95, 127, 128, 160, 253, 254, 255]
    );
    assert_eq!(capture.readings_db()[1], Some(23.75));
    assert_eq!(capture.readings_db()[4], Some(40.0));
    assert_eq!(capture.readings_db()[7], None);
    assert!(upstream::inputs::RxmerCapture::from_pnm(&original[..20]).is_err());
}

#[test]
fn facade_designer_uses_typed_owners_and_preserves_original_profile_identity() {
    let input = downstream::NamedCapture::new("profile", PROFILE.to_vec()).unwrap();
    let profile =
        downstream::inputs::CaptureLoader::profile(&input, downstream::ProfileId::new(0).unwrap())
            .unwrap();
    let key: &downstream::ScenarioKey = profile.key();
    let grid: &downstream::FrequencyGrid = profile.grid();
    assert_eq!(&key.grid, grid);
    let design = downstream::inputs::RxMerDesign::flat(
        &profile,
        downstream::MerDb::new(48.0).unwrap(),
        downstream::CaptureTime::new(7).unwrap(),
        downstream::inputs::RxmerRecipe::default(),
        downstream::inputs::RxMerDesignPosition::new(1, 1).unwrap(),
    )
    .unwrap();
    design.check_profile(&profile).unwrap();
    let output = design.materialize();
    assert_eq!(&output.frame().key, key);
    assert_eq!(output.source_kind(), "merlin_synthetic_rxmer");
    assert_eq!(output.values().len(), grid.sample_count);
}

#[test]
fn facade_profile_capture_keeps_binary_json_science_and_original_sources() {
    let first = downstream::NamedCapture::new("original binary", PROFILE.to_vec()).unwrap();
    let second = downstream::NamedCapture::new("original JSON", DECODED_PROFILE.to_vec()).unwrap();
    let binary = downstream::inputs::CaptureLoader::profile_capture(&first).unwrap();
    let decoded = downstream::inputs::CaptureLoader::profile_capture(&second).unwrap();
    assert_eq!(binary.profile_ids(), [0, 1, 2, 3]);
    assert_eq!(binary.profile_ids(), decoded.profile_ids());
    for id in binary.profile_ids() {
        assert_eq!(binary.summary(*id).unwrap(), decoded.summary(*id).unwrap());
        assert_eq!(
            binary.schemes(*id, 0, 256).unwrap().schemes,
            decoded.schemes(*id, 0, 256).unwrap().schemes
        );
    }
    assert_eq!(binary.provenance().original_bytes(), PROFILE);
    assert_eq!(decoded.provenance().original_bytes(), DECODED_PROFILE);
    let view = binary
        .inspect(&downstream::ProfileViewRequest {
            max_buckets: 16,
            ..Default::default()
        })
        .unwrap();
    assert!(
        view.series
            .iter()
            .map(|series| series.buckets.len())
            .sum::<usize>()
            <= 16
    );
}

#[test]
fn facade_configured_channel_preview_uses_public_typed_configuration_and_inputs() {
    let input = downstream::NamedCapture::new("profile", PROFILE.to_vec()).unwrap();
    let profile =
        downstream::inputs::CaptureLoader::profile(&input, downstream::ProfileId::new(0).unwrap())
            .unwrap();
    let transmission =
        downstream::TransmissionConfiguration::new(downstream::TransmissionSettings::default())
            .unwrap();
    let channel =
        downstream::ChannelConfiguration::new(downstream::ChannelConfigurationInput::default())
            .unwrap();
    let preview = channel
        .preview(
            &transmission,
            &profile,
            None,
            downstream::AssociationTolerance::ZERO,
        )
        .unwrap();
    assert_eq!(preview.key, *profile.key());
    assert!(preview
        .effective_response
        .iter()
        .flatten()
        .all(|sample| *sample == [1.0, 0.0]));
    assert_eq!(preview.sources[0].original_bytes(), PROFILE);
}
