/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Byte order is verified against PHY I20 Figure63, not just a round trip.
use docsis_ofdm_fec::transmission::plc::PlcInformationBlock;
#[test]
fn information_bits_follow_msb_first_transmission_order() {
    let mut bytes = [0; 36];
    bytes[..3].copy_from_slice(&[0x80, 0x01, 0xa6]);
    bytes[35] = 0x03;
    let block = PlcInformationBlock::from_bytes(&bytes).unwrap();
    let bits = block.bits();
    assert_eq!(
        &bits[..24],
        &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0]
    );
    assert_eq!(&bits[280..], &[0, 0, 0, 0, 0, 0, 1, 1]);
    assert_eq!(block.bytes(), bytes);
    bytes[0] = 0;
    assert_eq!(block.bytes()[0], 0x80); // Owned copy; caller remains free to edit.
    assert_eq!(serde_json::to_value(&block).unwrap()["bytes"][2], 0xa6);
}
#[test]
fn information_size_is_exactly_one_encoder_input() {
    for length in [0, 35, 37, 720] {
        assert!(PlcInformationBlock::from_bytes(&vec![0; length]).is_err());
    }
}
