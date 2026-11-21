/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Numerical diagnostics translated by each PHY's existing domain error owner.

/// Rejected input or nonfinite transform output; neither mutates caller data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DspError {
    /// Input length differs from the plan, or an input component is nonfinite.
    InvalidSamples,
    /// Finite input overflowed during numerical computation.
    NonfiniteOutput,
}

impl std::fmt::Display for DspError {
    /// Formats numerical context without assigning a PHY-specific category.
    ///
    /// # Arguments
    /// * `formatter` - Caller-owned diagnostic destination.
    /// # Returns
    /// The formatting operation's success or destination error.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::InvalidSamples => "invalid transform length or nonfinite I/Q",
            Self::NonfiniteOutput => "nonfinite transform output",
        })
    }
}
impl std::error::Error for DspError {}
