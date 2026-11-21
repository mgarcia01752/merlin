/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stable categories shared without flattening directional diagnostic causes.
use serde::Serialize;
/// Stable error category independent of diagnostic wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ErrorCode {
    /// Malformed or out-of-domain input.
    InvalidInput,
    /// Input format is not supported.
    UnsupportedFormat,
    /// Recognized decoder contract is not implemented.
    UnsupportedDecoderContract,
    /// Requested capability is outside the supported scope.
    UnsupportedCapability,
    /// Required configuration has not been supplied.
    MissingSetting,
    /// Frequency or time grids cannot be combined.
    IncompatibleGrid,
    /// Carrier roles are incompatible.
    IncompatibleRoles,
    /// Identifier is invalid or outside its owning session.
    InvalidIdentifier,
    /// Explicit input associations failed validation.
    AssociationFailed,
    /// Timestamp association is ambiguous.
    AmbiguousTimestamp,
    /// Required source coverage is missing.
    MissingCoverage,
    /// A finite work, input or retention budget was exceeded.
    ResourceLimit,
    /// Native input or output operation failed.
    Io,
    /// A mutable session or control is already executing.
    Busy,
    /// The scientific engine or adapter failed.
    Engine,
}
