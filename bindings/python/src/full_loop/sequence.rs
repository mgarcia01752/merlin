/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit sequence diagnostic conversion; no sample stitching or numerical work in Python.
use super::*;
#[pymethods]
impl PyFullLoopResult {
    /// Calibrates an explicit ordered-frame diagnostic through the same public Rust owner.
    /// # Arguments
    /// * `stage` - IFFT, prefix, window or RX-useful stage identifier.
    /// * `snapshot` - First actual retained event; no missing history is generated.
    /// * `max_averages` - Existing Welch analysis policy, 1..32 complete windows.
    /// * `start`, `count` - Original FFT-bin viewport; zero count selects the remaining span.
    /// # Returns
    /// Owned normalized sequence metadata and calibrated/relative power from one estimate.
    /// Original segment gaps/overlaps and contiguity stay explicit. Numerical work releases
    /// the GIL and changes neither input, retained result nor completed decoder outcomes.
    /// # Errors
    /// Shared sequence, grid, calibration, work and viewport failures raise ApiError.
    /// Nonrepresentable native integer conversion preserves Python's conversion error.
    #[pyo3(
        text_signature = "($self, stage='tx.ifft.samples', snapshot=0, max_averages=32, start=0, count=0)"
    )]
    #[pyo3(signature=(stage="tx.ifft.samples",snapshot=0,max_averages=api::FullLoopSpectrumQuery::default().max_averages,start=0,count=0))]
    fn calibrated_spectrum_sequence(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        max_averages: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let stage = crate::physical::stage_id(stage)?;
        let text = py
            .detach(|| {
                let result = self
                    .0
                    .calibrated_spectrum_sequence(api::FullLoopSpectrumQuery {
                        stage,
                        snapshot,
                        max_averages,
                        start,
                        count,
                    })?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Inspects an explicit ordered-frame diagnostic PSD through the public Rust owner.
    /// # Arguments
    /// * `stage` - IFFT, prefix, window or receiver useful stage; at most 128 UTF-8 bytes.
    /// * `snapshot` - First actual retained event.
    /// * `max_averages` - Existing Welch analysis work budget, 1..32.
    /// * `start`, `count` - Original FFT-bin viewport; zero count means remaining bins.
    /// # Returns
    /// Owned normalized PSD, original frame segments and explicit physical contiguity/stop state.
    /// No source advances, missing history is never filled, and work releases the GIL.
    /// # Errors
    /// Shared missing/context/stage/work/geometry/window/numerical failures raise ApiError.
    // Runtime defaults come from the public Rust query. PyO3 cannot render a Rust
    // expression as a Python numeric default, so this metadata is pinned by an import test.
    #[pyo3(
        text_signature = "($self, stage='tx.ifft.samples', snapshot=0, max_averages=32, start=0, count=0)"
    )]
    #[pyo3(signature=(stage="tx.ifft.samples",snapshot=0,max_averages=api::FullLoopSpectrumQuery::default().max_averages,start=0,count=0))]
    fn spectrum_sequence(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        max_averages: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let stage = crate::physical::stage_id(stage)?;
        let text = py
            .detach(|| {
                let result = self.0.spectrum_sequence(api::FullLoopSpectrumQuery {
                    stage,
                    snapshot,
                    max_averages,
                    start,
                    count,
                })?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
