/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local facade/input ownership and public namespace.
 */
//! Short-held session slot, Rust worker and cooperative interpreter signal stop.
mod grid;
mod receiver_observations;
mod response;
use super::{
    api,
    configuration::{PyConfig, PyOptions},
    error,
    reports::{PyBatchReport, PyPreview},
};
use pyo3::prelude::*;
use std::{
    sync::{mpsc, Arc, Mutex},
    time::Duration,
};

#[pyclass(
    from_py_object,
    frozen,
    module = "merlin.upstream",
    name = "RunControl"
)]
#[derive(Clone)]
pub(crate) struct PyControl(pub(crate) api::RunControl);
#[pymethods]
impl PyControl {
    /// Creates idle Rust cancellation/progress state; no worker starts.
    ///
    /// Returns a handle shareable with another Python thread.
    #[new]
    #[pyo3(signature=(cancellation=None))]
    fn new(cancellation: Option<PyRef<'_, crate::controller::PyRunControl>>) -> Self {
        let control = match cancellation {
            Some(shared) => api::RunControl::with_cancellation(shared.0.clone()),
            None => api::RunControl::new(),
        };
        Self(control)
    }
    /// Requests cooperative cancellation without discarding completed science.
    ///
    /// Returns None immediately; the Rust worker observes a safe boundary.
    fn cancel(&self) {
        self.0.cancel();
    }
    /// Reads directional and optional common stop requests without changing state.
    ///
    /// # Returns
    /// True after cancellation through either handle; pause remains independent.
    fn is_cancelled(&self) -> bool {
        self.0.is_cancelled()
    }

    /// Requests a Rust safe-boundary pause; poll progress for acknowledgement.
    ///
    /// Returns None without blocking; no Python scientific state is retained.
    fn pause(&self) {
        self.0.pause();
    }
    /// Resumes the same Rust worker without resetting clocks, histories or RNG.
    ///
    /// Returns None; acknowledged pause time is excluded from execution budgets.
    fn resume(&self) {
        self.0.resume();
    }
    /// Serializes an owned current Rust progress snapshot without session access.
    ///
    /// Returns JSON; polling never drives DSP or changes any generator state.
    fn progress_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.0.progress())
            .map_err(|cause| error(api::ApiError::Serialization(cause)))
    }
}

#[pyclass(frozen, module = "merlin.upstream", name = "Simulation")]
pub(crate) struct PySimulation {
    state: Arc<Mutex<Option<api::Simulation>>>,
}

fn busy() -> api::ApiError {
    api::PhyError::Association {
        field: "session.busy",
    }
    .into()
}

struct SessionLease {
    state: Arc<Mutex<Option<api::Simulation>>>,
    session: Option<api::Simulation>,
}
impl Drop for SessionLease {
    fn drop(&mut self) {
        *self
            .state
            .lock()
            .unwrap_or_else(|poison| poison.into_inner()) = self.session.take();
    }
}

impl PySimulation {
    fn access<T>(
        &self,
        operation: impl FnOnce(&mut api::Simulation) -> api::ApiResult<T>,
    ) -> PyResult<T> {
        let mut slot = self.state.try_lock().map_err(|_| error(busy()))?;
        let session = slot.as_mut().ok_or_else(|| error(busy()))?;
        operation(session).map_err(error)
    }
    fn access_long<T: Send>(
        &self,
        py: Python<'_>,
        operation: impl FnOnce(&mut api::Simulation) -> api::ApiResult<T> + Send,
    ) -> PyResult<T> {
        let mut slot = self.state.try_lock().map_err(|_| error(busy()))?;
        let session = slot.as_mut().ok_or_else(|| error(busy()))?;
        py.detach(|| operation(session)).map_err(error)
    }
    fn execute(&self, py: Python<'_>, control: api::RunControl) -> PyResult<PyBatchReport> {
        let session = {
            self.state
                .try_lock()
                .map_err(|_| error(busy()))?
                .take()
                .ok_or_else(|| error(busy()))?
        };
        let mut lease = SessionLease {
            state: self.state.clone(),
            session: Some(session),
        };
        let (sender, receiver) = mpsc::sync_channel(1);
        let receiver = Mutex::new(receiver);
        let worker_control = control.clone();
        let worker = std::thread::spawn(move || {
            let result = match lease.session.as_mut() {
                Some(session) => session.run_with_control(&worker_control),
                None => Err(api::ApiError::AdapterFailure("native session unavailable")),
            };
            drop(lease);
            let _ = sender.send(result);
        });
        loop {
            match py.detach(|| {
                receiver
                    .lock()
                    .unwrap_or_else(|poison| poison.into_inner())
                    .recv_timeout(Duration::from_millis(20))
            }) {
                Ok(result) => {
                    let joined = py.detach(|| worker.join());
                    if joined.is_err() {
                        return Err(error(api::ApiError::AdapterFailure(
                            "native worker panicked",
                        )));
                    }
                    return result.map(PyBatchReport).map_err(error);
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if py.check_signals().is_err() {
                        control.cancel();
                    }
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    let _ = py.detach(|| worker.join());
                    return Err(error(api::ApiError::AdapterFailure(
                        "native worker disconnected",
                    )));
                }
            }
        }
    }
}

