/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Independent capture fixtures, full association and actual scientific execution.
use docsis_ofdma::api::{ExperimentOptions, ReportJson, Simulation};
use pnm_input::upstream::{reassemble_ucd, MeasuredExperiment};
use serde_json::{json, Value};
const PLAN: &[u8] =
    include_bytes!("../../../examples/measurements/upstream/measured/capture-driven.json");
fn candidate() -> Value {
    serde_json::from_slice(PLAN).unwrap()
}
fn admit(value: Value) -> Result<MeasuredExperiment, docsis_ofdma::api::ApiError> {
    MeasuredExperiment::from_json(&serde_json::to_vec(&value).unwrap())
}
#[test]
fn captured_ucd_and_both_pnm_inputs_drive_actual_loop_with_original_provenance() {
    let plan = MeasuredExperiment::from_json(PLAN).unwrap();
    let (config, _) = plan.prepare().unwrap();
    assert_eq!(
        config
            .us_configuration
            .profiles()
            .iter()
            .map(|p| p.id())
            .collect::<Vec<_>>(),
        vec![5, 13]
    );
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    simulation
        .configure_measured(plan.clone().admit().unwrap())
        .unwrap();
    let report = simulation.run().unwrap();
    assert!(report.reference_recovered);
    assert_eq!(report.grants.len(), 1);
    let source: Value =
        serde_json::from_str(report.measurement_provenance_json.as_ref().unwrap()).unwrap();
    assert_eq!(source["original_plan_bytes"], json!(PLAN));
    assert_eq!(source["plan"], serde_json::to_value(&plan).unwrap());
    assert_eq!(source["source_hashes"].as_array().unwrap().len(), 4);
    assert!(report
        .to_json()
        .unwrap()
        .contains("measurement_provenance_json"));
    drop(simulation);
    drop(plan);
    assert!(report.reference_recovered);
}

#[test]
fn captured_25khz_mixed_zero_loading_uses_exact_4k_source_grid() {
    let bytes = include_bytes!(
        "../../../examples/measurements/upstream/measured/capture-driven-25khz.json"
    );
    let plan = MeasuredExperiment::from_json(bytes).unwrap();
    let source = reassemble_ucd(&[plan.ucd_fragments[0].bytes.clone()])
        .unwrap()
        .project_channel(20_000_000)
        .unwrap();
    assert_eq!(source.subcarrier_spacing_khz, 25);
    let groups = source.bursts[0].data_profile.as_ref().unwrap();
    assert_eq!(
        groups
            .iter()
            .map(|g| (g.modulation_index, g.pilot_pattern, g.additional_minislots))
            .collect::<Vec<_>>(),
        vec![(4, 8, 3), (0, 0, 1), (6, 8, 1)]
    );
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure_measured(plan.admit().unwrap()).unwrap();
    assert!(session.run().unwrap().reference_recovered);
}
#[test]
fn every_capture_association_and_late_update_is_admitted_atomically() {
    let original = MeasuredExperiment::from_json(PLAN).unwrap();
    let mut simulation = Simulation::new(ExperimentOptions::default()).unwrap();
    simulation
        .configure_measured(original.admit().unwrap())
        .unwrap();
    let expected = simulation.configuration().unwrap().clone();
    for (pointer, value) in [
        ("/schema_version", json!(2)),
        ("/association/if_index", json!(7)),
        ("/association/adapter_evidence", json!("")),
        ("/association/latest_epoch_seconds", json!(0)),
        ("/association/rpd_port", json!(4)),
        ("/association/cmts_mac/5", json!(3)),
        ("/configuration/presets/0/assignment/cm_mac/5", json!(2)),
        ("/pre_eq_events/0/application_status", json!(3)),
        ("/pre_eq_events/0/base_bank", json!(99)),
        ("/pre_eq_events/0/probe", json!(99)),
        ("/pre_eq_events/0/symbol", json!(0)),
        ("/pre_eq_events/0/operation", json!("snapshot")),
        ("/probes/0/symbol", json!(6)),
        ("/pre_eq_initial/bytes/28", json!(129)),
        ("/rxmer/0/bytes/292", json!(25)),
        ("/rxmer/0/bytes/285", json!(2)),
        ("/rxmer/0/bytes/297", json!(255)),
        ("/rxmer/0/bytes/284", json!(0)),
        ("/rxmer/0/bytes/291", json!(129)),
    ] {
        let mut changed = candidate();
        *changed.pointer_mut(pointer).unwrap() = value;
        let encoded = serde_json::to_vec(&changed).unwrap();
        assert!(
            MeasuredExperiment::from_json(&encoded)
                .and_then(|plan| plan.admit())
                .and_then(|prepared| simulation.configure_measured(prepared))
                .is_err(),
            "{pointer}"
        );
        assert_eq!(simulation.configuration().unwrap(), &expected, "{pointer}");
    }
}
#[test]
fn measured_power_policies_snapshots_init_adjust_and_composition_preflight() {
    for policy in ["average", "hold", "interpolate"] {
        let mut p = candidate();
        let mut source = p["rxmer"][0].clone();
        source["id"] = json!(2);
        // Second literal target=20dB; equal mean power=(.0001+.01)/2=.00505.
        for value in source["bytes"].as_array_mut().unwrap().iter_mut().skip(297) {
            *value = json!(80);
        }
        p["rxmer"].as_array_mut().unwrap().push(source);
        p["rxmer_policy"] = json!(policy);
        if policy != "average" {
            p["rxmer_offsets"] = json!([0, 6]);
        }
        let plan = admit(p).unwrap();
        let (config, _) = plan.prepare().unwrap();
        let playback = config.rxmer.as_ref().unwrap();
        let snapshot = playback
            .at(if policy == "interpolate" { 3 } else { 0 })
            .unwrap();
        let expected = if policy == "hold" { 0.0001 } else { 0.00505 };
        assert!((snapshot.powers[128].unwrap() - expected).abs() < 1e-14);
    }
    let mut p = candidate();
    p["pre_eq_events"][0]["operation"] = json!("initialize");
    assert!(admit(p).is_ok());
    let mut p = candidate();
    p["pre_eq_events"][0]["source"]["bytes"] = p["pre_eq_initial"]["bytes"].clone();
    p["pre_eq_events"][0]["operation"] = json!("snapshot");
    p["pre_eq_events"][0]["base_bank"] = Value::Null;
    p["pre_eq_events"][0]["probe"] = Value::Null;
    assert!(admit(p).is_ok());
    let mut p = candidate();
    p["pre_eq_events"][0]["source"]["bytes"][34] = json!(32); // factor .5 on one bin => nonunity late C.
    assert!(admit(p).is_err());
}

