/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original actual likelihood observations converted without Python statistics or decoder work.
use super::{cells::PyCellQuery, *};
#[pymethods]
impl PyFullLoopResult {
    /// Projects actual selected-cell LLRs through the existing public Rust diagnostic owner.
    /// # Arguments
    /// * `query` - Optional immutable actual-cell selection; at most 2048 cells contribute.
    /// # Returns
    /// Owned histogram/envelope/statistics dictionary with explicit selected-cell reference plane.
    /// Protected streams remain empty; no probability or reconstructed codeword is inferred.
    /// # Errors
    /// Shared selection/work/finite-value failures raise ApiError; work releases the GIL.
    #[pyo3(signature=(query=None))]
    fn cell_soft_view(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| {
                let result = self.0.cell_soft_view(query)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Projects an actual retained coding-order LDPC-input event through public Rust diagnostics.
    /// # Arguments
    /// * `snapshot` - Zero-based actual retained event, independent of physical-cell selection.
    /// # Returns
    /// Owned finite histogram/envelope/statistics with original context/offset/total metadata.
    /// # Errors
    /// Shared missing/invalid observation or finite/work/index failures raise ApiError.
    #[pyo3(signature=(snapshot=0))]
    fn decoder_soft_view(&self, py: Python<'_>, snapshot: usize) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.decoder_soft_view(snapshot)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
