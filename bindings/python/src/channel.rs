/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native ownership/transport over validated configured-channel planning.
use crate::{
    admission::PyTransmission,
    api, error,
    inputs::{PyChannelEstimate, PyDpd},
    object_from_json,
    profile_capture::PyProfileCapture,
    quantities::PyAssociationTolerance,
};
use pyo3::prelude::*;

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "ChannelConfiguration"
)]
#[derive(Clone)]
pub(crate) struct PyChannelConfiguration(api::ChannelConfiguration);

fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

#[pymethods]
impl PyChannelConfiguration {
    /// Owns Rust's validated unity/relative/diagonal default channel parameters.
    /// # Returns
    /// Immutable configuration; no stream, source or PHY simulation starts.
    /// # Errors
    /// Shared parameter errors raise ApiError.
    #[new]
    fn new() -> PyResult<Self> {
        api::ChannelConfiguration::new(api::ChannelConfigurationInput::default())
            .map(Self)
            .map_err(error)
    }

    /// Admits bounded strict JSON through the same typed Rust channel validator.
    /// # Arguments
    /// * `data` - At most 64 KiB settings bytes; unknown fields reject.
    /// # Returns
    /// Immutable accepted loss/power/filter configuration.
    /// # Errors
    /// Shared malformed/resource/parameter failures raise ApiError.
    #[staticmethod]
    fn from_json(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| api::ChannelConfiguration::from_json(data))
            .map(Self)
            .map_err(error)
    }

    /// Serializes exact accepted typed channel settings without changing state.
    /// # Returns
    /// UTF-8 settings JSON, independent of saved-workbench envelope versioning.
    /// # Errors
    /// Preserves structured transport failures.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0)
            .map_err(transport_error)
            .map_err(error)
    }

    /// Reads accepted settings as an owned Python snapshot.
    /// # Returns
    /// Independent loss/power/filter dictionary; edits never change this configuration.
    /// # Errors
    /// Preserves transport and Python allocation failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &self.to_json()?)
    }

    /// Resolves configured electrical/gain/noise references entirely through Rust.
    /// # Returns
    /// Owned configured-reference dictionary; absent calibration retains relative units.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[getter]
    fn power_reference(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.power_reference())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Previews total configured transfer before receiver correction, preserving original H.
    /// # Arguments
    /// * `transmission` - Immutable accepted numerology.
    /// * `profile` - Existing accepted profile and physical role context.
    /// * `response` - Original H; None explicitly selects synthetic unity.
    /// * `tolerance` - Original epoch window; None requires exact matching.
    /// # Returns
    /// Owned original-grid I/Q, bounded gain views/points and actual filter diagnostics.
    /// # Errors
    /// Shared profile/H/epoch/geometry/composition failures raise ApiError.
    #[pyo3(signature=(transmission, profile, response=None, tolerance=None))]
    fn preview(
        &self,
        py: Python<'_>,
        transmission: &PyTransmission,
        profile: &PyDpd,
        response: Option<&PyChannelEstimate>,
        tolerance: Option<&PyAssociationTolerance>,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let view = self.0.preview(
                    &transmission.0,
                    &profile.0,
                    response.map(|value| &value.0),
                    tolerance.map_or(api::AssociationTolerance::ZERO, |value| value.0),
                )?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Previews an original encoded source profile without changing legacy run-ID admission.
    /// # Arguments
    /// * `transmission` - Accepted immutable numerology.
    /// * `capture` - Complete validated original source catalog.
    /// * `profile_id` - Encoded byte-sized source identity.
    /// * `response` - Original H, or None for explicit synthetic unity.
    /// * `tolerance` - Original epoch window; None requires exact matching.
    /// # Returns
    /// Owned configured transfer; channel-plan admission does not execute a PHY run.
    /// # Errors
    /// Shared source/H/epoch/layout/composition failures raise ApiError.
    #[pyo3(signature=(transmission, capture, profile_id, response=None, tolerance=None))]
    fn preview_capture(
        &self,
        py: Python<'_>,
        transmission: &PyTransmission,
        capture: &PyProfileCapture,
        profile_id: u8,
        response: Option<&PyChannelEstimate>,
        tolerance: Option<&PyAssociationTolerance>,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let view = self.0.preview_capture(
                    &transmission.0,
                    &capture.0,
                    profile_id,
                    response.map(|value| &value.0),
                    tolerance.map_or(api::AssociationTolerance::ZERO, |value| value.0),
                )?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

/// Registers the immutable adapter in the existing single native extension.
/// # Arguments
/// * `module` - Existing native namespace; import starts no simulation or input read.
/// # Returns
/// Unit on registration, preserving Python type/allocation failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyChannelConfiguration>()
}
