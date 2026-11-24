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

#[derive(Debug)]
pub(crate) enum WorkerFailure {
    Detailed(Box<Retained<AdmissionError>>),
    AdmissionRejected(&'static str),
    Static(&'static str),
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
        let admitted = error
            .retained_resources()
            .and_then(|resources| admit_model(error, resources, budget));
        match admitted {
            Ok(error) => Self::Detailed(Box::new(error)),
            Err(_) => Self::AdmissionRejected(fallback),
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
