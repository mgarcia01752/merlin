/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    scenario::pnm_json::{import_with_contract, DecoderContract, JsonCapture},
    types::ProtocolScope,
};
use serde_json::json;
use std::path::Path;

fn contract() -> DecoderContract {
    DecoderContract {
        revision: 1,
        producer: "pnm-decode".into(),
        producer_version: "0.3.6".into(),
    }
}
#[test]
fn unknown_contract_rejects_before_parsing_and_unknown_envelope_fields_reject() {
    for bad in [
        json!({"revision":2,"producer":"pnm-decode","producer_version":"0.3.6"}),
        json!({"revision":1,"producer":"other","producer_version":"0.3.6"}),
        json!({"revision":1,"producer":"pnm-decode","producer_version":"0.3.7"}),
    ] {
        let contract: DecoderContract = serde_json::from_value(bad).unwrap();
        let error = import_with_contract(
            &contract,
            JsonCapture {
                name: "r",
                bytes: b"bad",
            },
            JsonCapture {
                name: "c",
                bytes: b"bad",
            },
            JsonCapture {
                name: "p",
                bytes: b"bad",
            },
            ProtocolScope::Docsis31,
            0,
        )
        .unwrap_err();
        assert_eq!(error.code, "UnsupportedDecoderContract");
    }
    assert!(serde_json::from_value::<DecoderContract>(
        json!({"revision":1,"producer":"pnm-decode","producer_version":"0.3.6","future":true})
    )
    .is_err());
}
#[test]
fn actual_fixture_contract_allows_additive_fields_but_rejects_required_field_drift() {
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/measurements/single-channel");
    let mut rxmer: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("rxmer.json")).unwrap()).unwrap();
    let channel = std::fs::read(root.join("channel_estimation.json")).unwrap();
    let profiles = std::fs::read(root.join("modulation_profile.json")).unwrap();
    let import = |bytes: &[u8]| {
        import_with_contract(
            &contract(),
            JsonCapture {
                name: "rxmer.json",
                bytes,
            },
            JsonCapture {
                name: "channel.json",
                bytes: &channel,
            },
            JsonCapture {
                name: "profiles.json",
                bytes: &profiles,
            },
            ProtocolScope::Docsis31,
            90_000,
        )
    };
    rxmer["new_display_only_field"] = json!(42);
    assert_eq!(
        import(&serde_json::to_vec(&rxmer).unwrap()).unwrap().frames[0]
            .mer_db
            .len(),
        7480
    );
    rxmer.as_object_mut().unwrap().remove("value_units");
    assert!(import(&serde_json::to_vec(&rxmer).unwrap()).is_err());
    rxmer["value_units"] = json!("linear");
    assert!(import(&serde_json::to_vec(&rxmer).unwrap()).is_err());
}
