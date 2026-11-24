/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin conversions over original profile capture metadata and bounded scientific views.
use crate::{
    api, error,
    inputs::{named_input, PyDpd},
    object_from_json,
};
use merlin::downstream::inputs::{read_capture, CaptureLoader};
use pyo3::{
    prelude::*,
    types::{PySequence, PyString, PyTuple},
};
use std::path::PathBuf;

#[pyclass(from_py_object, frozen, module = "merlin", name = "ProfileViewRequest")]
#[derive(Clone)]
pub(crate) struct PyProfileView(pub(crate) api::ProfileViewRequest);
#[pyclass(from_py_object, frozen, module = "merlin", name = "ProfileCapture")]
#[derive(Clone)]
pub(crate) struct PyProfileCapture(pub(crate) api::ProfileCapture);

fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

fn role(name: &str) -> PyResult<api::ProfileCarrierRole> {
    match name {
        "data" => Ok(api::ProfileCarrierRole::Data),
        "continuous_pilot" => Ok(api::ProfileCarrierRole::ContinuousPilot),
        "plc" => Ok(api::ProfileCarrierRole::Plc),
        "exclusion" => Ok(api::ProfileCarrierRole::Exclusion),
        "zero_bit_loaded" => Ok(api::ProfileCarrierRole::ZeroBitLoaded),
        "reserved" => Ok(api::ProfileCarrierRole::Reserved),
        "unknown" => Ok(api::ProfileCarrierRole::Unknown),
        _ => Err(error(api::ApiError::new(
            api::ErrorCode::InvalidInput,
            "Unknown profile carrier role",
        ))),
    }
}

#[pymethods]
impl PyProfileView {
    /// Owns a checked original-profile selection and bounded output request.
    /// # Arguments
    /// * `profiles` - Original byte-sized IDs; None selects the complete catalog, [] no trace.
    /// * `roles` - Backend role names; None selects every existing role, [] no role.
    /// * `metric` - bits or shannon_db, separate from receiver operating requirements.
    /// * `start`, `count` - Original carrier viewport; zero count selects the full grid.
    /// * `max_buckets` - Total scientific output budget across selected profiles, 1..2048.
    /// # Returns
    /// Immutable request; exact identity/window checks execute against the source capture.
    /// # Errors
    /// Shared resource failures raise ApiError; primitive conversion failures stay Python errors.
    #[new]
    #[pyo3(signature=(profiles=None, roles=None, metric="bits", start=0, count=0, max_buckets=2048))]
    fn new(
        profiles: Option<&Bound<'_, PySequence>>,
        roles: Option<&Bound<'_, PySequence>>,
        metric: &str,
        start: usize,
        count: usize,
        max_buckets: usize,
    ) -> PyResult<Self> {
        if let Some(values) = profiles {
            if values.len()? > api::MAX_PROFILE_SOURCE_IDS {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::ResourceLimit,
                    "Profile selection exceeds source-ID bounds",
                )));
            }
        }
        let profiles = profiles
            .map(|values| values.extract::<Vec<u8>>())
            .transpose()?;
        let roles = if let Some(values) = roles {
            if values.len()? > api::ProfileCarrierRole::ALL.len() {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::ResourceLimit,
                    "Profile role selection exceeds bounds",
                )));
            }
            values
                .try_iter()?
                .map(|item| {
                    let item = item?;
                    role(item.cast::<PyString>()?.to_str()?)
                })
                .collect::<PyResult<Vec<_>>>()?
        } else {
            api::ProfileCarrierRole::ALL.to_vec()
        };
        let metric = match metric {
            "bits" => api::ProfileMetric::BitLoading,
            "shannon_db" => api::ProfileMetric::IdealShannonSnr,
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    "Unknown profile scientific metric",
                )))
            }
        };
        let request = api::ProfileViewRequest {
            profiles,
            roles,
            metric,
            viewport: api::MeasurementViewport { start, count },
            max_buckets,
        };
        request.validate().map_err(error)?;
        Ok(Self(request))
    }
}

