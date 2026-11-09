/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual command wiring, typed JSON/JSONL and non-overwriting destinations.
use std::{path::PathBuf, process::Command};
fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples")
        .join(name)
}
#[test]
fn channel_command_exports_shared_science_and_selected_jsonl_without_clobber() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("result.json");
    let probes = temp.path().join("probes.jsonl");
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["channel", "--input"])
            .arg(example("channel-probes.json"))
            .arg("--output")
            .arg(&output)
            .arg("--probes-output")
            .arg(&probes)
            .output()
            .unwrap()
    };
    let completed = run();
    assert!(
        completed.status.success(),
        "{}",
        String::from_utf8_lossy(&completed.stderr)
    );
    assert!(completed.stdout.is_empty());
    let original = std::fs::read(&output).unwrap();
    let result: serde_json::Value = serde_json::from_slice(&original).unwrap();
    assert_eq!(result["status"], "complete");
    assert_eq!(result["summary"]["comparison"]["bit_errors"], 0);
    assert_eq!(result["summary"]["window"]["ber"], 0.0);
    assert!(
        result["summary"]["window"]["payload_rate_bps"]
            .as_f64()
            .unwrap()
            > 0.0
    );
    let bytes = std::fs::read(&probes).unwrap();
    let events: Vec<serde_json::Value> = bytes
        .split(|byte| *byte == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| serde_json::from_slice(line).unwrap())
        .collect();
    assert_eq!(events.len(), 32);
    assert!(events
        .iter()
        .any(|event| event["event"]["stage"] == "source.payload"));
    assert!(events
        .iter()
        .any(|event| event["event"]["stage"] == "rx.payload.bits"));
    assert!(!run().status.success());
    assert_eq!(std::fs::read(&output).unwrap(), original);
    assert_eq!(std::fs::read(&probes).unwrap(), bytes);
}
#[test]
fn invalid_requests_and_existing_probe_destination_do_not_create_results() {
    let temp = tempfile::tempdir().unwrap();
    let input = temp.path().join("input.json");
    let output = temp.path().join("result.json");
    let probes = temp.path().join("probes.jsonl");
    let base: serde_json::Value =
        serde_json::from_slice(&std::fs::read(example("channel-probes.json")).unwrap()).unwrap();
    for case in [0, 1, 2] {
        let mut request = base.clone();
        if case == 0 {
            request["schema_version"] = 99.into();
        }
        if case == 1 {
            request["limits"]["max_runtime_ms"] = 0.into();
        }
        if case == 2 {
            std::fs::write(&probes, b"keep me").unwrap();
        }
        std::fs::write(&input, serde_json::to_vec(&request).unwrap()).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["channel", "--input"])
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .arg("--probes-output")
            .arg(&probes)
            .output()
            .unwrap();
        assert!(!result.status.success());
        assert!(!output.exists());
        if case < 2 {
            assert!(!probes.exists());
        } else {
            assert_eq!(std::fs::read(&probes).unwrap(), b"keep me");
        }
    }
}

#[test]
fn channel_rejects_explicit_frame_option_including_zero() {
    let temp = tempfile::tempdir().unwrap();
    let output = temp.path().join("result.json");
    let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
        .args(["channel", "--input"])
        .arg(example("channel.json"))
        .arg("--output")
        .arg(&output)
        .args(["--frame", "0"])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(!output.exists());
}
