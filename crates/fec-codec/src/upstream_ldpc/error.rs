/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Stable error categories for Rust callers and future native wrappers.
use std::fmt;

/// Recoverable validation or matrix-construction failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LdpcError {
    /// Information count is outside the selected mother code's 1..=K range.
    InformationLength,
    /// Iteration budget is outside the application limit.
    IterationLimit,
    /// An input slice has an unexpected number of elements.
    InputLength {
        /// Required element count.
        expected: usize,
        /// Actual element count.
        actual: usize,
    },
    /// Information/codeword contains a value other than zero or one.
    NonBinary,
    /// A soft input is NaN or infinite.
    NonFiniteLlr,
    /// A pinned matrix failed structural admission.
    MatrixDefinition,
}

impl fmt::Display for LdpcError {
    /// Formats a human diagnostic without replacing the structured category.
    ///
    /// # Arguments
    /// * `formatter` - Borrowed output destination.
    /// # Returns
    /// The formatter's write result; this operation does not mutate the error.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InformationLength => write!(formatter, "information bits must be in 1..=K"),
            Self::IterationLimit => write!(formatter, "iterations must be in 1..=100"),
            Self::InputLength { expected, actual } => {
                write!(formatter, "expected {expected} elements, received {actual}")
            }
            Self::NonBinary => write!(formatter, "bits must be binary"),
            Self::NonFiniteLlr => write!(formatter, "LLRs must be finite"),
            Self::MatrixDefinition => write!(formatter, "invalid or non-peelable LDPC matrix"),
        }
    }
}

impl std::error::Error for LdpcError {}
