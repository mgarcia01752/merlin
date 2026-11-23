/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin immutable conversions over shared typed transmission and source admission.
use crate::{
    api, error,
    inputs::PyDpd,
    object_from_json,
    profile_capture::PyProfileCapture,
    quantities::{PyObservationDuration, PyPrbsSeed},
};
use pyo3::{prelude::*, types::PyBytes};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin",
    name = "TransmissionConfiguration"
)]
#[derive(Clone)]
pub(crate) struct PyTransmission(pub(crate) api::TransmissionConfiguration);

#[pyclass(from_py_object, frozen, module = "merlin", name = "FileFillPolicy")]
#[derive(Clone)]
pub(crate) struct PyFileFill(pub(crate) api::FileFillPolicy);

#[pyclass(from_py_object, frozen, module = "merlin", name = "SourceInput")]
#[derive(Clone)]
pub(crate) struct PySourceInput(pub(crate) api::SourceInput);

#[pymethods]
impl PyTransmission {
    /// Owns Rust's validated application-default numerology; no simulation starts.
    ///
    /// # Returns
    /// An immutable configuration; shared Rust admission errors raise ApiError.
    #[new]
    fn new() -> PyResult<Self> {
        api::TransmissionConfiguration::new(api::TransmissionSettings::default())
            .map(Self)
            .map_err(error)
    }

    /// Admits bounded settings bytes through the same public Rust constructor.
    ///
    /// # Arguments
    /// * `data` - At most 4096 UTF-8 JSON bytes in the existing settings schema.
    ///
    /// # Returns
    /// An immutable snapshot; malformed/unsupported/oversized data raises ApiError.
    #[staticmethod]
    fn from_json(data: &[u8]) -> PyResult<Self> {
        api::TransmissionConfiguration::from_json(data)
            .map(Self)
            .map_err(error)
    }

    /// Serializes accepted Rust settings without touching source, RNG or session state.
    ///
    /// # Returns
    /// UTF-8 settings JSON; transport failure raises the existing structured error.
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0).map_err(|cause| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                cause.to_string(),
            ))
        })
    }

    /// Returns an owned Python snapshot of the admitted Rust scalar settings.
    ///
    /// # Arguments
    /// * `py` - Interpreter ownership for converting Rust's serialized result.
    ///
    /// # Returns
    /// An independent dictionary; editing it does not change this configuration.
    #[getter]
    fn settings(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        object_from_json(py, &self.to_json()?)
    }

    /// Reads the authoritative bounded symbol timing and optional ideal cable-delay budget.
    /// # Arguments
    /// * `velocity_factor` - Explicit speed/c in (0,1]; None leaves distances unavailable.
    /// # Returns
    /// Owned times/coefficients and optional feet; no source/run is created.
    /// # Errors
    /// Shared nonfinite/invalid velocity errors raise ApiError.
    #[pyo3(signature=(velocity_factor=None))]
    fn symbol_timing(&self, py: Python<'_>, velocity_factor: Option<f64>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.symbol_timing(velocity_factor)?;
                serde_json::to_string(&result).map_err(crate::transmitter::transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Copies complete original sample-clock transmit-window coefficients and geometry.
    /// # Returns
    /// Owned weights; downstream presentation only positions/formats the existing coefficients.
    /// # Errors
    /// Shared geometry/transport failures raise ApiError; no waveform stream starts.
    fn window(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = py
            .detach(|| {
                let result = self.0.window()?;
                serde_json::to_string(&result).map_err(crate::transmitter::transport_error)
            })
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Plans source-symbol timing entirely through the public Rust timing operation.
    ///
    /// # Arguments
    /// * `duration` - Checked millisecond observation interval.
    /// * `py` - Interpreter ownership for result conversion.
    ///
    /// # Returns
    /// Exact integer sample/symbol accounting; shared admission errors raise ApiError.
    fn timing(&self, py: Python<'_>, duration: &PyObservationDuration) -> PyResult<Py<PyAny>> {
        let snapshot = self.0.timing(duration.0).map_err(error)?;
        let text = serde_json::to_string(&snapshot).map_err(|cause| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                cause.to_string(),
            ))
        })?;
        object_from_json(py, &text)
    }

    /// Queries the least whole-millisecond source window completing a selected symbol.
    ///
    /// # Arguments
    /// * `last_symbol_index` - Zero-based source-observation OFDM symbol index.
    ///
    /// # Returns
    /// Owned integer threshold, including values above observation/UI/run budgets.
    /// Acquisition, interleaver drain and window tails remain separate; no run is admitted,
    /// no input changes and no source, waveform, receiver or RNG is advanced.
    ///
    /// # Errors
    /// Shared timing/arithmetic failures raise ApiError; integer-width errors stay native.
    fn minimum_observation_milliseconds_after(&self, last_symbol_index: u64) -> PyResult<u64> {
        self.0
            .minimum_observation_milliseconds_after(last_symbol_index)
            .map_err(error)
    }

    /// Plans complete Welch-average capacity through the public Rust numerical owner.
    ///
    /// # Arguments
    /// * `max_samples` - Sample budget containing at least one accepted FFT.
    ///
    /// # Returns
    /// Work-policy-capped integer capacity without reading samples or running a job.
    /// Original configuration stays unchanged; actual retained availability is separate.
    ///
    /// # Errors
    /// Shared domain failures raise ApiError; nonrepresentable integer conversion stays native.
    fn spectrum_average_capacity(&self, max_samples: usize) -> PyResult<usize> {
        self.0.spectrum_average_capacity(max_samples).map_err(error)
    }

    /// Checks original-grid editor compatibility without claiming full TX readiness.
    ///
    /// # Arguments
    /// * `profile` - Existing admitted immutable modulation profile.
    ///
    /// # Returns
    /// None on compatibility; shared structured errors otherwise.
    fn check_profile(&self, profile: &PyDpd) -> PyResult<()> {
        self.0.check_profile(&profile.0).map_err(error)
    }

    /// Checks an original source identity through Rust's existing editor geometry owner.
    ///
    /// # Arguments
    /// * `capture` - Native owned complete source catalog; remains unchanged.
    /// * `profile_id` - Original byte-sized identity, independent of checked run IDs.
    ///
    /// # Returns
    /// None on compatibility; no source or simulation starts and no input mutates.
    ///
    /// # Errors
    /// Shared missing-ID/grid failures raise ApiError; integer-width errors stay native.
    fn check_profile_capture(&self, capture: &PyProfileCapture, profile_id: u8) -> PyResult<()> {
        self.0
            .check_profile_capture(&capture.0, profile_id)
            .map_err(error)
    }
}

