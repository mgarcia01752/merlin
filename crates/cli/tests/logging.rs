/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Spawned-process checks isolate the global logger and environment filters.
use std::{
    path::Path,
    process::{Command, Output},
};
fn run_simulation(input: &Path, output: &Path, filter: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"));
    command
        .arg("simulate")
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(output)
        .env_remove("RUST_LOG");
    if let Some(filter) = filter {
        command.env("RUST_LOG", filter);
    }
    command.output().unwrap()
}
#[test]
fn terminal_errors_log_once_to_stderr_and_filters_do_not_change_exit_status() {
    let directory = tempfile::tempdir().unwrap();
    let missing = directory.path().join("missing.json");
    let output = directory.path().join("result.json");
    let failed = run_simulation(&missing, &output, None);
    assert!(!failed.status.success());
    assert!(failed.stdout.is_empty());
    let stderr = String::from_utf8(failed.stderr).unwrap();
    assert_eq!(stderr.matches("[ERROR").count(), 1);
    assert!(stderr.contains("command-failed"));
    assert!(stderr.contains("json-input-open-failed"));
    assert!(!output.exists());
    let silent = run_simulation(&missing, &output, Some("off"));
    assert!(!silent.status.success());
    assert!(silent.stdout.is_empty());
    assert!(silent.stderr.is_empty());
}
#[test]
fn filters_preserve_json_results_and_existing_destination_protection() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("settings.json");
    let example = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/awgn.json");
    let mut settings: serde_json::Value =
        serde_json::from_slice(&std::fs::read(example).unwrap()).unwrap();
    settings["codewords"] = 2.into();
    settings["noiseless"] = true.into();
    settings["max_runtime_ms"] = 100_000.into();
    std::fs::write(&input, serde_json::to_vec(&settings).unwrap()).unwrap();
    let mut results = Vec::new();
    for (name, filter) in [
        ("default", None),
        ("debug", Some("debug")),
        ("off", Some("off")),
        ("module", Some("error,docsis_ofdm_fec::simulation=debug")),
    ] {
        let path = directory.path().join(format!("{name}.json"));
        let output = run_simulation(&input, &path, filter);
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        match name {
            "default" => {
                assert!(stderr.contains("[INFO"));
                assert!(!stderr.contains("simulation-progress"));
            }
            "debug" => {
                assert!(stderr.contains("[DEBUG"));
                assert!(stderr.contains("simulation-progress"));
            }
            "off" => assert!(stderr.is_empty()),
            _ => {
                assert!(stderr.contains("simulation-finished"));
                assert!(!stderr.contains("command-start"));
            }
        }
        let mut result: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        // Logging may change wall-clock duration; scientific values must match.
        result.as_object_mut().unwrap().remove("elapsed_ms");
        results.push(result);
        let before = std::fs::read(&path).unwrap();
        let rejected = run_simulation(&input, &path, Some("error"));
        assert!(!rejected.status.success());
        assert!(rejected.stdout.is_empty());
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }
    assert!(results.windows(2).all(|pair| pair[0] == pair[1]));
}
#[test]
fn help_and_version_stay_on_stdout_without_log_noise() {
    for argument in ["--help", "--version"] {
        let output = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .arg(argument)
            .env("RUST_LOG", "trace")
            .output()
            .unwrap();
        assert!(output.status.success());
        assert!(!output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }
}
