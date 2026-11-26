/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Stateless owned reference projections through the accepted public Rust namespace.
use crate::{api, controller::PyMerlin, error, object_from_json};
use pyo3::prelude::*;
fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}
#[pymethods]
impl PyMerlin {
    /// Copies original label-ordered reference points through the Rust modulation owner.
    /// # Arguments
    /// * `order` - Existing physical constellation size, including control references.
    /// * `amplitude` - Positive finite known reference multiplier, default unit energy.
    /// * `start`, `count` - Original label window; zero copies up to 4096 remaining points.
    /// # Returns
    /// Owned labelled coordinates and normalization metadata; GIL released and no session run.
    /// # Errors
    /// Shared unsupported-order/label/work/amplitude/numerical failures raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(order,amplitude=1.0,start=0,count=0))]
    fn constellation_reference(
        py: Python<'_>,
        order: u16,
        amplitude: f64,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result =
                    api::Simulation::constellation_reference(order, amplitude, start, count)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries a finite original-reference nearest-region without configuring any simulation.
    /// # Arguments
    /// * `order`, `label` - Existing physical constellation and original binary label.
    /// * `amplitude` - Explicit positive finite reference multiplier, never fitted to samples.
    /// # Returns
    /// Owned center-relative polygon, finite viewport and up to 49 labelled nearby references.
    /// # Errors
    /// Shared reference/label/amplitude/work/finite-range failures raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(order,label,amplitude=1.0))]
    fn decision_region(
        py: Python<'_>,
        order: u16,
        label: u16,
        amplitude: f64,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = api::Simulation::decision_region(order, label, amplitude)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
