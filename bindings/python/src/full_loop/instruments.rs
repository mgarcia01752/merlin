/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned native conversions; all numerical preparation stays in the public Rust API.
use super::*;

#[pymethods]
impl PyFullLoopResult {
    /// Queries actual retained time-sample PSD through the existing public numerical owner.
    /// # Arguments
    /// * `stage` - Registered time boundary, at most 128 UTF-8 bytes.
    /// * `snapshot` - First actual event; only matching adjacent sample origins may join.
    /// * `max_averages` - Existing Welch analysis work limit, 1..32.
    /// * `start` - First original centered FFT bin.
    /// * `count` - Positive bin count; zero selects the remaining FFT span.
    /// # Returns
    /// Owned metadata/extrema dictionary with actual sample accounting; GIL released for work.
    /// # Errors
    /// Shared missing-stage, invalid-domain/work/viewport or numerical failures raise ApiError.
    #[pyo3(signature=(stage="tx.overlap_add.samples",snapshot=0,max_averages=32,start=0,count=0))]
    fn spectrum(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
        max_averages: usize,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        if stage.len() > 128 {
            return Err(error(api::ApiError::new(
                api::ErrorCode::ResourceLimit,
                "Stage ID exceeds 128 bytes",
            )));
        }
        let stage = serde_json::from_value(serde_json::Value::String(stage.to_owned())).map_err(
            |cause| {
                error(api::ApiError::new(
                    api::ErrorCode::InvalidIdentifier,
                    cause.to_string(),
                ))
            },
        )?;
        let text = py
            .detach(|| {
                let result = self.0.spectrum(api::FullLoopSpectrumQuery {
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

    /// Queries configured H or actual retained receiver H-hat without substituting sources.
    /// # Arguments
    /// * `source` - Exactly configured or receiver_estimate.
    /// * `snapshot` - Actual receiver event; configured source requires zero.
    /// * `velocity_factor` - Explicit finite uniform-cable delay ruler factor in (0,1].
    /// # Returns
    /// Owned bounded original-bin/impulse diagnostics with explicit source and reference plane.
    /// # Errors
    /// Shared missing/incomplete/numerical/velocity failures raise ApiError; no hidden run.
    #[pyo3(signature=(source="configured",snapshot=0,velocity_factor=0.87))]
    fn response(
        &self,
        py: Python<'_>,
        source: &str,
        snapshot: usize,
        velocity_factor: f64,
    ) -> PyResult<Py<PyAny>> {
        let source = match source {
            "configured" if snapshot == 0 => api::FullLoopResponseSource::Configured,
            "receiver_estimate" => api::FullLoopResponseSource::ReceiverEstimate { snapshot },
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    "Response requires configured (snapshot zero) or receiver_estimate",
                )));
            }
        };
        let text = py
            .detach(|| {
                let result = self.0.response(api::FullLoopResponseQuery {
                    source,
                    velocity_factor,
                })?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Summarizes actual retained coding-order payload LDPC soft values without modification.
    /// # Arguments
    /// * `snapshot` - Zero-based retained actual codeword event.
    /// # Returns
    /// Owned 41-bin likelihood statistics and original retained/total/context/sign metadata.
    /// # Errors
    /// Shared unavailable/invalid snapshot or numerical failures raise ApiError.
    #[pyo3(signature=(snapshot=0))]
    fn llr_statistics(&self, py: Python<'_>, snapshot: usize) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.llr_statistics(snapshot)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
