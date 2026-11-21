/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Literal source-domain fixtures, strict schema, projection, atomic load and run.
use docsis_ofdma::api::*;
use serde_json::{json, Value};
const KHZ50: &[u8] = include_bytes!("../../../examples/cli/upstream/us-50khz.json");
const KHZ25: &[u8] = include_bytes!("../../../examples/cli/upstream/us-25khz.json");
const CONTROLS: &[u8] =
    include_bytes!("../../../examples/cli/upstream/us-controls-inspection.json");
fn reject(pointer: &str, value: Value) {
    let mut document: Value = serde_json::from_slice(KHZ50).unwrap();
    *document.pointer_mut(pointer).unwrap() = value;
    assert!(
        ConfigurationDocument::from_json(&serde_json::to_vec(&document).unwrap())
            .and_then(|doc| doc.prepare())
            .is_err(),
        "accepted {pointer}"
    );
}
#[test]
fn examples_roundtrip_complete_inventory_and_actual_both_numerology_runs() {
    for bytes in [
        KHZ25,
        KHZ50,
        include_bytes!("../../../examples/cli/upstream/us-synthetic-playback.json").as_slice(),
    ] {
        let document = ConfigurationDocument::from_json(bytes).unwrap();
        assert_eq!(
            serde_json::to_value(
                ConfigurationDocument::from_json(document.to_json().unwrap().as_bytes()).unwrap()
            )
            .unwrap(),
            serde_json::to_value(&document).unwrap()
        );
        let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
        session.configure_json(bytes).unwrap();
        let report = session.run().unwrap();
        assert_eq!(document.original_bytes(), Some(bytes));
        assert_eq!(report.source_configuration_original.as_deref(), Some(bytes));
        assert_eq!(
            report.source_configuration_sha256,
            document.original_sha256()
        );
        assert!(report.reference_recovered);
        assert_eq!(report.bit_errors, 0);
        assert_eq!(
            report.source_configuration_json,
            Some(document.to_json().unwrap())
        );
        assert_eq!(
            session.document().unwrap().inventory(),
            document.inventory()
        );
        assert_eq!(session.preview().unwrap().profile_ids, vec![5, 13]);
    }
    let control = ConfigurationDocument::from_json(CONTROLS).unwrap();
    assert_eq!(
        control.prepare().unwrap_err().code(),
        "UnsupportedCapability"
    );
    assert_eq!(
        control.presets[0]
            .channel
            .timestamp_snapshot
            .docsis_timestamp,
        0x12345678
    );
    assert_eq!(
        control.presets[0].probe_map.as_ref().unwrap().elements[1]
            .alternate_power_db()
            .unwrap(),
        Some(-9)
    );
    assert_eq!(
        serde_json::to_value(
            ConfigurationDocument::from_json(control.to_json().unwrap().as_bytes()).unwrap()
        )
        .unwrap(),
        serde_json::to_value(&control).unwrap()
    );
}
#[test]
fn source_reserved_width_range_and_cross_field_boundaries_reject() {
    for (pointer, value) in [
        ("/schema_version", json!(2)),
        ("/format", json!("merlin.upstream.experiment")),
        ("/presets/0/channel/message_type", json!(35)),
        ("/presets/0/channel/management_version", json!(4)),
        ("/presets/0/channel/upstream_channel_id", json!(0)),
        ("/presets/0/channel/downstream_channel_id", json!(0)),
        ("/presets/0/channel/ranging_required", json!(4)),
        ("/presets/0/channel/change_indicator_bitmask", json!(2048)),
        ("/presets/0/channel/timestamp_snapshot/reserved", json!(1)),
        (
            "/presets/0/channel/timestamp_snapshot/divide_by_20_msb",
            json!(16),
        ),
        ("/presets/0/channel/cyclic_prefix_samples", json!(97)),
        ("/presets/0/channel/rolloff_samples", json!(192)),
        ("/presets/0/channel/subcarrier_spacing_khz", json!(100)),
        ("/presets/0/channel/subcarrier_zero_hz", json!(1)),
        ("/presets/0/channel/frame_symbols", json!(5)),
        ("/presets/0/channel/randomization_seed", json!(8388608)),
        (
            "/presets/0/channel/encompassed_bandwidth_hz",
            json!(9_000_000),
        ),
        ("/presets/0/channel/excluded_bands/0/last", json!(72)),
        ("/presets/0/channel/unused_bands/0/first", json!(193)),
        (
            "/presets/0/channel/bursts/0/data_profile/0/modulation_index",
            json!(1),
        ),
        (
            "/presets/0/channel/bursts/0/data_profile/0/pilot_pattern",
            json!(8),
        ),
        (
            "/presets/0/channel/bursts/0/data_profile/0/additional_minislots",
            json!(255),
        ),
        (
            "/presets/0/channel/bursts/0/preamble_length_bits",
            json!(64),
        ),
        ("/presets/0/channel/bursts/1/iuc", json!(6)),
        ("/presets/0/assignment/assigned_iucs", json!([1])),
        ("/presets/0/assignment/testing_sid", json!(7)),
        ("/presets/0/assignment/ranging_sid", json!(16383)),
        ("/simulation/selected_iuc", json!(6)),
        ("/simulation/source_seed", json!(0)),
        ("/simulation/frames", json!(0)),
        ("/simulation/chunk_samples", json!(0)),
        ("/simulation/reference_variance", json!(0)),
        ("/simulation/ldpc_iterations", json!(101)),
        ("/simulation/retained_taps", json!(["unknown"])),
    ] {
        reject(pointer, value);
    }
}
#[test]
fn every_applicable_cp_rp_enum_and_loading_pattern_rule_has_a_case() {
    let mut document = ConfigurationDocument::from_json(KHZ50).unwrap();
    for cp in [96, 128, 160, 192, 224, 256, 288, 320, 384, 512, 640] {
        for rp in [0, 32, 64, 96, 128, 160, 192, 224] {
            document.presets[0].channel.cyclic_prefix_samples = cp;
            document.presets[0].channel.rolloff_samples = rp;
            assert_eq!(document.validate().is_ok(), rp < cp);
        }
    }
    let mut document = ConfigurationDocument::from_json(KHZ50).unwrap();
    for loading in 0..=15 {
        let group = &mut document.presets[0].channel.bursts[0]
            .data_profile
            .as_mut()
            .unwrap()[0];
        group.modulation_index = loading;
        group.pilot_pattern = if loading == 0 { 0 } else { 1 };
        assert_eq!(
            document.validate().is_ok(),
            loading == 0 || (2..=12).contains(&loading)
        );
    }
    for pattern in 0..=15 {
        let group = &mut document.presets[0].channel.bursts[0]
            .data_profile
            .as_mut()
            .unwrap()[0];
        group.modulation_index = 4;
        group.pilot_pattern = pattern;
        // Other IUC13 uses normal family, so boosted 5..7 cannot coexist.
        assert_eq!(document.validate().is_ok(), (1..=4).contains(&pattern));
    }
}
#[test]
fn unknown_fields_nulls_limits_and_failed_replacement_preserve_session() {
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure_json(KHZ50).unwrap();
    let before = session.preview().unwrap();
    assert!(session.configure_json(CONTROLS).is_err());
    assert_eq!(before, session.preview().unwrap());
    let mut value: Value = serde_json::from_slice(KHZ50).unwrap();
    value["unexpected"] = json!(1);
    assert!(session
        .configure_json(&serde_json::to_vec(&value).unwrap())
        .is_err());
    assert_eq!(before, session.preview().unwrap());
    assert!(ConfigurationDocument::from_json(&vec![b' '; 16 * 1024 * 1024 + 1]).is_err());
    let mut value: Value = serde_json::from_slice(KHZ50).unwrap();
    value["presets"][0]["channel"]["excluded_bands"] = json!(vec![json!({"first":0,"last":0}); 64]);
    assert!(ConfigurationDocument::from_json(&serde_json::to_vec(&value).unwrap()).is_err());
    // Every excluded channel/burst table row has no invented JSON equivalent.
    for name in [
        "modulation_rate",
        "frequency",
        "legacy_burst_descriptor",
        "extended_preamble",
        "scdma_mode",
        "scdma_spreading",
        "scdma_codes",
        "scdma_hopping",
        "deprecated_psd",
        "higher_ucd_bitmap",
    ] {
        let mut value: Value = serde_json::from_slice(KHZ50).unwrap();
        value["presets"][0]["channel"][name] = json!(0);
        assert!(
            ConfigurationDocument::from_json(&serde_json::to_vec(&value).unwrap()).is_err(),
            "{name}"
        );
    }
    let mut value: Value = serde_json::from_slice(KHZ50).unwrap();
    value["simulation"]["frames"] = Value::Null;
    assert!(ConfigurationDocument::from_json(&serde_json::to_vec(&value).unwrap()).is_err());
    assert!(session.run().unwrap().reference_recovered);
}

