/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Recoverable high-level API diagnostics; callers never parse display strings for categories.
use serde::Serialize;
/// Stable error category independent of diagnostic wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorCode {
    InvalidInput,
    UnsupportedFormat,
    UnsupportedDecoderContract,
    UnsupportedCapability,
    MissingSetting,
    IncompatibleGrid,
    IncompatibleRoles,
    InvalidIdentifier,
    AssociationFailed,
    AmbiguousTimestamp,
    MissingCoverage,
    ResourceLimit,
    Io,
    Busy,
    Engine,
}
/// Structured preflight/import failure with optional original engine diagnostic.
#[derive(Debug, Clone, Serialize)]
pub struct ApiError {
    pub(crate) code: ErrorCode,
    pub(crate) message: String,
    pub(crate) engine: Option<crate::SimulationError>,
}
pub type ApiResult<T> = Result<T, ApiError>;
impl ApiError {
    /// Constructs an adapter diagnostic without losing its stable category.
    /// # Arguments
    /// * `code` - Machine-readable category.
    /// * `message` - Owned diagnostic context.
    /// # Returns
    /// Independent diagnostic; no process state changes.
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            engine: None,
        }
    }
    /// Reads the stable category for exception mapping.
    /// # Returns
    /// Copy of the category.
    pub fn code(&self) -> ErrorCode {
        self.code
    }
    /// Borrows human-readable context without parsing it.
    /// # Returns
    /// Text valid for this error's lifetime.
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl std::fmt::Display for ApiError {
    /// Formats stable category and human-readable context without exiting the process.
    /// # Arguments
    /// * `formatter` - Caller-owned text destination.
    /// # Returns
    /// The destination's formatting result.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{:?}: {}", self.code, self.message)
    }
}
impl std::error::Error for ApiError {}
impl From<crate::SimulationError> for ApiError {
    /// Retains the original engine diagnostic while mapping its stable category.
    /// # Arguments
    /// * `engine` - Owned existing simulation diagnostic.
    /// # Returns
    /// Independent structured API error; messages are never parsed for categories.
    fn from(engine: crate::SimulationError) -> Self {
        Self {
            code: match engine.code {
                "InvalidLabInput" => ErrorCode::InvalidInput,
                "UnsupportedDecoderContract" => ErrorCode::UnsupportedDecoderContract,
                "UnsupportedMeasuredProfile" => ErrorCode::UnsupportedCapability,
                _ => ErrorCode::Engine,
            },
            message: engine.to_string(),
            engine: Some(engine),
        }
    }
}
pub(crate) fn invalid(message: impl Into<String>) -> ApiError {
    ApiError::new(ErrorCode::InvalidInput, message)
}
