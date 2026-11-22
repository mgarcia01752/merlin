/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native sweep admission and request dispatch over the public Rust scientific contracts.
use super::*;
use crate::profile_capture::PyProfileCapture;
use pyo3::types::PySequence;

#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopSweepPlan")]
#[derive(Clone)]
pub(crate) struct PyFullLoopSweepPlan(api::FullLoopSweepPlan);

impl PyFullLoopSweepPlan {
    /// Owns an already validated public plan without reconfiguring or executing it.
    /// # Arguments
    /// * `plan` - Idle Rust-owned sweep prepared through the public API.
    /// # Returns
    /// Independent native handle retaining the same immutable inputs and policies.
    pub(crate) fn from_native(plan: api::FullLoopSweepPlan) -> Self {
        Self(plan)
    }
}

#[pymethods]
impl PyFullLoopSweepPlan {
    /// Preflights a complete original selection without starting any profile run.
    /// # Arguments
    /// * `request` - Accepted current request, preserving source/measurements/channel/observations.
    /// * `catalog` - Immutable complete original source catalog with matching ScenarioKey.
    /// * `selection` - current, all_captured or selected; current preserves edited loading.
    /// * `profile_ids` - Required nonempty checked-ID sequence for selected; otherwise absent.
    /// # Returns
    /// Immutable bounded Rust plan; scientific preflight releases the GIL and preserves inputs.
    /// # Errors
    /// Shared selection/key/identity/resource/per-profile admission failures raise ApiError.
    #[new]
    #[pyo3(signature=(request,catalog,selection="current",profile_ids=None))]
    fn new(
        py: Python<'_>,
        request: &PyFullLoopRequest,
        catalog: &PyProfileCapture,
        selection: &str,
        profile_ids: Option<&Bound<'_, PySequence>>,
    ) -> PyResult<Self> {
        let selection = match (selection, profile_ids) {
            ("current", None) => api::FullLoopProfileSelection::Current,
            ("all_captured", None) => api::FullLoopProfileSelection::AllCaptured,
            ("selected", Some(values)) => {
                if values.len()? > api::MAX_FULL_LOOP_SWEEP_PROFILES {
                    return Err(error(api::ApiError::new(
                        api::ErrorCode::ResourceLimit,
                        "Sweep selection exceeds profile budget",
                    )));
                }
                let ids = values
                    .extract::<Vec<u8>>()?
                    .into_iter()
                    .map(api::ProfileId::new)
                    .collect::<api::ApiResult<Vec<_>>>()
                    .map_err(error)?;
                api::FullLoopProfileSelection::Selected(ids)
            }
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    "Sweep mode requires current/all_captured without IDs or selected with IDs",
                )));
            }
        };
        py.detach(|| api::FullLoopSweepPlan::new(&request.0, &catalog.0, selection))
            .map(Self)
            .map_err(error)
    }

    /// Reads full original IDs, sorted selected IDs and finite sequential budget metadata.
    /// # Returns
    /// Owned independent dictionary; editing it changes neither the plan nor accepted inputs.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[getter]
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.info()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Returns one original selected profile request without starting source or scientific work.
    /// # Arguments
    /// * `index` - Original sorted-selection position, not a profile ID.
    /// # Returns
    /// Immutable full-loop request with unchanged source/measurements/channel/options/retention.
    /// Execute it explicitly with its existing independent control; GIL released for preparation.
    /// # Errors
    /// Shared invalid-position and original profile admission failures raise ApiError.
    fn request(&self, py: Python<'_>, index: usize) -> PyResult<PyFullLoopRequest> {
        py.detach(|| self.0.request(index))
            .map(PyFullLoopRequest)
            .map_err(error)
    }
}
