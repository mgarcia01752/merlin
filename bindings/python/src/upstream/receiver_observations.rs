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
}
