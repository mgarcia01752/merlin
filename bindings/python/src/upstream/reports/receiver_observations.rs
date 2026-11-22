/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned dictionary projections of actual public receiver observations.
use super::*;

#[pymethods]
impl PyBatchReport {
    /// Inspects frozen retention availability and original completed-frame identities.
    /// # Returns
    /// Owned Rust-derived dictionary; disabled reports expose zero observation counts.
    /// Work releases the GIL and never runs estimation or changes retained history.
    /// # Errors
    /// Propagates serialization/conversion failures without changing the report.
    fn receiver_observations(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.receiver_observations()))
            .map_err(|cause| error(api::ApiError::Serialization(cause)))?;
        crate::object_from_json(py, &text)
    }

    /// Copies actual received-pilot estimates without replay or configured-response substitution.
    /// # Arguments
    /// * `retained_index` - Original retained inventory position, distinct from scheduled frame.
    /// * `first_carrier`, `carrier_count` - Original physical viewport; zero count means remaining.
    /// * `maximum_points` - Positive copy budget, at most the Rust query ceiling.
    /// # Returns
    /// Owned optional complex coefficients with original identities, estimator and units.
    /// Rust query/serialization releases the GIL; missing bins remain None.
    /// # Errors
    /// Shared unavailable, original-extent and resource-limit failures; native conversion errors.
    #[pyo3(
        text_signature = "($self, retained_index=0, first_carrier=0, carrier_count=0, maximum_points=4096)"
    )]
    #[pyo3(signature=(retained_index=0, first_carrier=0, carrier_count=0, maximum_points=api::ReceiverEstimateRequest::default().maximum_points))]
    fn receiver_estimate(
        &self,
        py: Python<'_>,
        retained_index: usize,
        first_carrier: usize,
        carrier_count: usize,
        maximum_points: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| -> api::ApiResult<String> {
                let snapshot = self.0.receiver_estimate(api::ReceiverEstimateRequest {
                    retained_index,
                    first_carrier,
                    carrier_count,
                    maximum_points,
                })?;
                serde_json::to_string(&snapshot).map_err(api::ApiError::Serialization)
            })
            .map_err(error)?;
        crate::object_from_json(py, &text)
    }
}
