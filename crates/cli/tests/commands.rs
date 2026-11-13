/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual standalone CLI round trips and destination protection.
use std::{path::PathBuf, process::Command};
fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/cli")
        .join(name)
}
#[test]
fn evaluates_and_simulates_with_shared_json_contracts() {
    let directory = tempfile::tempdir().unwrap();
    let candidate = directory.path().join("candidate.json");
    let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
        .arg("evaluate")
        .arg("--input")
        .arg(example("scenario.json"))
        .arg("--settings")
        .arg(example("conservative-16qam.json"))
        .args(["--profile", "0", "--output"])
        .arg(&candidate)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let bytes = std::fs::read(&candidate).unwrap();
    let document: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(document["evaluation"]["passing_frame_count"], 3);
    let output = directory.path().join("fec.json");
    let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
        .arg("simulate")
        .arg("--input")
        .arg(example("awgn.json"))
        .arg("--scenario")
        .arg(example("scenario.json"))
        .arg("--candidate")
        .arg(&candidate)
        .arg("--output")
        .arg(&output)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let result: serde_json::Value =
        serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(result["completed_codewords"], 10);
    assert_eq!(result["selected_carrier_offsets"], serde_json::json!([0]));
    let failed = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
        .arg("evaluate")
        .arg("--input")
        .arg(example("scenario.json"))
        .arg("--output")
        .arg(&candidate)
        .output()
        .unwrap();
    assert!(!failed.status.success());
    assert_eq!(std::fs::read(candidate).unwrap(), bytes);
}
