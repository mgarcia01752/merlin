/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};
fn request() -> serde_json::Value {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/channel-pilot-estimated.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    value["duration_ms"] = serde_json::json!(1);
    value["source"] = serde_json::json!({"kind":"file","fill":{"target_bits":20000,"seed":1}});
    value.as_object_mut().unwrap().remove("constellation");
    value
        .as_object_mut()
        .unwrap()
        .remove("protected_constellation");
    value
}
fn invoke(input: &Path, output: &Path, extra: &[&Path]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"));
    command
        .arg("channel")
        .arg("--input")
        .arg(input)
        .arg("--output")
        .arg(output);
    if let Some(path) = extra.first() {
        command.arg("--payload-file").arg(path);
    }
    if let Some(path) = extra.get(1) {
        command.arg("--recovered-file").arg(path);
    }
    command.output().unwrap()
}
#[test]
fn actual_cli_recovers_text_and_binary_and_matches_shared_core_without_leaking_bytes() {
    for bytes in [
        b"\xef\xbb\xbfMERLIN\r\n\x00\xff\x81".to_vec(),
        (0..256).map(|value| value as u8).collect(),
    ] {
        let temporary = tempfile::tempdir().unwrap();
        let input = temporary.path().join("request.json");
        let source = temporary.path().join("data.any-extension");
        let output = temporary.path().join("result.json");
        let recovered = temporary.path().join("recovered.bin");
        let value = request();
        std::fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
        std::fs::write(&source, &bytes).unwrap();
        let completed = invoke(&input, &output, &[&source, &recovered]);
        assert!(
            completed.status.success(),
            "{}",
            String::from_utf8_lossy(&completed.stderr)
        );
        assert_eq!(std::fs::read(&recovered).unwrap(), bytes);
        let exported: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
        assert_eq!(exported["summary"]["comparison"]["bits_tested"], 20000);
        assert_eq!(
            exported["summary"]["file_source"]["file"]["bits_tested"],
            bytes.len() * 8
        );
        assert_eq!(exported["summary"]["file_source"]["fully_compared"], true);
        assert!(exported["settings"]["source"]["payload"]
            .get("bytes")
            .is_none());
        assert!(exported["recovered_file"].get("bytes").is_none());
        let document: docsis_ofdm_fec::simulation::channel::document::ChannelDocument =
            serde_json::from_value(value).unwrap();
        let direct = docsis_ofdm_fec::simulation::channel::run_channel(
            &document.settings_with_inputs(None, Some(&bytes)).unwrap(),
            &document.limits,
            &std::sync::atomic::AtomicBool::new(false),
            &mut docsis_ofdm_fec::simulation::test_points::TestPointProbe::disabled(),
            &mut docsis_ofdm_fec::simulation::test_points::NoopObserver,
            |_| {},
        )
        .unwrap();
        assert_eq!(
            serde_json::to_value(direct.summary).unwrap(),
            exported["summary"]
        );
        assert!(!invoke(&input, &output, &[&source, &recovered])
            .status
            .success());
        assert_eq!(std::fs::read(&recovered).unwrap(), bytes);
        let other_output = temporary.path().join("other.json");
        assert!(!invoke(&input, &other_output, &[&source, &recovered])
            .status
            .success());
        assert!(!other_output.exists());
    }
}
#[test]
fn cli_rejects_source_mismatch_targets_seeds_and_oversized_files_before_creating_output() {
    let temporary = tempfile::tempdir().unwrap();
    let input = temporary.path().join("request.json");
    let source = temporary.path().join("payload.bin");
    let output = temporary.path().join("result.json");
    std::fs::write(&source, [1, 2, 3]).unwrap();
    for selection in [
        serde_json::json!({"kind":"prbs15"}),
        serde_json::json!({"kind":"file","fill":{"target_bits":23,"seed":1}}),
        serde_json::json!({"kind":"file","fill":{"target_bits":100,"seed":0}}),
        serde_json::json!({"kind":"file","fill":{"target_bits":134217729,"seed":1}}),
        serde_json::json!({"kind":"file","unknown":true}),
    ] {
        let mut value = request();
        value["source"] = selection;
        std::fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(!invoke(&input, &output, &[&source]).status.success());
        assert!(!output.exists());
    }
    std::fs::write(&input, serde_json::to_vec(&request()).unwrap()).unwrap();
    assert!(!invoke(&input, &output, &[]).status.success());
    assert!(!output.exists());
    std::fs::File::create(&source)
        .unwrap()
        .set_len(16 * 1024 * 1024 + 1)
        .unwrap();
    assert!(!invoke(&input, &output, &[&source]).status.success());
    assert!(!output.exists());
    let result = Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
        .args(["simulate", "--input"])
        .arg(&input)
        .arg("--output")
        .arg(&output)
        .arg("--payload-file")
        .arg(&source)
        .output()
        .unwrap();
    assert!(!result.status.success());
}
