/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Saved settings retain their original meanings through public full-loop admission.
use base64::{engine::general_purpose::STANDARD, Engine};
use docsis_ofdm_fec::{api::*, simulation::source::PayloadSourceConfig};
use pnm_input::Workbench;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

const SINGLE: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/single/workbench.merlin.json");
const MULTI: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/multi-rxmer/workbench.merlin.json");
const PROFILE_SELECTION_WB: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json");
const DESIGNER_WB: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/cl-192mhz-8mhz-notch/cmce.merlin.json");
const RECIPE: &[u8] = include_bytes!("fixtures/designer-flat.json");
// Original bytes exercise high/low bits and a zero octet; no text/filename conversion.
const FILE_BYTES: &[u8] = &[0x81, 0x00, 0x5a];
// A finite 64-bit file-plus-fill fixture, distinct from any codec or retention bound.
const FILE_TARGET_BITS: u64 = 64;
// Independent PRBS fill recurrence seed; the dormant continuous-source seed is different.
const FILE_FILL_SEED: u16 = 7;

fn document(bytes: &[u8]) -> Value {
    let mut value: Value = serde_json::from_slice(bytes).unwrap();
    value["receiver"]["all_profiles"] = json!(false);
    value
}
fn prepare(value: &Value) -> ApiResult<FullLoopSweepPlan> {
    Workbench::prepare_full_loop(&serde_json::to_vec(value).unwrap())
}
fn stored(name: &str, bytes: &[u8]) -> Value {
    json!({"name":name,"byte_length":bytes.len(),"sha256":format!("{:x}",Sha256::digest(bytes)),"base64":STANDARD.encode(bytes)})
}