#[pymethods]
impl PyFileFill {
    /// Admits a finite target and independent checked PRBS15 fill seed in Rust.
    ///
    /// # Arguments
    /// * `target_bits` - Total file-plus-fill bits within the existing work ceiling.
    /// * `seed` - Nonzero 15-bit recurrence seed, independent of PHY scrambling.
    ///
    /// # Returns
    /// Immutable policy; invalid targets raise the same Rust ApiError.
    #[new]
    fn new(target_bits: u64, seed: &PyPrbsSeed) -> PyResult<Self> {
        api::FileFillPolicy::new(target_bits, seed.0)
            .map(Self)
            .map_err(error)
    }

    /// Reads the accepted total meaningful-bit target without generating fill.
    ///
    /// # Returns
    /// Original file-plus-fill bit target, independent of FEC padding.
    #[getter]
    fn target_bits(&self) -> u64 {
        self.0.target_bits
    }

    /// Reads the independent admitted fill seed.
    ///
    /// # Returns
    /// Existing immutable PrbsSeed quantity.
    #[getter]
    fn seed(&self) -> PyPrbsSeed {
        PyPrbsSeed(self.0.seed)
    }
}

#[pymethods]
impl PySourceInput {
    /// Returns an independent finite fill selection sharing Rust's original byte buffer.
    /// # Arguments
    /// * `fill` - Optional admitted total-bit target and independent fill seed.
    /// # Returns
    /// Owned file source with unchanged original name/bytes/hash; None removes append.
    /// Rust reuses the original immutable buffer and performs all target validation.
    /// # Errors
    /// Continuous inputs or invalid finite targets raise ApiError without mutating self.
    #[pyo3(signature=(fill=None))]
    fn with_fill(&self, py: Python<'_>, fill: Option<&PyFileFill>) -> PyResult<Self> {
        let policy = fill.map(|fill| fill.0);
        py.detach(|| self.0.with_fill(policy))
            .map(Self)
            .map_err(error)
    }

