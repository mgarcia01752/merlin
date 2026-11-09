/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual TS+MC frame recovery through PLC FEC; receiver gets only LLRs and spacing.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::transmission::{
    plc::{ExtendedTimestamp, MessageChannelBlock, PlcBlockCodec, PlcMessageFrame},
    SubcarrierSpacing,
};
#[test]
fn both_frame_sizes_recover_all_information_bytes_and_timestamp_crc() {
    let mut codec = PlcBlockCodec::new().unwrap();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let idle =
            PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(123, 19, 0).unwrap()).unwrap();
        // Opaque MAC segment varies across the frame, avoiding an idle-only test.
        let channel_bytes = idle.geometry().payload_bytes - 15;
        let segment: Vec<_> = (0..channel_bytes)
            .map(|index| ((index * 17 + index / 3) % 256) as u8)
            .collect();
        let original = PlcMessageFrame::new(
            spacing,
            ExtendedTimestamp::new(123, 19, 0).unwrap(),
            MessageChannelBlock::new(&segment, Some(0)).unwrap(),
        )
        .unwrap();
        let original_bytes = original.to_bytes();
        let mut recovered = Vec::new();
        let mut compared_bits = 0;
        for information in original.information_blocks().unwrap() {
            let encoded = codec.encode(&information).unwrap();
            assert_eq!(encoded.mother_bits.len(), 480);
            assert_eq!(encoded.transmitted_bits.len(), 384);
            let llrs: Vec<_> = encoded
                .transmitted_bits
                .iter()
                .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
                .collect();
            let decoded = codec.decode(&llrs, 100).unwrap();
            assert!(decoded.success);
            assert!(decoded.iterations <= 100);
            assert_eq!(decoded.information, information);
            assert_eq!(serde_json::to_value(&decoded).unwrap()["success"], true);
            recovered.extend_from_slice(decoded.information.bytes());
            compared_bits += 288;
        }
        assert_eq!(compared_bits, original.geometry().payload_bytes * 8);
        assert_eq!(recovered, original_bytes);
        assert_eq!(
            PlcMessageFrame::from_bytes(spacing, &recovered).unwrap(),
            original
        );
    }
}
#[test]
fn adapter_returns_structured_diagnostics_and_preserves_failed_decisions() {
    let mut codec = PlcBlockCodec::new().unwrap();
    assert_eq!(
        codec.decode(&[0.0; 383], 10).unwrap_err().code,
        "PlcDecodeFailure"
    );
    assert!(codec.decode(&[0.0; 384], 0).is_err());
    let contradictory: Vec<_> = (0..384)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    let result = codec.decode(&contradictory, 1).unwrap();
    assert!(!result.success);
    assert_eq!(result.information.bytes().len(), 36);
    assert_eq!(result.mother_decisions.len(), 480);
    assert_eq!(result.iterations, 1);
}
