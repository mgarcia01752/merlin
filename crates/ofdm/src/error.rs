/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared simulation diagnostics, independent of a particular feature.
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::Serialize;

/// Domain failure independent of filesystem, Qt and transport diagnostics.
#[derive(Debug, Clone, Serialize)]
pub struct SimulationError {
    pub code: &'static str,
    pub message: String,
}
impl SimulationError {
    /// Creates a structured simulation diagnostic.
    /// # Arguments
    /// * `code` - Stable error category.
    /// * `message` - Domain-specific explanation, without secret data.
    /// # Returns
    /// Owned error suitable for application adaptation and JSON diagnostics.
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
impl std::fmt::Display for SimulationError {
    /// Writes the domain explanation without formatting measurement payloads.
    /// # Arguments
    /// * `formatter` - Destination.
    /// # Returns
    /// The formatter's result.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for SimulationError {}

/// Creates an invalid scientific-input diagnostic without transport coupling.
/// # Arguments
/// * `message` - Caller-visible validation explanation.
/// # Returns
/// Owned error with the established InvalidLabInput category.
pub(crate) fn invalid(message: impl Into<String>) -> SimulationError {
    SimulationError::new("InvalidLabInput", message)
}

impl RetainedPayload for SimulationError {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { code: _, message } = self;
        merlin_api::retention_support::sum([message.retained_children()])
    }
}
