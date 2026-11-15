/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin bindings over the same Rust PNM encoder used by Designer.
use crate::{api, error, inputs::PyRxMer, object_from_json};
use pnm_input::rxmer_export::{self, ExportDestination, ExportError, ExportFormat, RangePolicy};
use pyo3::{prelude::*, types::PyBytes};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicBool, AtomicUsize},
};
fn policy(clamp: bool) -> RangePolicy {
    if clamp {
        RangePolicy::Clamp
    } else {
        RangePolicy::Reject
    }
}
fn export_error(value: ExportError) -> PyErr {
    let code = match value.code {
        "io" => api::ErrorCode::Io,
        "limit" => api::ErrorCode::ResourceLimit,
        _ => api::ErrorCode::InvalidInput,
    };
    let exception = error(api::ApiError::new(code, value.to_string()));
    Python::attach(|py| {
        let _ = exception.value(py).setattr("export_code", value.code);
        let _ = exception.value(py).setattr("capture", value.capture);
        let _ = exception.value(py).setattr("subcarrier", value.subcarrier);
    });
    exception
}
#[pymethods]
impl PyRxMer {
    /// Converts a loaded capture into full-precision decoded JSON in Rust.
    /// # Returns
    /// Owned UTF-8 bytes with unchanged finite values, missing nulls and provenance.
    /// # Errors
    /// Raises structured ApiError for invalid metadata or nonfinite readings.
    fn to_json_bytes(&self, py: Python<'_>) -> PyResult<Py<PyBytes>> {
        let encoded = py
            .detach(|| rxmer_export::encode_json(self.0.frame()))
            .map_err(export_error)?;
        Ok(PyBytes::new(py, &encoded.bytes).unbind())
    }
    /// Saves new decoded JSON and its recipe/source provenance sidecar in Rust.
    /// # Arguments
    /// * `path` - New destination; existing files and sidecars are never overwritten.
    /// # Returns
    /// None after successful publication; original captures remain unchanged.
    /// # Errors
    /// Preserves structured validation, existing-path and I/O failures.
    fn save_json(&self, py: Python<'_>, path: PathBuf) -> PyResult<()> {
        py.detach(|| {
            rxmer_export::save_with_format(
                ExportDestination::Single(&path),
                std::slice::from_ref(self.0.frame()),
                RangePolicy::Reject,
                &serde_json::json!({"adapter":"Python","source":"loaded RxMER"}),
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
                ExportFormat::Json,
            )
        })
        .map(|_| ())
        .map_err(export_error)
    }
    /// Converts this loaded RxMER to owned PNM 1.0 bytes through Rust validation.
    /// # Arguments
    /// * `clamp` - Explicitly opt into clipping outside 0..63.5 dB; default false.
    /// # Returns
    /// New binary bytes; quarter-dB rounding preserves missing samples and source data.
    /// No file is written or simulation started. Export provenance needs save_pnm.
    /// # Errors
    /// Raises structured ApiError with export_code/capture/subcarrier context.
    #[pyo3(signature=(*,clamp=false))]
    fn to_pnm_bytes(&self, py: Python<'_>, clamp: bool) -> PyResult<Py<PyBytes>> {
        let encoded = py
            .detach(|| rxmer_export::encode(self.0.frame(), policy(clamp)))
            .map_err(export_error)?;
        Ok(PyBytes::new(py, &encoded.bytes).unbind())
    }
    /// Computes a full-grid export summary using the shared Rust encoder.
    /// # Arguments
    /// * `clamp` - Explicit out-of-range conversion policy; defaults to rejection.
    /// # Returns
    /// Independent counts, rounding/clipping changes and encoded byte count.
    /// # Errors
    /// Preserves the same structured export failures as actual conversion.
    #[pyo3(signature=(*,clamp=false))]
    fn pnm_export_summary(&self, py: Python<'_>, clamp: bool) -> PyResult<Py<PyAny>> {
        let output = py
            .detach(|| rxmer_export::encode(self.0.frame(), policy(clamp)))
            .map_err(export_error)?;
        object_from_json(
            py,
            &serde_json::to_string(&output.summary).map_err(|value| {
                error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    value.to_string(),
                ))
            })?,
        )
    }
    /// Saves a new PNM file and its MERLIN provenance sidecar through Rust I/O.
    /// # Arguments
    /// * `path` - New destination, never an input replacement.
    /// * `clamp` - Explicitly enable range clipping; nonfinite readings always fail.
    /// # Returns
    /// None after publishing the new file and `<path>.merlin.json` sidecar.
    /// Original samples, bytes, identities and capture times remain unchanged.
    /// # Errors
    /// Preserves validation, existing-path and filesystem errors with export context.
    #[pyo3(signature=(path,*,clamp=false))]
    fn save_pnm(&self, py: Python<'_>, path: PathBuf, clamp: bool) -> PyResult<()> {
        py.detach(|| {
            rxmer_export::save(
                ExportDestination::Single(&path),
                std::slice::from_ref(self.0.frame()),
                policy(clamp),
                &serde_json::json!({"adapter":"Python","source":"loaded RxMER"}),
                &AtomicBool::new(false),
                &AtomicUsize::new(0),
            )
        })
        .map(|_| ())
        .map_err(export_error)
    }
}