#[pymethods]
impl PySimulation {
    /// Constructs an idle session from optional validated native run options.
    ///
    /// Returns an unconfigured controller; construction never executes a trial.
    #[new]
    #[pyo3(signature=(options=None))]
    fn new(options: Option<PyRef<'_, PyOptions>>) -> PyResult<Self> {
        let options = options.map(|value| value.0.clone()).unwrap_or_default();
        api::Simulation::new(options)
            .map(|session| Self {
                state: Arc::new(Mutex::new(Some(session))),
            })
            .map_err(error)
    }
    /// Atomically accepts `config` through the public Rust API while idle.
    ///
    /// Returns None; rejection preserves settings and competing runs report Busy.
    fn configure(&self, config: PyRef<'_, PyConfig>) -> PyResult<()> {
        self.access(|session| session.configure(config.0.clone()))
    }
    /// Admits original-preserving measured JSON through the public Rust session.
    ///
    /// Returns None while idle; invalid associations preserve accepted state and
    /// concurrent access returns Busy. Rust owns all decoding, preflight and math.
    fn configure_measured_json(&self, py: Python<'_>, bytes: &[u8]) -> PyResult<()> {
        py.detach(|| {
            self.access(|session| {
                session.configure_measured(api::MeasuredExperiment::from_json(bytes)?.admit()?)
            })
        })
    }
    /// Atomically accepts the complete Rust-parsed document and run budgets.
    ///
    /// Returns None; competing native runs return Busy and failed preparation
    /// preserves accepted settings, including unused original inventory entries.
    fn configure_document(&self, document: PyRef<'_, super::document::PyDocument>) -> PyResult<()> {
        self.access(|session| session.configure_document(document.0.clone()))
    }
    /// Parses bounded generic US Configuration bytes and accepts them atomically.
    ///
    /// Returns None without execution; native Busy and shared Rust format/domain
    /// errors preserve the previously accepted configuration and run options.
    fn configure_json(&self, py: Python<'_>, bytes: &[u8]) -> PyResult<()> {
        self.access_long(py, |session| session.configure_json(bytes))
    }
    /// Atomically accepts finite runtime `options` through the same Rust validator.
    ///
    /// Returns None; no source is clocked and failures preserve accepted state.
    fn set_options(&self, options: PyRef<'_, PyOptions>) -> PyResult<()> {
        self.access(|session| session.set_options(options.0.clone()))
    }
    /// Returns an owned accepted configuration or None while unconfigured.
    ///
    /// Competing runs return Busy; the immutable snapshot survives later calls.
    fn configuration(&self) -> PyResult<Option<PyConfig>> {
        self.access(|session| Ok(session.configuration().cloned().map(PyConfig)))
    }
    /// Returns the retained full source document or None for typed-only settings.
    ///
    /// No source fields or unused presets are dropped from the owned snapshot.
    fn document(&self) -> PyResult<Option<super::document::PyDocument>> {
        self.access(|session| Ok(session.document().cloned().map(super::document::PyDocument)))
    }
    /// Returns an owned Rust readiness/inventory snapshot without running work.
    ///
    /// Missing configuration and a busy native session produce structured errors.
    fn preview(&self) -> PyResult<PyPreview> {
        self.access(|session| session.preview()).map(PyPreview)
    }
    /// Runs the Rust engine with interpreter release and owned complete/partial results.
    ///
    /// Returns BatchReport. SIGINT requests cooperative stop and returns its partial
    /// report; session ownership is restored before this operation returns.
    fn run(&self, py: Python<'_>) -> PyResult<PyBatchReport> {
        self.execute(py, api::RunControl::new())
    }
    /// Runs using independently shareable `control`, releasing the interpreter.
    ///
    /// Returns all completed science on cancellation; competing session calls
    /// return Busy and no Python-side numerical calculations occur.
    fn run_with_control(
        &self,
        py: Python<'_>,
        control: PyRef<'_, PyControl>,
    ) -> PyResult<PyBatchReport> {
        self.execute(py, control.0.clone())
    }
}

/// Registers direction controls, owned resource snapshots and the existing native session.
/// # Arguments
/// * `module` - Existing private upstream conversion namespace; registration starts no work.
/// # Returns
/// Unit after real native types are registered.
/// # Errors
/// Propagates Python type/module allocation and registration failures.
pub(crate) fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyControl>()?;
    module.add_class::<grid::PyResourceGridSnapshot>()?;
    module.add_class::<PySimulation>()?;
    Ok(())
}
