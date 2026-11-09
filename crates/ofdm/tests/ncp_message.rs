/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent field placement, including current U bit replacing an old R bit.
use docsis_ofdm_fec::transmission::ncp::{NcpMessage, NULL_POINTER};
#[test]
fn message_bytes_and_flags_follow_figure92() {
    let mut message = NcpMessage {
        profile_id: 10,
        zero_loaded: false,
        data_profile_odd: true,
        ncp_profile_odd: true,
        last: true,
        tagged: true,
        ncp_profile_update: true,
        pointer: 0x1234,
    };
    assert_eq!(message.to_bytes().unwrap(), [0xa7, 0xd2, 0x34]);
    assert_eq!(
        NcpMessage::from_bytes(&[0xa7, 0xd2, 0x34]).unwrap(),
        message
    );
    message.zero_loaded = true;
    assert_eq!(message.to_bytes().unwrap(), [0xaf, 0xd2, 0x34]);
    message.zero_loaded = false;
    message.pointer = NULL_POINTER;
    assert_eq!(message.to_bytes().unwrap(), [0xa7, 0xdf, 0xff]);
    message.zero_loaded = true;
    assert!(message.to_bytes().is_err());
    message.zero_loaded = false;
    message.pointer = 0x2000;
    assert!(message.to_bytes().is_err());
    message.pointer = 0;
    message.profile_id = 16;
    assert!(message.to_bytes().is_err());
    assert!(NcpMessage::from_bytes(&[0, 0x20, 0]).is_err());
    assert!(NcpMessage::from_bytes(&[8, 0x1f, 0xff]).is_err());
}
