/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Concrete queued Designer errors and an allocation-free admission fallback.
use crate::{
    downstream::{
        common::constants::DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE,
        help::{AdmissionError, Diagnostic},
        inputs::retention::admit_model,
    },
    shell::{Retained, RetentionBudget},
};

#[derive(Debug)]
pub(super) enum WorkerFailure {
    Detailed(Retained<AdmissionError>),
    AdmissionRejected,
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
    pub(super) fn admit(error: AdmissionError, budget: &RetentionBudget) -> Self {
        let admitted = error
            .retained_resources()
            .and_then(|resources| admit_model(error, resources, budget));
        match admitted {
            Ok(error) => Self::Detailed(error),
            Err(_) => Self::AdmissionRejected,
        }
    }

    /// Borrows the unchanged detailed message or fixed admission fallback.
    /// # Returns
    /// UTF-8 text valid while this error owner remains alive, without a clone.
    pub(super) fn message(&self) -> &str {
        match self {
            Self::Detailed(error) => &error.value().message,
            Self::AdmissionRejected => DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE,
            Self::Static(message) => message,
        }
    }

    /// Borrows optional original help without rebuilding its private fields.
    /// # Returns
    /// Diagnostic valid for this error's lifetime; no fallback help is invented.
    pub(super) fn diagnostic(&self) -> Option<&Diagnostic> {
        match self {
            Self::Detailed(error) => error.value().diagnostic.as_deref(),
            Self::AdmissionRejected | Self::Static(_) => None,
        }
    }
}

#[cfg(test)]
mod tests;
