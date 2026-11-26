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
    /// Measures this existing Rust owner without copying or advancing its data.
    ///
    /// # Returns
    /// Immutable target-layout payload, ordered-entry and shared-root quantities,
    /// including private caches and spare capacity; excludes Python projections
    /// and temporary execution storage. This is not a future-product bound.
    ///
    /// # Errors
    /// Preserves Rust ResourceLimit on checked overflow; releases the interpreter.
    fn retained_footprint(
        &self,
        py: Python<'_>,
    ) -> PyResult<crate::retention::PyRetainedFootprint> {
        py.detach(|| self.0.retained_footprint())
            .map(crate::retention::PyRetainedFootprint)
            .map_err(error)
    }

    /// Reads actual idle TX PSD in explicit calibrated or relative electrical units.
    /// # Arguments
    /// * `channel` - Accepted electrical reference; gain, CPA and noise are not applied.
    /// * `start`, `count` - Original FFT viewport; zero count selects the remaining span.
    /// # Returns
    /// Owned PSD and zoom-independent full-band power before the amplifier; GIL released.
    /// # Errors
    /// Shared original geometry, viewport and calibration failures raise ApiError.
    #[pyo3(signature=(channel,start=0,count=0))]
    fn calibrated_spectrum(
        &self,
        py: Python<'_>,
        channel: &crate::channel::PyChannelConfiguration,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let view = self.0.calibrated_spectrum(&channel.0, start, count)?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

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
