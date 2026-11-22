/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native stage policy and owned query conversion; no numerical preparation in the binding.
use super::*;
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "FullLoopObservationPolicy"
)]
#[derive(Clone)]
pub(super) struct PyObservationPolicy(api::FullLoopObservationPolicy);
#[pymethods]
impl PyObservationPolicy {
    /// Constructs the existing finite workbench stage/phase retention policy without a run.
    /// # Returns
    /// Validated immutable policy; only attaching and explicitly running a request captures data.
    /// # Errors
    /// Shared finite-policy validation failures raise ApiError.
    #[new]
    fn new() -> PyResult<Self> {
        api::FullLoopObservationPolicy::new(api::FullLoopObservationInput::default())
            .map(Self)
            .map_err(error)
    }
    /// Admits at most 64 KiB strict JSON through the typed Rust stage-policy owner.
    /// # Arguments
    /// * `data` - Explicit registered stage IDs, phases and finite retention budgets.
    /// # Returns
    /// Frozen policy; no original source, worker or scientific stage advances.
    /// # Errors
    /// Shared malformed/unsupported/duplicate/work/byte-limit failures raise ApiError.
    #[staticmethod]
    fn from_json(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| api::FullLoopObservationPolicy::from_json(data))
            .map(Self)
            .map_err(error)
    }
    /// Copies exact accepted policy as an owned independent native dictionary.
    /// # Returns
    /// Stage IDs/limits/phase selections; editing it cannot change any request.
    /// # Errors
    /// Preserves structured transport/Python allocation failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(self.0.settings())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Serializes exact accepted policy without executing a source or job.
    /// # Returns
    /// Owned UTF-8 policy JSON; transport errors retain their structured category.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0)
            .map_err(transport_error)
            .map_err(error)
    }
}
#[pymethods]
impl PyFullLoopRequest {
    /// Creates a fresh request with validated opt-in retention, preserving this request.
    /// # Arguments
    /// * `policy` - Immutable accepted finite stage/event/element/byte selections.
    /// # Returns
    /// Independent immutable request; no source/RNG/worker state advances.
    /// # Errors
    /// Shared probe selection validation failures raise ApiError.
    fn with_observations(&self, py: Python<'_>, policy: &PyObservationPolicy) -> PyResult<Self> {
        py.detach(|| self.0.with_observations(policy.0.clone()))
            .map(Self)
            .map_err(error)
    }
    /// Reads accepted opt-in stage policy independently of execution options.
    /// # Returns
    /// Another immutable policy wrapper, or None for default capture-disabled requests.
    #[getter]
    fn observation_policy(&self) -> Option<PyObservationPolicy> {
        self.0
            .observation_policy()
            .cloned()
            .map(PyObservationPolicy)
    }
}
#[pymethods]
impl PyFullLoopResult {
    /// Reads the original accepted retention policy independently of request disposal.
    /// # Returns
    /// Immutable policy snapshot, or None for capture-disabled results.
    #[getter]
    fn observation_policy(&self) -> Option<PyObservationPolicy> {
        self.0
            .observation_policy()
            .cloned()
            .map(PyObservationPolicy)
    }

    /// Copies registered retained-stage availability and actual event/element counts.
    /// # Returns
    /// Owned inventory list; missing stages are never filled with ideal placeholders.
    /// # Errors
    /// Preserves structured transport/Python allocation failures.
    fn stage_inventory(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.stage_inventory()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Reads accounted retention and dropped-copy counts independently of core dispatch counts.
    /// # Returns
    /// Owned scalar statistics; byte exhaustion does not change science or completed trials.
    /// # Errors
    /// Preserves structured transport/Python allocation failures.
    fn observation_stats(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.observation_stats())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Copies at most 4096 actual contiguous values with original phase/grid/role coordinates.
    /// # Arguments
    /// * `stage` - Registered original stage string, at most 128 UTF-8 bytes.
    /// * `snapshot` - Zero-based actual retained event for this stage.
    /// * `start` - First retained event-element offset.
    /// * `count` - Positive copy count up to 4096; zero selects up to 4096 remaining values.
    /// # Returns
    /// Owned typed-kind data/context dictionary; no waveform or ideal signal is reconstructed.
    /// # Errors
    /// Shared identifier/missing/overflow/window/budget failures raise ApiError.
    #[pyo3(signature=(stage,snapshot=0,start=0,count=0))]
    fn stage(
        &self,
        py: Python<'_>,
        stage: &str,
        snapshot: usize,
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
                let result =
                    self.0
                        .stage(stage, snapshot, api::FullLoopStageViewport { start, count })?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
