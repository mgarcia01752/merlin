/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Test-only independent Gaussian solution, not the production peeling encoder.
//! Both read the transcribed normative data; this is not an external vector set.
use crate::upstream_ldpc::DocsisLdpcCode;

pub(super) fn encode(code: DocsisLdpcCode, information: &[u8]) -> Vec<u8> {
    let parameters = code.parameters();
    let parity_bits = parameters.codeword_bits - parameters.information_bits;
    let lift = parameters.lifting_factor;
    let packed_words = (parity_bits + 1).div_ceil(64);
    let mut equations = vec![vec![0_u64; packed_words]; parity_bits];
    // Build by column with inverse circulant displacement, independently of
    // production's row + shift mapping and SparseMatrix representation.
    for (block_row, shifts) in code.shifts().iter().enumerate() {
        for (block_column, &shift) in shifts.iter().enumerate() {
            if shift < 0 {
                continue;
            }
            for local_column in 0..lift {
                let column = block_column * lift + local_column;
                let row = block_row * lift + (local_column + lift - shift as usize) % lift;
                if column < parameters.information_bits {
                    if information[column] != 0 {
                        equations[row][parity_bits / 64] ^= 1 << (parity_bits % 64);
                    }
                } else {
                    let parity_column = column - parameters.information_bits;
                    equations[row][parity_column / 64] ^= 1 << (parity_column % 64);
                }
            }
        }
    }
    for pivot in 0..parity_bits {
        let selected = (pivot..parity_bits)
            .find(|row| bit(&equations[*row], pivot))
            .expect("invertible normative parity");
        equations.swap(pivot, selected);
        let pivot_equation = equations[pivot].clone();
        for (row, equation) in equations.iter_mut().enumerate() {
            if row != pivot && bit(equation, pivot) {
                for (value, pivot_value) in equation.iter_mut().zip(&pivot_equation) {
                    *value ^= pivot_value;
                }
            }
        }
    }
    let mut codeword = information.to_vec();
    codeword.extend(equations.iter().map(|row| u8::from(bit(row, parity_bits))));
    codeword
}

fn bit(packed: &[u64], index: usize) -> bool {
    packed[index / 64] & (1 << (index % 64)) != 0
}
