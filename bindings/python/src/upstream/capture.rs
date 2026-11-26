/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Native conversion only; shared Rust owns bounded offline capture inspection.
use super::{api, error};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "UcdCapture"
)]
#[derive(Clone)]
pub(crate) struct PyUcdCapture(api::UcdCapture);

#[pymethods]
impl PyUcdCapture {
    /// Inspects one explicit DA-through-payload message using shared Rust.
    ///
    /// Returns an owned snapshot; malformed and unsupported bytes retain fields
    /// and offsets. No Python parsing, association or simulation work occurs.
    #[staticmethod]
    fn from_mac_management(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::UcdCapture::from_mac_management(bytes))
            .map(Self)
            .map_err(error)
    }

    /// Imports a bounded source envelope through the original Rust decoder.
    ///
    /// Returns the same owned capture as binary admission. Exact source bytes,
    /// envelope validation and structured binary errors remain owned by Rust.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust admission.
    /// * `bytes` - Complete UTF-8 source envelope within the Rust transport policy.
    ///
    /// # Returns
    /// An immutable native capture owning the original binary snapshot.
    ///
    /// # Errors
    /// Shared ApiError for envelope/binary failures; TypeError for non-byte input.
    #[staticmethod]
    fn from_source_json(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::UcdCapture::from_source_json(bytes))
            .map(Self)
            .map_err(error)
    }

    /// Copies exact source bytes into an owned native Python bytes object.
    ///
    /// Returns immutable bytes that remain valid after this snapshot is dropped.
    fn original_bytes(&self, py: Python<'_>) -> Py<PyBytes> {
        PyBytes::new(py, self.0.original_bytes()).unbind()
    }

    /// Uses shared Rust to hash the exact retained management-message envelope.
    ///
    /// Returns lowercase SHA-256; canonical export never replaces the original.
    fn original_sha256(&self, py: Python<'_>) -> String {
        py.detach(|| self.0.original_sha256())
    }

    /// Uses shared Rust to export bounded versioned structural-inspection JSON.
    ///
    /// Returns owned text containing header/TLV/offset/provenance data only.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }

    /// Exports a lossless source envelope with the interpreter released.
    ///
    /// Returns bounded owned JSON containing exact original octets and their
    /// Rust-validated identity; existing inspection JSON remains separate.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust serialization.
    ///
    /// # Returns
    /// Owned UTF-8 JSON text, independent of this capture's lifetime.
    ///
    /// # Errors
    /// Shared ApiError if Rust transport bounds or serialization fail.
    fn to_source_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_source_json()).map_err(error)
    }
}

pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyUcdCapture>()?;
    module.add_class::<PyRxmerCapture>()?;
    module.add_class::<PyPreEqCapture>()?;
    Ok(())
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "PreEqCapture"
)]
#[derive(Clone)]
pub(crate) struct PyPreEqCapture(api::PreEqCapture);
#[pymethods]
impl PyPreEqCapture {
    /// Decodes a bounded bank/update using shared Rust without model mutation.
    ///
    /// Returns owned immutable source provenance and scaled observations.
    #[staticmethod]
    fn from_pnm(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::PreEqCapture::from_pnm(bytes))
            .map(Self)
            .map_err(error)
    }

    /// Imports a bounded source envelope through the original Rust decoder.
    ///
    /// Returns the same owned capture as binary admission. Exact source bytes,
    /// envelope validation and structured binary errors remain owned by Rust.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust admission.
    /// * `bytes` - Complete UTF-8 source envelope within the Rust transport policy.
    ///
    /// # Returns
    /// An immutable native capture owning the original binary snapshot.
    ///
    /// # Errors
    /// Shared ApiError for envelope/binary failures; TypeError for non-byte input.
    #[staticmethod]
    fn from_source_json(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::PreEqCapture::from_source_json(bytes))
            .map(Self)
            .map_err(error)
    }
    /// Copies exact source bytes to an independently owned Python bytes value.
    ///
    /// Retains every source header and signed coefficient encoding.
    fn original_bytes(&self, py: Python<'_>) -> Py<PyBytes> {
        PyBytes::new(py, self.0.original_bytes()).unbind()
    }
    /// Hashes immutable originals in Rust with the interpreter released.
    ///
    /// Returns lowercase SHA-256 text, independent of JSON.
    fn original_sha256(&self, py: Python<'_>) -> String {
        py.detach(|| self.0.original_sha256())
    }
    /// Copies Rust-scaled source I/Q without Python numerical operations.
    ///
    /// Returns an owned list of real/imaginary tuples in physical-grid order.
    fn coefficients(&self) -> Vec<(f64, f64)> {
        self.0.coefficients().iter().map(|z| (z.re, z.im)).collect()
    }
    /// Exports shared observation JSON; missing RNG-RSP context is never guessed.
    ///
    /// Returns owned bounded schema-versioned text.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }

    /// Exports a lossless source envelope with the interpreter released.
    ///
    /// Returns bounded owned JSON containing exact original octets and their
    /// Rust-validated identity; existing inspection JSON remains separate.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust serialization.
    ///
    /// # Returns
    /// Owned UTF-8 JSON text, independent of this capture's lifetime.
    ///
    /// # Errors
    /// Shared ApiError if Rust transport bounds or serialization fail.
    fn to_source_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_source_json()).map_err(error)
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "RxmerCapture"
)]
#[derive(Clone)]
pub(crate) struct PyRxmerCapture(api::RxmerCapture);

