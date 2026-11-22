/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned immutable scientific report/provenance/file-byte queries with no global result routing.
use super::*;
use pyo3::types::PyBytes;
#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopResult")]
#[derive(Clone)]
pub(super) struct PyFullLoopResult(pub(super) api::FullLoopResult);
#[pymethods]
impl PyFullLoopResult {
    /// Copies the actual bounded complete/partial scientific job report without rerunning it.
    /// # Returns
    /// Owned report dictionary including real status/FEC/PRBS/settings and retained details.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    fn report(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(self.0.report()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Copies bounded original input identities, separate from simulated outcomes.
    /// # Returns
    /// Owned profile/measurement/H/source provenance; original file/capture bytes stay retained.
    /// # Errors
    /// Shared lookup/transport and Python allocation failures raise ApiError.
    fn input_summary(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.inputs().summary()?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Copies actual decoded original-file whole bytes without fill/padding or filesystem I/O.
    /// # Returns
    /// None for PRBS, or owned bytes including actual decoded errors/partial recovery.
    fn recovered_bytes(&self, py: Python<'_>) -> Option<Py<PyBytes>> {
        self.0
            .recovered_bytes()
            .map(|bytes| PyBytes::new(py, bytes).unbind())
    }
}
