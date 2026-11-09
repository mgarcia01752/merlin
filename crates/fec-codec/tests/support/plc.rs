/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independently transcribed sequential parity solution, not production matrix expansion.
// Table21, PHY I20 PDF96: each block row introduces one invertible48-bit
// parity group. Solve shifted equations in order rather than Gaussian elimination.
/// Solves the independently transcribed triangular parity equations.
/// # Arguments
/// * `information` - Exactly288 binary test values.
/// # Returns
/// Full480-bit oracle word with every independently evaluated syndrome zero.
pub fn oracle(information: &[u8; 288]) -> [u8; 480] {
    let mut word = [0; 480];
    word[..288].copy_from_slice(information);
    let rows = [
        [16, 1, 28, 9, 40, 38, 16, -1, -1, -1],
        [28, 42, 36, 11, 39, 9, 8, 38, -1, -1],
        [5, 2, 18, 16, 25, 47, -1, 2, 19, -1],
        [18, 18, 40, 18, 0, 34, -1, -1, 7, 32],
    ];
    for (group, shifts) in rows.iter().enumerate() {
        for row in 0..48 {
            let mut parity = 0;
            for (column, shift) in shifts[..6 + group].iter().enumerate() {
                if *shift >= 0 {
                    parity ^= word[column * 48 + (row + *shift as usize) % 48];
                }
            }
            word[(6 + group) * 48 + (row + shifts[6 + group] as usize) % 48] = parity;
        }
    }
    for shifts in rows {
        for row in 0..48 {
            let syndrome = shifts
                .iter()
                .enumerate()
                .filter(|(_, shift)| **shift >= 0)
                .fold(0, |parity, (column, shift)| {
                    parity ^ word[column * 48 + (row + *shift as usize) % 48]
                });
            assert_eq!(syndrome, 0);
        }
    }
    word
}
