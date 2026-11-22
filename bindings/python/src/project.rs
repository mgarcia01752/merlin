/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned project handles over native Rust admission; no scientific reconstruction.
use merlin::common::{Project, ProjectDirection, ProjectError, MAX_PROJECT_COLLECTION_COUNT};
use pyo3::{
    prelude::*,
    types::{PyBytes, PySequence},
};
use std::sync::Arc;

#[pyclass(frozen, module = "merlin.common", name = "Project")]
struct PyProject(Arc<Project>);

fn error(cause: ProjectError) -> PyErr {
    match cause {
        ProjectError::Downstream(cause) => crate::error(cause),
        ProjectError::Upstream(cause) => crate::upstream::error(cause),
    }
}
#[pymethods]
impl PyProject {
    /// Admits an owned idle project through the shared original-preserving Rust owner.
    /// # Arguments
    /// * `data` - Complete bounded legacy/downstream/upstream project JSON bytes.
    /// # Returns
    /// Independent immutable project handle; admission releases the interpreter lock.
    /// # Errors
    /// Preserves scientific, schema, integrity and resource categories from Rust.
    #[staticmethod]
    fn from_bytes(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| Project::from_bytes(data))
            .map(|project| Self(Arc::new(project)))
            .map_err(error)
    }
    /// Reads one regular project file through bounded read-only Rust admission.
    /// # Arguments
    /// * `path` - Existing project path; extension does not select its direction.
    /// # Returns
    /// Owned idle project retaining exact source bytes; GIL is released for I/O/admission.
    /// # Errors
    /// Preserves native filesystem, byte-budget, schema and scientific failures.
    #[staticmethod]
    fn from_file(py: Python<'_>, path: String) -> PyResult<Self> {
        py.detach(|| Project::from_file(path))
            .map(|project| Self(Arc::new(project)))
            .map_err(error)
    }
    /// Reads the independently admitted scientific direction without running work.
    /// # Returns
    /// The string downstream or upstream, matching the persisted direction tag.
    #[getter]
    fn direction(&self) -> &'static str {
        match self.0.direction() {
            ProjectDirection::Downstream => "downstream",
            ProjectDirection::Upstream => "upstream",
        }
    }
    /// Copies complete exact originally admitted bytes, independent of project lifetime.
    /// # Returns
    /// Owned Python bytes; no original is canonicalized or overwritten.
    #[getter]
    fn original_document<'py>(&self, py: Python<'py>) -> Bound<'py, PyBytes> {
        PyBytes::new(py, self.0.original_document())
    }
    /// Serializes a bounded direction-tagged envelope without opening a destination.
    /// # Returns
    /// Owned UTF-8 JSON retaining exact embedded documents; GIL is released.
    /// # Errors
    /// Preserves Rust export size and serialization categories.
    fn to_json(&self, py: Python<'_>) -> PyResult<String> {
        py.detach(|| self.0.to_json()).map_err(error)
    }
    /// Prepares a separate idle downstream sweep through its existing public API.
    /// # Returns
    /// Owned FullLoopSweepPlan, independent of the project and other requests.
    /// # Errors
    /// Wrong direction and scientific admission failures preserve the project.
    fn downstream_plan(&self, py: Python<'_>) -> PyResult<crate::full_loop::PyFullLoopSweepPlan> {
        py.detach(|| self.0.downstream_plan())
            .map(crate::full_loop::PyFullLoopSweepPlan::from_native)
            .map_err(error)
    }
    /// Creates a separate configured idle upstream public session.
    /// # Returns
    /// Existing upstream Simulation with unchanged Busy, run and cancellation contracts.
    /// # Errors
    /// Wrong direction and scientific admission failures preserve accepted project state.
    fn upstream_simulation(&self, py: Python<'_>) -> PyResult<crate::upstream::PySimulation> {
        py.detach(|| self.0.upstream_simulation())
            .map(crate::upstream::PySimulation::from_native)
            .map_err(error)
    }
    /// Checks a bounded collection of independently admitted direction projects.
    /// # Arguments
    /// * `projects` - Sequence of at most two Project objects with distinct directions.
    /// # Returns
    /// Owned count/byte/direction dictionary derived only by Rust, without execution.
    /// # Errors
    /// Rejects excessive counts before converting elements; preserves shared Rust categories.
    #[staticmethod]
    fn collection_info(py: Python<'_>, projects: &Bound<'_, PySequence>) -> PyResult<Py<PyAny>> {
        let count = projects.len()?;
        if count > MAX_PROJECT_COLLECTION_COUNT {
            return Err(crate::error(merlin::downstream::ApiError::new(
                merlin::common::ErrorCode::ResourceLimit,
                "Project collection exceeds the direction count budget",
            )));
        }
        let owned = (0..count)
            .map(|index| {
                let item = projects.get_item(index)?;
                let project = item.extract::<PyRef<'_, Self>>()?;
                Ok(project.0.clone())
            })
            .collect::<PyResult<Vec<_>>>()?;
        let info = py
            .detach(|| Project::collection_info(&owned.iter().map(Arc::as_ref).collect::<Vec<_>>()))
            .map_err(error)?;
        let json = serde_json::to_string(&info).map_err(|cause| {
            crate::error(merlin::downstream::ApiError::new(
                merlin::common::ErrorCode::InvalidInput,
                cause.to_string(),
            ))
        })?;
        crate::object_from_json(py, &json)
    }
}
/// Registers the actual immutable native Project type; imports perform no simulation.
/// # Arguments
/// * `module` - One shared private native extension module.
/// # Returns
/// Unit after registering the conversion type.
/// # Errors
/// Propagates native Python class registration failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyProject>()
}
