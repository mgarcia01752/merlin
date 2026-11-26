/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original directional diagnostics behind one shared category query.
use docsis_ofdm_fec::api::{ApiError as DownstreamError, ErrorCode};
use docsis_ofdma::api::ApiError as UpstreamError;
use std::fmt;

/// Project admission failure retaining its original directional diagnostic.
#[derive(Debug)]
pub enum ProjectError {
    /// Downstream input, project envelope, resource or filesystem diagnostic.
    Downstream(DownstreamError),
    /// Upstream configuration, captured source or observation admission diagnostic.
    Upstream(UpstreamError),
}
impl ProjectError {
    /// Reads the shared category without parsing or replacing the original cause.
    /// # Returns
    /// Stable cross-direction error category for adapters and acceptance tests.
    pub fn code(&self) -> ErrorCode {
        match self {
            Self::Downstream(cause) => cause.code(),
            Self::Upstream(cause) => cause.category(),
        }
    }
}
impl fmt::Display for ProjectError {
    /// Writes the retained directional diagnostic without changing caller state.
    /// # Arguments
    /// * `formatter` - Destination supplied by Rust formatting.
    /// # Returns
    /// The original diagnostic's formatting result.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Downstream(cause) => cause.fmt(formatter),
            Self::Upstream(cause) => cause.fmt(formatter),
        }
    }
}
impl std::error::Error for ProjectError {
    /// Borrows the original typed failure for diagnostic consumers.
    /// # Returns
    /// The downstream or upstream error; no scientific state is accessed.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Downstream(cause) => Some(cause),
            Self::Upstream(cause) => Some(cause),
        }
    }
}
impl From<DownstreamError> for ProjectError {
    /// Retains an owned downstream diagnostic without changing its category.
    /// # Arguments
    /// * `cause` - Original native/scientific admission failure.
    /// # Returns
    /// Project error retaining the original typed cause.
    fn from(cause: DownstreamError) -> Self {
        Self::Downstream(cause)
    }
}
impl From<UpstreamError> for ProjectError {
    /// Retains an owned upstream diagnostic without changing its structured fields.
    /// # Arguments
    /// * `cause` - Original upstream admission failure.
    /// # Returns
    /// Project error retaining the original typed cause.
    fn from(cause: UpstreamError) -> Self {
        Self::Upstream(cause)
    }
}
