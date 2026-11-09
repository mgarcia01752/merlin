/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! MULPI I25 §6.2.1.3 and6.4.1: X.25 header CRC and Ethernet message FCS.
const X25_REFLECTED_POLYNOMIAL: u16 = 0x8408;
const ETHERNET_REFLECTED_POLYNOMIAL: u32 = 0xedb88320;

/// Computes the reflected CRC-16/X25 value for a MAC header.
/// # Arguments
/// * `bytes` - Header octets excluding HCS.
/// # Returns
/// Complemented CRC, serialized least-significant octet first on the wire.
pub fn header_crc(bytes: &[u8]) -> u16 {
    let mut crc = u16::MAX;
    for byte in bytes {
        crc ^= u16::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ X25_REFLECTED_POLYNOMIAL
            } else {
                crc >> 1
            };
        }
    }
    !crc
}
/// Computes the reflected IEEE/Ethernet CRC32 over the management PDU.
/// # Arguments
/// * `bytes` - DA through message payload, excluding FCS and outer MAC header.
/// # Returns
/// Complemented FCS, serialized least-significant octet first.
pub fn message_crc(bytes: &[u8]) -> u32 {
    let mut crc = u32::MAX;
    for byte in bytes {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ ETHERNET_REFLECTED_POLYNOMIAL
            } else {
                crc >> 1
            };
        }
    }
    !crc
}
