/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Valid TS+MC frame subset, exact shared FEC boundaries and rejection paths.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::transmission::{
    plc::{ExtendedTimestamp, MessageChannelBlock, PlcMessageFrame},
    SubcarrierSpacing,
};
#[test]
fn idle_frame_has_one_timestamp_and_exact_capacity_at_both_spacings() {
    for (spacing, capacity, words) in [
        (SubcarrierSpacing::Khz25, 720, 20),
        (SubcarrierSpacing::Khz50, 360, 10),
    ] {
        let frame =
            PlcMessageFrame::idle(spacing, ExtendedTimestamp::new(0, 0, 0).unwrap()).unwrap();
        let bytes = frame.to_bytes();
        assert_eq!(bytes.len(), capacity);
        assert_eq!(bytes[0], 0x10);
        assert_eq!(&bytes[1..9], &[0; 8]);
        assert_eq!(bytes[12], 0x30);
        assert!(bytes[13..].iter().all(|byte| *byte == 0xff));
        assert_eq!(PlcMessageFrame::from_bytes(spacing, &bytes).unwrap(), frame);
        let blocks = frame.information_blocks().unwrap();
        assert_eq!(blocks.len(), words);
        assert_eq!(
            blocks
                .iter()
                .flat_map(|block| block.bytes())
                .copied()
                .collect::<Vec<_>>(),
            bytes
        );
        assert_eq!(blocks[0].bytes()[12], 0x30); // TS and MC share a FEC input word.
        assert!(blocks[1..]
            .iter()
            .all(|block| block.bits().iter().all(|bit| *bit == 1)));
    }
}
#[test]
fn packet_fragment_frames_preserve_pointer_and_source_bytes() {
    let timestamp = ExtendedTimestamp::from_sample_ticks(204_800_000, 0).unwrap();
    let source = vec![0xaa; 345]; //360 minus TS12, MC header1 and pointer2.
    let mc = MessageChannelBlock::new(&source, Some(344)).unwrap();
    let frame = PlcMessageFrame::new(SubcarrierSpacing::Khz50, timestamp, mc).unwrap();
    let bytes = frame.to_bytes();
    assert_eq!(&bytes[12..15], &[0x31, 1, 88]);
    assert_eq!(&bytes[15..], source);
    assert_eq!(
        PlcMessageFrame::from_bytes(SubcarrierSpacing::Khz50, &bytes).unwrap(),
        frame
    );
    assert_eq!(
        serde_json::to_value(&frame).unwrap()["message_channel"]["packet_start_pointer"],
        344
    );
}
#[test]
fn invalid_capacity_crc_and_unsupported_blocks_do_not_produce_a_frame() {
    let timestamp = ExtendedTimestamp::new(0, 0, 0).unwrap();
    let frame = PlcMessageFrame::idle(SubcarrierSpacing::Khz50, timestamp).unwrap();
    let bytes = frame.to_bytes();
    assert!(PlcMessageFrame::from_bytes(SubcarrierSpacing::Khz25, &bytes).is_err());
    assert!(PlcMessageFrame::from_bytes(SubcarrierSpacing::Khz50, &bytes[..359]).is_err());
    assert!(PlcMessageFrame::new(
        SubcarrierSpacing::Khz50,
        timestamp,
        MessageChannelBlock::idle(347).unwrap()
    )
    .is_err());
    let mut damaged = bytes.clone();
    damaged[1] ^= 1;
    assert_eq!(
        PlcMessageFrame::from_bytes(SubcarrierSpacing::Khz50, &damaged)
            .unwrap_err()
            .code,
        "PlcTimestampCrcMismatch"
    );
    let mut other = bytes.clone();
    other[12] = 0x20;
    assert_eq!(
        PlcMessageFrame::from_bytes(SubcarrierSpacing::Khz50, &other)
            .unwrap_err()
            .code,
        "UnsupportedPlcMessageBlock"
    );
    assert_eq!(frame.to_bytes(), bytes);
}
