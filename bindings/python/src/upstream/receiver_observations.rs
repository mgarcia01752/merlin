/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native policy conversion; all validation and retention remain in the public Rust API.
use super::{api, error};
use pyo3::prelude::*;

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ReceiverObservationPolicy"
)]
#[derive(Clone)]
pub(crate) struct PyReceiverObservationPolicy(pub(crate) api::ReceiverObservationPolicy);

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "SoftObservationPolicy"
)]
#[derive(Clone)]
pub(crate) struct PySoftObservationPolicy(pub(crate) api::SoftObservationPolicy);

#[pymethods]
impl PySoftObservationPolicy {
    /// Validates independent computed-cell, completed-word and word-input budgets in Rust.
    /// # Arguments
    /// * `maximum_cells`, `maximum_words`, `maximum_word_llrs` - Finite aggregate limits; all zero disables.
    /// # Returns
    /// Owned immutable policy; no metric buffer or scientific processing starts.
    /// # Errors
    /// Shared exceeded-ceiling and inconsistent word/scalar failures raise ApiError.
    #[new]
    #[pyo3(signature=(maximum_cells=0, maximum_words=0, maximum_word_llrs=0))]
    fn new(maximum_cells: usize, maximum_words: usize, maximum_word_llrs: usize) -> PyResult<Self> {
        api::SoftObservationPolicy::new(maximum_cells, maximum_words, maximum_word_llrs)
            .map(Self)
            .map_err(error)
    }

    /// Inspects the actual immutable Rust soft-retention budgets.
    /// # Returns
    /// Owned dictionary; serialization releases the GIL and starts no science.
    /// # Errors
    /// Propagates Rust serialization and native conversion/allocation failures.
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.info()))
            .map_err(|cause| error(api::ApiError::Serialization(cause)))?;
        crate::object_from_json(py, &text)
    }
}

#[pymethods]
impl PyReceiverObservationPolicy {
    /// Validates independent aggregate frame/scalar budgets through Rust.
    /// # Arguments
    /// * `maximum_frames`, `maximum_scalar_slots` - Zero together disables retention.
    /// # Returns
    /// Owned immutable policy; no scientific execution or buffer allocation starts.
    /// # Errors
    /// Rust rejects inconsistent disabled values and exceeded application ceilings.
    #[new]
    #[pyo3(signature=(maximum_frames=0, maximum_scalar_slots=0))]
    fn new(maximum_frames: usize, maximum_scalar_slots: usize) -> PyResult<Self> {
        api::ReceiverObservationPolicy::new(maximum_frames, maximum_scalar_slots)
            .map(Self)
            .map_err(error)
    }

    /// Inspects the validated Rust budgets without changing observer state.
    /// # Returns
    /// Owned dictionary; Rust serialization releases the GIL.
    /// # Errors
    /// Propagates native allocation or Rust serialization failures.
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.info()))
            .map_err(|cause| error(api::ApiError::Serialization(cause)))?;
        crate::object_from_json(py, &text)
    }

    /// Composes independent validated soft budgets without modifying this immutable policy.
    /// # Arguments
    /// * `policy` - Existing native soft observation policy; all zero removes soft retention.
    /// # Returns
    /// New owned receiver policy preserving estimate limits, including soft-only observation.
    fn with_soft_metrics(&self, policy: PyRef<'_, PySoftObservationPolicy>) -> Self {
        Self(self.0.with_soft_metrics(policy.0))
    }
}
