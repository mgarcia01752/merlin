/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned bounded result conversions; scientific queries execute through the facade.
use super::*;
#[pyclass(from_py_object, frozen, module = "merlin", name = "TransmitterPreview")]
#[derive(Clone)]
pub(crate) struct PyTransmitterPreview(pub(super) api::TransmitterPreview);

#[pymethods]
impl PyTransmitterPreview {
    /// Reads scalar original geometry/provenance and actual sample/source accounting.
    /// # Returns
    /// Owned metadata dictionary; input deletion and later calls cannot invalidate it.
    /// # Errors
    /// Preserves Python allocation/structured transport failures.
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.info()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Copies a bounded contiguous actual post-overlap sample window.
    /// # Arguments
    /// * `start` - Original retained-sample offset; defaults to zero.
    /// * `count` - Up to 4096 samples; zero selects up to 4096 remaining samples.
    /// # Returns
    /// Owned actual I/Q and absolute first-sample index; no resampling occurs.
    /// # Errors
    /// Shared invalid/overflow/resource-window failures raise ApiError.
    #[pyo3(signature=(start=0,count=0))]
    fn sample_view(&self, py: Python<'_>, start: usize, count: usize) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.sample_view(start, count)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries existing centered PSD extrema with original frequency/normalization metadata.
    /// # Arguments
    /// * `start` - First centered FFT bin; defaults to zero.
    /// * `count` - Selected bin count; zero selects the remaining FFT span.
    /// # Returns
    /// Owned dictionary with at most 2048 min/max buckets and explicit normalized units.
    /// # Errors
    /// Shared invalid/overflow/outside FFT failures raise ApiError.
    #[pyo3(signature=(start=0,count=0))]
    fn spectrum(&self, py: Python<'_>, start: usize, count: usize) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.spectrum(start, count)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
