/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native conversion/lifecycle only; the public Rust API owns idle TX generation.
use crate::{
    admission::{PySourceInput, PyTransmission},
    api, error,
    inputs::PyDpd,
    object_from_json,
    profile_capture::PyProfileCapture,
};
use pyo3::prelude::*;
mod result;
use result::PyTransmitterPreview;

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "TransmitterPreviewControl"
)]
#[derive(Clone)]
pub(crate) struct PyTransmitterControl(api::TransmitterPreviewControl);
#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "TransmitterPreviewRequest"
)]
#[derive(Clone)]
pub(crate) struct PyTransmitterRequest(api::TransmitterPreviewRequest);

pub(super) fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

#[pymethods]
impl PyTransmitterControl {
    /// Creates one-shot scalar progress/cancellation storage without a worker.
    /// # Returns
    /// Idle control; clones share one execution slot and one-way cancellation.
    #[new]
    fn new() -> Self {
        Self(api::TransmitterPreviewControl::new())
    }

    /// Requests cooperative cancellation without locking a running native request.
    /// # Returns
    /// Unit; no pause/resume is exposed and prior results remain valid.
    fn cancel(&self) {
        self.0.cancel();
    }

    /// Reads bounded scalar lifecycle/progress while generation releases the GIL.
    /// # Returns
    /// Owned state/completed/total/cancellation dictionary; pre-start stop stays idle.
    /// # Errors
    /// Preserves Python allocation/transport failures.
    fn snapshot(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.snapshot())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

#[pymethods]
impl PyTransmitterRequest {
    /// Freezes accepted inputs through existing Rust TX admission without advancing a source.
    /// # Arguments
    /// * `transmission` - Validated immutable numerology/timing.
    /// * `profile` - Accepted original/hypothesis profile, retaining physical roles.
    /// * `source` - Passive file/fill or continuous PRBS15 input.
    /// # Returns
    /// Reusable immutable request; only explicit generate performs TX work.
    /// # Errors
    /// Shared geometry/profile/loading/source failures raise ApiError.
    #[new]
    fn new(
        py: Python<'_>,
        transmission: &PyTransmission,
        profile: &PyDpd,
        source: &PySourceInput,
    ) -> PyResult<Self> {
        py.detach(|| api::TransmitterPreviewRequest::new(&transmission.0, &profile.0, &source.0))
            .map(Self)
            .map_err(error)
    }

    /// Admits an original encoded source profile through the same TX preview validator.
    /// # Arguments
    /// * `transmission` - Accepted timing/numerology.
    /// * `capture` - Complete original profile catalog.
    /// * `profile_id` - Encoded identity; current idle TX preview supports 0..15.
    /// * `source` - Passive immutable source.
    /// # Returns
    /// Reusable request bootstrapping supported selected loading as Profile A.
    /// # Errors
    /// Shared absent/unsupported profile and TX admission failures raise ApiError.
    #[staticmethod]
    fn from_capture(
        py: Python<'_>,
        transmission: &PyTransmission,
        capture: &PyProfileCapture,
        profile_id: u8,
        source: &PySourceInput,
    ) -> PyResult<Self> {
        py.detach(|| {
            api::TransmitterPreviewRequest::from_capture(
                &transmission.0,
                &capture.0,
                profile_id,
                &source.0,
            )
        })
        .map(Self)
        .map_err(error)
    }

    /// Measures this existing Rust owner without copying or advancing its data.
    ///
    /// # Returns
    /// Immutable target-layout payload, ordered-entry and shared-root quantities,
    /// including private caches and spare capacity; excludes Python projections
    /// and temporary execution storage. This is not a future-product bound.
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

    /// Generates actual bounded idle TX samples/PSD while releasing the interpreter lock.
    /// # Arguments
    /// * `control` - Optional fresh one-shot progress/cancel token; None creates a fresh token.
    /// # Returns
    /// Owned native immutable result, independent of input/control/session disposal.
    /// # Errors
    /// Shared Busy/engine/cancellation failures raise ApiError; cancellation state is polled.
    #[pyo3(signature=(control=None))]
    fn generate(
        &self,
        py: Python<'_>,
        control: Option<&PyTransmitterControl>,
    ) -> PyResult<PyTransmitterPreview> {
        let control =
            control.map_or_else(api::TransmitterPreviewControl::new, |value| value.0.clone());
        py.detach(|| self.0.generate(&control))
            .map(PyTransmitterPreview)
            .map_err(error)
    }
}

/// Registers immutable requests/controls/results in the existing single extension.
/// # Arguments
/// * `module` - Native import namespace; registration performs no TX work.
/// # Returns
/// Unit on registration; propagates Python type/allocation failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyTransmitterControl>()?;
    module.add_class::<PyTransmitterRequest>()?;
    module.add_class::<PyTransmitterPreview>()
}