#[test]
fn multiple_updates_keep_corresponding_older_bases_and_cancel_keeps_sources() {
    let mut p = candidate();
    let mut event = p["pre_eq_events"][0].clone();
    event["source"]["id"] = json!(3);
    event["symbol"] = json!(8);
    event["probe"] = json!(11);
    event["source"]["bytes"][9] = json!(105); // next original epoch; base is still explicit bank1.
    p["pre_eq_events"].as_array_mut().unwrap().push(event);
    let mut probe = p["probes"][0].clone();
    probe["id"] = json!(11);
    probe["symbol"] = json!(3);
    p["probes"].as_array_mut().unwrap().push(probe);
    let plan = admit(p.clone()).unwrap();
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure_measured(plan.admit().unwrap()).unwrap();
    assert!(session.run().unwrap().reference_recovered);
    let control = docsis_ofdma::api::RunControl::new();
    control.cancel();
    let cancelled = session.run_with_control(&control).unwrap();
    assert_eq!(cancelled.status, docsis_ofdma::api::RunStatus::Cancelled);
    assert!(cancelled.measurement_provenance_json.is_some());
    p["probes"][1]["symbol"] = json!(7); // At this probe time bank2 is active, so bank1 is stale.
    assert!(admit(p).is_err());
}

#[test]
fn independent_pre_eq_input_does_not_require_invented_ccap_or_rxmer_metadata() {
    let mut p = candidate();
    p["rxmer"] = json!([]);
    p["rxmer_policy"] = json!("none");
    for field in ["ccap_id_raw", "if_index", "rpd_id", "rpd_port"] {
        p["association"][field] = Value::Null;
    }
    let plan = admit(p).unwrap();
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure_measured(plan.admit().unwrap()).unwrap();
    assert!(session.run().unwrap().reference_recovered);
}

