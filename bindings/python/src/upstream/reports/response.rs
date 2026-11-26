/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen configured-response dispatch; no received observation is reconstructed.
use super::super::response;
use super::*;

#[pymethods]
impl PyBatchReport {
    /// Reads configured C/H diagnostics from this report's original frozen settings.
    /// # Arguments
    /// * `source` - cm_pre_eq, physical_channel or composite; not a received estimate.
    /// * `grant_symbol` - Original base-grant symbol for C replay selection.
    /// * `first_carrier`, `carrier_count` - Original physical window; zero count means remaining.
    /// * `maximum_points` - Positive frequency display budget with shared Rust validation.
    /// * `include_impulse` - Complete periodic inverse opt-in, independent of the display window.
    /// # Returns
    /// Owned dictionary; Rust numerical/serialization work releases the GIL. Retained run and
    /// generation identities remain absent in reports without a started grant.
    /// # Errors
    /// Shared source/window/budget/numerical and time-varying static-H failures raise ApiError.
    // The six public request fields are mirrored directly without a second simulation policy.
    #[allow(clippy::too_many_arguments)]
    #[pyo3(
        text_signature = "($self, source='physical_channel', grant_symbol=0, first_carrier=0, carrier_count=0, maximum_points=4096, include_impulse=False)"
    )]
    #[pyo3(signature=(source="physical_channel",grant_symbol=0,first_carrier=0,carrier_count=0,maximum_points=api::ConfiguredResponseRequest::default().maximum_points,include_impulse=false))]
    fn configured_response(
        &self,
        py: Python<'_>,
        source: &str,
        grant_symbol: usize,
        first_carrier: usize,
        carrier_count: usize,
        maximum_points: usize,
        include_impulse: bool,
    ) -> PyResult<Py<PyAny>> {
        let request = response::request(
            source,
            grant_symbol,
            first_carrier,
            carrier_count,
            maximum_points,
            include_impulse,
        )?;
        let text = py
            .detach(|| {
                let snapshot = self.0.configured_response(request)?;
                serde_json::to_string(&snapshot).map_err(api::ApiError::Serialization)
            })
            .map_err(error)?;
        crate::object_from_json(py, &text)
    }
}
