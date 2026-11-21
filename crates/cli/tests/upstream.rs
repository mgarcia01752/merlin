/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual directional CLI processes, canonical Rust parity and protected exports.
use merlin::upstream::{ChannelConfig, ExperimentOptions, Iq, Simulation};
use pnm_input::upstream::{MeasuredExperiment, PreEqCapture, RxmerCapture, UcdCapture};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn fixture(path: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(path)
}

fn command(operation: &str, input: &Path, output: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"));
    command
        .args(["upstream", operation, "--input"])
        .arg(input)
        .arg("--output")
        .arg(output)
        .env_remove("RUST_LOG");
    command
}

fn successful(result: Output, output: &Path) -> Value {
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stdout.is_empty());
    serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap()
}

fn compare_science(actual: &Value, expected: &Value) {
    for key in [
        "format",
        "schema_version",
        "configuration",
        "options",
        "planned",
        "status",
        "source_configuration_json",
        "source_configuration_original",
        "source_configuration_sha256",
        "measurement_provenance_json",
        "generated_source_bits",
        "compared_source_bits",
        "bit_errors",
        "ber",
        "reference_recovered",
        "not_started_grants",
    ] {
        assert_eq!(actual[key], expected[key], "{key}");
    }
    let actual_grants = actual["grants"].as_array().unwrap();
    let expected_grants = expected["grants"].as_array().unwrap();
    assert_eq!(actual_grants.len(), expected_grants.len());
    for (observed, reference) in actual_grants.iter().zip(expected_grants) {
        for (key, value) in observed.as_object().unwrap() {
            if key != "identity" {
                assert_eq!(value, &reference[key], "grant {key}");
            }
        }
        // Session/run counters are process-relative; the physical grant stays exact.
        assert_eq!(
            observed["identity"]["grant"],
            reference["identity"]["grant"]
        );
    }
}

fn generic_parity(name: &str) {
    let input = fixture(&format!("cli/upstream/{name}"));
    let bytes = std::fs::read(&input).unwrap();
    let mut direct = Simulation::new(ExperimentOptions::default()).unwrap();
    direct.configure_json(&bytes).unwrap();
    let expected = serde_json::to_value(direct.run().unwrap()).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let output = directory.path().join("report.json");
    let actual = successful(command("run", &input, &output).output().unwrap(), &output);
    assert_eq!(actual["reference_recovered"], true);
    compare_science(&actual, &expected);
    assert_eq!(std::fs::read(&input).unwrap(), bytes);
}

#[test]
fn generic_50khz_actual_cli_matches_direct_rust() {
    generic_parity("us-50khz.json");
}

#[test]
fn generic_25khz_actual_cli_matches_direct_rust() {
    generic_parity("us-25khz.json");
}

#[test]
fn preview_exports_only_the_idle_rust_plan_for_both_numerologies() {
    for name in ["us-50khz.json", "us-25khz.json"] {
        let input = fixture(&format!("cli/upstream/{name}"));
        let mut direct = Simulation::new(ExperimentOptions::default()).unwrap();
        direct
            .configure_json(&std::fs::read(&input).unwrap())
            .unwrap();
        let expected = serde_json::to_value(direct.preview().unwrap()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("preview.json");
        let actual = successful(
            command("preview", &input, &output).output().unwrap(),
            &output,
        );
        assert_eq!(actual, expected);
        assert!(actual.get("generated_source_bits").is_none());
        assert!(actual.get("grants").is_none());
    }
}

#[test]
fn measured_plans_preserve_original_provenance_and_both_rust_loops() {
    for name in ["capture-driven.json", "capture-driven-25khz.json"] {
        let input = fixture(&format!("measurements/upstream/measured/{name}"));
        let bytes = std::fs::read(&input).unwrap();
        let prepared = MeasuredExperiment::from_json(&bytes)
            .unwrap()
            .admit()
            .unwrap();
        let mut direct = Simulation::new(ExperimentOptions::default()).unwrap();
        direct.configure_measured(prepared).unwrap();
        let expected = serde_json::to_value(direct.run().unwrap()).unwrap();
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("measured.json");
        let actual = successful(
            command("measured", &input, &output).output().unwrap(),
            &output,
        );
        assert!(actual["measurement_provenance_json"].is_string());
        assert_eq!(actual["reference_recovered"], true);
        compare_science(&actual, &expected);
        assert_eq!(std::fs::read(&input).unwrap(), bytes);
    }
}

#[test]
fn capture_inspection_uses_exact_decoded_rust_documents_and_originals() {
    for (operation, name) in [
        ("ucd", "ucd/scheduled-50khz.hex"),
        ("rxmer", "rxmer/ccap-50khz.hex"),
        ("rxmer", "rxmer/ccap-25khz.hex"),
        ("pre-eq", "pre-eq/applied.hex"),
        ("pre-eq", "pre-eq/last-update.hex"),
    ] {
        let source =
            std::fs::read_to_string(fixture(&format!("measurements/upstream/{name}"))).unwrap();
        let hex = source.split_whitespace().collect::<String>();
        let bytes = hex
            .as_bytes()
            .chunks_exact(2)
            .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
            .collect::<Vec<_>>();
        let expected = match operation {
            "ucd" => UcdCapture::from_mac_management(&bytes)
                .unwrap()
                .to_json()
                .unwrap(),
            "rxmer" => RxmerCapture::from_pnm(&bytes).unwrap().to_json().unwrap(),
            _ => PreEqCapture::from_pnm(&bytes).unwrap().to_json().unwrap(),
        };
        let directory = tempfile::tempdir().unwrap();
        let input = directory.path().join("original.bin");
        std::fs::write(&input, &bytes).unwrap();
        let output = directory.path().join("decoded.json");
        let actual = successful(
            command(operation, &input, &output).output().unwrap(),
            &output,
        );
        assert_eq!(actual, serde_json::from_str::<Value>(&expected).unwrap());
        assert_eq!(std::fs::read(&input).unwrap(), bytes);
        std::fs::write(&input, &bytes[..bytes.len() - 1]).unwrap();
        let rejected = directory.path().join("truncated.json");
        assert!(!command(operation, &input, &rejected)
            .output()
            .unwrap()
            .status
            .success());
        assert!(!rejected.exists());
    }
}

#[test]
fn invalid_commands_schemas_limits_and_existing_destinations_are_rejected() {
    let directory = tempfile::tempdir().unwrap();
    let input = fixture("cli/upstream/us-50khz.json");
    let output = directory.path().join("existing.json");
    std::fs::write(&output, b"caller-owned destination").unwrap();
    assert!(!command("run", &input, &output)
        .output()
        .unwrap()
        .status
        .success());
    assert_eq!(std::fs::read(&output).unwrap(), b"caller-owned destination");
    let invalid = directory.path().join("invalid.json");
    std::fs::write(&invalid, b"{\"format\":\"wrong\",\"schema_version\":999}").unwrap();
    let rejected = directory.path().join("rejected.json");
    for operation in ["run", "measured", "unknown"] {
        assert!(!command(operation, &invalid, &rejected)
            .output()
            .unwrap()
            .status
            .success());
        assert!(!rejected.exists());
    }
    for option in [
        "--scenario",
        "--candidate",
        "--settings",
        "--profile",
        "--frame",
    ] {
        let result = command("preview", &input, &rejected)
            .args([option, "0"])
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!rejected.exists());
    }
    let oversized = directory.path().join("oversized.json");
    std::fs::File::create(&oversized)
        .unwrap()
        .set_len(16 * 1024 * 1024 + 1)
        .unwrap();
    let result = command("run", &oversized, &rejected).output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("Input exceeds16MiB"));
    assert!(!rejected.exists());
}

