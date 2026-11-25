/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Categories callers can handle without parsing display messages.
pub enum ErrorCode {
    Truncated,
    UnsupportedType,
    UnsupportedVersion,
    UnsupportedLayout,
    UnsupportedDecoder,
    LimitExceeded,
    LengthMismatch,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Decoder failure with a byte offset and static field identifier.
pub struct ParseError {
    pub code: ErrorCode,
    pub offset: usize,
    pub field: &'static str,
}
impl fmt::Display for ParseError {
    /// Formats a diagnostic for human-readable display.
    ///
    /// # Arguments
    /// * `formatter` - Destination formatter.
    ///
    /// # Returns
    /// The formatter result; machine callers should use structured fields.
    ///
    /// # Errors
    /// Propagates errors from the destination formatter.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{:?}: {} at byte {}",
            self.code, self.field, self.offset
        )
    }
}
impl std::error::Error for ParseError {}
