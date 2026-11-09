/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real CLI contract tests for shared CH-EST-RX/protected-stream outputs.
use std::{path::PathBuf, process::Command};
#[test]
fn pilot_receiver_json_matches_shared_rust_and_preserves_no_clobber() {
    let temporary = tempfile::tempdir().unwrap();
    let base =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/channel-constellation.json");
    let mut request: serde_json::Value =
        serde_json::from_slice(&std::fs::read(base).unwrap()).unwrap();
    request["receiver_mode"] = serde_json::json!("pilot_estimated");
    request["protected_constellation"] = serde_json::json!({"pairs_per_stream":16});
    request["constellation"]["requested_pairs"] = serde_json::json!(16);
    request["duration_ms"] = serde_json::json!(1);
    request["data_bit_loading"] = serde_json::json!(vec![8; 4096]);
    request["probes"] = serde_json::json!({
        "selections":[
            {"stage":"rx.channel_estimation.response","elements":{"start":0,"end":16},"channel_symbols":{"start":0,"end":1}},
            {"stage":"tx.payload.interleaved_bits","elements":{"start":0,"end":16}}
        ],
        "max_events":2,"max_elements_per_event":16,"max_total_elements":32
    });
    let input = temporary.path().join("input.json");
    let output = temporary.path().join("result.json");
    let probes = temporary.path().join("probes.jsonl");
    let bytes = serde_json::to_vec(&request).unwrap();
    std::fs::write(&input, &bytes).unwrap();
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_ofdm-fec"))
            .args(["channel", "--input"])
            .arg(&input)
            .arg("--output")
            .arg(&output)
            .arg("--probes-output")
            .arg(&probes)
            .output()
            .unwrap()
    };
    let completed = invoke();
    assert!(
        completed.status.success(),
        "{}",
        String::from_utf8_lossy(&completed.stderr)
    );
    let original = std::fs::read(&output).unwrap();
    let records: Vec<serde_json::Value> = std::fs::read_to_string(&probes)
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(records.len(), 2);
    assert!(records.iter().any(|record| record["event"]["stage"]
        == "rx.channel_estimation.response"
        && record["event"]["units"] == "dimensionless_gain"));
    assert!(records.iter().any(
        |record| record["event"]["stage"] == "tx.payload.interleaved_bits"
            && record["event"]["units"] == "binary"
    ));

    let exported: serde_json::Value = serde_json::from_slice(&original).unwrap();
    assert_eq!(exported["status"], "complete");
    assert_eq!(exported["channel_estimate"]["ready"], true);
    assert_eq!(
        exported["protected_constellation"]["plc_preamble"]["collected_pairs"],
        16
    );
    let document: docsis_ofdm_fec::simulation::channel::document::ChannelDocument =
        serde_json::from_slice(&bytes).unwrap();
    let direct = docsis_ofdm_fec::simulation::channel::run_channel(
        &document.settings().unwrap(),
        &document.limits,
        &std::sync::atomic::AtomicBool::new(false),
        &mut docsis_ofdm_fec::simulation::test_points::TestPointProbe::disabled(),
        &mut docsis_ofdm_fec::simulation::test_points::NoopObserver,
        |_| {},
    )
    .unwrap();
    let direct: serde_json::Value =
        serde_json::from_slice(&serde_json::to_vec(&direct).unwrap()).unwrap();
    assert_eq!(
        exported["protected_constellation"],
        direct["protected_constellation"]
    );
    assert_eq!(exported["channel_estimate"], direct["channel_estimate"]);
    assert_eq!(exported["summary"], direct["summary"]);
    assert!(!invoke().status.success());
    assert_eq!(std::fs::read(&output).unwrap(), original);
}
