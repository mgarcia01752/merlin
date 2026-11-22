/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native conversion for the immutable authoritative resource-grid observation.
use super::*;

#[pyclass(frozen, module = "merlin.upstream", name = "ResourceGridSnapshot")]
pub(super) struct PyResourceGridSnapshot(api::ResourceGridSnapshot);

#[pymethods]
impl PyResourceGridSnapshot {
    /// Serializes the owned configured grid through Rust's bounded public export.
    /// # Returns
    /// Lossless original-coordinate JSON, at most eight MiB; work releases the GIL.
    /// It remains usable after session destruction or reconfiguration and starts no run.
    /// # Errors
    /// Shared bounded serialization/transport failures raise the existing ApiError.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }
}

#[pymethods]
impl PySimulation {
    /// Inspects the authoritative configured base-grant resources without executing a waveform.
    /// # Arguments
    /// * `frame_index` - Zero-based original frame inside the configured base grant.
    /// * `first_symbol`, `symbol_count` - Original frame-symbol window; zero count means remaining.
    /// * `first_carrier`, `carrier_count` - Original physical FFT window; zero count means remaining.
    /// * `max_cells` - Positive rectangle budget, at most the Rust projection policy of 16384.
    /// # Returns
    /// Owned typed snapshot with unchanged generation and authoritative plan roles/metadata.
    /// Uses the existing Busy guard and releases the GIL; no source/RNG or session state advances.
    /// # Errors
    /// Missing settings, Busy and shared original extent/work/overflow failures raise ApiError.
    // Six public request fields are mirrored directly; this adapter adds no scientific policy.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(
        text_signature = "($self, frame_index=0, first_symbol=0, symbol_count=1, first_carrier=0, carrier_count=0, max_cells=16384)"
    )]
    #[pyo3(signature=(frame_index=0,first_symbol=0,symbol_count=api::ResourceGridRequest::default().symbol_count,first_carrier=0,carrier_count=0,max_cells=api::ResourceGridRequest::default().max_cells))]
    fn resource_grid(
        &self,
        py: Python<'_>,
        frame_index: usize,
        first_symbol: usize,
        symbol_count: usize,
        first_carrier: usize,
        carrier_count: usize,
        max_cells: usize,
    ) -> PyResult<PyResourceGridSnapshot> {
        self.access_long(py, |session| {
            session.resource_grid(api::ResourceGridRequest {
                frame_index,
                first_symbol,
                symbol_count,
                first_carrier,
                carrier_count,
                max_cells,
            })
        })
        .map(PyResourceGridSnapshot)
    }
}
