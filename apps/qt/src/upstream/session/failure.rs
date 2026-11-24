/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application terminal projection; no opaque scientific error enters the queue.
use crate::{
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::RetentionBudget,
};
use merlin::upstream::{ApiError, ErrorCode};
use std::fmt;

/// Original public category plus the existing bounded Qt text policy.
///
/// The owned detailed graph is guarded separately from this fixed envelope.
/// A static admission fallback preserves the category without retaining ApiError.
#[derive(Debug)]
pub struct RunFailure {
    category: ErrorCode,
    failure: WorkerFailure,
}

impl RunFailure {
    /// Projects and admits one actual scientific failure before terminal publication.
    ///
    /// # Arguments
    /// * `error` - Original public failure, dropped before any terminal is queued.
    /// * `budget` - Same canonical counter covering the accepted session and target.
    ///
    /// # Returns
    /// Original category and unchanged bounded message, or borrowed fallback text.
    /// Formatting is finite temporary preparation; rejected owned text is dropped.
    pub(super) fn admit(error: ApiError, budget: &RetentionBudget) -> Self {
        let category = error.category();
        let message = crate::upstream::workbench::error_text(error.to_string());
        drop(error);
        Self {
            category,
            failure: WorkerFailure::admit_with_fallback(
                message.into(),
                budget,
                WORKER_FAILURE_ADMISSION_MESSAGE,
            ),
        }
    }

    /// Reads the original typed public category, including under admission pressure.
    ///
    /// # Returns
    /// Nominal category without parsing or changing diagnostic text.
    pub fn category(&self) -> ErrorCode {
        self.category
    }

    /// Borrows the bounded application text without cloning its charged storage.
    ///
    /// # Returns
    /// Original projected message or consumer-selected static fallback.
    pub fn message(&self) -> &str {
        self.failure.message()
    }
}

impl fmt::Display for RunFailure {
    /// Formats the already bounded application message without copying its graph.
    ///
    /// # Arguments
    /// * `formatter` - Existing GUI diagnostic destination.
    ///
    /// # Returns
    /// Standard formatting result; category remains available independently.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}
