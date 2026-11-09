/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use std::{
    path::{Path, PathBuf},
    process::Command,
};
fn fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .unwrap()
        .join("examples/measurements/single-channel")
}
#[test]
fn actual_import_command_hashes_exact_json_and_refuses_clobber_and_bad_epochs() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("import.json");
    let output = directory.path().join("scenario.json");
    let request = serde_json::json!({"schema_version":1,"protocol":"docsis31","rxmer":fixtures().join("rxmer.json"),
        "channel_estimation":fixtures().join("channel_estimation.json"),"modulation_profile":fixtures().join("modulation_profile.json"),"tolerance_seconds":90000});
    std::fs::write(&input, serde_json::to_vec(&request).unwrap()).unwrap();
    let run = |destination: &Path| {
        Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["import-pnm", "--input"])
            .arg(&input)
            .arg("--output")
            .arg(destination)
            .output()
            .unwrap()
    };
    let success = run(&output);
    assert!(
        success.status.success(),
        "{}",
        String::from_utf8_lossy(&success.stderr)
    );
    assert!(success.stdout.is_empty());
    let original = std::fs::read(&output).unwrap();
    let scenario: serde_json::Value = serde_json::from_slice(&original).unwrap();
    assert_eq!(
        scenario["frames"][0]["mer_db"].as_array().unwrap().len(),
        7480
    );
    assert!(!run(&output).status.success());
    assert_eq!(std::fs::read(&output).unwrap(), original);
    let mut strict = request;
    strict["tolerance_seconds"] = serde_json::json!(0);
    std::fs::write(&input, serde_json::to_vec(&strict).unwrap()).unwrap();
    let rejected = directory.path().join("rejected.json");
    assert!(!run(&rejected).status.success());
    assert!(!rejected.exists());
}
