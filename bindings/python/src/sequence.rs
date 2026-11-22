/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native conversions over immutable public measured-sequence contracts.
use crate::{
    admission::PyTransmission,
    api, error,
    inputs::{PyChannelEstimate, PyDpd, PyRxMer},
    object_from_json,
    quantities::{PyAssociationTolerance, PyObservationDuration},
};
use pyo3::{prelude::*, types::PySequence as PythonSequence};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "MeasurementSequenceOptions"
)]
#[derive(Clone)]
pub(crate) struct PySequenceOptions(api::MeasurementSequenceOptions);

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "MeasurementSequence"
)]
#[derive(Clone)]
pub(crate) struct PySequence(pub(crate) api::MeasurementSequence);

fn transport_error(cause: serde_json::Error) -> api::ApiError {
    api::ApiError::new(api::ErrorCode::InvalidInput, cause.to_string())
}

#[pymethods]
impl PySequenceOptions {
    /// Owns explicit Rust playback options, separate from original capture timestamps.
    /// # Arguments
    /// * `mode` - single, average, hold or interpolate.
    /// * `interval_symbols` - Positive for temporal modes; checked with the sequence.
    /// * `tolerance` - Explicit original epoch tolerance; None selects exact epoch matching.
    /// # Returns
    /// Immutable options; sequence admission validates mode/count/geometry together.
    /// # Errors
    /// Unknown names raise InvalidInput; primitive conversion errors stay Python errors.
    #[new]
    #[pyo3(signature=(mode="single", interval_symbols=1, tolerance=None))]
    fn new(
        mode: &str,
        interval_symbols: u64,
        tolerance: Option<&PyAssociationTolerance>,
    ) -> PyResult<Self> {
        let mode = match mode {
            "single" => api::MeasurementMode::Single,
            "average" => api::MeasurementMode::Average,
            "hold" => api::MeasurementMode::Hold,
            "interpolate" => api::MeasurementMode::Interpolate,
            _ => {
                return Err(error(api::ApiError::new(
                    api::ErrorCode::InvalidInput,
                    "Unknown measurement sequence mode",
                )))
            }
        };
        Ok(Self(api::MeasurementSequenceOptions {
            mode,
            interval_symbols,
            tolerance: tolerance.map_or(api::AssociationTolerance::ZERO, |v| v.0),
        }))
    }

    /// Reads the accepted policy name without changing scientific state.
    /// # Returns
    /// single, average, hold or interpolate.
    #[getter]
    fn mode(&self) -> &'static str {
        match self.0.mode {
            api::MeasurementMode::Single => "single",
            api::MeasurementMode::Average => "average",
            api::MeasurementMode::Hold => "hold",
            api::MeasurementMode::Interpolate => "interpolate",
        }
    }

    /// Reads the explicit synthetic interval, independent of original epochs.
    /// # Returns
    /// Physical OFDM symbols between temporal observations.
    #[getter]
    fn interval_symbols(&self) -> u64 {
        self.0.interval_symbols
    }

    /// Reads the original epoch association tolerance.
    /// # Returns
    /// Existing immutable seconds quantity.
    #[getter]
    fn tolerance(&self) -> PyAssociationTolerance {
        PyAssociationTolerance(self.0.tolerance)
    }
}

#[pymethods]
impl PySequence {
    /// Admits ordered original measurements entirely through the public Rust planner.
    /// # Arguments
    /// * `profile` - Accepted original spatial identity and required carrier roles.
    /// * `transmission` - Immutable accepted numerology.
    /// * `captures` - Bounded sequence of existing immutable RxMer objects.
    /// * `options` - Explicit synthetic mode/clock and original epoch tolerance.
    /// # Returns
    /// Immutable reusable sequence, without source advancement or simulation execution.
    /// # Errors
    /// Shared Rust duplicate/grid/epoch/coverage/mode/work errors raise ApiError.
    #[new]
    fn new(
        py: Python<'_>,
        profile: &PyDpd,
        transmission: &PyTransmission,
        captures: &Bound<'_, PythonSequence>,
        options: &PySequenceOptions,
    ) -> PyResult<Self> {
        if captures.len()? > api::MAX_SEQUENCE_CAPTURES {
            return Err(error(api::ApiError::new(
                api::ErrorCode::ResourceLimit,
                "Measurement capture count exceeds bounds",
            )));
        }
        let values = captures
            .extract::<Vec<PyRxMer>>()?
            .into_iter()
            .map(|v| v.0)
            .collect();
        py.detach(|| api::MeasurementSequence::new(&profile.0, &transmission.0, values, options.0))
            .map(Self)
            .map_err(error)
    }

