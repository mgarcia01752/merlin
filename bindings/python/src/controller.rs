/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native dispatch with GIL release, Busy protection and cooperative signal handling.
use crate::quantities::PyProfileId;
use crate::{api, error, inputs::*, options::*, reports::PyBatchReport};
use pnm_input::{read_capture, CaptureLoader};
use pyo3::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
/// Shareable native stop token, callable from another Python thread.
#[pyclass(from_py_object, frozen, module = "merlin", name = "RunControl")]
#[derive(Clone, Default)]
pub(crate) struct PyRunControl(pub(crate) api::RunControl);
#[pymethods]
impl PyRunControl {
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    fn new() -> Self {
        Self(api::RunControl::new())
    }
    /// Request cooperative stopping without discarding completed scientific results.
    /// # Returns
    /// None; the next native step or dispatch boundary observes cancellation.
    fn cancel(&self) {
        self.0.cancel();
    }
}
/// Single-channel simulation session; create empty or supply a validated OCD.
///
/// Loading inputs and accepting options never runs simulation. Only run() or
/// run_with_control() or run_all_profiles() starts the native engine, releasing the interpreter lock.
/// Captures retain original bytes; returned identifiers belong to this controller.
/// Reports independently own complete/partial outcomes. ApiError.code identifies
/// validation/preflight/Busy errors; scientific results never come from Python.
#[pyclass(frozen, module = "merlin", name = "Simulation")]
pub(crate) struct PyMerlin {
    state: Arc<Mutex<Option<api::Simulation>>>,
}
impl PyMerlin {
    /// Wraps an already validated Rust controller without changing its configuration.
    /// # Arguments
    /// * `sim` - Owned idle native controller, including original captured inputs.
    /// # Returns
    /// Python controller using the existing Busy and GIL-releasing execution path.
    pub(crate) fn from_native(sim: api::Merlin) -> Self {
        Self {
            state: Arc::new(Mutex::new(Some(sim.into()))),
        }
    }
    fn access<T>(
        &self,
        operation: impl FnOnce(&mut api::Simulation) -> api::ApiResult<T>,
    ) -> PyResult<T> {
        let mut state = self.state.try_lock().map_err(|_| {
            error(api::ApiError::new(
                api::ErrorCode::Busy,
                "Controller is busy",
            ))
        })?;
        let sim = state.as_mut().ok_or_else(|| {
            error(api::ApiError::new(
                api::ErrorCode::Busy,
                "Controller is running",
            ))
        })?;
        operation(sim).map_err(error)
    }
    /// Dispatch the same Rust job while releasing the interpreter lock and polling cancellation.
    /// # Arguments
    /// * `py` - Internal interpreter token, never a Python-visible method argument.
    /// * `control` - Shared native cooperative-cancellation token.
    /// # Returns
    /// An owned BatchReport or a structured preflight error, with the controller restored.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    pub(crate) fn execute(
        &self,
        py: Python<'_>,
        control: api::RunControl,
    ) -> PyResult<PyBatchReport> {
        self.execute_mode(py, control, false)
    }
    fn execute_mode(
        &self,
        py: Python<'_>,
        control: api::RunControl,
        all_profiles: bool,
    ) -> PyResult<PyBatchReport> {
        let mut sim = {
            let mut state = self.state.try_lock().map_err(|_| {
                error(api::ApiError::new(
                    api::ErrorCode::Busy,
                    "Controller is busy",
                ))
            })?;
            state.take().ok_or_else(|| {
                error(api::ApiError::new(
                    api::ErrorCode::Busy,
                    "Controller is running",
                ))
            })?
        };
        let (sender, receiver) = std::sync::mpsc::sync_channel(1);
        let receiver = Mutex::new(receiver);
        let worker_control = control.clone();
        let state = self.state.clone();
        let handle = std::thread::spawn(move || {
            let result = if all_profiles {
                sim.run_all_profiles_with_control(&worker_control)
            } else {
                sim.run_with_control(&worker_control)
            };
            // Restore controller ownership before notifying the waiting interpreter.
            if let Ok(mut slot) = state.lock() {
                *slot = Some(sim);
            }
            let _ = sender.send(result);
        });
        loop {
            let result = py.detach(|| {
                receiver.lock().map_or(
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected),
                    |channel| channel.recv_timeout(std::time::Duration::from_millis(20)),
                )
            });
            // Consume interpreter interrupts as cooperative cancellation, preserving results.
            if py.check_signals().is_err() {
                control.cancel();
            }
            match result {
                Ok(result) => {
                    let _ = py.detach(|| handle.join());
                    return result
                        .map(|report| PyBatchReport(Arc::new(report)))
                        .map_err(error);
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    let _ = py.detach(|| handle.join());
                    return Err(error(api::ApiError::new(
                        api::ErrorCode::Engine,
                        "Native worker terminated unexpectedly",
                    )));
                }
            }
        }
    }
}
#[pymethods]
impl PyMerlin {
    /// Construct a Rust-backed value or request using the declared units and defaults.
    /// # Arguments
    /// * `ocd` - Optional validated OCD; omit and call load_profiles for captured inputs.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    #[pyo3(signature=(ocd=None))]
    fn new(ocd: Option<&PyOcd>) -> Self {
        Self {
            state: Arc::new(Mutex::new(Some(
                ocd.map(|value| api::Simulation::with_ocd(value.0.clone()))
                    .unwrap_or_default(),
            ))),
        }
    }
    /// Load a profile capture and retain its complete inventory without simulation.
    /// # Arguments
    /// * `path` - Existing binary PNM or decoded JSON capture.
    /// * `protocol` - Optional interpretation; Rust defaults to DOCSIS 3.1.
    /// * `overrides` - Optional channel settings.
    /// # Returns
    /// Controller with its first loaded profile selected and all IDs available.
    /// # Errors
    /// Raises structured file, decoding and Rust configuration errors.
    #[staticmethod]
    #[pyo3(signature=(path,protocol=None,overrides=None))]
    fn from_profile_file(
        path: PathBuf,
        protocol: Option<PyProtocolScope>,
        overrides: Option<PyOcdOverrides>,
    ) -> PyResult<Self> {
        let capture = read_capture(path).map_err(error)?;
        CaptureLoader::controller_from_profiles(
            &capture,
            protocol.map(PyProtocolScope::native).unwrap_or_default(),
            overrides.unwrap_or_default().native(),
        )
        .map(Self::from_native)
        .map_err(error)
    }
    /// Load all profiles from a binary or JSON capture without running simulation.
    /// # Arguments
    /// * `path` - Original profile capture to decode through Rust.
    /// * `protocol`, `overrides` - Optional interpretation and channel settings.
    /// # Returns
    /// None; first loaded profile selected and compatible measurements retained.
    /// # Errors
    /// Structured decoding/configuration errors leave accepted state unchanged.
    #[pyo3(signature=(path,protocol=None,overrides=None))]
    fn load_profiles(
        &self,
        path: PathBuf,
        protocol: Option<PyProtocolScope>,
        overrides: Option<PyOcdOverrides>,
    ) -> PyResult<()> {
        self.access(|sim| {
            let profiles = CaptureLoader::profiles(&read_capture(path)?)?;
            sim.load_profiles_with_options(
                protocol.map(PyProtocolScope::native).unwrap_or_default(),
                profiles,
                overrides.unwrap_or_default().native(),
            )
        })
    }
    /// Decode and add an original RxMER file through the Rust input adapter.
    /// # Arguments
    /// * `path` - Supported binary or JSON capture.
    /// # Returns
    /// Controller-scoped ID for selection and lookup.
    /// # Errors
    /// Preserves Rust file, decoding and compatibility errors.
    fn add_rxmer_file(&self, path: PathBuf) -> PyResult<PyRxMerId> {
        self.access(|sim| {
            sim.add_rxmer(CaptureLoader::rxmer(&read_capture(path)?)?)
                .map(PyRxMerId)
        })
    }
    /// Decode and add an original channel-estimate file through Rust.
    /// # Arguments
    /// * `path` - Supported binary or JSON capture.
    /// # Returns
    /// Controller-scoped ID for channel selection.
    /// # Errors
    /// Preserves Rust file, decoding and compatibility errors.
    fn add_channel_estimate_file(&self, path: PathBuf) -> PyResult<PyChannelId> {
        self.access(|sim| {
            sim.add_channel_estimate(CaptureLoader::channel_estimate(&read_capture(path)?)?)
                .map(PyChannelId)
        })
    }
    /// Read profile IDs retained by this object, without rereading a file.
    /// # Returns
    /// Independent validated profile identifiers in loading order.
    /// # Errors
    /// Raises Busy while this controller is running.
    #[getter]
    fn profile_ids(&self) -> PyResult<Vec<PyProfileId>> {
        self.access(|sim| Ok(sim.profile_ids().into_iter().map(PyProfileId).collect()))
    }
    /// Look up an already loaded profile by its captured ID.
    /// # Arguments
    /// * `profile_id` - Identifier from this object's profile_ids.
    /// # Returns
    /// Immutable independent profile retaining capture provenance.
    /// # Errors
    /// Raises a structured error for an absent ID or busy controller.
    fn profile_by_id(&self, profile_id: &PyProfileId) -> PyResult<PyDpd> {
        self.access(|sim| sim.profile_by_id(profile_id.0).cloned().map(PyDpd))
    }
    /// Select an already loaded profile for run(), without executing it.
    /// # Arguments
    /// * `profile_id` - Identifier from this object's inventory.
    /// # Returns
    /// None after accepting the selection.
    /// # Errors
    /// Raises structured validation or Busy errors without changing selection.
    fn select_profile(&self, profile_id: &PyProfileId) -> PyResult<()> {
        self.access(|sim| sim.select_profile(profile_id.0))
    }
    /// Run all loaded profiles using Rust preflight, limits and cancellation.
    /// # Arguments
    /// * `control` - Optional cooperative stop token; Python interrupts also stop work.
    /// # Returns
    /// Owned batch with profile_id on every complete, partial or unstarted entry.
    /// # Errors
    /// Raises preflight errors before any simulation starts; preserves Busy protection.
    #[pyo3(signature=(control=None))]
    fn run_all_profiles(
        &self,
        py: Python<'_>,
        control: Option<PyRunControl>,
    ) -> PyResult<PyBatchReport> {
        self.execute_mode(py, control.map(|token| token.0).unwrap_or_default(), true)
    }
    /// Validate and atomically accept a copied profile for subsequent runs.
    /// # Arguments
    /// * `dpd` - Selected immutable captured or hypothesis profile.
    /// # Returns
    /// None after accepting the typed request.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn set_profile(&self, dpd: &PyDpd) -> PyResult<()> {
        self.access(|sim| sim.set_profile(dpd.0.clone()))
    }
    /// Add one independent original RxMER and return a controller-scoped native identifier.
    /// # Arguments
    /// * `frame` - Immutable validated original capture.
    /// # Returns
    /// An opaque RxMerId scoped to this controller.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn add_rxmer(&self, frame: &PyRxMer) -> PyResult<PyRxMerId> {
        self.access(|sim| sim.add_rxmer(frame.0.clone()).map(PyRxMerId))
    }
    /// Add one independent original response and return a controller-scoped identifier.
    /// # Arguments
    /// * `frame` - Immutable validated original capture.
    /// # Returns
    /// An opaque ChannelEstimateId scoped to this controller.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn add_channel_estimate(&self, frame: &PyChannelEstimate) -> PyResult<PyChannelId> {
        self.access(|sim| sim.add_channel_estimate(frame.0.clone()).map(PyChannelId))
    }
    /// Accept an independent Rust-validated request snapshot for subsequent runs.
    /// # Arguments
    /// * `options` - Mutable request copied at acceptance; later edits do not change accepted state.
    /// # Returns
    /// None after accepting the typed request.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn set_options(&self, options: &PyEvaluationOptions) -> PyResult<()> {
        self.access(|sim| sim.set_options(options.native()))
    }
    /// Return an immutable original observation after validating controller-scoped identity.
    /// # Arguments
    /// * `capture_id` - Opaque identifier returned by this controller.
    /// # Returns
    /// An immutable original RxMER snapshot after scoped-ID validation.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn get_rxmer(&self, capture_id: &PyRxMerId) -> PyResult<PyRxMer> {
        self.access(|sim| sim.rxmer(capture_id.0).cloned().map(PyRxMer))
    }
    /// Return an immutable original response after validating controller-scoped identity.
    /// # Arguments
    /// * `capture_id` - Opaque identifier returned by this controller.
    /// # Returns
    /// An immutable original response snapshot after scoped-ID validation.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn get_channel_estimate(&self, capture_id: &PyChannelId) -> PyResult<PyChannelEstimate> {
        self.access(|sim| {
            sim.channel_estimate(capture_id.0)
                .cloned()
                .map(PyChannelEstimate)
        })
    }
    /// Read ocd from the native Rust snapshot.
    /// # Returns
    /// The immutable native value named by this read-only property.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn ocd(&self) -> PyResult<PyOcd> {
        self.access(|sim| Ok(PyOcd(sim.ocd()?.clone())))
    }
    /// Read profile from the native Rust snapshot.
    /// # Returns
    /// The selected immutable DPD or None.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn profile(&self) -> PyResult<Option<PyDpd>> {
        self.access(|sim| Ok(sim.profile().cloned().map(PyDpd)))
    }
    /// Read options from the native Rust snapshot.
    /// # Returns
    /// An independent mutable request snapshot; editing it cannot change accepted state.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    #[getter]
    fn options(&self) -> PyResult<PyEvaluationOptions> {
        self.access(|sim| Ok(PyEvaluationOptions::from_native(sim.options()?)))
    }
    /// Run the accepted Rust configuration explicitly and return an independently owned batch.
    /// # Returns
    /// An owned BatchReport retaining complete, partial and not-started entries.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn run(&self, py: Python<'_>) -> PyResult<PyBatchReport> {
        self.execute(py, api::RunControl::new())
    }
    /// Run with cooperative cancellation, preserving partial and not-started results.
    /// # Arguments
    /// * `control` - Shared native cooperative-cancellation token.
    /// # Returns
    /// An owned BatchReport; cancellation preserves completed scientific results.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn run_with_control(&self, py: Python<'_>, control: &PyRunControl) -> PyResult<PyBatchReport> {
        self.execute(py, control.0.clone())
    }
}
/// Registers the native controller and cancellation handle.
/// # Arguments
/// * `module` - Python extension namespace.
/// # Returns
/// Unit after registration; imports do not start workers.
/// # Errors
/// Propagates Python type allocation failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyRunControl>()?;
    module.add_class::<PyMerlin>()?;
    module.add("Merlin", module.getattr("Simulation")?)?;
    Ok(())
}
