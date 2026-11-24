/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable native projection of Rust-owned retained-payload metadata.
use merlin::common::RetainedFootprint;
use pyo3::prelude::*;

#[pyclass(frozen, module = "merlin.common", name = "RetainedFootprint")]
pub(crate) struct PyRetainedFootprint(pub(crate) RetainedFootprint);

#[pymethods]
impl PyRetainedFootprint {
    /// Reads fixed owned values and requested payload storage, including capacity.
    ///
    /// # Returns
    /// Target-layout bytes excluding allocator/tree/refcount bookkeeping; not RSS
    /// or total heap. This does not bound future results or Python-owned copies.
    #[getter]
    fn payload_bytes(&self) -> usize {
        self.0.payload_bytes
    }

    /// Reads ordered-container entries independently of excluded private nodes.
    ///
    /// # Returns
    /// Aggregate entries reachable from the measured Rust root.
    #[getter]
    fn ordered_entries(&self) -> usize {
        self.0.ordered_entries
    }

    /// Reads shared-owner edges with conservatively repeated pointee payloads.
    ///
    /// # Returns
    /// Aggregate shared roots, excluding private reference-count headers in bytes.
    #[getter]
    fn shared_roots(&self) -> usize {
        self.0.shared_roots
    }
}
