/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Offline profile hypotheses, measured RF scenarios and explicit coding experiments.
pub mod association;
pub mod evaluation;
pub mod model;
#[cfg(feature = "simulation")]
pub mod simulation;

use serde::Serialize;

/// Domain failure independent of filesystem, Qt and transport diagnostics.
#[derive(Debug, Clone, Serialize)]
pub struct LabError {
    pub code: &'static str,
    pub message: String,
}
impl LabError {
    /// Creates a structured laboratory diagnostic.
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
impl std::fmt::Display for LabError {
    /// Writes the domain explanation without formatting measurement payloads.
    /// # Arguments
    /// * `formatter` - Destination.
    /// # Returns
    /// The formatter's result.
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for LabError {}

pub(crate) fn invalid(message: impl Into<String>) -> LabError {
    LabError::new("InvalidLabInput", message)
}
pub mod document;