#[pymethods]
impl PyRxmerCapture {
    /// Decodes one exact major2 CCAP upstream RxMER file through shared Rust.
    ///
    /// Returns an immutable owned observation; bounds/errors retain source offsets.
    /// No Python parsing, identity association or simulation work occurs.
    #[staticmethod]
    fn from_pnm(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::RxmerCapture::from_pnm(bytes))
            .map(Self)
            .map_err(error)
    }

    /// Imports a bounded source envelope through the original Rust decoder.
    ///
    /// Returns the same owned capture as binary admission. Exact source bytes,
    /// envelope validation and structured binary errors remain owned by Rust.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust admission.
    /// * `bytes` - Complete UTF-8 source envelope within the Rust transport policy.
    ///
    /// # Returns
    /// An immutable native capture owning the original binary snapshot.
    ///
    /// # Errors
    /// Shared ApiError for envelope/binary failures; TypeError for non-byte input.
    #[staticmethod]
    fn from_source_json(py: Python<'_>, bytes: &[u8]) -> PyResult<Self> {
        py.detach(|| api::RxmerCapture::from_source_json(bytes))
            .map(Self)
            .map_err(error)
    }

    /// Copies exact original file bytes into an independently owned bytes object.
    ///
    /// Includes all raw metadata/padding and survives this capture object's life.
    fn original_bytes(&self, py: Python<'_>) -> Py<PyBytes> {
        PyBytes::new(py, self.0.original_bytes()).unbind()
    }

    /// Hashes the exact retained file using shared Rust SHA-256.
    ///
    /// Returns owned lowercase hexadecimal without replacing original bytes.
    fn original_sha256(&self, py: Python<'_>) -> String {
        py.detach(|| self.0.original_sha256())
    }

    /// Copies all original data octets in exact contiguous physical-grid order.
    ///
    /// Missing0xff entries remain present and are never compacted or substituted.
    fn raw_readings(&self, py: Python<'_>) -> Py<PyBytes> {
        PyBytes::new(py, self.0.raw_readings()).unbind()
    }

    /// Copies bounded Rust-decoded dB values, preserving missing readings as None.
    ///
    /// Returns an owned list; no Python arithmetic or power/noise model is applied.
    fn readings_db(&self) -> Vec<Option<f64>> {
        self.0.readings_db().to_vec()
    }

    /// Copies optional UTF-8 rendering of the NUL-terminated raw CCAP identifier.
    ///
    /// Returns None for unrenderable bytes; raw metadata remains authoritative.
    fn ccap_id_text(&self) -> Option<String> {
        self.0.ccap_id_text().map(str::to_owned)
    }

    /// Returns optional object-model0/1 rendering of the preserved raw PreEq flag.
    ///
    /// Unknown flag values return None, never a guessed false or SNMP conversion.
    fn pre_eq_enabled(&self) -> Option<bool> {
        self.0.pre_eq_enabled()
    }

    /// Exports shared versioned decoded-observation JSON with interpretation limits.
    ///
    /// Returns owned bounded text; full raw provenance remains independently usable.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }

    /// Exports a lossless source envelope with the interpreter released.
    ///
    /// Returns bounded owned JSON containing exact original octets and their
    /// Rust-validated identity; existing inspection JSON remains separate.
    ///
    /// # Arguments
    /// * `py` - Current interpreter token, released during Rust serialization.
    ///
    /// # Returns
    /// Owned UTF-8 JSON text, independent of this capture's lifetime.
    ///
    /// # Errors
    /// Shared ApiError if Rust transport bounds or serialization fail.
    fn to_source_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_source_json()).map_err(error)
    }
}
