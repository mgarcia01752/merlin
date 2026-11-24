/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin immutable native archive transport; Rust owns retention, admission and all diagnostics.
use super::{cells::PyCellQuery, *};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "FullLoopReceiverArchivePolicy"
)]
#[derive(Clone)]
pub(super) struct PyReceiverArchivePolicy(api::FullLoopReceiverArchivePolicy);
#[pymethods]
impl PyReceiverArchivePolicy {
    /// Creates the public Rust default finite history policy without copying observations.
    /// # Returns
    /// Immutable policy with separate cell/decoder/byte budgets.
    /// # Errors
    /// Shared policy admission failures raise ApiError.
    #[new]
    fn new() -> PyResult<Self> {
        api::FullLoopReceiverArchivePolicy::new(api::FullLoopReceiverArchiveInput::default())
            .map(Self)
            .map_err(error)
    }

    /// Admits strict bounded policy JSON through public Rust validation.
    /// # Arguments
    /// * `data` - At most4096 UTF-8 bytes with reducible count and byte caps.
    /// # Returns
    /// Immutable policy; no simulation, capture or query is executed.
    /// # Errors
    /// Shared malformed/unknown-field/domain/work errors raise ApiError.
    #[staticmethod]
    fn from_json(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| api::FullLoopReceiverArchivePolicy::from_json(data))
            .map(Self)
            .map_err(error)
    }

    /// Copies accepted budgets into an independent dictionary.
    /// # Returns
    /// Original accepted count/byte policy; dictionary edits cannot change it.
    /// # Errors
    /// Preserves structured serialization/allocation failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(self.0.settings())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Serializes the accepted policy without touching scientific observations.
    /// # Returns
    /// Owned UTF-8 policy JSON, with no native-only defaults or calculations.
    /// # Errors
    /// Preserves structured transport failures.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0)
            .map_err(transport_error)
            .map_err(error)
    }
}

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "FullLoopReceiverArchive"
)]
#[derive(Clone)]
pub(super) struct PyReceiverArchive(api::FullLoopReceiverArchive);
#[pymethods]
impl PyFullLoopResult {
    /// Detaches bounded actual receiver history through the authoritative Rust API.
    /// # Arguments
    /// * `policy` - Optional immutable history policy; None uses the public Rust defaults.
    /// # Returns
    /// Independent archive; deleting requests/results/captures leaves it usable without
    /// retaining their jobs, waveforms, source bytes or unrelated stage stores.
    /// # Errors
    /// Shared checked-byte/domain/observation failures raise ApiError; Rust work releases the GIL.
    #[pyo3(signature=(policy=None))]
    fn receiver_archive(
        &self,
        py: Python<'_>,
        policy: Option<&PyReceiverArchivePolicy>,
    ) -> PyResult<PyReceiverArchive> {
        let policy = match policy {
            Some(policy) => policy.0.clone(),
            None => api::FullLoopReceiverArchivePolicy::new(
                api::FullLoopReceiverArchiveInput::default(),
            )
            .map_err(error)?,
        };
        py.detach(|| self.0.receiver_archive(policy))
            .map(PyReceiverArchive)
            .map_err(error)
    }
}
#[pymethods]
impl PyReceiverArchive {
    /// Measures detached observations without changing historical archive statistics.
    ///
    /// # Returns
    /// Immutable target-layout metadata including private pilot/stage ownership.
    /// The existing info.retained_bytes keeps its original length-based scope.
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

    /// Copies actual archive counts/budgets and the optional original decoder identity.
    /// # Returns
    /// Owned independent dictionary; unavailable data stays explicitly absent/zero.
    /// # Errors
    /// Preserves structured transport/allocation failures.
    #[getter]
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.info()))
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries original archived cells and all-selected reference-plane metrics.
    /// # Arguments
    /// * `query` - Optional existing immutable selection; None selects the default payload view.
    /// # Returns
    /// The same bounded Rust QAM contract as a live result; no rerun or Python statistics.
    /// # Errors
    /// Shared selection/work/numerical failures raise ApiError; Rust work releases the GIL.
    #[pyo3(signature=(query=None))]
    fn qam(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| serde_json::to_string(&self.0.qam(query)?).map_err(transport_error))
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries physical decisions from archived actual observations without modifying FEC.
    /// # Arguments
    /// * `query` - Optional existing selection; at most128 decision rows.
    /// # Returns
    /// Original physical/normalized decisions and actual selected-cell statistics.
    /// # Errors
    /// Shared selection/work/numerical failures raise ApiError; Rust work releases the GIL.
    #[pyo3(signature=(query=None))]
    fn cell_decisions(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| {
                serde_json::to_string(&self.0.cell_decisions(query)?).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries actual archived selected-cell likelihoods on their original physical-cell plane.
    /// # Arguments
    /// * `query` - Optional existing bounded role/profile/order/coordinate/window selection.
    /// # Returns
    /// Existing Rust histogram/envelope/statistics; no synthesized protected likelihoods.
    /// # Errors
    /// Shared selection/work/finite-input failures raise ApiError; Rust work releases the GIL.
    #[pyo3(signature=(query=None))]
    fn cell_soft_view(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| {
                serde_json::to_string(&self.0.cell_soft_view(query)?).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries the one actual archived decoder event, independently of physical-cell selection.
    /// # Returns
    /// Existing Rust diagnostics and original coding context/offset/total of the retained prefix.
    /// # Errors
    /// Shared missing-event or finite-input failures raise ApiError; Rust work releases the GIL.
    fn decoder_soft_view(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.decoder_soft_view()?).map_err(transport_error))
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
