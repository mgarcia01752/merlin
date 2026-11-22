/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin idle configured-response dispatch using the existing session lease.
use super::super::response;
use super::*;

#[pymethods]
impl PySimulation {
    /// Inspects admitted C, deterministic H or their composite without executing a simulation.
    /// # Arguments
    /// * `source` - cm_pre_eq, physical_channel or composite, excluding received estimates.
    /// * `grant_symbol` - Original base-grant symbol for C replay selection.
    /// * `first_carrier`, `carrier_count` - Original physical viewport; zero count means remaining.
    /// * `maximum_points` - Positive frequency-point budget, at most the Rust policy ceiling.
    /// * `include_impulse` - Request the complete periodic inverse independently of frequency zoom.
    /// # Returns
    /// Owned dictionary with unchanged accepted generation; numerical and serialization work
    /// releases the GIL. No source, channel stream, receiver or RNG advances.
    /// # Errors
    /// Existing Busy/MissingSetting and shared source/window/budget/numerical/static-H failures.
    // Six typed request fields plus native self/interpreter parameters preserve the public contract.
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
        let snapshot = self.access_long(py, |session| session.configured_response(request))?;
        let text = py
            .detach(|| serde_json::to_string(&snapshot))
            .map_err(|cause| error(api::ApiError::Serialization(cause)))?;
        crate::object_from_json(py, &text)
    }
}
