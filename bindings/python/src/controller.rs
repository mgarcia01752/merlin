/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native dispatch with GIL release, Busy protection and cooperative signal handling.
use crate::{api, error, inputs::*, options::*, reports::PyBatchReport};
use pyo3::prelude::*;
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
/// High-level controller. Native scientific work never calls Python or the CLI.
#[pyclass(frozen, module = "merlin", name = "Merlin")]
pub(crate) struct PyMerlin {
    state: Arc<Mutex<Option<api::Merlin>>>,
}
impl PyMerlin {
    /// Wraps an already validated Rust controller without changing its configuration.
    /// # Arguments
    /// * `sim` - Owned idle native controller, including original captured inputs.
    /// # Returns
    /// Python controller using the existing Busy and GIL-releasing execution path.
    pub(crate) fn from_native(sim: api::Merlin) -> Self {
        Self {
            state: Arc::new(Mutex::new(Some(sim))),
        }
    }
    fn access<T>(
        &self,
        operation: impl FnOnce(&mut api::Merlin) -> api::ApiResult<T>,
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
    /// * `py` - Interpreter token used only for conversion, signal handling or lock release.
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
            let result = sim.run_with_control(&worker_control);
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
    /// * `ocd` - Validated immutable OCD to preserve.
    /// # Returns
    /// A fresh Rust-backed value or request, with native defaults where documented.
    #[new]
    fn new(ocd: &PyOcd) -> Self {
        Self {
            state: Arc::new(Mutex::new(Some(api::Merlin::new(ocd.0.clone())))),
        }
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
        self.access(|sim| Ok(PyOcd(sim.ocd().clone())))
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
        self.access(|sim| Ok(PyEvaluationOptions::from_native(sim.options())))
    }
    /// Run the accepted Rust configuration explicitly and return an independently owned batch.
    /// # Arguments
    /// * `py` - Interpreter token used only for conversion, signal handling or lock release.
    /// # Returns
    /// An owned BatchReport retaining complete, partial and not-started entries.
    /// # Errors
    /// Preserves structured Rust validation errors and Python conversion failures.
    fn run(&self, py: Python<'_>) -> PyResult<PyBatchReport> {
        self.execute(py, api::RunControl::new())
    }
    /// Run with cooperative cancellation, preserving partial and not-started results.
    /// # Arguments
    /// * `py` - Interpreter token used only for conversion, signal handling or lock release.
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
    Ok(())
}
