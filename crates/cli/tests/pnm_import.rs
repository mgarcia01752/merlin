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

#[test]
fn contract_request_version_two_admits_known_exports_and_rejects_unknown_producers() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("import.json");
    let output = directory.path().join("scenario.json");
    let mut request = serde_json::json!({"schema_version":2,"decoder_contract":{"revision":1,"producer":"pnm-decode","producer_version":"0.3.6"},"protocol":"docsis31",
        "rxmer":fixtures().join("rxmer.json"),"channel_estimation":fixtures().join("channel_estimation.json"),"modulation_profile":fixtures().join("modulation_profile.json"),"tolerance_seconds":90000});
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["import-pnm", "--input"])
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap()
    };
    std::fs::write(&input, serde_json::to_vec(&request).unwrap()).unwrap();
    assert!(run().status.success());
    std::fs::remove_file(&output).unwrap();
    for (schema, producer) in [(2, "0.3.7"), (1, "0.3.6")] {
        request["schema_version"] = serde_json::json!(schema);
        request["decoder_contract"]["producer_version"] = serde_json::json!(producer);
        std::fs::write(&input, serde_json::to_vec(&request).unwrap()).unwrap();
        assert!(!run().status.success());
        assert!(!output.exists());
    }
    request["schema_version"] = serde_json::json!(2);
    request.as_object_mut().unwrap().remove("decoder_contract");
    std::fs::write(&input, serde_json::to_vec(&request).unwrap()).unwrap();
    assert!(!run().status.success());
    assert!(!output.exists());
}

#[test]
fn synthetic_widths_load_with_actual_import_and_channel_commands() {
    let directory = tempfile::tempdir().unwrap();
    let root = fixtures().parent().unwrap().join("channel-widths");
    for width in [24, 90] {
        let input = root.join(format!("{width}mhz"));
        let output = directory.path().join(format!("scenario-{width}.json"));
        let imported = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["import-pnm", "--input"])
            .arg(input.join("import.json"))
            .arg("--output")
            .arg(&output)
            .output()
            .unwrap();
        assert!(
            imported.status.success(),
            "{}",
            String::from_utf8_lossy(&imported.stderr)
        );
        let scenario: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
        assert_eq!(scenario["profiles"].as_array().unwrap().len(), 4);
        for profile in 0..4 {
            let result = directory
                .path()
                .join(format!("result-{width}-{profile}.json"));
            let executed = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
                .args(["channel", "--input"])
                .arg(input.join(format!("channel-profile-{profile}.json")))
                .arg("--output")
                .arg(&result)
                .output()
                .unwrap();
            assert!(
                executed.status.success(),
                "width={width},profile={profile}: {}",
                String::from_utf8_lossy(&executed.stderr)
            );
            let report: serde_json::Value =
                serde_json::from_slice(&std::fs::read(result).unwrap()).unwrap();
            assert_eq!(report["status"], "complete");
            assert_eq!(report["summary"]["comparison"]["bit_errors"], 0);
            assert!(
                report["summary"]["comparison"]["bits_tested"]
                    .as_u64()
                    .unwrap()
                    > 0
            );
        }
    }
}