#[pymethods]
impl PyProfileCapture {
    /// Previews explicit synthetic unity H through the public Rust numerical owner.
    /// # Arguments
    /// * `profile_id` - Original byte identity, independent of simulation-ID admission.
    /// * `metric` - gain_db, phase_degrees, real or imaginary.
    /// * `start`, `count` - Original source offsets; zero count restores full span.
    /// # Returns
    /// Owned synthetic reference/grid/view dictionary without invented capture provenance.
    /// # Errors
    /// Shared identity/viewport/projection errors raise ApiError; work releases the GIL.
    #[pyo3(signature=(profile_id, metric="gain_db", start=0, count=0))]
    fn unity_response_preview(
        &self,
        py: Python<'_>,
        profile_id: u8,
        metric: &str,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let metric = crate::measurement::response_metric(metric)?;
        let text = py
            .detach(|| {
                let report = self.0.unity_response_preview(
                    profile_id,
                    api::MeasurementViewport { start, count },
                    metric,
                )?;
                serde_json::to_string(&report).map_err(|cause| {
                    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
                })
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Loads an original binary/JSON profile capture through the existing native decoder.
    /// # Arguments
    /// * `path` - Original caller-selected file; no output is written.
    /// # Returns
    /// Immutable complete catalog with exact original/intermediate provenance.
    /// # Errors
    /// Shared file/format/contract/profile/resource failures raise ApiError.
    #[staticmethod]
    fn from_file(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        py.detach(|| CaptureLoader::profile_capture(&read_capture(path)?))
            .map(Self)
            .map_err(error)
    }

    /// Admits exact original bytes by content rather than extension.
    /// # Arguments
    /// * `name` - Original display identity.
    /// * `data` - Bounded original binary/JSON bytes, checked before native copying.
    /// # Returns
    /// Immutable read-only catalog; no simulation or source advance occurs.
    /// # Errors
    /// Shared input/format/contract/profile/resource failures raise ApiError.
    #[staticmethod]
    fn from_bytes(py: Python<'_>, name: &str, data: &[u8]) -> PyResult<Self> {
        let input = named_input(name, data)?;
        py.detach(|| CaptureLoader::profile_capture(&input))
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

    /// Admits original selected loading in the existing checked simulation-ID domain.
    /// # Arguments
    /// * `profile_id` - Existing ProfileId, 0..15 and present in the original capture.
    /// # Returns
    /// Immutable DPD with exact selected loading/provenance and supported run-ID inventory.
    /// This capture retains every raw source ID; no simulation starts and work releases the GIL.
    /// # Errors
    /// Shared absent-identity failures raise ApiError; unrelated raw IDs remain preserved.
    fn simulation_profile(
        &self,
        py: Python<'_>,
        profile_id: &crate::quantities::PyProfileId,
    ) -> PyResult<PyDpd> {
        py.detach(|| self.0.simulation_profile(profile_id.0))
            .map(PyDpd)
            .map_err(error)
    }

    /// Reads every original encoded source ID, independent of the legacy runtime quantity.
    /// # Returns
    /// Immutable sorted tuple of byte-sized source identities.
    #[getter]
    fn profile_ids<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyTuple>> {
        PyTuple::new(py, self.0.profile_ids().iter().copied())
    }

    /// Reads original header/key and both sorted/source-order catalogs.
    /// # Returns
    /// Owned small metadata dictionary; compact ranges have a separate bounded query.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[getter]
    fn metadata(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.metadata())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads exact original/intermediate provenance without parsing another capture.
    /// # Returns
    /// Owned metadata dictionary, preserving original encoding/hash/epoch/decoder declaration.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[getter]
    fn provenance(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(self.0.provenance())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Copies one original source frame through Rust's read-only catalog operation.
    /// # Arguments
    /// * `profile_id` - Original byte identity, independent of checked simulation IDs.
    /// # Returns
    /// Owned bounded key/epoch/decoded-source/assignment dictionary. Editing the copy
    /// cannot change the catalog; original capture provenance is available separately.
    /// # Errors
    /// Missing identities raise InvalidIdentifier; transport/allocation failures remain errors.
    fn frame(&self, py: Python<'_>, profile_id: u8) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(self.0.frame(profile_id)?).map_err(transport_error))
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads authoritative source role counts and uncoded useful-symbol capacity.
    /// # Arguments
    /// * `profile_id` - Original source identity, never an array index or wire assignment.
    /// # Returns
    /// Owned scientific summary dictionary; no measured/net-throughput inference.
    /// # Errors
    /// Unavailable identities raise shared InvalidIdentifier errors.
    fn summary(&self, py: Python<'_>, profile_id: u8) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let summary = self.0.summary(profile_id)?;
                serde_json::to_string(&summary).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads a bounded original compact-scheme page and authoritative source coordinates.
    /// # Arguments
    /// * `profile_id` - Original encoded source identity.
    /// * `start` - Zero-based original scheme offset.
    /// * `limit` - Maximum returned rows, 1..256.
    /// # Returns
    /// Owned page with source codes, roles, loadings, constellation and ideal references.
    /// # Errors
    /// Shared identity/window/resource failures raise ApiError.
    #[pyo3(signature=(profile_id, start=0, limit=256))]
    fn schemes(
        &self,
        py: Python<'_>,
        profile_id: u8,
        start: usize,
        limit: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let page = self.0.schemes(profile_id, start, limit)?;
                serde_json::to_string(&page).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Builds a bounded scientific view from the exact original capture.
    /// # Arguments
    /// * `request` - Immutable source IDs/roles/metric/window/total budget.
    /// # Returns
    /// Owned raw extrema/role support/coordinates/provenance; no pixel normalization.
    /// # Errors
    /// Shared identity/window/resource failures raise ApiError without changing the capture.
    fn inspect(&self, py: Python<'_>, request: &PyProfileView) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let view = self.0.inspect(&request.0)?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

#[pymethods]
impl PyDpd {
    /// Uses the same backend summary owner for this accepted original/hypothesis profile.
    /// # Returns
    /// Owned source-role and uncoded-capacity summary with explicit profile origin.
    /// # Errors
    /// Shared rate/size/transport failures raise ApiError.
    fn summary(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let summary = self.0.summary()?;
                serde_json::to_string(&summary).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
    /// Views this accepted profile through the same bounded scientific owner.
    /// # Arguments
    /// * `request` - Immutable roles/metric/window; explicit IDs must name this profile.
    /// # Returns
    /// Owned view retaining original/created/edited origin and all actual provenance.
    /// # Errors
    /// Shared identity/window/resource failures raise ApiError.
    fn view(&self, py: Python<'_>, request: &PyProfileView) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let view = self.0.view(&request.0)?;
                serde_json::to_string(&view).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

/// Registers thin immutable capture/request classes in the existing extension.
/// # Arguments
/// * `module` - Existing native namespace; import loads no file or simulation.
/// # Returns
/// Unit on registration, preserving Python type/allocation failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyProfileView>()?;
    module.add_class::<PyProfileCapture>()
}
