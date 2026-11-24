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
    /// Constructs immutable caller-supplied resource quantities or admission limits.
    ///
    /// Mirrors the public Rust struct literal; this does not measure storage,
    /// reserve resources or validate a scientific owner.
    ///
    /// # Arguments
    /// * `payload_bytes` - Nonnegative target-usize retained data bytes.
    /// * `ordered_entries`, `shared_roots` - Nonnegative counts; omitted counts
    ///   are zero, independently of the byte quantity.
    ///
    /// # Returns
    /// Owned immutable common quantities with the existing read-only properties.
    ///
    /// # Errors
    /// Native integer conversion rejects negative or target-usize overflow values.
    #[new]
    #[pyo3(signature=(payload_bytes, ordered_entries=0, shared_roots=0))]
    fn new(payload_bytes: usize, ordered_entries: usize, shared_roots: usize) -> Self {
        Self(RetainedFootprint {
            payload_bytes,
            ordered_entries,
            shared_roots,
        })
    }

    /// Reads fixed owned values and requested payload storage, including capacity.
    ///
    /// # Returns
    /// Target-layout bytes excluding allocator/tree/refcount bookkeeping; not RSS
    /// or total heap. The producing operation defines whether these are current
    /// metadata, preflight bounds or caller limits; Python copies are separate.
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
