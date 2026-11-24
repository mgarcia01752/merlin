/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Typed draft transport tests, independent of native scientific admission tests.
use super::*;
use merlin::upstream::ExperimentOptions;

const SOURCE: &[u8] = include_bytes!("../../../../../../../examples/cli/upstream/us-50khz.json");
const PLAYBACK: &[u8] =
    include_bytes!("../../../../../../../examples/cli/upstream/us-synthetic-playback.json");

fn simulation(document: ConfigurationDocument) -> Simulation {
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    simulation.configure_document(document).unwrap();
    simulation
}

fn fixture() -> Simulation {
    simulation(ConfigurationDocument::from_json(SOURCE).unwrap())
}

fn key(fields: &Fields, field: &str) -> String {
    fields.presentation["options"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|descriptor| descriptor["field"] == field)
        .unwrap()["key"]
        .as_str()
        .unwrap()
        .into()
}

fn changes(values: impl IntoIterator<Item = (String, Value)>) -> Value {
    Value::Object(values.into_iter().collect())
}

fn assert_rejected_unchanged(simulation: &Simulation, fields: &Fields, changes: Value) {
    let mut document = simulation.document().unwrap().clone();
    let before = serde_json::to_value(&document).unwrap();
    let original = document.original_bytes().unwrap().as_ptr();
    assert!(merge(&mut document, fields, &changes).is_err());
    assert_eq!(serde_json::to_value(&document).unwrap(), before);
    assert_eq!(document.original_bytes().unwrap(), SOURCE);
    assert_eq!(document.original_bytes().unwrap().as_ptr(), original);
}

#[test]
fn every_supported_section_projects_only_string_values_and_unique_frozen_bindings() {
    let simulation = fixture();
    for section in [
        "ucd",
        "profiles",
        "grants",
        "source",
        "receiver",
        "channel",
        "preEqualization",
    ] {
        let fields = build(&simulation, section).unwrap();
        let descriptors = fields.presentation["options"]["fields"].as_array().unwrap();
        assert_eq!(fields.presentation["section"], section);
        assert_eq!(descriptors.len(), fields.bindings.len());
        let mut keys = BTreeSet::new();
        for descriptor in descriptors {
            let key = descriptor["key"].as_str().unwrap();
            assert!(keys.insert(key));
            assert_eq!(descriptor["editable"], true);
            let value = &fields.presentation["values"][key];
            assert!(
                value.is_string()
                    || value
                        .as_array()
                        .is_some_and(|values| values.iter().all(Value::is_string))
            );
            if matches!(descriptor["kind"].as_str(), Some("integer" | "decimal")) {
                assert!(descriptor["max_length"].as_u64().unwrap() > 0);
            }
        }
        if section == "preEqualization" {
            assert!(descriptors.is_empty());
            assert!(fields.presentation["summary"]
                .as_str()
                .unwrap()
                .contains("No explicit synthetic Pre-Eq"));
        } else {
            assert!(!descriptors.is_empty());
        }
    }
    assert!(build(&simulation, "receiver ").is_err());
    let empty = Simulation::new(ExperimentOptions::default()).unwrap();
    assert!(build(&empty, "ucd")
        .unwrap_err()
        .contains("Load an upstream"));
}

#[test]
fn wide_grant_integer_edits_preserve_exact_digits_and_the_original_shared_source() {
    let simulation = fixture();
    let fields = build(&simulation, "grants").unwrap();
    let mut document = simulation.document().unwrap().clone();
    let original_pointer = document.original_bytes().unwrap().as_ptr();
    let original_hash = document.original_sha256();
    let wide_epoch = (1_u64 << 53) + 1;
    merge(
        &mut document,
        &fields,
        &changes([
            (key(&fields, "grant_id"), u64::MAX.to_string().into()),
            (key(&fields, "epoch_sample"), wide_epoch.to_string().into()),
            (
                key(&fields, "first_frame"),
                (u64::MAX - 1).to_string().into(),
            ),
        ]),
    )
    .unwrap();
    assert_eq!(document.simulation.grant_id, u64::MAX);
    assert_eq!(document.simulation.epoch_sample, wide_epoch);
    assert_eq!(document.simulation.first_frame, u64::MAX - 1);
    assert_eq!(document.original_bytes().unwrap(), SOURCE);
    assert_eq!(
        document.original_bytes().unwrap().as_ptr(),
        original_pointer
    );
    assert_eq!(document.original_sha256(), original_hash);
    assert_eq!(document.presets, simulation.document().unwrap().presets);
}

