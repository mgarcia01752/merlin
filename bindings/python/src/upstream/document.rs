/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Idle bounded generic configuration inventory, through the shared Rust parser.
use super::{api, error};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "ConfigurationDocument"
)]
#[derive(Clone)]
pub(crate) struct PyDocument(pub(crate) api::ConfigurationDocument);
#[pymethods]
impl PyDocument {
    /// Parses borrowed JSON bytes through the bounded source-valid Rust loader.
    ///
    /// Returns an owned document; unknown fields/versions reject and no job starts.
    #[staticmethod]
    fn from_json(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::ConfigurationDocument::from_json(bytes))
            .map(Self)
            .map_err(error)
    }
    /// Copies exact original UTF-8 JSON bytes, preserving whitespace and null spellings.
    ///
    /// Returns bytes or None when no original exists; later export never rewrites it.
    fn original_bytes(&self, py: Python<'_>) -> Option<Py<PyBytes>> {
        self.0
            .original_bytes()
            .map(|bytes| PyBytes::new(py, bytes).unbind())
    }
    /// Returns SHA-256 over the exact retained original bytes using shared Rust.
    ///
    /// Returns lowercase hexadecimal or None; no Python hashing drives admission.
    fn original_sha256(&self, py: Python<'_>) -> Option<String> {
        py.detach(|| self.0.original_sha256())
    }
    /// Returns every retained generic preset ID in original order.
    ///
    /// Inventory includes unselected entries and never constructs automatic trials.
    fn inventory(&self) -> Vec<String> {
        self.0.inventory().into_iter().map(str::to_owned).collect()
    }
    /// Exports the full same-schema source document through bounded Rust JSON.
    ///
    /// Returns owned UTF-8 text; no original field is silently dropped or executed.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }
}
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyDocument>()?;
    Ok(())
}
