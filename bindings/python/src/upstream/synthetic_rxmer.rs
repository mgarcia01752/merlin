/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin native access to the Rust-owned synthetic CCAP RxMER generator.
use super::error;
use pnm_input::upstream::{SyntheticRxmerCapture, MAX_SYNTHETIC_RXMER_REQUEST_BYTES};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "SyntheticRxmerCapture"
)]
#[derive(Clone)]
struct PySyntheticRxmerCapture(SyntheticRxmerCapture);

#[pymethods]
impl PySyntheticRxmerCapture {
    /// Rust-owned request byte limit, independent of wire/artifact sizes.
    #[classattr]
    const MAX_REQUEST_BYTES: usize = MAX_SYNTHETIC_RXMER_REQUEST_BYTES;

    /// Generates an owned synthetic artifact through shared Rust validation.
    ///
    /// Args:
    ///     request: UTF-8 request JSON; bounded before parsing in Rust.
    /// Returns:
    ///     An immutable artifact without file I/O, measurement or simulation.
    /// Raises:
    ///     ApiError: Shared syntax, domain, unsupported-profile or resource errors.
    #[staticmethod]
    fn from_json(py: Python<'_>, request: &str) -> PyResult<Self> {
        py.detach(|| SyntheticRxmerCapture::from_json(request))
            .map(Self)
            .map_err(error)
    }

    /// Copies standard-format binary bytes into an independently owned value.
    ///
    /// Returns:
    ///     Immutable bytes surviving this artifact; keep the provenance sidecar.
    fn original_bytes(&self, py: Python<'_>) -> Py<PyBytes> {
        PyBytes::new(py, self.0.capture().original_bytes()).unbind()
    }

    /// Copies the Rust-generated synthetic provenance and conversion summary.
    ///
    /// Returns:
    ///     Owned JSON with the original request, binary hash and synthetic origin.
    /// Raises:
    ///     ApiError: Shared serialization or artifact-size errors.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }
}

/// Registers the real native generator without constructing or running anything.
///
/// # Arguments
/// * `module` - Existing upstream namespace in the one native distribution.
///
/// # Returns
/// Unit after adding the immutable generator wrapper.
///
/// # Errors
/// Propagates Python registration/allocation errors.
pub(super) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PySyntheticRxmerCapture>()
}
