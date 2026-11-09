/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Table21/PDF96; lifting by48 using §7.4.3.2 right-shifted identities.
use super::parameters::{LIFTING_FACTOR, PARITY_CHECKS, PLC_MOTHER_BITS};
use ldpc_toolbox::sparse::SparseMatrix;
// Table21's first six columns are information; last four are triangular parity.
// A negative entry denotes an all-zero48x48 block, not a shift by zero.
const SHIFTS: [[i8; 10]; 4] = [
    [16, 1, 28, 9, 40, 38, 16, -1, -1, -1],
    [28, 42, 36, 11, 39, 9, 8, 38, -1, -1],
    [5, 2, 18, 16, 25, 47, -1, 2, 19, -1],
    [18, 18, 40, 18, 0, 34, -1, -1, 7, 32],
];
/// Expands the compact mother-code matrix into192 binary parity checks.
/// # Returns
/// Owned192-by480 matrix with fixed48-coordinate lifting; no external input.
pub(super) fn mother_matrix() -> SparseMatrix {
    let mut matrix = SparseMatrix::new(PARITY_CHECKS, PLC_MOTHER_BITS);
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
