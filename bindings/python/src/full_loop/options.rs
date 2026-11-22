/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen native option snapshots; all scientific/work validation remains Rust-owned.
use super::*;
#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopOptions")]
#[derive(Clone)]
pub(super) struct PyFullLoopOptions(pub(super) api::FullLoopOptions);
#[pymethods]
impl PyFullLoopOptions {
    /// Selects the same explicit bounded full-loop defaults as Rust.
    /// # Returns
    /// Immutable policy; no source, worker or run begins.
    /// # Errors
    /// Shared default-policy validation errors raise ApiError.
    #[new]
    fn new() -> PyResult<Self> {
        let options = api::FullLoopOptions::default();
        options.validate().map_err(error)?;
        Ok(Self(options))
    }
    /// Admits up to 64 KiB strict options JSON through the public typed Rust validator.
    /// # Arguments
    /// * `data` - Existing named receiver/limit/capture fields; unknown keys reject.
    /// # Returns
    /// Frozen options; profile-specific selection admission remains request preparation.
    /// # Errors
    /// Shared malformed/limit/receiver-policy failures raise ApiError.
    #[staticmethod]
    fn from_json(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| api::FullLoopOptions::from_json(data))
            .map(Self)
            .map_err(error)
    }
    /// Serializes the exact accepted options without changing any job or input.
    /// # Returns
    /// Owned UTF-8 JSON; transport failure preserves the structured error category.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0)
            .map_err(transport_error)
            .map_err(error)
    }
    /// Copies the accepted typed policy as an independent native dictionary.
    /// # Returns
    /// Owned fields; editing the dictionary cannot alter a prepared request.
    /// # Errors
    /// Preserves transport/Python allocation failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &self.to_json()?)
    }
}