#[test]
fn interior_exclusion_zeros_remain_in_sources_and_never_become_update_addresses() {
    let mut p = candidate();
    p["ucd_fragments"] = json!([]);
    let channel = &mut p["configuration"]["presets"][0]["channel"];
    channel["upstream_channel_id"] = json!(7);
    channel["subcarrier_zero_hz"] = json!(12_800_000);
    channel["excluded_bands"]
        .as_array_mut()
        .unwrap()
        .push(json!({"first":136,"last":143}));
    for burst in channel["bursts"].as_array_mut().unwrap() {
        burst["data_profile"][0]["additional_minislots"] = json!(6);
    }
    p["configuration"]["simulation"]["minislots"] = json!([0, 1, 2, 3, 4, 5, 6]);
    for value in p["pre_eq_initial"]["bytes"].as_array_mut().unwrap()[66..98].iter_mut() {
        *value = json!(0);
    }
    let mut update = p["pre_eq_events"][0]["source"]["bytes"].as_array().unwrap()[..34].to_vec();
    update[30..34].clone_from_slice(&[json!(0), json!(0), json!(6), json!(64)]);
    for offset in 0..400 {
        update.extend(if (8..16).contains(&offset) {
            vec![json!(0); 4]
        } else {
            vec![json!(64), json!(0), json!(0), json!(0)]
        });
    }
    p["pre_eq_events"][0]["source"]["bytes"] = json!(update);
    let plan = admit(p.clone()).unwrap();
    let mut session = Simulation::new(ExperimentOptions::default()).unwrap();
    session.configure_measured(plan.admit().unwrap()).unwrap();
    assert!(session.run().unwrap().reference_recovered);
    p["pre_eq_events"][0]["source"]["bytes"][66] = json!(1);
    assert!(admit(p).is_err());
}

#[test]
fn typed_handoff_validates_envelope_and_preserves_original_and_accepted_state() {
    use docsis_ofdma::api::{MeasuredConfiguration, RunControl};
    let plan = MeasuredExperiment::from_json(PLAN).unwrap();
    let (configuration, options) = plan.prepare().unwrap();
    let mut simulation = Simulation::new(options.clone()).unwrap();
    simulation
        .configure_measured(plan.admit().unwrap())
        .unwrap();
    let accepted = simulation.configuration().unwrap().clone();
    for provenance in [
        "{}".to_owned(),
        "{broken".to_owned(),
        json!({"format":"merlin.upstream.measured-provenance","schema_version":2}).to_string(),
        json!({"format":"merlin.downstream.measured-provenance","schema_version":1}).to_string(),
        " ".repeat(16 * 1024 * 1024 + 1),
    ] {
        assert!(MeasuredConfiguration::new(
            configuration.clone(),
            options.clone(),
            plan.configuration.clone(),
            provenance,
        )
        .is_err());
        assert_eq!(simulation.configuration().unwrap(), &accepted);
    }
    let mut invalid_options = options;
    invalid_options.chunk_samples = 0;
    assert!(MeasuredConfiguration::new(
        configuration,
        invalid_options,
        plan.configuration.clone(),
        plan.provenance_json().unwrap(),
    )
    .is_err());
    let control = RunControl::new();
    control.cancel();
    let report = simulation.run_with_control(&control).unwrap();
    assert_eq!(report.configuration, accepted);
    assert_eq!(
        report.source_configuration_json,
        Some(plan.configuration.to_json().unwrap())
    );
    let provenance = report.measurement_provenance_json.clone().unwrap();
    let decoded: Value = serde_json::from_str(&provenance).unwrap();
    assert_eq!(decoded["original_plan_bytes"], json!(PLAN));

    // Replacing measured input with generic input clears only future provenance.
    simulation
        .configure_document(plan.configuration.clone())
        .unwrap();
    let replacement = simulation.run_with_control(&control).unwrap();
    assert!(replacement.measurement_provenance_json.is_none());
    drop(simulation);
    assert_eq!(report.measurement_provenance_json, Some(provenance));
    assert_eq!(report.configuration, accepted);
}

#[test]
fn measured_plan_roundtrip_preserves_admission_and_both_original_representations() {
    let plan = MeasuredExperiment::from_json(PLAN).unwrap();
    let encoded = plan.to_json().unwrap();
    let reloaded = MeasuredExperiment::from_json(encoded.as_bytes()).unwrap();
    assert_eq!(plan.original_bytes(), Some(PLAN));
    assert_eq!(reloaded.original_bytes(), Some(encoded.as_bytes()));
    assert_eq!(
        serde_json::to_value(&plan).unwrap(),
        serde_json::to_value(&reloaded).unwrap()
    );
    assert_eq!(plan.prepare().unwrap(), reloaded.prepare().unwrap());
    let original_provenance: Value =
        serde_json::from_str(&plan.provenance_json().unwrap()).unwrap();
    let reloaded_provenance: Value =
        serde_json::from_str(&reloaded.provenance_json().unwrap()).unwrap();
    assert_eq!(
        original_provenance["source_hashes"],
        reloaded_provenance["source_hashes"]
    );
    assert_eq!(original_provenance["plan"], reloaded_provenance["plan"]);
    assert_eq!(original_provenance["original_plan_bytes"], json!(PLAN));
    assert_eq!(
        reloaded_provenance["original_plan_bytes"],
        json!(encoded.as_bytes())
    );
    assert!(MeasuredExperiment::from_json(&vec![b' '; 16 * 1024 * 1024 + 1]).is_err());
}