#[test]
fn bounded_output_rejects_large_preserved_originals_before_destination_creation() {
    let directory = tempfile::tempdir().unwrap();
    // Valid original whitespace is provenance, not a reason to discard bytes.
    // Its pretty-printed byte array exceeds the CLI's documented output budget.
    let mut original = vec![b' '; 2 * 1024 * 1024];
    original.extend(std::fs::read(fixture("cli/upstream/us-50khz.json")).unwrap());
    let input = directory.path().join("large-original.json");
    std::fs::write(&input, &original).unwrap();
    let output = directory.path().join("bounded-report.json");
    let result = command("run", &input, &output).output().unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8(result.stderr)
        .unwrap()
        .contains("Output exceeds16MiB"));
    assert!(!output.exists());
    assert_eq!(std::fs::read(input).unwrap(), original);
}

#[test]
fn failed_source_recovery_retains_completed_decoder_outcomes_and_nonzero_exit() {
    let directory = tempfile::tempdir().unwrap();
    let mut document: Value =
        serde_json::from_slice(&std::fs::read(fixture("cli/upstream/us-50khz.json")).unwrap())
            .unwrap();
    let channel = ChannelConfig {
        taps: vec![Iq::default()],
        ..ChannelConfig::default()
    };
    document["simulation"]["channel"] = serde_json::to_value(channel).unwrap();
    let input = directory.path().join("null-channel.json");
    std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
    let output = directory.path().join("failed-report.json");
    let result = command("run", &input, &output).output().unwrap();
    assert!(!result.status.success());
    let report: Value = serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    // The engine completed its observations; this does not mean source recovery.
    assert_eq!(report["status"], "Completed");
    assert_eq!(report["reference_recovered"], false);
    assert!(report["failure"].is_null());
    assert!(report["generated_source_bits"].as_u64().unwrap() > 0);
    assert!(report["bit_errors"].as_u64().unwrap() > 0);
    assert!(!report["grants"][0]["words"].as_array().unwrap().is_empty());
}

#[cfg(unix)]
#[test]
fn sigint_reaches_the_existing_handler_and_retains_a_cancelled_report() {
    use std::{
        process::Stdio,
        time::{Duration, Instant},
    };
    let directory = tempfile::tempdir().unwrap();
    let mut document: Value =
        serde_json::from_slice(&std::fs::read(fixture("cli/upstream/us-50khz.json")).unwrap())
            .unwrap();
    document["simulation"]["grants"] = 256.into();
    document["simulation"]["chunk_samples"] = 127.into();
    document["simulation"]["retained_points"] = 0.into();
    document["simulation"]["retained_taps"] = serde_json::json!([]);
    let input = directory.path().join("long.json");
    std::fs::write(&input, serde_json::to_vec(&document).unwrap()).unwrap();
    let output = directory.path().join("cancelled.json");
    let stderr = directory.path().join("stderr.log");
    let mut child = command("run", &input, &output)
        .stdout(Stdio::null())
        .stderr(std::fs::File::create(&stderr).unwrap())
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    // command-start occurs after the one existing signal handler is installed.
    while !std::fs::read_to_string(&stderr)
        .unwrap()
        .contains("command-start")
    {
        if Instant::now() >= deadline || child.try_wait().unwrap().is_some() {
            let _ = child.kill();
            let _ = child.wait();
            panic!("CLI did not reach its signal-ready dispatch boundary");
        }
        std::thread::sleep(Duration::from_millis(2));
    }
    assert!(Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap()
        .success());
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("CLI did not finish after cooperative Ctrl-C");
        }
        std::thread::sleep(Duration::from_millis(2));
    };
    assert!(!status.success());
    let report: Value = serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(report["status"], "Cancelled");
    assert_eq!(report["reference_recovered"], false);
    assert!(report["source_configuration_original"].is_array());
}