    /// Reports the Rust-resolved synthetic clock and collection size.
    /// # Returns
    /// Owned mode/count/interval/span dictionary, with no fabricated capture epoch.
    /// # Errors
    /// Preserves structured transport errors and Python allocation failures.
    fn info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.info())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Copies one original capture into independent native Python ownership.
    /// # Arguments
    /// * `index` - Zero-based position in the admitted order.
    /// # Returns
    /// Existing immutable RxMer snapshot, unchanged by previous queries.
    /// # Errors
    /// InvalidIdentifier for an out-of-collection position.
    fn capture(&self, py: Python<'_>, index: usize) -> PyResult<PyRxMer> {
        py.detach(|| self.0.capture(index).cloned())
            .map(PyRxMer)
            .map_err(error)
    }

    /// Reports permitted selection capacity for a proposed observation without running it.
    /// # Arguments
    /// * `duration` - Checked source observation milliseconds.
    /// # Returns
    /// Bounded capacity from Rust's exact symbol cadence.
    /// # Errors
    /// Shared timing errors raise ApiError.
    fn capacity(&self, duration: &PyObservationDuration) -> PyResult<usize> {
        self.0.capacity(duration.0).map_err(error)
    }

    /// Queries normalized residual powers and every actual original contributor.
    /// # Arguments
    /// * `symbol` - Explicit zero-based synthetic physical observation symbol.
    /// # Returns
    /// Owned powers/indexes/linear weights/provenance dictionary from Rust.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    fn at(&self, py: Python<'_>, symbol: u64) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| serde_json::to_string(&self.0.at(symbol)).map_err(transport_error))
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Previews equivalent RxMER in dB through the same public Rust sequence operation.
    /// # Arguments
    /// * `symbol` - Explicit synthetic observation symbol, independent of original epochs.
    /// * `start`, `count` - Original viewport; zero count requests the complete grid.
    /// # Returns
    /// Owned bounded equivalent view/grid/contributors without source/RNG changes.
    /// # Errors
    /// Shared viewport/range errors raise ApiError.
    #[pyo3(signature=(symbol, start=0, count=0))]
    fn preview(
        &self,
        py: Python<'_>,
        symbol: u64,
        start: usize,
        count: usize,
    ) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let snapshot = self
                    .0
                    .preview(symbol, api::MeasurementViewport { start, count })?;
                serde_json::to_string(&snapshot).map_err(transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }
}

#[pymethods]
impl PyTransmission {
    /// Admits explicit unity or original H through Rust's spatial/epoch/coverage checks.
    /// # Arguments
    /// * `profile` - Accepted physical carrier roles and original identity.
    /// * `response` - Existing immutable H, or None for explicit synthetic unity.
    /// * `tolerance` - Original epoch window; None requires exact epoch matching.
    /// # Returns
    /// None on compatibility without generating samples or running a job.
    /// # Errors
    /// Shared physical/association errors raise ApiError.
    #[pyo3(signature=(profile, response=None, tolerance=None))]
    fn check_response(
        &self,
        py: Python<'_>,
        profile: &PyDpd,
        response: Option<&PyChannelEstimate>,
        tolerance: Option<&PyAssociationTolerance>,
    ) -> PyResult<()> {
        py.detach(|| {
            self.0.check_response(
                &profile.0,
                response.map(|v| &v.0),
                tolerance.map_or(api::AssociationTolerance::ZERO, |v| v.0),
            )
        })
        .map_err(error)
    }
}

/// Registers immutable sequence converters in the existing native extension.
/// # Arguments
/// * `module` - Existing native namespace; import performs no scientific work.
/// # Returns
/// Unit on registration, preserving Python allocation/type errors.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PySequenceOptions>()?;
    module.add_class::<PySequence>()
}
