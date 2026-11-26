/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared queued errors and an allocation-free consumer-selected fallback.
use crate::{
    downstream::{
        common::constants::DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE,
        help::{AdmissionError, Diagnostic},
        inputs::retention::admit_model,
    },
    shell::{Retained, RetentionBudget},
};

/// Bounds native worker diagnostics before retaining or publishing UI text.
/// # Arguments
/// * `message` - Original backend/parser error, borrowed without another full copy.
/// # Returns
/// At most the shared scalar-character ceiling, preserving valid UTF-8 and the
/// existing upstream prefix-truncation behavior. This is not a byte or PHY limit.
pub(crate) fn bounded_message(message: impl AsRef<str>) -> String {
    message
        .as_ref()
        .chars()
        .take(super::presentation::constants::MAX_WORKER_ERROR_CHARACTERS)
        .collect()
}

#[derive(Debug)]
pub(crate) enum WorkerFailure {
    Detailed(Box<Retained<AdmissionError>>),
    AdmissionRejected(&'static str),
    Static(&'static str),
}

impl Default for WorkerFailure {
    /// Creates an empty persistent status without retaining owned capacity.
    /// # Returns
    /// Borrowed empty text and no invented Diagnostic or reservation.
    fn default() -> Self {
        Self::Static("")
    }
}

impl WorkerFailure {
    /// Admits one actual error before its worker publishes the terminal message.
    /// # Arguments
    /// * `error` - Existing owned message and optional private help diagnostic.
    /// * `budget` - Exact application counter shared with accepted and queued owners.
    /// # Returns
    /// Detailed guarded error, or a fixed rejection after the unadmitted data drops.
    /// Metadata/host rejection never queues an uncharged owned error String.
    pub(crate) fn admit(error: AdmissionError, budget: &RetentionBudget) -> Self {
        Self::admit_with_fallback(error, budget, DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE)
    }

    /// Admits an actual error before publication with a consumer-selected fallback.
    ///
    /// # Arguments
    /// * `error` - Unchanged owned message and optional original diagnostic.
    /// * `budget` - Canonical counter covering queued and current owners together.
    /// * `fallback` - Borrowed static text used only if actual admission fails.
    ///
    /// # Returns
    /// A boxed guarded pointee, or borrowed text after the rejected graph drops.
    /// The outer envelope/Box handle is a separate fixed owner; no owned error
    /// String survives rejection and no diagnostic is invented for the fallback.
    pub(crate) fn admit_with_fallback(
        error: AdmissionError,
        budget: &RetentionBudget,
        fallback: &'static str,
    ) -> Self {
        Self::try_admit(error, budget).unwrap_or(Self::AdmissionRejected(fallback))
    }

    /// Admits actual detail strictly before a target/host transaction can commit.
    /// # Arguments
    /// * `error` - Finite candidate, measured after its actual message/help copies.
    /// * `budget` - Destination counter shared with still-live source owners.
    /// # Returns
    /// A guarded boxed pointee; no fallback substitutes for rejected target coverage.
    /// # Errors
    /// Actual metadata/layout or host rejection, dropping only the candidate.
    pub(crate) fn try_admit(
        error: AdmissionError,
        budget: &RetentionBudget,
    ) -> Result<Self, String> {
        let resources = error.retained_resources()?;
        Ok(Self::Detailed(Box::new(admit_model(
            error, resources, budget,
        )?)))
    }

    /// Checks the actual detailed reservation rather than comparing policy ceilings.
    /// # Arguments
    /// * `budget` - Intended accepting host.
    /// # Returns
    /// True for its guarded detail or non-owning static/default/fallback text.
    pub(crate) fn owns(&self, budget: &RetentionBudget) -> bool {
        match self {
            Self::Detailed(error) => budget.owns(error.as_ref()),
            Self::AdmissionRejected(_) | Self::Static(_) => true,
        }
    }

    /// Copies/admit actual detail while this original source remains charged.
    /// # Arguments
    /// * `budget` - Counter covering the independent destination graph.
    /// # Returns
    /// A separately guarded actual clone, or unchanged borrowed static text.
    /// Clone capacities are measured, never assumed equal to the source.
    /// # Errors
    /// Target rejection before any source or Context mutation; no fallback is used.
    pub(crate) fn try_copy_for_budget(&self, budget: &RetentionBudget) -> Result<Self, String> {
        match self {
            Self::Detailed(error) => Self::try_admit(
                AdmissionError {
                    message: error.value().message.clone(),
                    diagnostic: error.value().diagnostic.clone(),
                },
                budget,
            ),
            Self::AdmissionRejected(message) => Ok(Self::AdmissionRejected(message)),
            Self::Static(message) => Ok(Self::Static(message)),
        }
    }

    /// Replaces a normal message while preserving admitted original guidance.
    /// # Arguments
    /// * `message` - Actual independent replacement text.
    /// * `budget`, `fallback` - Current host and existing borrowed rejection wording.
    /// # Returns
    /// Guarded replacement/help clone while this source remains alive; rejection
    /// drops candidate detail and uses fallback without inventing a Diagnostic.
    pub(crate) fn replace_message(
        &self,
        message: String,
        budget: &RetentionBudget,
        fallback: &'static str,
    ) -> Self {
        let diagnostic = self.diagnostic().cloned().map(Box::new);
        if message.is_empty() && diagnostic.is_none() {
            return Self::default();
        }
        Self::admit_with_fallback(
            AdmissionError {
                message,
                diagnostic,
            },
            budget,
            fallback,
        )
    }

    /// Borrows a normal literal unless preserving existing help requires a copy.
    /// # Arguments
    /// * `message` - Existing static status wording, including empty resets.
    /// * `budget`, `fallback` - Current host and existing rejection wording.
    /// # Returns
    /// Non-owning literal without help, or an admitted message/help replacement.
    pub(crate) fn replace_static_message(
        &self,
        message: &'static str,
        budget: &RetentionBudget,
        fallback: &'static str,
    ) -> Self {
        if self.diagnostic().is_none() {
            Self::Static(message)
        } else {
            self.replace_message(message.to_owned(), budget, fallback)
        }
    }

    /// Borrows the unchanged detailed message or fixed admission fallback.
    /// # Returns
    /// UTF-8 text valid while this error owner remains alive, without a clone.
    pub(crate) fn message(&self) -> &str {
        match self {
            Self::Detailed(error) => &error.value().message,
            Self::AdmissionRejected(message) => message,
            Self::Static(message) => message,
        }
    }

    /// Borrows optional original help without rebuilding its private fields.
    /// # Returns
    /// Diagnostic valid for this error's lifetime; no fallback help is invented.
    pub(crate) fn diagnostic(&self) -> Option<&Diagnostic> {
        match self {
            Self::Detailed(error) => error.value().diagnostic.as_deref(),
            Self::AdmissionRejected(_) | Self::Static(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
