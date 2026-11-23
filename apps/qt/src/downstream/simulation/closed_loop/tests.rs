/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Preserved downstream compatibility oracles through public request execution.

use super::*;
use crate::downstream::workbench::project_file::{decode, encode, Project};
use merlin::downstream::{ChannelJobStatus, FullLoopControl, NamedCapture, ReceiverMode};
use std::time::Duration;

// Independent compatibility fixtures preserve the old 15-bit maximal seed,
// 1000-symbol replay interval and 90000-second capture tolerance. These are
// test inputs, not values derived from production defaults or domain bounds.
fn fixture(profile: Value, rxmer: Value) -> (Inputs, TransmissionSettings, Sections) {
    let sections = Sections::from([
        (
            "source".into(),
            json!({"options":{"mode":"prbs15","seed":32767,
                "append":false,"target_bits":0,"fill_seed":32767},"file":null}),
        ),
        (
            "profile".into(),
            json!({"profile_id":0,"file":encode("dpd.json", &serde_json::to_vec(&profile).unwrap())}),
        ),
        (
            "measurements".into(),
            json!({"rxmer":[encode("rxmer.json", &serde_json::to_vec(&rxmer).unwrap())],
                "response":null,"options":{"mode":"single","interval_symbols":"1000","tolerance_seconds":90000},
                "response_tolerance":90000}),
        ),
        (
            "channel_power".into(),
            json!({"schema_version":1,"conditioning":{
                "amplifier_gain_db":6.0,"cpa_attenuation_db":6.0,
                "input_data_level_dbmv_per_6mhz":0.0}}),
        ),
    ]);
    let project = Project {
        schema_version: 2,
        application: "MERLIN".into(),
        transmission: Default::default(),
        sections: sections.clone(),
        receiver: None,
        comparison_enabled: true,
        dark: true,
    };
    (project.validate().unwrap(), project.transmission, sections)
}
fn original_profile() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../../../../examples/measurements/single-channel/modulation_profile.json"
    ))
    .unwrap()
}
fn original_rxmer() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../../../../examples/measurements/single-channel/rxmer.json"
    ))
    .unwrap()
}
pub(super) fn accepted_inputs() -> (Inputs, TransmissionSettings, Sections) {
    fixture(original_profile(), original_rxmer())
}
#[test]
fn mce_settings_persist_and_actual_rx_ignores_calibrated_reference_variance() {
    let legacy: Options = serde_json::from_str("{}").unwrap();
    assert_eq!(legacy.receiver_mode, ReceiverMode::SuppliedChannel);
    let options = Options {
        receiver_mode: ReceiverMode::MerlinChannelEstimated,
        duration_ms: 1,
        max_iterations: 10,
        ..Default::default()
    };
    let saved = serde_json::to_string(&options).unwrap();
    let restored: Options = serde_json::from_str(&saved).unwrap();
    assert_eq!(restored.receiver_mode, ReceiverMode::MerlinChannelEstimated);
    assert_eq!(restored.mce.averaging_symbols, 16);
    let (inputs, transmission, sections) = accepted_inputs();
    let settings = prepare(&transmission, &inputs, &restored, &sections).unwrap();
    let result = run_one(&settings, 60000, &FullLoopControl::new()).unwrap();
    assert!(matches!(result.job.status, ChannelJobStatus::Complete));
    assert!(result.job.receiver_reliability.as_ref().unwrap().tracking);
    assert_eq!(result.job.summary.comparison.bit_errors, 0);
    let profile = inputs.profile.clone().unwrap();
    crate::downstream::instruments::receiver::qam_history::begin(std::slice::from_ref(&profile));
    crate::downstream::instruments::receiver::qam_history::retain(profile.profile_id, &result);
    let snapshot =
        crate::downstream::instruments::receiver::qam_history::get(i32::from(profile.profile_id))
            .unwrap();
    // Completed public reports are immutable: use a separate real current-mode run.
    let supplied = prepare(
        &transmission,
        &inputs,
        &Options {
            receiver_mode: ReceiverMode::SuppliedChannel,
            ..restored.clone()
        },
        &sections,
    )
    .unwrap();
    let supplied = run_one(&supplied, 60000, &FullLoopControl::new()).unwrap();
    let selected = crate::downstream::instruments::receiver::selection::select_run(
        &supplied,
        Some(&snapshot),
        &Default::default(),
    )
    .unwrap();
    assert_eq!(selected.metadata["correction_mode"], "MCE");
    assert_eq!(
        selected.metadata["correction_name"],
        "MERLIN Channel Estimation (MCE)"
    );
    crate::downstream::instruments::receiver::qam_history::clear();
    publish(Some(Arc::new(result)));
    let response =
        crate::downstream::instruments::receiver::diagnostics::response(r#"{"source":2}"#).unwrap();
    assert_eq!(response["available"], true);
    assert_eq!(response["comparison"]["available"], true);
    assert_eq!(response["mce"]["tracking"], true);
    publish(None);
}
#[test]
fn channel193_rxmer_controls_tx_reference_mer_while_nearest_points_are_optimistic() {
    let bytes = std::fs::read(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/workbench/multi-rxmer-2/workbench.merlin.json"),
    )
    .unwrap();
    let project: crate::downstream::workbench::project_file::Project =
        serde_json::from_slice(&bytes).unwrap();
    let mut inputs = project.validate().unwrap();
    let mut sections = project.sections.clone();
    assert_eq!(inputs.playback.as_ref().unwrap().frames.len(), 132);
    inputs.profile = inputs.profiles.last().cloned();
    let options = Options {
        max_iterations: 5,
        max_runtime_ms: 60000,
        ..project.receiver.unwrap()
    };
    for impaired in [false, true] {
        if impaired {
            // Make explicit test-only decoded captures with fresh original hashes.
            for stored in sections.get_mut("measurements").unwrap()["rxmer"]
                .as_array_mut()
                .unwrap()
            {
                let (name, bytes) = decode(stored).unwrap();
                let capture = merlin::downstream::inputs::CaptureLoader::rxmer(
                    &NamedCapture::new(&name, bytes).unwrap(),
                )
                .unwrap();
                let mut decoded: Value =
                    serde_json::from_slice(capture.provenance().decoded_bytes()).unwrap();
                for value in decoded["values"].as_array_mut().unwrap() {
                    if !value.is_null() {
                        *value = json!(20.0);
                    }
                }
                *stored = encode(&name, &serde_json::to_vec(&decoded).unwrap());
            }
            let edited: Project =
                serde_json::from_value(json!({"schema_version":2,"application":"MERLIN",
                    "transmission":project.transmission,"sections":sections,"receiver":null,
                    "comparison_enabled":true,"dark":true}))
                .unwrap();
            inputs = edited.validate().unwrap();
            inputs.profile = inputs.profiles.last().cloned();
        }
        let settings = prepare(&project.transmission, &inputs, &options, &sections).unwrap();
        let result = run_one(&settings, 60000, &FullLoopControl::new()).unwrap();
        let query = crate::downstream::instruments::receiver::selection::Query::default();
        let selected =
            crate::downstream::instruments::receiver::selection::select(&result, &query).unwrap();
        let raw =
            crate::downstream::instruments::receiver::selection::reference_mer(&selected, true);
        let playback = settings.settings().playback.as_ref().unwrap();
        let residual = result.backend().inputs().measurements.at(0);
        let expected_error = selected
            .cells
            .iter()
            .map(|cell| {
                residual.normalized_power
                    [usize::from(cell.bin - playback.profile.key.grid.first_subcarrier_index)]
                .unwrap()
            })
            .sum::<f64>();
        let expected_raw = 10.0
            * (raw["result"]["signal_power"].as_f64().unwrap().log10() - expected_error.log10());
        assert!((raw["result"]["mer_db"].as_f64().unwrap() - expected_raw).abs() < 0.3);
        let measured =
            crate::downstream::instruments::receiver::selection::reference_mer(&selected, false);
        let estimated = crate::downstream::instruments::receiver::selection::mer(&selected, false);
        let actual = measured["result"]["mer_db"].as_f64().unwrap();
        let nearest = estimated["result"]["mer_db"].as_f64().unwrap();
        eprintln!(
            "channel193 injected20dB={impaired} TX-reference={actual:.3} nearest={nearest:.3}"
        );
        assert!(nearest >= actual - 0.05);
        if impaired {
            assert!((actual - 20.0).abs() < 1.0, "{measured}");
            assert!(nearest > actual + 10.0);
        }
    }
}

#[test]
fn accepted_measurements_run_real_closed_loop_and_invalidate_views() {
    let (inputs, transmission, sections) = accepted_inputs();
    let options = Options {
        duration_ms: 1,
        ..Default::default()
    };
    let settings = prepare(&transmission, &inputs, &options, &sections).unwrap();
    let mut model = Model {
        options: Some(options),
        verify: true,
        request: Some(settings),
        plan: None,
        worker: None,
        timing: None,
        error: String::new(),
        diagnostic: None,
        token: 0,
    };
    model.start();
    let deadline = std::time::Instant::now() + Duration::from_secs(60);
    while model.poll()["busy"] == true {
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(10));
    }
    let timing = model.state()["timing"].clone();
    assert!(timing["elapsed_ms"].as_u64().unwrap() > 0);
    assert_eq!(
        model.state()["timing"],
        timing,
        "Completed elapsed time must be frozen"
    );
    assert_eq!(timing["profiles"].as_array().unwrap().len(), 1);
    let result = latest().expect("Actual result must be retained");
    assert!(
        result.job.summary.comparison.bits_tested > 0,
        "{}",
        model.state()
    );
    assert_eq!(result.job.summary.comparison.bit_errors, 0);
    let source = crate::transmitter_instruments::stage(0, 0);
    assert_eq!(source["available"], true, "{source}");
    assert_eq!(source["rows"][0]["value"].as_str().unwrap().len(), 32);
    let coded = crate::transmitter_instruments::stage(1, 0);
    assert_eq!(coded["total"], 16200);
    let page = crate::transmitter_instruments::stage(1, 2048);
    assert_eq!(page["rows"][0]["index"], 2048);
    assert_eq!(
        crate::transmitter_instruments::stage(1, -1)["available"],
        false
    );
    assert_eq!(
        crate::transmitter_instruments::stage(20, 0)["available"],
        false
    );
    let mapped = crate::transmitter_instruments::stage(4, 0);
    assert_eq!(mapped["available"], true);
    assert!(!mapped["points"].as_array().unwrap().is_empty());
    assert!(mapped["points"].as_array().unwrap().len() <= 2048);
    let bins = crate::transmitter_instruments::stage(6, 0);
    assert_eq!(bins["total"], 8192);
    assert_eq!(source["graph"]["kind"], "bits");
    assert_eq!(coded["graph"]["kind"], "raster");
    assert_eq!(coded["graph"]["points"].as_array().unwrap().len(), 2048);
    assert_eq!(page["graph"]["points"][0]["index"], 2048);
    assert_eq!(mapped["graph"]["kind"], "constellation");
    assert_eq!(bins["graph"]["kind"], "physical");
    assert!(bins["graph"]["points"].as_array().unwrap().len() <= 2048);
    let previews = crate::transmitter_instruments::stage_previews();
    assert_eq!(previews.as_array().unwrap().len(), 6);
    assert_eq!(previews[0]["observation"]["graph"]["kind"], "bits");
    assert!(previews.to_string().len() < 1_500_000);
    assert!(previews[0]["observation"]["rows"].is_null());
    let input = crate::transmitter_instruments::channel_input::qam(-1, -1);
    assert_eq!(input["available"], true, "{input}");
    let point = &input["points"][0];
    let bin = point["index"].as_i64().unwrap() as i32;
    let selected = crate::transmitter_instruments::channel_input::qam(bin, bin);
    assert_eq!(selected["points"].as_array().unwrap().len(), 1);
    assert_eq!(selected["points"][0]["i"], point["i"]);
    assert!(crate::transmitter_instruments::channel_input::qam(4, 3)["available"] == false);
    let before = crate::downstream::instruments::channel::spectrum(0, 0, 0, false);
    let after = crate::downstream::instruments::channel::spectrum(1, 0, 0, false);
    let zoom = crate::downstream::instruments::channel::spectrum(1, 100, 32, true);
    assert!(before["channel_power"]["dbmv"].is_number(), "{before}");
    assert!(
        (after["channel_power"]["dbmv"].as_f64().unwrap()
            - before["channel_power"]["dbmv"].as_f64().unwrap()
            - 6.0)
            .abs()
            < 1e-8
    );
    assert_eq!(after["channel_power"], zoom["channel_power"]);
    let pre = crate::downstream::instruments::channel::qam(0, "{}");
    let post = crate::downstream::instruments::channel::qam(1, "{}");
    assert_eq!(
        pre["points"].as_array().unwrap().len(),
        post["points"].as_array().unwrap().len()
    );
    let gain = 10.0_f64.powf(6.0 / 20.0);
    assert!(
        (post["points"][0]["i"].as_f64().unwrap() - pre["points"][0]["i"].as_f64().unwrap() * gain)
            .abs()
            < 1e-10
    );

    let messages = crate::transmitter_instruments::messages();
    let packets = messages["messages"].as_array().unwrap();
    assert!(packets.iter().any(|p| p["type"] == 49), "{messages}");
    assert!(packets.iter().any(|p| p["profile"] == 0), "{messages}");
    assert!(packets.iter().any(|p| p["profile"] == 255), "{messages}");
    for packet in packets {
        assert_eq!(packet["channel"], 34);
        assert_eq!(packet["completed_frame"], true);
        assert_eq!(packet["status"], "Carried in completed TX PLC frame");
        let bytes: Vec<u8> = packet["hex_rows"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|row| {
                row["hex"]
                    .as_str()
                    .unwrap()
                    .split_whitespace()
                    .map(|b| u8::from_str_radix(b, 16).unwrap())
            })
            .collect();
        let authoritative = result.backend().plc_messages().unwrap();
        let parsed = authoritative
            .messages
            .iter()
            .find(|message| message.packet_bytes == bytes)
            .expect("Exact actual packet must be retained by the public message operation");
        assert_eq!(parsed.descriptor.channel_id, 34);
        for tlv in packet["tlvs"].as_array().unwrap() {
            let offset = tlv["offset"].as_u64().unwrap() as usize;
            assert_eq!(bytes[offset], tlv["kind"].as_u64().unwrap() as u8);
            assert_eq!(bytes[offset + 1], tlv["length"].as_u64().unwrap() as u8);
        }
    }
    for tap in 0..4 {
        assert_eq!(
            crate::transmitter_instruments::spectrum_state(tap)["has_result"],
            true
        );
        let view = crate::transmitter_instruments::spectrum(tap, 0, 0, false);
        assert!(view.get("error").is_none(), "{view}");
        assert!(view["points"].as_array().unwrap().len() <= 2048);
    }
    assert!(!result.observations.taps[0].values.is_empty());
    assert!(result.job.constellation.as_ref().unwrap().collected_pairs > 0);
    let query = crate::downstream::instruments::receiver::selection::Query::default();
    let scatter = crate::downstream::instruments::receiver::selection::qam(&query);
    assert_eq!(scatter["available"], true);
    assert_eq!(
        scatter["raw_mer"]["result"]["sample_count"],
        scatter["selected"]
    );
    assert_eq!(
        scatter["equalized_mer"]["result"]["sample_count"],
        scatter["selected"]
    );
    let channel_qam = crate::downstream::instruments::channel::qam(2, "{}");
    assert_eq!(channel_qam["mer"], scatter["raw_mer"]);
    let control = result
        .job
        .constellation
        .as_ref()
        .unwrap()
        .ncp_pairs
        .first()
        .expect("Actual CRC-classified NCP");
    let ncp_query = crate::downstream::instruments::receiver::selection::Query {
        signal: 1,
        first_bin: i32::from(control.physical_bin),
        last_bin: i32::from(control.physical_bin),
        first_symbol: control.channel_symbol_index as i64,
        last_symbol: control.channel_symbol_index as i64,
        ..Default::default()
    };
    let scatter = crate::downstream::instruments::receiver::selection::qam(&ncp_query);
    assert_eq!(scatter["available"], true);
    assert_eq!(scatter["points"][0]["bin"], control.physical_bin);
    let empty_query = crate::downstream::instruments::receiver::selection::Query {
        first_bin: 8191,
        last_bin: 8191,
        ..Default::default()
    };
    assert_eq!(
        crate::downstream::instruments::receiver::selection::qam(&empty_query)["points"],
        serde_json::json!([])
    );
    let decisions =
        crate::downstream::instruments::receiver::diagnostics::slicer("{}", false).unwrap();
    assert!(!decisions["rows"].as_array().unwrap().is_empty());
    assert!(decisions["llr"]["count"].as_u64().unwrap() > 0);
    let coding = crate::downstream::instruments::receiver::diagnostics::slicer("{}", true).unwrap();
    assert_eq!(coding["llr"]["count"], 16200);
    assert_eq!(
        crate::downstream::instruments::receiver::diagnostics::response("{}").unwrap()["available"],
        true
    );
    assert_eq!(
        crate::downstream::instruments::receiver::diagnostics::response(r#"{"source":1}"#).unwrap()
            ["available"],
        false
    );
    assert_eq!(
        crate::downstream::instruments::receiver::diagnostics::fec()["compared_bits"],
        result.job.summary.comparison.bits_tested
    );
    let charts =
        crate::downstream::instruments::receiver::diagnostics::fec()["verification_charts"].clone();
    assert_eq!(
        charts["recovery"]["points"][0]["count"],
        result.job.summary.recovered_payloads.to_string()
    );
    assert!(charts["word_errors"]["points"].as_array().unwrap().len() <= 64);
    assert!(charts["word_errors"]["points"]
        .as_array()
        .unwrap()
        .iter()
        .all(|point| point["count"] == "0"));
    model.invalidate();
    assert!(model.state()["timing"].is_null());
    assert!(latest().is_none());
    assert!(crate::downstream::instruments::receiver::qam_history::get(0).is_err());
    assert_eq!(
        crate::transmitter_instruments::stage(0, 0)["available"],
        false
    );
    assert!(crate::transmitter_instruments::messages()["messages"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        crate::transmitter_instruments::spectrum_state(3)["has_result"],
        false
    );
    assert_eq!(
        crate::downstream::instruments::receiver::selection::qam(&query)["available"],
        false
    );
}
#[test]
fn independent_sweep_preserves_each_profile_and_legacy_run_options() {
    let mut profile = original_profile();
    let mut rxmer = original_rxmer();
    // Preserve the old independent 1920-carrier fixture, clipping original RLE runs.
    const TEST_SWEEP_CARRIERS: usize = 1920;
    for profile in profile["profiles"].as_array_mut().unwrap() {
        let mut remaining = TEST_SWEEP_CARRIERS;
        let mut schemes = Vec::new();
        for scheme in profile["schemes"].as_array().unwrap() {
            let mut scheme = scheme.clone();
            let count = (scheme["num_subcarriers"].as_u64().unwrap() as usize).min(remaining);
            if count == 0 {
                break;
            }
            scheme["num_subcarriers"] = json!(count);
            schemes.push(scheme);
            remaining -= count;
        }
        assert_eq!(remaining, 0);
        profile["schemes"] = json!(schemes);
    }
    rxmer["values"]
        .as_array_mut()
        .unwrap()
        .truncate(TEST_SWEEP_CARRIERS);
    rxmer["data_length"] = json!(TEST_SWEEP_CARRIERS);
    let (mut inputs, transmission, mut sections) = fixture(profile, rxmer);
    let options = Options {
        all_profiles: true,
        duration_ms: 1,
        max_iterations: 5,
        ..Default::default()
    };
    let plan = prepare_sweep(&transmission, &inputs, &options, &sections).unwrap();
    assert_eq!(
        plan.info()
            .selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    let mut model = Model {
        options: Some(options.clone()),
        verify: true,
        request: Some(prepare(&transmission, &inputs, &options, &sections).unwrap()),
        plan: Some(plan),
        worker: None,
        timing: None,
        error: String::new(),
        diagnostic: None,
        token: 0,
    };
    model.start();
    let deadline = std::time::Instant::now() + Duration::from_secs(180);
    while model.poll()["busy"] == true {
        assert!(std::time::Instant::now() < deadline, "{}", model.state());
        std::thread::sleep(Duration::from_millis(10));
    }
    let view = crate::downstream::instruments::receiver::performance::view(-1, 0, 0);
    assert_eq!(view["profiles"].as_array().unwrap().len(), 4, "{view}");
    assert_eq!(view["profile_id"], 3);
    let timing = model.state()["timing"].clone();
    assert_eq!(timing["profiles"].as_array().unwrap().len(), 4);
    assert_eq!(timing["profiles"][3]["profile_id"], 3);
    assert_eq!(model.state()["timing"], timing);
    for id in 0..4 {
        let view = crate::downstream::instruments::receiver::performance::view(id, 0, 0);
        assert_eq!(view["status"], "complete", "{view}");
        assert!(view["fec"]["total_cw"].as_u64().unwrap() > 0);
        assert_eq!(
            view["fec"]["total_cw"].as_u64().unwrap(),
            view["fec"]["clean_cw"].as_u64().unwrap()
                + view["fec"]["ccw"].as_u64().unwrap()
                + view["fec"]["ufec"].as_u64().unwrap()
        );
        let qam = crate::downstream::instruments::receiver::selection::qam(
            &crate::downstream::instruments::receiver::selection::Query {
                run_profile: id,
                ..Default::default()
            },
        );
        assert_eq!(qam["run_profile"], id, "{qam}");
        assert!(qam["available"].as_bool().unwrap(), "{qam}");
        assert!(
            qam["raw_reference_mer"]["result"]["sample_count"]
                .as_u64()
                .unwrap()
                > 0
        );
        assert_eq!(qam["runs"].as_array().unwrap().len(), 4);
        assert!(qam["retained"].as_u64().unwrap() <= 65536);
        let fec = crate::downstream::instruments::receiver::performance::fec_view(id);
        assert_eq!(fec["profile_id"], id);
        assert_eq!(fec["fec"], view["fec"]);
        assert!(fec["words"].as_array().unwrap().len() <= 64);
        assert!(fec["iterations"]["points"].as_array().unwrap().len() <= 101);
        let recovery_total: u64 = fec["recovery"]["segments"]
            .as_array()
            .unwrap()
            .iter()
            .map(|segment| segment["count"].as_u64().unwrap())
            .sum();
        assert_eq!(recovery_total, fec["generated"].as_u64().unwrap());
        assert!(view["samples"].as_u64().unwrap() > 0);
        assert!(
            view["series"][1]["points"]
                .as_array()
                .unwrap()
                .iter()
                .any(|point| point["minimum_y"] == 1.0),
            "Captured BPSK pilot reference must retain its 0 dB dip"
        );
        assert!(view["series"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| s["points"].as_array().unwrap().len() <= 1024));
        let zoom = crate::downstream::instruments::receiver::performance::view(id, 10, 4);
        assert_eq!(zoom["start"], 10);
        assert_eq!(zoom["count"], 4);
    }
    assert_eq!(
        crate::downstream::instruments::receiver::performance::fec_view(15)["available"],
        false
    );
    let missing = crate::downstream::instruments::receiver::selection::qam(
        &crate::downstream::instruments::receiver::selection::Query {
            run_profile: 15,
            ..Default::default()
        },
    );
    assert_eq!(missing["available"], false);
    assert!(missing["points"].as_array().unwrap().is_empty());
    for id in 0..4 {
        let snapshot = crate::downstream::instruments::receiver::qam_history::get(id).unwrap();
        assert_eq!(i32::from(snapshot.id), id);
        let data = snapshot.data.as_ref().unwrap();
        assert!(data.pairs.len() <= 8192 && data.ncp_pairs.len() <= 1024);
        assert!(data.pairs.iter().all(|pair| pair.reference_iq.is_some()));
    }
    assert_eq!(inputs.profile.as_ref().unwrap().profile_id, 0);
    assert_eq!(
        latest()
            .unwrap()
            .job
            .settings
            .playback
            .as_ref()
            .unwrap()
            .profile
            .profile_id,
        3
    );
    assert!(
        prepare_sweep(
            &transmission,
            &inputs,
            &Options {
                all_profiles: false,
                ..options.clone()
            },
            &sections,
        )
        .unwrap()
        .info()
        .selected_profile_ids
        .len()
            == 1
    );
    let legacy: Options = serde_json::from_str(r#"{"duration_ms":1}"#).unwrap();
    assert!(legacy.all_profiles);
    let saved = serde_json::to_string(&Options {
        all_profiles: false,
        ..options
    })
    .unwrap();
    assert!(
        !serde_json::from_str::<Options>(&saved)
            .unwrap()
            .all_profiles
    );
    inputs.profiles[1].profile_id = 0;
    let (name, bytes) = decode(&sections["profile"]["file"]).unwrap();
    let mut duplicate: Value = serde_json::from_slice(&bytes).unwrap();
    duplicate["profiles"][1]["profile_id"] = duplicate["profiles"][0]["profile_id"].clone();
    sections.get_mut("profile").unwrap()["file"] =
        encode(&name, &serde_json::to_vec(&duplicate).unwrap());
    assert!(prepare_sweep(&transmission, &inputs, &legacy, &sections).is_err());
    model.invalidate();
    assert_eq!(
        crate::downstream::instruments::receiver::performance::view(-1, 0, 0)["available"],
        false
    );
}
#[test]
fn selected_profiles_are_ordered_validated_and_persisted_without_running() {
    let mut profile = original_profile();
    let loading = profile["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|profile| profile["profile_id"] == 0)
        .unwrap()
        .clone();
    profile["profiles"] = Value::Array(
        (0..4)
            .map(|id| {
                let mut value = loading.clone();
                value["profile_id"] = json!(id);
                value
            })
            .collect(),
    );
    let (inputs, transmission, sections) = fixture(profile, original_rxmer());
    let selected = Options {
        selected_profile_ids: Some(vec![3, 1]),
        ..Default::default()
    };
    let plan = prepare_sweep(&transmission, &inputs, &selected, &sections).unwrap();
    assert_eq!(
        plan.info()
            .selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
    // An excluded applied identity must not be eagerly admitted as a run profile.
    let mut excluded_current = inputs.clone();
    excluded_current.profile.as_mut().unwrap().profile_id = 16;
    assert_eq!(
        prepare_sweep(&transmission, &excluded_current, &selected, &sections)
            .unwrap()
            .info()
            .selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![1, 3]
    );
    let restored: Options =
        serde_json::from_slice(&serde_json::to_vec(&selected).unwrap()).unwrap();
    assert_eq!(restored.selected_profile_ids, Some(vec![3, 1]));
    for ids in [vec![], vec![1, 1], vec![9], vec![16]] {
        let options = Options {
            selected_profile_ids: Some(ids),
            ..Default::default()
        };
        assert!(prepare_sweep(&transmission, &inputs, &options, &sections).is_err());
    }
    let applied = prepare_sweep(&transmission, &inputs, &Options::default(), &sections).unwrap();
    assert_eq!(applied.info().selected_profile_ids.len(), 1);
    let legacy: Options = serde_json::from_str(r#"{"duration_ms":3}"#).unwrap();
    assert_eq!(
        prepare_sweep(&transmission, &inputs, &legacy, &sections)
            .unwrap()
            .info()
            .selected_profile_ids
            .len(),
        4
    );
    let partial = Options {
        selected_profile_ids: Some(vec![]),
        ..Default::default()
    };
    partial.validate(&Inputs::default()).unwrap();
    assert!(prepare_sweep(
        &transmission,
        &Inputs::default(),
        &partial,
        &Sections::new()
    )
    .is_err());
}

#[test]
fn accepted_originals_reject_mismatched_seed_metadata_and_capture_identity() {
    let (mut inputs, transmission, mut sections) = accepted_inputs();
    // A distinct valid seed proves pairing, rather than exercising seed-domain rejection.
    inputs.source.as_mut().unwrap().seed = 1;
    assert!(
        prepare(&transmission, &inputs, &Options::default(), &sections)
            .err()
            .unwrap()
            .message
            .contains("source seed")
    );
    inputs.source.as_mut().unwrap().seed = 32767;
    sections.get_mut("measurements").unwrap()["rxmer"][0]["sha256"] = json!("0".repeat(64));
    assert!(prepare(&transmission, &inputs, &Options::default(), &sections).is_err());
    let (mut inputs, transmission, sections) = accepted_inputs();
    inputs.playback.as_mut().unwrap().frames[0].source.sha256 = "unpaired".into();
    assert!(prepare(&transmission, &inputs, &Options::default(), &sections).is_err());
}

#[test]
fn unnamed_file_source_retains_bytes_and_ignores_its_dormant_continuous_seed() {
    let (_, transmission, mut sections) = accepted_inputs();
    let original = b"exact\0payload\n";
    // Zero continuous/fill seeds and target are dormant when finite append is disabled.
    sections.insert(
        "source".into(),
        json!({"file":encode("", original),"options":{
        "mode":"file","seed":0,"append":false,"target_bits":0,"fill_seed":0}}),
    );
    let project = Project {
        schema_version: 2,
        application: "MERLIN".into(),
        transmission: transmission.clone(),
        sections: sections.clone(),
        receiver: None,
        comparison_enabled: true,
        dark: true,
    };
    let inputs = project.validate().unwrap();
    let request = prepare(
        &transmission,
        &inputs,
        &Options {
            duration_ms: 1,
            max_runtime_ms: 60000,
            ..Default::default()
        },
        &sections,
    )
    .unwrap();
    let result = run_one(&request, 60000, &FullLoopControl::new()).unwrap();
    assert_eq!(result.backend().inputs().source.name(), Some(""));
    assert_eq!(
        result.backend().inputs().source.original_bytes(),
        Some(original.as_slice())
    );
    assert_eq!(
        result.backend().recovered_bytes(),
        Some(original.as_slice())
    );
    assert_eq!(result.job.status, ChannelJobStatus::Complete);
    assert_eq!(result.job.summary.comparison.bit_errors, 0);
    assert_eq!(
        result.job.summary.comparison.bits_tested,
        original.len() as u64 * 8
    );
}

#[test]
fn invalidated_paused_worker_never_publishes_a_late_result() {
    let (inputs, transmission, sections) = accepted_inputs();
    let options = Options {
        // A long source window prevents ordinary completion racing this control test.
        duration_ms: 1000,
        max_runtime_ms: 60000,
        ..Default::default()
    };
    let request = prepare(&transmission, &inputs, &options, &sections).unwrap();
    let mut model = Model {
        options: Some(options),
        verify: true,
        request: Some(request),
        plan: None,
        worker: None,
        timing: None,
        error: String::new(),
        diagnostic: None,
        token: 0,
    };
    model.start();
    model.pause();
    assert_eq!(model.state()["paused"], true);
    model.invalidate();
    assert_eq!(model.state()["can_start"], false);
    // Bounded scheduling wait, unrelated to the simulated duration or host budget.
    let deadline = std::time::Instant::now() + Duration::from_secs(15);
    while model.poll()["busy"] == true {
        assert!(std::time::Instant::now() < deadline);
        std::thread::yield_now();
    }
    assert!(latest().is_none());
    assert_eq!(model.state()["has_result"], false);
    assert!(model.state()["timing"].is_null());
    assert!(model.error.is_empty());
}

#[test]
fn response_admission_honors_its_independent_accepted_epoch_tolerance() {
    let (_, transmission, mut sections) = accepted_inputs();
    let mut response: Value = serde_json::from_slice(include_bytes!(
        "../../../../../../examples/measurements/single-channel/channel_estimation.json"
    ))
    .unwrap();
    // Independent fixture: H is 100000 seconds from DPD, beyond the unchanged
    // 90000-second RxMER tolerance. H's own accepted tolerance admits equality.
    const TEST_RESPONSE_DELTA_SECONDS: u32 = 100000;
    let epoch = original_profile()["pnm_header"]["capture_time"]
        .as_u64()
        .unwrap();
    response["pnm_header"]["capture_time"] = json!(epoch - u64::from(TEST_RESPONSE_DELTA_SECONDS));
    sections.get_mut("measurements").unwrap()["response"] =
        encode("response.json", &serde_json::to_vec(&response).unwrap());
    sections.get_mut("measurements").unwrap()["response_tolerance"] =
        json!(TEST_RESPONSE_DELTA_SECONDS);
    let mut project = Project {
        schema_version: 2,
        application: "MERLIN".into(),
        transmission: transmission.clone(),
        sections,
        receiver: None,
        comparison_enabled: true,
        dark: true,
    };
    let inputs = project.validate().unwrap();
    let request = prepare(
        &transmission,
        &inputs,
        &Options::default(),
        &project.sections,
    )
    .unwrap();
    assert_eq!(request.response_tolerance().value(), 100000);
    assert_eq!(
        request
            .settings()
            .playback
            .as_ref()
            .unwrap()
            .tolerance_seconds,
        90000
    );
    project.sections.get_mut("measurements").unwrap()["response_tolerance"] =
        json!(TEST_RESPONSE_DELTA_SECONDS - 1);
    assert!(prepare(
        &transmission,
        &inputs,
        &Options::default(),
        &project.sections
    )
    .is_err());
}