#[test]
fn preset_change_never_retargets_a_channel_edit_to_the_new_selection() {
    let mut document = ConfigurationDocument::from_json(SOURCE).unwrap();
    let mut alternate = document.presets[0].clone();
    alternate.id = "another accepted preset".into();
    document.presets.push(alternate);
    let simulation = simulation(document);
    let fields = build(&simulation, "ucd").unwrap();
    let mut candidate = simulation.document().unwrap().clone();
    let unchanged = candidate.presets[1].channel.cyclic_prefix_samples;
    merge(
        &mut candidate,
        &fields,
        &changes([
            (
                key(&fields, "selected_preset"),
                "another accepted preset".into(),
            ),
            (key(&fields, "cyclic_prefix_samples"), "224".into()),
        ]),
    )
    .unwrap();
    assert_eq!(candidate.selected_preset, "another accepted preset");
    assert_eq!(candidate.presets[0].channel.cyclic_prefix_samples, 224);
    assert_eq!(
        candidate.presets[1].channel.cyclic_prefix_samples,
        unchanged
    );
    assert_eq!(candidate.original_bytes().unwrap(), SOURCE);
}

#[test]
fn profile_keys_keep_their_original_burst_and_group_when_iuc_selection_changes() {
    let mut document = ConfigurationDocument::from_json(SOURCE).unwrap();
    let groups = document.presets[0].channel.bursts[0]
        .data_profile
        .as_mut()
        .unwrap();
    groups[0].additional_minislots = 3;
    groups.push(groups[0].clone());
    let simulation = simulation(document);
    let fields = build(&simulation, "profiles").unwrap();
    let group_index = fields
        .bindings
        .iter()
        .position(|binding| {
            matches!(
                binding,
                Binding::Profile {
                    preset: 0,
                    burst: 0,
                    group: 1,
                    field: ProfileField::Modulation
                }
            )
        })
        .unwrap();
    let mut candidate = simulation.document().unwrap().clone();
    let original_first = candidate.presets[0].channel.bursts[0]
        .data_profile
        .as_ref()
        .unwrap()[0]
        .clone();
    let original_other_iuc = candidate.presets[0].channel.bursts[1].clone();
    merge(
        &mut candidate,
        &fields,
        &changes([
            (key(&fields, "selected_iuc"), "13".into()),
            (format!("field/{group_index}"), "6".into()),
        ]),
    )
    .unwrap();
    assert_eq!(candidate.simulation.selected_iuc, 13);
    let groups = candidate.presets[0].channel.bursts[0]
        .data_profile
        .as_ref()
        .unwrap();
    assert_eq!(groups[0], original_first);
    assert_eq!(groups[1].modulation_index, 6);
    assert_eq!(candidate.presets[0].channel.bursts[1], original_other_iuc);
}

#[test]
fn scheduled_iuc_choices_come_from_actual_profiles_and_the_assignment() {
    let mut document = ConfigurationDocument::from_json(SOURCE).unwrap();
    document.presets[0].assignment.assigned_iucs = vec![5];
    let simulation = simulation(document);
    let fields = build(&simulation, "profiles").unwrap();
    let descriptor = &fields.presentation["options"]["fields"][0];
    assert_eq!(descriptor["field"], "selected_iuc");
    assert_eq!(
        descriptor["choices"],
        json!([{"value":"5","label":"IUC 5"}])
    );
    assert_rejected_unchanged(
        &simulation,
        &fields,
        changes([(key(&fields, "selected_iuc"), "13".into())]),
    );
}

