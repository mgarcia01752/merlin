/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Saved WB conversion; all decoding, validation and simulation remain Rust-owned.
use crate::{
    api,
    controller::{PyMerlin, PyRunControl},
    error,
    options::PyEvaluationOptions,
    quantities::PyProfileId,
    reports::PyBatchReport,
};
use pyo3::prelude::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Independent profile controllers configured from a self-contained saved WB.
#[pyclass(frozen, module = "merlin", name = "Workbench")]
pub(crate) struct PyWorkbench {
    controllers: Vec<PyMerlin>,
    ids: Vec<api::ProfileId>,
    options: Vec<api::EvaluationOptions>,
    _original: Arc<[u8]>,
    busy: Mutex<()>,
}
impl PyWorkbench {
    fn from_native(value: pnm_input::Workbench) -> Self {
        let ids = value
            .controllers()
            .iter()
            .map(|sim| sim.profile().expect("preflighted WB profile").profile_id())
            .collect();
        let options = value
            .controllers()
            .iter()
            .map(|sim| sim.options().clone())
            .collect();
        let original = value.original_document();
        Self {
            controllers: value
                .into_controllers()
                .into_iter()
                .map(PyMerlin::from_native)
                .collect(),
            ids,
            options,
            _original: original,
            busy: Mutex::new(()),
        }
    }
    fn execute(&self, py: Python<'_>, control: api::RunControl) -> PyResult<Vec<PyBatchReport>> {
        let _guard = self.busy.try_lock().map_err(|_| {
            error(api::ApiError::new(
                api::ErrorCode::Busy,
                "Workbench is running",
            ))
        })?;
        self.controllers
            .iter()
            .map(|sim| sim.execute(py, control.clone()))
            .collect()
    }
}
#[pymethods]
impl PyWorkbench {
    /// Loads the saved WB and embedded originals through native Rust preflight.
    /// # Arguments
    /// * `py` - Interpreter released during filesystem access, decoding and validation.
    /// * `path` - Existing self-contained workbench JSON path.
    /// # Returns
    /// Idle saved-profile controllers; simulation starts only on run.
    /// # Errors
    /// Structured Rust I/O, integrity, unsupported-capability or validation errors.
    #[staticmethod]
    fn from_file(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        py.detach(|| pnm_input::Workbench::from_file(path))
            .map(Self::from_native)
            .map_err(error)
    }
    /// Loads copied WB bytes with the same native semantics as file loading.
    /// # Arguments
    /// * `py` - Interpreter released during decoding and validation.
    /// * `data` - Complete bounded saved-WB JSON bytes, retained independently.
    /// # Returns
    /// Validated idle WB; no simulation or CLI subprocess starts.
    /// # Errors
    /// Preserves structured Rust size, schema, integrity and capability errors.
    #[staticmethod]
    fn from_bytes(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| pnm_input::Workbench::from_bytes(data))
            .map(Self::from_native)
            .map_err(error)
    }
    /// Reads the saved selected profile identities in native run order.
    /// # Returns
    /// Independent typed IDs; these are captured profile IDs, not modulation orders.
    #[getter]
    fn profile_ids(&self) -> Vec<PyProfileId> {
        self.ids.iter().copied().map(PyProfileId).collect()
    }
    /// Reads independent saved option snapshots, one per selected profile.
    /// # Returns
    /// Mutable copies; editing them does not alter accepted WB settings.
    #[getter]
    fn options(&self) -> Vec<PyEvaluationOptions> {
        self.options
            .iter()
            .map(PyEvaluationOptions::from_native)
            .collect()
    }
    /// Explicitly simulates every saved selected profile through the existing Rust API.
    /// # Arguments
    /// * `py` - Interpreter released by the existing native job adapter during simulation.
    /// # Returns
    /// One owned BatchReport per profile, in profile_ids order; IDs remain controller-scoped.
    /// # Errors
    /// Structured Busy/preflight errors and Python interrupts; inspect batch status for partials.
    fn run(&self, py: Python<'_>) -> PyResult<Vec<PyBatchReport>> {
        self.execute(py, api::RunControl::new())
    }
    /// Simulates saved profiles with shared cooperative cancellation.
    /// # Arguments
    /// * `py` - Interpreter released during simulation with existing signal polling.
    /// * `control` - Shareable cancellation token; later profiles become not-started.
    /// # Returns
    /// Independently owned complete/partial/not-started batches in profile order.
    /// # Errors
    /// Existing Busy, preflight and Python-interrupt errors.
    fn run_with_control(
        &self,
        py: Python<'_>,
        control: &PyRunControl,
    ) -> PyResult<Vec<PyBatchReport>> {
        self.execute(py, control.0.clone())
    }
}
/// Registers only the native WB type; import never reads files or starts simulation.
/// # Arguments
/// * `module` - Python extension namespace.
/// # Returns
/// Unit after registration.
/// # Errors
/// Python allocation or type-registration failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyWorkbench>()
}
