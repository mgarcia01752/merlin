/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! One arithmetic recurrence transition, separate from source admission and PHY scrambling.

/// Advances the low fifteen register bits with `s[n+15] = s[n+14] XOR s[n]`.
///
/// # Arguments
/// * `register` - Raw LSB-first register. Only the low fifteen bits participate;
///   zero is a degenerate all-zero recurrence, not a valid PRBS15 experiment seed.
///   Each source owner retains its existing nonzero seed and work/position checks.
/// # Returns
/// `(next_register, emitted_bit)` without allocating, changing external state,
/// reseeding, counting source bits or consuming randomness. The output is binary.
#[inline]
pub fn step(register: u16) -> (u16, u8) {
    let register = register & 0x7fff;
    let output = (register & 1) as u8;
    let feedback = (register ^ (register >> 14)) & 1;
    ((register >> 1) | (feedback << 14), output)
}
