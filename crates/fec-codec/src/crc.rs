/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! DOCSIS CRC-24-D, Annex E of PHY3.1 I20, PDF210.
const CRC_MASK: u32 = 0x00ff_ffff;
// Lower24 coefficients of x^24+x^22+x^20+x^19+x^18+x^16+x^14+x^13+
// x^11+x^10+x^8+x^7+x^6+x^3+x+1 (octal127266713 including x^24).
const CRC_POLYNOMIAL: u32 = 0x5d_6d_cb;

/// Calculates CRC-24-D over an MSB-first byte stream without changing its input.
///
/// # Arguments
/// * `bytes` - Message bytes in transmission order, excluding FEC parity.
///
/// # Returns
/// Three CRC bytes, MSB first; zero initialization, no reflection or final XOR.
/// Work is linear in the borrowed input length; callers bound their messages.
pub fn crc24d(bytes: &[u8]) -> [u8; 3] {
    let mut remainder = 0u32;
    for byte in bytes {
        remainder ^= u32::from(*byte) << 16;
        for _ in 0..8 {
            let feedback = remainder & (1 << 23) != 0;
            remainder = (remainder << 1) & CRC_MASK;
            if feedback {
                remainder ^= CRC_POLYNOMIAL;
            }
        }
    }
    let bytes = remainder.to_be_bytes();
    [bytes[1], bytes[2], bytes[3]]
}
