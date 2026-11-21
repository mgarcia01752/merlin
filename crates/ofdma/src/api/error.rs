/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Stable adapter error categories with original Rust domain causes.
use crate::PhyError;
use std::fmt;

/// Shared API result consumed directly by Rust and native adapters.
pub type ApiResult<T> = Result<T, ApiError>;

/// Capture adapter failure category, shared by Rust and native Python.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaptureFailure {
    /// Malformed bytes within the explicitly selected envelope.
    InvalidInput,
    /// Recognized framing or version outside the delivered capture scope.
    Unsupported,
    /// Input or expanded record count exceeds the declared adapter budget.
    ResourceLimit,
}

/// Expected public failure; scientific failed recovery remains a report.
#[derive(Debug)]
pub enum ApiError {
    /// Offline capture failure at a zero-based position in the supplied bytes.
    Capture {
        /// Stable category; unsupported is distinct from malformed input.
        category: CaptureFailure,
        /// Stable field name without requiring diagnostic-string parsing.
        field: &'static str,
        /// Byte offset, including the first unavailable position for truncation.
        offset: usize,
    },
    /// An optional adapter failed outside scientific domain validation.
    AdapterFailure(&'static str),
    /// Configuration is absent; construction and preview never invent defaults.
    MissingSetting,
    /// Shared PHY validation/engine error with its original structured context.
    Domain(PhyError),
    /// Serialization failed without overwriting a destination.
    Serialization(serde_json::Error),
}

impl ApiError {
    /// Reads the shared nominal category while preserving directional causes.
    ///
    /// # Returns
    /// A common category without parsing human-readable diagnostic text. Legacy
    /// code() strings remain available, including the Serialization diagnostic.
    pub fn category(&self) -> merlin_api::ErrorCode {
        use merlin_api::ErrorCode;
        match self {
            Self::Capture { category, .. } => match category {
                CaptureFailure::InvalidInput => ErrorCode::InvalidInput,
                CaptureFailure::Unsupported => ErrorCode::UnsupportedCapability,
                CaptureFailure::ResourceLimit => ErrorCode::ResourceLimit,
            },
            Self::AdapterFailure(_) => ErrorCode::Engine,
            Self::MissingSetting => ErrorCode::MissingSetting,
            Self::Serialization(error) => {
                if error.is_syntax() || error.is_data() || error.is_eof() {
                    ErrorCode::InvalidInput
                } else if error.is_io() {
                    ErrorCode::Io
                } else {
                    ErrorCode::Engine
                }
            }
            Self::Domain(error) => match error {
                PhyError::InvalidInput { .. } => ErrorCode::InvalidInput,
                PhyError::Unsupported { .. } => ErrorCode::UnsupportedCapability,
                PhyError::ResourceLimit { .. } => ErrorCode::ResourceLimit,
                PhyError::Association {
                    field: "control.busy" | "session.busy",
                } => ErrorCode::Busy,
                PhyError::Association { .. } => ErrorCode::AssociationFailed,
                PhyError::Codec { .. } => ErrorCode::Engine,
            },
        }
    }
    /// Returns a stable machine-readable category without parsing diagnostics.
    ///
    /// Native bindings use the same category and retain the human cause text.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Capture { category, .. } => match category {
                CaptureFailure::InvalidInput => "InvalidInput",
                CaptureFailure::Unsupported => "UnsupportedCapability",
                CaptureFailure::ResourceLimit => "ResourceLimit",
            },
            Self::AdapterFailure(_) => "Engine",
            Self::MissingSetting => "MissingSetting",
            Self::Serialization(_) => "Serialization",
            Self::Domain(error) => match error {
                PhyError::InvalidInput { .. } => "InvalidInput",
                PhyError::Unsupported { .. } => "UnsupportedCapability",
                PhyError::ResourceLimit { .. } => "ResourceLimit",
                PhyError::Association {
                    field: "control.busy" | "session.busy",
                } => "Busy",
                PhyError::Association { .. } => "AssociationFailed",
                PhyError::Codec { .. } => "Engine",
            },
        }
    }
}

impl fmt::Display for ApiError {
    /// Writes a stable category plus retained diagnostic to `formatter`.
    ///
    /// Returns formatter status without changing the API session or cause.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {self:?}", self.code())
    }
}

impl std::error::Error for ApiError {
    /// Borrows the original domain/serialization cause when one exists.
    ///
    /// Missing configuration has no lower-level failure cause.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Domain(error) => Some(error),
            Self::Serialization(error) => Some(error),
            Self::MissingSetting | Self::AdapterFailure(_) | Self::Capture { .. } => None,
        }
    }
}

impl From<PhyError> for ApiError {
    /// Wraps owned `error` without flattening its structured field/cause.
    ///
    /// Returns the shared domain failure for all adapters.
    fn from(error: PhyError) -> Self {
        Self::Domain(error)
    }
}

impl From<serde_json::Error> for ApiError {
    /// Wraps owned serialization `error` without losing its cause.
    ///
    /// Returns an API error; no files are created or overwritten.
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}