    /// Reads authoritative Rust finite-source byte and meaningful-bit resource limits.
    /// # Returns
    /// Owned max_original_bytes/max_total_bits dictionary; no source or generator starts.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    #[staticmethod]
    fn file_limits(py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&api::SourceInput::file_limits()).map_err(|cause| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                cause.to_string(),
            ))
        })?;
        object_from_json(py, &text)
    }

    /// Reads the actual finite-source metadata without copying or generating payload bits.
    /// # Returns
    /// None for continuous PRBS; owned original_bytes/original_bits/target_bits/fill_bits
    /// and optional fill policy for finite files, including empty input. Rust owns counts.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    fn file_info(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.file_info()).map_err(|cause| {
            error(api::ApiError::new(
                api::ErrorCode::InvalidInput,
                cause.to_string(),
            ))
        })?;
        object_from_json(py, &text)
    }

    /// Admits a passive continuous PRBS15 choice through public Rust data contracts.
    ///
    /// # Arguments
    /// * `seed` - Existing checked nonzero 15-bit recurrence seed.
    ///
    /// # Returns
    /// Immutable selection; no generator or verifier starts.
    #[staticmethod]
    fn prbs15(seed: &PyPrbsSeed) -> Self {
        Self(api::SourceInput::prbs15(seed.0))
    }

    /// Admits exact named file bytes and an optional independent finite fill policy.
    ///
    /// # Arguments
    /// * `name` - Original display identity; never a content/encoding hint.
    /// * `data` - Exact caller bytes, including BOM, zeros and line endings.
    /// * `fill` - Optional admitted total-bit/seed policy.
    ///
    /// # Returns
    /// Immutable input; Rust size/target/name errors raise ApiError atomically.
    #[staticmethod]
    #[pyo3(signature=(name, data, fill=None))]
    fn from_bytes(
        py: Python<'_>,
        name: String,
        data: Vec<u8>,
        fill: Option<&PyFileFill>,
    ) -> PyResult<Self> {
        let policy = fill.map(|policy| policy.0);
        py.detach(move || {
            let input = api::NamedCapture::new(name, data)?;
            api::SourceInput::file(input, policy)
        })
        .map(Self)
        .map_err(error)
    }

    /// Admits an exact-byte file with an explicitly absent original display label.
    ///
    /// # Arguments
    /// * `data` - Original caller bytes, including zeros, BOM and line endings.
    /// * `fill` - Optional admitted total-bit target and independent fill seed.
    ///
    /// # Returns
    /// Immutable file with an empty name and exact bytes/hash. Rust owns validation
    /// and copying while the GIL is released; no source or RNG advances.
    ///
    /// # Errors
    /// Original Rust byte/work/fill failures raise ApiError without changing inputs.
    #[staticmethod]
    #[pyo3(signature=(data, fill=None))]
    fn from_unnamed_bytes(
        py: Python<'_>,
        data: Vec<u8>,
        fill: Option<&PyFileFill>,
    ) -> PyResult<Self> {
        let policy = fill.map(|policy| policy.0);
        py.detach(move || api::SourceInput::unnamed_file(&data, policy))
            .map(Self)
            .map_err(error)
    }

    /// Reads the Rust-owned source family without inspecting a file extension.
    ///
    /// # Returns
    /// `prbs15` or `file`; random data is never relabeled PRBS15.
    #[getter]
    fn kind(&self) -> &'static str {
        self.0.kind()
    }

    /// Reads only the continuous-source seed, not a file's separate fill policy.
    ///
    /// # Returns
    /// An immutable checked seed or None for a finite file.
    #[getter]
    fn prbs_seed(&self) -> Option<PyPrbsSeed> {
        self.0.prbs_seed().map(PyPrbsSeed)
    }

    /// Reads the caller's original display name without changing source state.
    ///
    /// # Returns
    /// A name for files or None for continuous PRBS15.
    #[getter]
    fn name(&self) -> Option<&str> {
        self.0.name()
    }

    /// Copies original bytes into independent Python ownership, without generated fill.
    ///
    /// # Arguments
    /// * `py` - Interpreter ownership for allocating the bounded bytes result.
    ///
    /// # Returns
    /// Exact original bytes, including empty bytes, or None for PRBS15.
    #[getter]
    fn original_bytes<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyBytes>> {
        self.0.original_bytes().map(|bytes| PyBytes::new(py, bytes))
    }

    /// Reads the original byte identity computed by Rust admission.
    ///
    /// # Returns
    /// Lowercase SHA-256 for files or None for continuous PRBS15.
    #[getter]
    fn original_sha256(&self) -> Option<&str> {
        self.0.original_sha256()
    }

    /// Reads finite meaningful-bit extent before coding padding or observation windows.
    ///
    /// # Returns
    /// File-plus-fill bits, including zero for empty input; None for continuous PRBS15.
    #[getter]
    fn target_bits(&self) -> Option<u64> {
        self.0.target_bits()
    }
}

/// Registers native conversion classes; no generator, file or thread starts.
///
/// # Arguments
/// * `module` - Existing native MERLIN extension module.
///
/// # Returns
/// Unit on registration; Python allocation/registration errors otherwise.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyTransmission>()?;
    module.add_class::<PyFileFill>()?;
    module.add_class::<PySourceInput>()?;
    Ok(())
}
