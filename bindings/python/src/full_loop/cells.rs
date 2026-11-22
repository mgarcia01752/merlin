/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Frozen selection ownership and owned native receiver observations through public Rust only.
use super::*;

#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopCellQuery")]
#[derive(Clone)]
pub(super) struct PyCellQuery(pub(super) api::FullLoopCellQuery);
#[pymethods]
impl PyCellQuery {
    /// Constructs a passive default bounded receiver query for one registered actual role.
    /// # Arguments
    /// * `stream` - payload, ncp, plc_data, plc_preamble, continuous_pilots or scattered_pilots.
    /// # Returns
    /// Immutable query; no result, source, decoder or worker advances.
    /// # Errors
    /// Shared unknown-stream and transport-limit failures raise ApiError.
    #[new]
    #[pyo3(signature=(stream="payload"))]
    fn new(stream: &str) -> PyResult<Self> {
        if stream.len() > 128 {
            return Err(error(api::ApiError::new(
                api::ErrorCode::ResourceLimit,
                "Cell stream exceeds 128 bytes",
            )));
        }
        let bytes = serde_json::to_vec(&serde_json::json!({"stream":stream}))
            .map_err(transport_error)
            .map_err(error)?;
        api::FullLoopCellQuery::from_json(&bytes)
            .map(Self)
            .map_err(error)
    }

    /// Admits strict bounded typed receiver-selection JSON without executing a query.
    /// # Arguments
    /// * `data` - At most 4096 bytes; known fields, optional inclusive coordinates and row window.
    /// # Returns
    /// Immutable accepted query; later result calls apply their own actual coverage/row limits.
    /// # Errors
    /// Shared malformed/unknown/order/range/work failures raise ApiError.
    #[staticmethod]
    fn from_json(py: Python<'_>, data: &[u8]) -> PyResult<Self> {
        py.detach(|| api::FullLoopCellQuery::from_json(data))
            .map(Self)
            .map_err(error)
    }

    /// Copies exact query parameters into an independent dictionary.
    /// # Returns
    /// Owned explicit role/profile/order/coordinate/window settings; edits do not affect queries.
    /// # Errors
    /// Preserves transport and Python allocation failures.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0)
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

#[pymethods]
impl PyFullLoopResult {
    /// Queries actual paired cells and all-selected scientific MER through public Rust owners.
    /// # Arguments
    /// * `query` - Optional immutable selection; None selects the default payload view.
    /// # Returns
    /// Owned at-most-2048-cell view with original identities and distinct raw/corrected,
    /// nearest-point/TX-reference metrics computed before display pagination.
    /// # Errors
    /// Shared invalid limits/ranges/orders/offsets raise ApiError; empty selections stay empty.
    #[pyo3(signature=(query=None))]
    fn qam(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| {
                let result = self.0.qam(query)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Queries actual physical diagnostic decisions without replacing production soft inputs.
    /// # Arguments
    /// * `query` - Optional selection; this operation permits at most 128 decision rows.
    /// # Returns
    /// Owned original/normalized decisions and existing statistics of actual cell LLRs from at
    /// most 2048 selected cells. Protected LLRs remain absent; GIL released during Rust work.
    /// # Errors
    /// Shared work/range/order/offset/numerical failures raise ApiError.
    #[pyo3(signature=(query=None))]
    fn cell_decisions(&self, py: Python<'_>, query: Option<&PyCellQuery>) -> PyResult<Py<PyAny>> {
        let query = query.map_or_else(Default::default, |query| query.0.clone());
        let text = py
            .detach(|| {
                let result = self.0.cell_decisions(query)?;
                serde_json::to_string(&result).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}