#[test]
fn control_fields_and_conditional_options_are_independently_exercised() {
    let mut doc = ConfigurationDocument::from_json(CONTROLS).unwrap();
    for start in 0..8 {
        let probe = &mut doc.presets[0].probe_map.as_mut().unwrap().elements[1];
        probe.start_subcarrier = start;
        assert_eq!(probe.alternate_power_db().unwrap(), Some(-2 - start as i8));
        assert!(doc.validate().is_ok());
    }
    for bit in 0..32 {
        doc.presets[0].channel.ranging_holdoff_priority = Some(1_u32 << bit);
        assert_eq!(doc.validate().is_ok(), !(5..=15).contains(&bit));
    }
    let doc = ConfigurationDocument::from_json(CONTROLS).unwrap();
    for (pointer, value) in [
        (
            "/presets/0/channel/bursts/2/preamble_length_bits",
            json!(513),
        ),
        (
            "/presets/0/channel/bursts/2/preamble_offset_bits",
            json!(500),
        ),
        (
            "/presets/0/channel/bursts/2/initial_ranging_subcarriers",
            json!(65),
        ),
        (
            "/presets/0/channel/bursts/3/fine_ranging_subcarriers",
            json!(63),
        ),
        (
            "/presets/0/channel/bursts/3/preamble_offset_bits",
            json!(127),
        ),
        ("/presets/0/map/version", json!(1)),
        ("/presets/0/map/upstream_channel_id", json!(2)),
        ("/presets/0/map/ucd_count", json!(1)),
        ("/presets/0/map/number_of_elements", json!(2)),
        ("/presets/0/map/reserved", json!(1)),
        ("/presets/0/map/channel_allocation_type", json!(1)),
        ("/presets/0/map/ranging_backoff_start", json!(16)),
        ("/presets/0/map/ranging_backoff_end", json!(16)),
        ("/presets/0/map/data_backoff_start", json!(16)),
        ("/presets/0/map/data_backoff_end", json!(16)),
        ("/presets/0/map/elements/0/sid", json!(16383)),
        ("/presets/0/map/elements/0/iuc", json!(8)),
        ("/presets/0/map/elements/0/minislot_offset", json!(16384)),
        ("/presets/0/map/elements/1/expansion_words", json!([])),
        ("/presets/0/probe_map/version", json!(1)),
        ("/presets/0/probe_map/ucd_count", json!(1)),
        ("/presets/0/probe_map/number_of_elements", json!(3)),
        ("/presets/0/probe_map/reserved", json!(1)),
        ("/presets/0/probe_map/channel_allocation_type", json!(0)),
        ("/presets/0/probe_map/elements/0/sid", json!(8)),
        ("/presets/0/probe_map/elements/0/frame", json!(4)),
        ("/presets/0/probe_map/elements/0/symbol", json!(6)),
        ("/presets/0/probe_map/elements/0/start_subcarrier", json!(1)),
        ("/presets/0/probe_map/elements/1/start_subcarrier", json!(8)),
        ("/presets/0/probe_map/elements/1/subcarrier_skip", json!(8)),
    ] {
        let mut candidate = serde_json::to_value(&doc).unwrap();
        *candidate.pointer_mut(pointer).unwrap() = value;
        assert!(
            ConfigurationDocument::from_json(&serde_json::to_vec(&candidate).unwrap()).is_err(),
            "{pointer}"
        );
    }
}

#[test]
fn direct_nonfinite_rust_edits_never_export_as_missing_json_values() {
    for bytes in [KHZ50, CONTROLS] {
        let mut document = ConfigurationDocument::from_json(bytes).unwrap();
        document.simulation.reference_variance = f64::NAN;
        assert!(document.to_json().is_err());
        let mut document = ConfigurationDocument::from_json(bytes).unwrap();
        document.simulation.synthetic_rxmer =
            Some(docsis_ofdma::configuration::RxmerSelectionDocument {
                sources: vec![docsis_ofdma::configuration::RxmerDocument {
                    id: 1,
                    mer_db: vec![Some(f64::INFINITY); 2048],
                }],
                policy: "single".into(),
                offsets: None,
            });
        assert!(document.to_json().is_err());
    }
}
