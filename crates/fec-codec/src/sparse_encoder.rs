/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Sparse systematic encoding for parity matrices admitting substitution.
use ldpc_toolbox::{gf2::GF2, sparse::SparseMatrix};
use ndarray::Array1;
use num_traits::Zero;
use std::collections::VecDeque;

struct Equation {
    pivot: usize,
    inputs: Vec<usize>,
}

/// Private immutable substitution plan for the supported NCP/PLC parity systems.
/// A parity coordinate is solved only when every other coordinate in its check
/// is known. This does not implement Gaussian elimination for arbitrary matrices.
pub(crate) struct SparseEncoder {
    information_bits: usize,
    codeword_bits: usize,
    equations: Vec<Equation>,
}
impl SparseEncoder {
    /// Builds a sparse substitution order without allocating a dense generator.
    ///
    /// # Arguments
    /// * `matrix` - Binary checks H=[information | parity]; parity is square.
    /// # Returns
    /// Owned immutable plan; setup/storage are linear in dimensions and nonzeros.
    /// Caller matrix/order remains unchanged. The plan borrows no matrix storage.
    /// # Errors
    /// Rejects zero checks, absent information columns and non-peelable parity
    /// dependencies, including singular/cyclic systems. Some invertible general
    /// matrices are not peelable and are deliberately outside this helper's scope.
    pub(crate) fn from_h(matrix: &SparseMatrix) -> Result<Self, String> {
        let parity_bits = matrix.num_rows();
        let codeword_bits = matrix.num_cols();
        if parity_bits == 0 || codeword_bits <= parity_bits {
            return Err("Sparse encoder needs checks and systematic information columns".into());
        }
        let information_bits = codeword_bits - parity_bits;
        let mut remaining: Vec<_> = (0..parity_bits)
            .map(|row| {
                matrix
                    .iter_row(row)
                    .filter(|column| **column >= information_bits)
                    .count()
            })
            .collect();
        let mut ready: VecDeque<_> = remaining
            .iter()
            .enumerate()
            .filter_map(|(row, degree)| (*degree == 1).then_some(row))
            .collect();
        let mut solved = vec![false; parity_bits];
        let mut equations = Vec::with_capacity(parity_bits);
        while let Some(row) = ready.pop_front() {
            if remaining[row] != 1 {
                continue;
            }
            let pivot = matrix
                .iter_row(row)
                .copied()
                .find(|column| *column >= information_bits && !solved[*column - information_bits])
                .ok_or("Sparse encoder dependency degree is inconsistent")?;
            equations.push(Equation {
                pivot,
                inputs: matrix
                    .iter_row(row)
                    .copied()
                    .filter(|column| *column != pivot)
                    .collect(),
            });
            solved[pivot - information_bits] = true;
            // Existing SparseMatrix column adjacency visits each parity edge once.
            for &dependent in matrix.iter_col(pivot) {
                remaining[dependent] -= 1;
                if remaining[dependent] == 1 {
                    ready.push_back(dependent);
                }
            }
        }
        if equations.len() != parity_bits {
            return Err("Parity submatrix does not admit sparse substitution".into());
        }
        Ok(Self {
            information_bits,
            codeword_bits,
            equations,
        })
    }

    /// Encodes one complete systematic GF(2) word using the sparse plan.
    ///
    /// # Arguments
    /// * `message` - Exactly the configured information-bit count; typed binary values.
    /// # Returns
    /// Independent systematic information+parity array; inputs/plan remain unchanged.
    /// Work is linear in the codeword size and recorded sparse equation entries.
    /// # Panics
    /// Panics for an internal caller's incorrect message length. Public NCP/PLC
    /// entry points preserve their existing fixed-size/binary validation first.
    pub(crate) fn encode(&self, message: &Array1<GF2>) -> Array1<GF2> {
        assert_eq!(
            message.len(),
            self.information_bits,
            "Sparse encoder information length"
        );
        let mut word = Array1::zeros(self.codeword_bits);
        for (target, source) in word.iter_mut().zip(message) {
            *target = *source;
        }
        for equation in &self.equations {
            word[equation.pivot] = equation
                .inputs
                .iter()
                .fold(GF2::zero(), |sum, column| sum + word[*column]);
        }
        word
    }

    /// Encodes validated binary information with the same immutable sparse plan.
    ///
    /// # Arguments
    /// * `information` - Exactly the configured information count, each value 0 or 1.
    ///   Public upstream admission validates length and values before this operation.
    /// # Returns
    /// One owned systematic information/parity buffer, without changing inputs or
    /// plan state. Allocation and edge traversal match the source byte encoder.
    /// # Panics
    /// Panics for an internal caller's incorrect length. Public codecs validate
    /// the complete mother information buffer before calling this private helper.
    pub(crate) fn encode_bits(&self, information: &[u8]) -> Vec<u8> {
        assert_eq!(information.len(), self.information_bits);
        let mut word = vec![0; self.codeword_bits];
        word[..information.len()].copy_from_slice(information);
        for equation in &self.equations {
            word[equation.pivot] = equation
                .inputs
                .iter()
                .fold(0, |parity, column| parity ^ word[*column]);
        }
        word
    }
}

#[cfg(test)]
mod tests;
