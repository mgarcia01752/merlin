/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    scenario::pnm_json::{import, JsonCapture},
    simulation::channel::measurement::captured_layout,
    transmission::TransmissionSettings,
    types::ProtocolScope,
};
use std::path::{Path, PathBuf};
fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("examples/measurements/single-channel")
        .join(name)
}
fn load(name: &str) -> Vec<u8> {
    std::fs::read(fixture(name)).unwrap()
}
fn decode(
    rxmer: &[u8],
    channel: &[u8],
    profile: &[u8],
    tolerance: u32,
) -> Result<docsis_ofdm_fec::scenario::document::ScenarioDocument, docsis_ofdm_fec::LabError> {
    import(
        JsonCapture {
            name: "rxmer.json",
            bytes: rxmer,
        },
        JsonCapture {
            name: "channel.json",
            bytes: channel,
        },
        JsonCapture {
            name: "profile.json",
            bytes: profile,
        },
        ProtocolScope::Docsis31,
        tolerance,
    )
}
#[test]
fn imports_real_current_decoder_shapes_hashes_and_exact_grid() {
    let rxmer = load("rxmer.json");
    let channel = load("channel_estimation.json");
    let profile = load("modulation_profile.json");
    let scenario = decode(&rxmer, &channel, &profile, 90_000).unwrap();
    assert_eq!(scenario.frames[0].mer_db.len(), 7480);
    assert_eq!(scenario.channels[0].iq.len(), 7480);
    assert_eq!(scenario.profiles.len(), 4);
    assert_eq!(
        scenario.frames[0].key.grid.frequency(0).unwrap(),
        640_000_000
    );
    let provenance: serde_json::Value = serde_json::from_slice(&load("provenance.json")).unwrap();
    assert_eq!(
        scenario.frames[0].source.sha256,
        provenance["sources"][0]["decoded_sha256"].as_str().unwrap()
    );
    assert_eq!(scenario.frames[0].capture_time, 1380970);
    assert_eq!(scenario.channels[0].capture_time, 1391100);
    assert!(decode(&rxmer, &channel, &profile, 0).is_err());
    let profile = scenario
        .profiles
        .iter()
        .find(|profile| profile.profile_id == 0)
        .unwrap();
    let layout = captured_layout(profile, &TransmissionSettings::default()).unwrap();
    assert_eq!(layout.first_active_bin, 356);
    assert_eq!(layout.active_end_exclusive, 7836);
    let mut settings = docsis_ofdm_fec::simulation::channel::ChannelRunSettings::new(
        TransmissionSettings::default(),
        layout,
    );
    let mut selection = docsis_ofdm_fec::simulation::channel::measurement::MeasurementSelection {
        frame_index: 0,
        profile_id: 3,
        channel_index: Some(0),
        tolerance_seconds: 90000,
        noise_seed: 41,
    };
    assert_eq!(
        selection.apply(&scenario, &mut settings).unwrap_err().code,
        "UnsupportedMeasuredProfile"
    );
    selection.profile_id = 0;
    selection.apply(&scenario, &mut settings).unwrap();
    assert!(selection.apply(&scenario, &mut settings).is_err()); // no second noise source
}
#[test]
fn rejects_grid_encoding_coverage_drift_and_preserves_missing_readings() {
    let mut rxmer: serde_json::Value = serde_json::from_slice(&load("rxmer.json")).unwrap();
    rxmer["values"][0] = serde_json::Value::Null;
    let channel = load("channel_estimation.json");
    let profile = load("modulation_profile.json");
    let source = serde_json::to_vec(&rxmer).unwrap();
    assert_eq!(
        decode(&source, &channel, &profile, 90_000).unwrap().frames[0].mer_db[0],
        None
    );
    for field in [
        "subcarrier_zero_frequency",
        "subcarrier_spacing",
        "first_active_subcarrier_index",
        "channel_id",
    ] {
        let mut bad: serde_json::Value = serde_json::from_slice(&channel).unwrap();
        bad[field] = serde_json::json!(1);
        assert!(decode(
            &source,
            &serde_json::to_vec(&bad).unwrap(),
            &profile,
            90_000
        )
        .is_err());
    }
    let mut bad: serde_json::Value = serde_json::from_slice(&profile).unwrap();
    bad["profiles"][0]["schemes"][0]["bit_loading"] = serde_json::json!(8);
    assert!(decode(
        &source,
        &channel,
        &serde_json::to_vec(&bad).unwrap(),
        90_000
    )
    .is_err());
    bad["profiles"][0]["schemes"][0]["bit_loading"] = serde_json::json!(12);
    bad["profiles"][0]["schemes"][0]["num_subcarriers"] = serde_json::json!(0);
    assert!(decode(
        &source,
        &channel,
        &serde_json::to_vec(&bad).unwrap(),
        90_000
    )
    .is_err());
}
