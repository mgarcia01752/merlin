/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Table19, PDF91; right-shifted identities per section7.4.3.2, PDF56.
use super::{MOTHER_CODEWORD_BITS, MOTHER_INFORMATION_BITS};
use ldpc_toolbox::sparse::SparseMatrix;
const LIFTING_FACTOR: usize = 16;
// Negative entries denote all-zero16x16 blocks; columns5..9 are parity.
const SHIFTS: [[i8; 10]; 5] = [
    [1, 11, 10, 12, 7, 9, -1, -1, -1, -1],
    [2, 1, 14, 15, 14, 14, 12, -1, -1, -1],
    [0, 9, 3, 2, -1, -1, 11, 7, -1, -1],
    [6, 8, -1, 10, 3, -1, -1, 10, 4, -1],
    [12, 13, 11, -1, 0, -1, -1, -1, 5, 2],
];
/// Expands Table19's compact right-shifted identities into a binary matrix.
///
/// # Returns
/// Owned80-by160 matrix with fixed16-coordinate lifting and no external input.
pub(super) fn mother_matrix() -> SparseMatrix {
    let mut matrix = SparseMatrix::new(MOTHER_INFORMATION_BITS, MOTHER_CODEWORD_BITS);
    for (block_row, shifts) in SHIFTS.iter().enumerate() {
        for (block_column, shift) in shifts.iter().enumerate() {
            if *shift >= 0 {
                for row in 0..LIFTING_FACTOR {
                    matrix.insert(
                        block_row * LIFTING_FACTOR + row,
                        block_column * LIFTING_FACTOR + (row + *shift as usize) % LIFTING_FACTOR,
                    );
                }
            }
        }
    }
    matrix
}
