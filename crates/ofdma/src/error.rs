/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Structured scientific admission failures; failed recovery remains an outcome.
use std::fmt;

/// Domain result retaining machine-readable error categories.
pub type PhyResult<T> = Result<T, PhyError>;

/// Expected failure at a named input or processing boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhyError {
    /// Invalid finite value, length, binary input or domain association.
    InvalidInput {
        /// Stable field path.
        field: &'static str,
    },
    /// Recognized capability without an accepted implementation.
    Unsupported {
        /// Unsupported field/capability.
        field: &'static str,
    },
    /// Exceeded explicit work/storage bound, including arithmetic overflow.
    ResourceLimit {
        /// Exhausted budget or coordinate.
        field: &'static str,
    },
    /// Stale, wrong-run or noncontiguous input.
    Association {
        /// Invalid identity or offset.
        field: &'static str,
    },
    /// The FEC owner rejected input or matrix construction.
    Codec {
        /// Preserved lower-level typed error.
        error: fec_codec::upstream_ldpc::LdpcError,
    },
}

impl fmt::Display for PhyError {
    /// Formats a diagnostic without replacing the structured error category.
    ///
    /// `formatter` borrows the output; returns its write result without mutation.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for PhyError {
    /// Exposes the retained codec error as the lower-level cause.
    ///
    /// Returns None for admission errors owned directly by the PHY.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Codec { error } => Some(error),
            _ => None,
        }
    }
}

impl From<fec_codec::upstream_ldpc::LdpcError> for PhyError {
    /// Preserves the FEC diagnostic at the PHY boundary.
    ///
    /// `error` is consumed; returns an owned codec-category diagnostic.
    fn from(error: fec_codec::upstream_ldpc::LdpcError) -> Self {
        Self::Codec { error }
    }
}

pub(crate) fn invalid(field: &'static str) -> PhyError {
    PhyError::InvalidInput { field }
}

pub(crate) fn limit(field: &'static str) -> PhyError {
    PhyError::ResourceLimit { field }
}
