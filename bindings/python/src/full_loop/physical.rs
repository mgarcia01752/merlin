/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin original-bin and transport-cell conversion; scientific preparation stays in Rust.
use super::*;
#[pymethods]
impl PyFullLoopResult {
    /// Copies actual original FFT-bin I/Q, power and authoritative phase-specific roles.
    /// # Arguments
    /// * `stage` - Actual TX physical bins or raw RX FFT stage identity.
    /// * `snapshot`, `start`, `count` - Retained event and contiguous window, at most 8192.
    /// # Returns
    /// Owned dictionary preserving missing history and known zeros separately.
    /// # Errors
    /// Shared stage/grid/missing/window/numerical errors raise ApiError.
    #[pyo3(signature=(stage,snapshot=0,start=0,count=0))]
    fn physical_bins(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let stage = crate::physical::stage_id(stage)?;
        let text = py
            .detach(|| {
                let result = self.0.physical_bins(stage, snapshot, start, count)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Copies actual transmitted cells with proven optional packing-role provenance.
    /// # Arguments
    /// * `stage` - Mapped or interleaved transport-cell stage identity.
    /// * `snapshot`, `start`, `count` - Retained event and contiguous window, at most 8192.
    /// # Returns
    /// Owned typed original values; unavailable roles are never inferred from I/Q.
    /// # Errors
    /// Shared stage/missing/window/metadata errors raise ApiError.
    #[pyo3(signature=(stage,snapshot=0,start=0,count=0))]
    fn transmit_cells(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let stage = crate::physical::stage_id(stage)?;
        let text = py
            .detach(|| {
                let result = self.0.transmit_cells(stage, snapshot, start, count)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
