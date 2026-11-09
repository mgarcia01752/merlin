/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent polynomial division verifies the full transmitted timestamp block.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::transmission::plc::{ExtendedTimestamp, TimestampMessageBlock};
// Long division of a96-bit message polynomial, unlike the production CRC register.
// The leading x^24 coefficient is included in0x15d6dcb (PHY I20 Annex E).
fn polynomial_remainder(message: &[u8; 9]) -> [u8; 3] {
    let mut dividend = message
        .iter()
        .fold(0u128, |value, byte| (value << 8) | u128::from(*byte))
        << 24;
    for exponent in (24..96).rev() {
        if dividend & (1u128 << exponent) != 0 {
            dividend ^= 0x15d6dcbu128 << (exponent - 24);
        }
    }
    [
        (dividend >> 16) as u8,
        (dividend >> 8) as u8,
        dividend as u8,
    ]
}
#[test]
fn timestamp_crc_covers_header_and_all_eight_time_bytes() {
    let timestamp = ExtendedTimestamp::new(0x0000_0102_0304_0506, 19, 10).unwrap();
    let block = TimestampMessageBlock { timestamp };
    let source = [0x10, 0, 2, 4, 6, 8, 10, 13, 0x3a];
    let bytes = block.to_bytes();
    assert_eq!(&bytes[..9], source);
    assert_eq!(&bytes[9..], polynomial_remainder(&source));
    assert_eq!(TimestampMessageBlock::from_bytes(&bytes).unwrap(), block);
    for index in 0..12 {
        let mut damaged = bytes;
        damaged[index] ^= 1;
        assert!(TimestampMessageBlock::from_bytes(&damaged).is_err());
    }
}
#[test]
fn timestamp_parser_rejects_reserved_bits_wrong_size_and_invalid_time() {
    let valid = TimestampMessageBlock {
        timestamp: ExtendedTimestamp::new(0, 0, 0).unwrap(),
    }
    .to_bytes();
    assert!(TimestampMessageBlock::from_bytes(&valid[..11]).is_err());
    assert!(TimestampMessageBlock::from_bytes(&[]).is_err());
    let mut reserved = valid;
    reserved[0] = 0x11;
    let prefix = reserved[..9].try_into().unwrap();
    let crc = polynomial_remainder(prefix);
    reserved[9..].copy_from_slice(&crc);
    assert!(TimestampMessageBlock::from_bytes(&reserved).is_err());
    let mut invalid_time = valid;
    invalid_time[7] = 1;
    invalid_time[8] = 0x40; // Sample phase20, despite a correct CRC.
    let prefix = invalid_time[..9].try_into().unwrap();
    let crc = polynomial_remainder(prefix);
    invalid_time[9..].copy_from_slice(&crc);
    assert!(TimestampMessageBlock::from_bytes(&invalid_time).is_err());
}
