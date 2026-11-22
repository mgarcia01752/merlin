/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native conversion and GIL handling for public original-measurement previews.
use crate::{
    api, error,
    inputs::{PyChannelEstimate, PyRxMer},
    object_from_json,
};
use pyo3::prelude::*;

fn snapshot(py: Python<'_>, value: api::MeasurementPreview) -> PyResult<Py<PyAny>> {
    let text = serde_json::to_string(&value).map_err(|cause| {
        error(api::ApiError::new(
            api::ErrorCode::InvalidInput,
            cause.to_string(),
        ))
    })?;
    object_from_json(py, &text)
}

#[pymethods]
impl PyRxMer {
    /// Previews original RxMER via Rust's bounded measurement owner without running a job.
    ///
    /// # Arguments
    /// * `start`, `count` - Original offsets; zero count requests the complete grid.
    /// * `py` - Interpreter ownership for detached work and owned result conversion.
    ///
    /// # Returns
    /// Owned view/grid/provenance/projection dictionary; no input changes.
    ///
    /// # Errors
    /// Raises shared Rust ApiError for invalid bounds or unrepresentable ranges.
    #[pyo3(signature=(start=0, count=0))]
    fn preview(&self, py: Python<'_>, start: usize, count: usize) -> PyResult<Py<PyAny>> {
        let result = py
            .detach(|| self.0.preview(api::MeasurementViewport { start, count }))
            .map_err(error)?;
        snapshot(py, result)
    }
}

#[pymethods]
impl PyChannelEstimate {
    /// Previews original complex H, distinct from reciprocal receiver equalizer gain.
    ///
    /// # Arguments
    /// * `metric` - gain_db, phase_degrees, real or imaginary projection.
    /// * `start`, `count` - Original offsets; zero count requests the complete grid.
    /// * `py` - Interpreter ownership for detached work and owned result conversion.
    ///
    /// # Returns
    /// Owned view/grid/provenance/projection dictionary; missing values remain gaps.
    ///
    /// # Errors
    /// Raises shared ApiError for an unknown metric, invalid viewport or range.
    #[pyo3(signature=(metric="gain_db", start=0, count=0))]
    fn preview(
        &self,
        py: Python<'_>,
        metric: &str,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let metric = match metric {
            "gain_db" => api::ChannelResponseMetric::GainDb,
            "phase_degrees" => api::ChannelResponseMetric::PhaseDegrees,
            "real" => api::ChannelResponseMetric::Real,
            "imaginary" => api::ChannelResponseMetric::Imaginary,
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    "Unknown channel response projection",
                )))
            }
        };
        let result = py
            .detach(|| {
                self.0
                    .preview(api::MeasurementViewport { start, count }, metric)
            })
            .map_err(error)?;
        snapshot(py, result)
    }
}
