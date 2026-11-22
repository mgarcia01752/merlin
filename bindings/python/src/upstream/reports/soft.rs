/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin public soft-view dispatch; no native demapping, mask or decoder implementation.
use super::*;

#[pymethods]
impl PyBatchReport {
    /// Inspects actual computed-cell/completed-word availability and original word identities.
    /// # Returns
    /// Owned Rust-derived dictionary; disabled scopes have zero observation counts.
    /// Query/serialization releases the GIL and changes no receiver/session state.
    /// # Errors
    /// Propagates serialization and native conversion failures.
    fn soft_observations(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            serde_json::to_string(&self.0.soft_observations()).map_err(api::ApiError::Serialization)
        })
    }

    /// Copies actual raw-demapper/derandomized cells through finite independent budgets.
    /// # Arguments
    /// * `first_index` - Original retained history position; exact end returns empty.
    /// * `maximum_cells`, `maximum_llrs` - Positive cell and raw-plus-derandomized copy limits.
    /// # Returns
    /// Owned original identities, physical/coded positions, optional I/Q and actual metrics.
    /// Rust work releases the GIL; computed metrics do not imply completed FEC consumption.
    /// # Errors
    /// Shared unavailable, invalid start/budget and excessive-copy failures; native conversion errors.
    #[pyo3(text_signature = "($self, first_index=0, maximum_cells=32, maximum_llrs=4096)")]
    #[pyo3(signature=(first_index=0, maximum_cells=api::SoftCellRequest::default().maximum_cells, maximum_llrs=api::SoftCellRequest::default().maximum_llrs))]
    fn soft_cells(
        &self,
        py: Python<'_>,
        first_index: usize,
        maximum_cells: usize,
        maximum_llrs: usize,
    ) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            let view = self.0.soft_cells(api::SoftCellRequest {
                first_index,
                maximum_cells,
                maximum_llrs,
            })?;
            serde_json::to_string(&view).map_err(api::ApiError::Serialization)
        })
    }

    /// Copies exact transmitted-position input from one actually completed LDPC call.
    /// # Arguments
    /// * `retained_index` - Original bounded word-inventory position, independent of word ordinal.
    /// * `maximum_llrs` - Positive finite input-vector copy ceiling.
    /// # Returns
    /// Owned input metrics, original code dimensions/identities and actual independent decoder flags.
    /// Rust work releases the GIL; no posterior values or restored shortened priors are invented.
    /// # Errors
    /// Shared unavailable/incomplete-word and invalid/insufficient-budget failures; native conversion errors.
    #[pyo3(text_signature = "($self, retained_index=0, maximum_llrs=32768)")]
    #[pyo3(signature=(retained_index=0, maximum_llrs=api::SoftWordRequest::default().maximum_llrs))]
    fn soft_word(
        &self,
        py: Python<'_>,
        retained_index: usize,
        maximum_llrs: usize,
    ) -> PyResult<Py<PyAny>> {
        snapshot(py, || {
            let view = self.0.soft_word(api::SoftWordRequest {
                retained_index,
                maximum_llrs,
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
