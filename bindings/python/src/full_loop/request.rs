/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed input collection and independent request conversion, preserving the legacy session API.
use super::*;
use crate::{
    admission::{PySourceInput, PyTransmission},
    channel::PyChannelConfiguration,
    controller::PyMerlin,
    inputs::{PyChannelEstimate, PyDpd},
    sequence::PySequence,
};
#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopRequest")]
#[derive(Clone)]
pub(super) struct PyFullLoopRequest(pub(super) api::FullLoopRequest);

#[pymethods]
impl PyMerlin {
    /// Prepares a reusable full-loop request without changing this class's existing sessions.
    /// # Arguments
    /// * `transmission`, `profile`, `source`, `measurements`, `channel` - Accepted passive typed inputs.
    /// * `response` - Optional original H; None explicitly selects synthetic unity.
    /// * `options` - Optional explicit full-loop policy; None selects bounded Rust defaults.
    /// # Returns
    /// Immutable independent request; source/RNG/worker state is not advanced.
    /// # Errors
    /// Shared full-loop admission/limits/identity/epoch/coverage failures raise ApiError.
    #[staticmethod]
    #[pyo3(signature=(transmission,profile,source,measurements,channel,response=None,options=None))]
    #[allow(clippy::too_many_arguments)]
    fn prepare_full_loop(
        py: Python<'_>,
        transmission: &PyTransmission,
        profile: &PyDpd,
        source: &PySourceInput,
        measurements: &PySequence,
        channel: &PyChannelConfiguration,
        response: Option<&PyChannelEstimate>,
        options: Option<&PyFullLoopOptions>,
    ) -> PyResult<PyFullLoopRequest> {
        let inputs = api::FullLoopInputs {
            transmission: transmission.0.clone(),
            profile: profile.0.clone(),
            source: source.0.clone(),
            measurements: measurements.0.clone(),
            channel: channel.0.clone(),
            response: response.map(|value| value.0.clone()),
        };
        let options = options.map_or_else(api::FullLoopOptions::default, |value| value.0.clone());
        py.detach(|| api::Simulation::prepare_full_loop(inputs, options))
            .map(PyFullLoopRequest)
            .map_err(error)
    }

    /// Preserves independently validated H/profile and RxMER/profile timestamp policies.
    /// # Arguments
    /// * `transmission`, `profile`, `source`, `measurements`, `channel` - Accepted passive inputs.
    /// * `tolerance` - Checked H/profile epoch distance in whole seconds.
    /// * `response` - Optional original H; None explicitly selects unity.
    /// * `options` - Optional finite execution policy; None selects existing defaults.
    /// # Returns
    /// Owned idle request; preparation releases the GIL and leaves existing sessions unchanged.
    /// # Errors
    /// Preserves Rust scientific, association, identity and resource failure categories.
    #[staticmethod]
    #[pyo3(signature=(transmission,profile,source,measurements,channel,tolerance,response=None,options=None))]
    #[allow(clippy::too_many_arguments)]
    fn prepare_full_loop_with_response_tolerance(
        py: Python<'_>,
        transmission: &PyTransmission,
        profile: &PyDpd,
        source: &PySourceInput,
        measurements: &PySequence,
        channel: &PyChannelConfiguration,
        tolerance: &crate::quantities::PyAssociationTolerance,
        response: Option<&PyChannelEstimate>,
        options: Option<&PyFullLoopOptions>,
    ) -> PyResult<PyFullLoopRequest> {
        let inputs = api::FullLoopInputs {
            transmission: transmission.0.clone(),
            profile: profile.0.clone(),
            source: source.0.clone(),
            measurements: measurements.0.clone(),
            channel: channel.0.clone(),
            response: response.map(|value| value.0.clone()),
        };
        let options = options.map_or_else(api::FullLoopOptions::default, |value| value.0.clone());
        py.detach(|| {
            api::Simulation::prepare_full_loop_with_response_tolerance(inputs, options, tolerance.0)
        })
        .map(PyFullLoopRequest)
        .map_err(error)
    }
}
#[pymethods]
impl PyFullLoopRequest {
    /// Measures the accepted request's complete current Rust ownership.
    ///
    /// # Returns
    /// Immutable target-layout payload/entry/root metadata, including private
    /// playback caches; no source advancement, serialization or payload copy.
    ///
    /// # Errors
    /// Preserves Rust ResourceLimit on checked overflow; releases the interpreter.
    fn retained_footprint(
        &self,
        py: Python<'_>,
    ) -> PyResult<crate::retention::PyRetainedFootprint> {
        py.detach(|| self.0.retained_footprint())
            .map(crate::retention::PyRetainedFootprint)
            .map_err(error)
    }

    /// Copies frozen exact scientific settings; later caller edits cannot affect the request.
    /// # Returns
    /// Owned settings dictionary, separate from mutable legacy Simulation evaluation state.
    /// # Errors
    /// Preserves Python allocation/structured transport failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(self.0.settings()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Reads frozen accepted full-loop options as another immutable native snapshot.
    /// # Returns
    /// Owned native options; dictionary edits never change the request.
    #[getter]
    fn options(&self) -> PyFullLoopOptions {
        PyFullLoopOptions(self.0.options().clone())
    }

    /// Copies the accepted H/profile timestamp policy independently of RxMER settings.
    /// # Returns
    /// Owned checked tolerance in whole seconds; inspection starts no scientific work.
    #[getter]
    fn response_tolerance(&self) -> crate::quantities::PyAssociationTolerance {
        crate::quantities::PyAssociationTolerance(self.0.response_tolerance())
    }

    /// Narrows the request's active host-runtime limit without starting scientific work.
    ///
    /// # Arguments
    /// * `max_runtime_ms` - Positive active host milliseconds within the original request budget.
    ///
    /// # Returns
    /// Independent native request preserving original inputs, settings and observations.
    /// The original remains reusable and unchanged; pause time stays excluded from runtime.
    ///
    /// # Errors
    /// Native integer conversion and shared Rust runtime/widening validation failures.
    fn with_runtime_limit(&self, max_runtime_ms: u64) -> PyResult<Self> {
        self.0
            .with_runtime_limit(max_runtime_ms)
            .map(Self)
            .map_err(error)
    }
    /// Executes the existing full loop with independent safe-boundary controls and GIL release.
    /// # Arguments
    /// * `control` - Optional fresh one-shot token; None creates a fresh directional control.
    /// # Returns
    /// Owned actual complete/partial result; interpreter interrupts request cooperative stop.
    /// # Errors
    /// Shared Busy/construction errors raise ApiError; runtime/step/cancel outcomes retain reports.
    #[pyo3(signature=(control=None))]
    fn run(
        &self,
        py: Python<'_>,
        control: Option<&PyFullLoopControl>,
    ) -> PyResult<PyFullLoopResult> {
        let control = control.map_or_else(api::FullLoopControl::new, |value| value.0.clone());
        worker::execute(py, self.0.clone(), control).map(PyFullLoopResult)
    }
}
