/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual channel command exposes the same bounded scientific pairs, never CLI calculations.
use std::{path::PathBuf, process::Command};
fn input() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/cli/channel-constellation.json")
}
#[test]
fn paired_channel_cli_exports_default_count_and_preserves_existing_destinations() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("result.json");
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["channel", "--input"])
            .arg(input())
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap()
    };
    let completed = invoke();
    assert!(
        completed.status.success(),
        "{}",
        String::from_utf8_lossy(&completed.stderr)
    );
    assert!(completed.stdout.is_empty());
    let original = std::fs::read(&output).unwrap();
    let result: serde_json::Value = serde_json::from_slice(&original).unwrap();
    assert_eq!(result["status"], "complete");
    assert_eq!(result["constellation"]["status"], "complete");
    assert_eq!(result["constellation"]["collected_pairs"], 8192);
    assert_eq!(
        result["constellation"]["pairs"].as_array().unwrap().len(),
        8192
    );
    let config: docsis_ofdm_fec::simulation::channel::document::ChannelDocument =
        serde_json::from_slice(&std::fs::read(input()).unwrap()).unwrap();
    let direct = docsis_ofdm_fec::simulation::channel::run_channel(
        &config.settings().unwrap(),
        &config.limits,
        &std::sync::atomic::AtomicBool::new(false),
        &mut docsis_ofdm_fec::simulation::test_points::TestPointProbe::disabled(),
        &mut docsis_ofdm_fec::simulation::test_points::NoopObserver,
        |_| {},
    )
    .unwrap();
    // Compare both through the same JSON floating-point parse boundary.
    let expected: serde_json::Value =
        serde_json::from_slice(&serde_json::to_vec(&direct.constellation).unwrap()).unwrap();
    assert!(
        result["constellation"] == expected,
        "CLI and serialized shared Rust result differ"
    );
    assert!(!invoke().status.success());
    assert_eq!(std::fs::read(&output).unwrap(), original);
}
#[test]
fn invalid_capture_configuration_is_rejected_before_export() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("result.json");
    let request_path = temp.path().join("request.json");
    let base: serde_json::Value = serde_json::from_slice(&std::fs::read(input()).unwrap()).unwrap();
    for invalid in [
        serde_json::json!({"requested_pairs":0}),
        serde_json::json!({"profile_id":3}),
        serde_json::json!({"start_hz":1}),
        serde_json::json!({"invented":true}),
    ] {
        let mut request = base.clone();
        request["constellation"] = invalid;
        std::fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["channel", "--input"])
            .arg(&request_path)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!output.exists());
    }
}
