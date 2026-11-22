/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Checked dictionary projections of the public typed recovery views.
use super::*;

#[pymethods]
impl PyBatchReport {
    /// Reads independent PRBS15 comparison and actual complete/partial coverage through Rust.
    /// # Returns
    /// Owned dictionary with original source counts, optional BER, lifecycle and recovery flag.
    /// Rust work releases the GIL; this passive query starts no source or decoder.
    /// # Errors
    /// Shared inconsistent-report/overflow failures raise ApiError; conversion errors propagate.
    fn verification(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            serde_json::to_string(&self.0.verification()?).map_err(api::ApiError::Serialization)
        })
    }

    /// Summarizes actual upstream decoder flags and iterations through the public Rust owner.
    /// # Returns
    /// Owned dictionary whose scalar totals are independent of page/observation limits.
    /// Independent convergence, parity, shortening and fill failures remain distinct.
    /// # Errors
    /// Shared report/word metadata and checked overflow failures raise ApiError.
    fn fec_summary(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            serde_json::to_string(&self.0.fec_summary()?).map_err(api::ApiError::Serialization)
        })
    }

    /// Copies a bounded original completed-word page, without decoding or whole-report export.
    /// # Arguments
    /// * `grant_index`, `first_word` - Original started-grant index and completed-word ordinal.
    /// * `maximum_words` - Positive page ceiling, at most Rust's 32-word application policy.
    /// * `max_decision_bits` - Positive copied mother/information/payload bit budget, at most 262144.
    /// # Returns
    /// Owned dictionary with original decisions, code dimensions, identities and continuation.
    /// End positions return empty pages; work releases the GIL and changes no report/session.
    /// # Errors
    /// Checked native conversion errors and shared position/metadata/resource failures propagate.
    #[pyo3(
        text_signature = "($self, grant_index=0, first_word=0, maximum_words=1, max_decision_bits=262144)"
    )]
    #[pyo3(signature=(grant_index=0,first_word=0,maximum_words=api::FecWordRequest::default().maximum_words,max_decision_bits=api::FecWordRequest::default().max_decision_bits))]
    fn fec_words(
        &self,
        py: Python<'_>,
        grant_index: usize,
        first_word: usize,
        maximum_words: usize,
        max_decision_bits: usize,
    ) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            let view = self.0.fec_words(api::FecWordRequest {
                grant_index,
                first_word,
                maximum_words,
                max_decision_bits,
            })?;
            serde_json::to_string(&view).map_err(api::ApiError::Serialization)
        })
    }
}

fn snapshot(
    py: Python<'_>,
    query: impl FnOnce() -> api::ApiResult<String> + Send,
) -> PyResult<Py<PyAny>> {
    let text = py.detach(query).map_err(error)?;
    crate::object_from_json(py, &text)
}