#[test]
fn legacy_and_selected_projects_preserve_idle_settings_and_original_inputs() {
    let value = document(SINGLE);
    let plan = prepare(&value).unwrap();
    assert_eq!(plan.info().selected_profile_ids.len(), 1);
    let request = plan.request(0).unwrap();
    let old = Workbench::from_bytes(&serde_json::to_vec(&value).unwrap()).unwrap();
    let expected = old.controllers()[0].prepare().unwrap();
    assert_eq!(
        request.settings().transmission,
        expected[0].engine_settings().transmission
    );
    assert_eq!(request.options().duration_ms, 1);
    assert_eq!(request.options().noise_seed, 41);
    assert_eq!(
        request.options().receiver_mode,
        ReceiverMode::SuppliedChannel
    );
    assert_eq!(
        request.response_tolerance().value(),
        value["sections"]["measurements"]["response_tolerance"]
            .as_u64()
            .unwrap() as u32
    );
    assert!(request.observation_policy().is_none());
    let control = FullLoopControl::new();
    control.cancel();
    let result = request.run(&control).unwrap();
    assert_eq!(result.report().summary.comparison.bits_tested, 0);
    assert_eq!(result.inputs().summary().unwrap().source_kind, "prbs15");
    let provenance = result.inputs().summary().unwrap().profile_sources;
    let original = STANDARD
        .decode(
            value["sections"]["profile"]["file"]["base64"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
    assert_eq!(provenance[0].original_bytes(), original);
    let mut selected = document(PROFILE_SELECTION_WB);
    selected["schema_version"] = json!(2);
    selected["receiver"]["selected_profile_ids"] = json!([4, 0]);
    selected["receiver"]["receiver_mode"] = json!("merlin_estimated");
    selected["receiver"]["mce"] = json!({"averaging_symbols":32,"minimum_variance":1e-7});
    selected["receiver"]["channel_filter"] = json!({"mode":"echoes","paths":[
        {"delay_samples":0,"gain":[1.0,0.0]}, {"delay_samples":10,"gain":[0.1,0.0]}
    ]});
    selected["sections"]["channel_model"] =
        json!({"schema_version":1,"additional_loss":{"mode":"manual","tilt_db":-2.0}});
    selected["sections"]["channel_power"] =
        json!({"schema_version":1,"conditioning":{"amplifier_gain_db":3.0}});
    let selected_plan = prepare(&selected).unwrap();
    let configured = selected_plan.request(0).unwrap();
    assert_eq!(
        configured.options().receiver_mode,
        ReceiverMode::MerlinChannelEstimated
    );
    assert_eq!(configured.options().mce.averaging_symbols, 32);
    assert_eq!(
        configured.settings().impairments.power.amplifier_gain_db,
        3.0
    );
    assert_eq!(
        serde_json::to_value(&configured.settings().channel_filter).unwrap()["mode"],
        "echoes"
    );
    assert_eq!(
        serde_json::to_value(&configured.settings().impairments.additional_loss).unwrap()
            ["tilt_db"],
        -2.0
    );
    assert_eq!(
        prepare(&selected)
            .unwrap()
            .info()
            .selected_profile_ids
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        [0, 4]
    );
}

#[test]
fn saved_response_and_rxmer_epoch_policies_remain_independent() {
    // Separate original H/profile and RxMER/profile tolerances, measured in seconds.
    const RESPONSE_DISTANCE_SECONDS: u32 = 17;
    let mut value = document(SINGLE);
    let profile_bytes = STANDARD
        .decode(
            value["sections"]["profile"]["file"]["base64"]
                .as_str()
                .unwrap(),
        )
        .unwrap();
    let profile = pnm_input::CaptureLoader::profile(
        &NamedCapture::new("profile", profile_bytes).unwrap(),
        ProfileId::new(value["sections"]["profile"]["profile_id"].as_u64().unwrap() as u8).unwrap(),
    )
    .unwrap();
    let epoch = profile.sources()[0].original_epoch().value();
    let measurements = &mut value["sections"]["measurements"];
    let rxmer_bytes = STANDARD
        .decode(measurements["rxmer"][0]["base64"].as_str().unwrap())
        .unwrap();
    let rxmer =
        pnm_input::CaptureLoader::rxmer(&NamedCapture::new("RxMER", rxmer_bytes).unwrap()).unwrap();
    let mut rxmer: Value = serde_json::from_slice(rxmer.provenance().decoded_bytes()).unwrap();
    rxmer["pnm_header"]["capture_time"] = json!(epoch);
    measurements["rxmer"][0] = stored("same-epoch RxMER", &serde_json::to_vec(&rxmer).unwrap());
    let response_bytes = STANDARD
        .decode(measurements["response"]["base64"].as_str().unwrap())
        .unwrap();
    let response = pnm_input::CaptureLoader::channel_estimate(
        &NamedCapture::new("H", response_bytes).unwrap(),
    )
    .unwrap();
    let mut response: Value =
        serde_json::from_slice(response.provenance().decoded_bytes()).unwrap();
    response["pnm_header"]["capture_time"] = json!(epoch + RESPONSE_DISTANCE_SECONDS);
    measurements["response"] = stored("later H", &serde_json::to_vec(&response).unwrap());
    measurements["response_tolerance"] = json!(RESPONSE_DISTANCE_SECONDS);
    measurements["options"]["tolerance_seconds"] = json!(0);
    let request = prepare(&value).unwrap().request(0).unwrap();
    assert_eq!(
        request.response_tolerance().value(),
        RESPONSE_DISTANCE_SECONDS
    );
    assert_eq!(
        request
            .settings()
            .playback
            .as_ref()
            .unwrap()
            .tolerance_seconds,
        0
    );
    value["sections"]["measurements"]["response_tolerance"] = json!(RESPONSE_DISTANCE_SECONDS - 1);
    assert_eq!(
        prepare(&value).err().unwrap().code(),
        ErrorCode::AssociationFailed
    );
}

#[test]
fn unnamed_saved_file_preserves_original_label_bytes_fill_and_capture_name_validation() {
    let mut value = document(SINGLE);
    value["sections"]["source"]["file"] = stored("", FILE_BYTES);
    value["sections"]["source"]["options"]["mode"] = json!("file");
    value["sections"]["source"]["options"]["append"] = json!(true);
    value["sections"]["source"]["options"]["target_bits"] = json!(FILE_TARGET_BITS);
    value["sections"]["source"]["options"]["fill_seed"] = json!(FILE_FILL_SEED);
    value["sections"]["source"]["options"]["seed"] = json!(0); // Dormant continuous seed.
    let result = prepare(&value)
        .unwrap()
        .request(0)
        .unwrap()
        .run(&FullLoopControl::new())
        .unwrap();
    assert_eq!(result.inputs().source.name(), Some(""));
    assert_eq!(result.inputs().source.original_bytes(), Some(FILE_BYTES));
    assert_eq!(result.inputs().source.target_bits(), Some(FILE_TARGET_BITS));
    assert_eq!(
        result.inputs().source.original_sha256(),
        value["sections"]["source"]["file"]["sha256"].as_str()
    );
    assert_eq!(result.recovered_bytes(), Some(FILE_BYTES));
    let mut invalid = value.clone();
    invalid["sections"]["profile"]["file"]["name"] = json!("");
    assert!(prepare(&invalid).is_err());
    invalid = value.clone();
    invalid["sections"]["source"]["file"]["sha256"] = json!("invalid");
    assert!(prepare(&invalid).is_err());
    invalid = value.clone();
    // Independent one-past original saved display-name bound (1024 bytes).
    invalid["sections"]["source"]["file"]["name"] = json!("x".repeat(1025));
    assert!(prepare(&invalid).is_err());
}

#[test]
fn file_and_fill_keep_exact_bytes_and_dormant_fields_do_not_replace_selection() {
    let mut value = document(SINGLE);
    value["sections"]["source"]["file"] = stored("opaque.bin", FILE_BYTES);
    value["sections"]["source"]["options"]["mode"] = json!("file");
    value["sections"]["source"]["options"]["append"] = json!(true);
    value["sections"]["source"]["options"]["target_bits"] = json!(FILE_TARGET_BITS);
    value["sections"]["source"]["options"]["fill_seed"] = json!(FILE_FILL_SEED);
    value["sections"]["source"]["options"]["seed"] = json!(0);
    let request = prepare(&value).unwrap().request(0).unwrap();
    let PayloadSourceConfig::File { payload } = &request.settings().source else {
        panic!("Saved file choice must remain a file source");
    };
    assert_eq!(payload.bytes(), FILE_BYTES);
    assert_eq!(payload.target_bits(), FILE_TARGET_BITS);
    let metadata = serde_json::to_value(payload).unwrap();
    assert_eq!(metadata["fill"]["seed"], FILE_FILL_SEED);
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(result.recovered_bytes(), Some(FILE_BYTES));
    assert_eq!(result.inputs().source.original_bytes(), Some(FILE_BYTES));
    assert_eq!(
        Workbench::from_bytes(&serde_json::to_vec(&value).unwrap())
            .err()
            .unwrap()
            .code(),
        ErrorCode::UnsupportedCapability
    );
    value["sections"]["source"]["options"]["target_bits"] = json!(1);
    assert!(prepare(&value).is_err());
}

#[test]
fn timeline_uses_stored_selection_order_and_exact_synthetic_clock_capacity() {
    for mode in ["hold", "interpolate"] {
        let mut value = document(MULTI);
        let originals = value["sections"]["measurements"]["rxmer"]
            .as_array()
            .unwrap()
            .clone();
        let last = originals.len() - 1;
        value["sections"]["measurements"]["options"]["selected_captures"] =
            json!([originals[last]["sha256"], originals[0]["sha256"]]);
        value["sections"]["measurements"]["options"]["mode"] = json!(mode);
        value["sections"]["measurements"]["options"]["interval_symbols"] = json!("1");
        let request = prepare(&value).unwrap().request(0).unwrap();
        let playback = request.settings().playback.as_ref().unwrap();
        assert_eq!(playback.frames.len(), 2);
        assert_eq!(playback.interval_symbols, 1);
        for (frame, source) in playback
            .frames
            .iter()
            .zip([&originals[0], &originals[last]])
        {
            let bytes = STANDARD.decode(source["base64"].as_str().unwrap()).unwrap();
            let capture = pnm_input::CaptureLoader::rxmer(
                &NamedCapture::new("expected original", bytes).unwrap(),
            )
            .unwrap();
            // Frame identity describes decoded data; provenance separately retains binary bytes.
            assert_eq!(
                capture.provenance().original_sha256(),
                source["sha256"].as_str().unwrap()
            );
            assert_eq!(frame.source.sha256, capture.frame().source.sha256);
        }
        value["sections"]["measurements"]["options"]["interval_symbols"] =
            json!(u64::MAX.to_string());
        assert!(
            prepare(&value).is_err(),
            "Second capture cannot be reached in the saved duration"
        );
        // The original legacy loader keeps its documented unsupported timeline result.
        assert_eq!(
            Workbench::from_bytes(&serde_json::to_vec(&value).unwrap())
                .err()
                .unwrap()
                .code(),
            ErrorCode::UnsupportedCapability
        );
    }
}

#[test]
fn designer_recipe_reuses_shared_materialization_without_rewriting_original() {
    let mut value = document(DESIGNER_WB);
    value["sections"]["measurements"]["rxmer"] = json!([stored("Designer", RECIPE)]);
    let bytes = serde_json::to_vec(&value).unwrap();
    let old = Workbench::from_bytes(&bytes).unwrap();
    let prepared = old.controllers()[0].prepare().unwrap();
    let docsis_ofdm_fec::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } =
        &prepared[0].engine_settings().impairments.noise
    else {
        panic!("Legacy Designer fixture must resolve an RxMER observation");
    };
    let request = Workbench::prepare_full_loop(&bytes)
        .unwrap()
        .request(0)
        .unwrap();
    assert_eq!(
        request.settings().playback.as_ref().unwrap().frames[0].mer_db,
        frame.mer_db
    );
    assert_eq!(old.original_document().as_ref(), bytes);
}

#[test]
fn corrupted_unselected_originals_unknown_fields_and_budget_fail_before_adoption() {
    let accepted = prepare(&document(SINGLE)).unwrap();
    let before = serde_json::to_value(accepted.request(0).unwrap().settings()).unwrap();
    let mut value = document(MULTI);
    let first = value["sections"]["measurements"]["rxmer"][0]["sha256"].clone();
    value["sections"]["measurements"]["options"]["selected_captures"] = json!([first]);
    value["sections"]["measurements"]["rxmer"][1]["sha256"] = json!("bad");
    assert_eq!(
        prepare(&value).err().unwrap().code(),
        ErrorCode::InvalidInput
    );
    let mut value = document(SINGLE);
    value["sections"]["unexpected"] = json!({});
    assert_eq!(
        prepare(&value).err().unwrap().code(),
        ErrorCode::InvalidInput
    );
    let mut value = document(SINGLE);
    value["sections"]["source"]["file"] = stored("bad", &[]);
    value["sections"]["source"]["file"]["byte_length"] = json!(MAX_CAPTURE_BYTES + 1);
    assert_eq!(
        prepare(&value).err().unwrap().code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(
        serde_json::to_value(accepted.request(0).unwrap().settings()).unwrap(),
        before
    );
}

#[test]
fn actual_saved_full_loop_preserves_fec_and_independent_recovered_source() {
    let started = std::time::Instant::now();
    let plan = prepare(&document(SINGLE)).unwrap();
    let request = plan.request(0).unwrap();
    let result = request.run(&FullLoopControl::new()).unwrap();
    assert_eq!(result.report().status, ChannelJobStatus::Complete);
    assert!(result.report().summary.comparison.bits_tested > 0);
    assert_eq!(result.report().summary.comparison.bit_errors, 0);
    assert_eq!(result.report().summary.payload_ldpc_failures, 0);
    assert_eq!(result.report().summary.payload_bch_failures, 0);
    println!(
        "Saved full-loop actual run: {:.3}s",
        started.elapsed().as_secs_f64()
    );
}
