/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! MULPI I25 Table77 header/pointer vectors and application input bounds.
use docsis_ofdm_fec::transmission::plc::MessageChannelBlock;
#[test]
fn idle_and_packet_pointer_headers_follow_table77() {
    assert_eq!(MessageChannelBlock::idle(1).unwrap().to_bytes(), [0x30]);
    assert_eq!(
        MessageChannelBlock::idle(4).unwrap().to_bytes(),
        [0x30, 0xff, 0xff, 0xff]
    );
    let source = [0xaa, 0xbb, 0xcc]; // Opaque synthetic segment, not a valid MAC packet.
    let block = MessageChannelBlock::new(&source, Some(2)).unwrap();
    assert_eq!(block.to_bytes(), [0x31, 0, 2, 0xaa, 0xbb, 0xcc]);
    assert_eq!(block.packet_start_pointer(), Some(2));
    assert_eq!(block.segment(), source);
    assert_eq!(
        MessageChannelBlock::from_bytes(&block.to_bytes()).unwrap(),
        block
    );
    let large_pointer = MessageChannelBlock::new(&[0xff; 258], Some(256)).unwrap();
    assert_eq!(&large_pointer.to_bytes()[..3], &[0x31, 1, 0]);
    assert_eq!(
        MessageChannelBlock::new(&[1], Some(0)).unwrap().to_bytes(),
        [0x31, 0, 0, 1]
    );
}
#[test]
fn invalid_headers_truncation_and_pointer_bounds_are_rejected() {
    for bytes in [
        &[][..],
        &[0x31],
        &[0x31, 0],
        &[0x31, 0, 0],
        &[0x31, 0, 1, 0xff],
        &[0x32],
        &[0x20],
    ] {
        assert!(MessageChannelBlock::from_bytes(bytes).is_err(), "{bytes:?}");
    }
    assert!(MessageChannelBlock::new(&[], Some(0)).is_err());
    assert!(MessageChannelBlock::new(&[1], Some(1)).is_err());
    assert!(MessageChannelBlock::idle(0).is_err());
    assert!(MessageChannelBlock::idle(709).is_err());
    assert!(MessageChannelBlock::new(&[0; 708], None).is_err());
    assert!(MessageChannelBlock::new(&[0; 706], Some(0)).is_err());
    assert_eq!(
        MessageChannelBlock::new(&[0; 707], None)
            .unwrap()
            .encoded_len(),
        708
    );
    assert_eq!(
        MessageChannelBlock::new(&[0; 705], Some(704))
            .unwrap()
            .encoded_len(),
        708
    );
}