#[test]
fn minislot_choices_use_native_physical_slots_and_never_sort_a_proposed_selection() {
    let simulation = fixture();
    let fields = build(&simulation, "grants").unwrap();
    let key = key(&fields, "minislots");
    let descriptor = fields.presentation["options"]["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|descriptor| descriptor["key"] == key)
        .unwrap();
    let slots = simulation
        .configuration()
        .unwrap()
        .us_configuration
        .selected_profile()
        .minislots();
    assert_eq!(descriptor["choices"].as_array().unwrap().len(), slots.len());
    for (index, slot) in slots.iter().enumerate() {
        assert_eq!(descriptor["choices"][index]["value"], index.to_string());
        assert!(descriptor["choices"][index]["label"]
            .as_str()
            .unwrap()
            .contains(&format!("physical carrier {}", slot.first_carrier)));
    }
    let mut document = simulation.document().unwrap().clone();
    merge(
        &mut document,
        &fields,
        &changes([(key.clone(), json!(["1", "0"]))]),
    )
    .unwrap();
    assert_eq!(document.simulation.minislots, [1, 0]);
    assert!(
        document.prepare().is_err(),
        "Complete native admission owns source ordering constraints"
    );
    for value in [
        json!(["0", "0"]),
        json!(["unknown"]),
        json!([0]),
        json!(vec!["0"; slots.len() + 1]),
    ] {
        assert_rejected_unchanged(&simulation, &fields, changes([(key.clone(), value)]));
    }
}

#[test]
fn source_and_receiver_edits_parse_native_types_but_do_not_replace_scientific_validation() {
    let simulation = fixture();
    let source = build(&simulation, "source").unwrap();
    let mut document = simulation.document().unwrap().clone();
    merge(
        &mut document,
        &source,
        &changes([(key(&source, "source_seed"), "0".into())]),
    )
    .unwrap();
    assert_eq!(document.simulation.source_seed, 0);
    assert!(
        document.prepare().is_err(),
        "The public source validator rejects an inadmissible seed"
    );
    let fields = build(&simulation, "receiver").unwrap();
    let mut document = simulation.document().unwrap().clone();
    merge(
        &mut document,
        &fields,
        &changes([
            (key(&fields, "estimator"), LINEAR_ESTIMATOR.into()),
            (key(&fields, "maximum_pilot_gap"), "17".into()),
            (key(&fields, "reference_variance"), "0.25".into()),
            (key(&fields, "equalizer_power_floor"), "1e-10".into()),
            (key(&fields, "ldpc_iterations"), "12".into()),
        ]),
    )
    .unwrap();
    assert_eq!(document.simulation.estimator, LINEAR_ESTIMATOR);
    assert_eq!(document.simulation.maximum_pilot_gap, 17);
    assert_eq!(document.simulation.reference_variance, 0.25);
    assert_eq!(document.simulation.equalizer_power_floor, 1e-10);
    assert_eq!(document.simulation.ldpc_iterations, 12);
    assert!(document.prepare().is_ok());
    assert_eq!(document.presets, simulation.document().unwrap().presets);
}

#[test]
fn channel_uses_the_resolved_native_defaults_and_only_materializes_them_on_an_edit() {
    let simulation = fixture();
    assert!(simulation.document().unwrap().simulation.channel.is_none());
    let fields = build(&simulation, "channel").unwrap();
    let mut document = simulation.document().unwrap().clone();
    merge(&mut document, &fields, &json!({})).unwrap();
    assert!(document.simulation.channel.is_none());
    let seed_keys: Vec<_> = fields
        .bindings
        .iter()
        .enumerate()
        .filter_map(|(index, binding)| {
            if let Binding::NoiseSeed(byte) = binding {
                Some((*byte, format!("field/{index}")))
            } else {
                None
            }
        })
        .collect();
    let native = &simulation.configuration().unwrap().channel;
    assert_eq!(seed_keys.len(), native.noise_seed.len());
    for (byte, key) in &seed_keys {
        assert_eq!(
            fields.presentation["values"][key],
            native.noise_seed[*byte].to_string()
        );
    }
    merge(
        &mut document,
        &fields,
        &changes([
            (key(&fields, "delay_samples"), "4".into()),
            (key(&fields, "noise_variance"), "0.125".into()),
            (seed_keys[0].1.clone(), "255".into()),
            (seed_keys.last().unwrap().1.clone(), "9".into()),
        ]),
    )
    .unwrap();
    let channel = document.simulation.channel.as_ref().unwrap();
    assert_eq!(channel.delay_samples, 4);
    assert_eq!(channel.noise_variance, 0.125);
    assert_eq!(channel.noise_seed[0], 255);
    assert_eq!(*channel.noise_seed.last().unwrap(), 9);
    assert_eq!(channel.taps, native.taps);
    assert_eq!(channel.phase_radians, native.phase_radians);
    assert_eq!(channel.frequency_offset_hz, native.frequency_offset_hz);
    assert_eq!(document.original_bytes().unwrap(), SOURCE);
    assert!(document.prepare().is_ok());
}

#[test]
fn pre_equalizer_id_edit_preserves_complete_banks_events_probes_and_source_bytes() {
    let simulation = simulation(ConfigurationDocument::from_json(PLAYBACK).unwrap());
    let fields = build(&simulation, "preEqualization").unwrap();
    let mut document = simulation.document().unwrap().clone();
    let original = document
        .simulation
        .synthetic_pre_eq
        .as_ref()
        .unwrap()
        .clone();
    let pointer = document.original_bytes().unwrap().as_ptr();
    let hash = document.original_sha256();
    merge(
        &mut document,
        &fields,
        &changes([(key(&fields, "initial_id"), u64::MAX.to_string().into())]),
    )
    .unwrap();
    let changed = document.simulation.synthetic_pre_eq.as_ref().unwrap();
    assert_eq!(changed.initial_id, u64::MAX);
    assert_eq!(changed.initial, original.initial);
    assert_eq!(changed.events, original.events);
    assert_eq!(changed.probes, original.probes);
    assert_eq!(document.original_bytes().unwrap(), PLAYBACK);
    assert_eq!(document.original_bytes().unwrap().as_ptr(), pointer);
    assert_eq!(document.original_sha256(), hash);
    assert!(
        document.prepare().is_err(),
        "Native lineage validation must still run after the merge"
    );
}

#[test]
fn unknown_wrong_section_or_malformed_changes_leave_the_complete_candidate_unchanged() {
    let simulation = fixture();
    let mut fields = build(&simulation, "ucd").unwrap();
    for value in [
        json!(null),
        json!([]),
        json!({"unknown":"1"}),
        json!({"field/00":"1"}),
        json!({"field/8192":"1"}),
    ] {
        assert_rejected_unchanged(&simulation, &fields, value);
    }
    assert_rejected_unchanged(
        &simulation,
        &fields,
        changes([(
            key(&fields, "selected_preset"),
            "not an inventory entry".into(),
        )]),
    );
    fields.presentation["section"] = "source".into();
    assert_rejected_unchanged(
        &simulation,
        &fields,
        changes([(key(&fields, "frame_symbols"), "6".into())]),
    );
    let grants = build(&simulation, "grants").unwrap();
    for value in [
        json!(1),
        json!("-1"),
        json!("+1"),
        json!("1.0"),
        json!("18446744073709551616"),
    ] {
        assert_rejected_unchanged(
            &simulation,
            &grants,
            changes([(key(&grants, "grant_id"), value)]),
        );
    }
    let source = build(&simulation, "source").unwrap();
    assert_rejected_unchanged(
        &simulation,
        &source,
        changes([(key(&source, "source_seed"), "65536".into())]),
    );
    let receiver = build(&simulation, "receiver").unwrap();
    for value in [
        json!(0.25),
        json!("NaN"),
        json!("inf"),
        json!("1e999"),
        json!("invalid"),
        json!("0".repeat(MAX_DECIMAL_TEXT_CHARACTERS + 1)),
    ] {
        assert_rejected_unchanged(
            &simulation,
            &receiver,
            changes([(key(&receiver, "reference_variance"), value)]),
        );
    }
    assert_rejected_unchanged(
        &simulation,
        &receiver,
        changes([(key(&receiver, "estimator"), "oracle".into())]),
    );
}

#[test]
fn a_late_bad_field_rolls_back_earlier_valid_edits_and_unavailable_indices_do_not_panic() {
    let simulation = fixture();
    let fields = build(&simulation, "grants").unwrap();
    assert_rejected_unchanged(
        &simulation,
        &fields,
        changes([
            (key(&fields, "grant_id"), "123".into()),
            (key(&fields, "frames"), "invalid".into()),
        ]),
    );
    let fields = build(&simulation, "profiles").unwrap();
    let mut document = simulation.document().unwrap().clone();
    document.presets[0].channel.bursts.clear();
    let before = serde_json::to_value(&document).unwrap();
    let profile_key = key(&fields, "modulation_index");
    assert!(merge(
        &mut document,
        &fields,
        &changes([(profile_key, "6".into())])
    )
    .is_err());
    assert_eq!(serde_json::to_value(document).unwrap(), before);
}
