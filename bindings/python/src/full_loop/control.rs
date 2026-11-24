/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin polling/control conversion; no scientific/result ownership or callbacks in Python.
use super::*;
#[pyclass(from_py_object, frozen, module = "merlin", name = "FullLoopControl")]
#[derive(Clone)]
pub(super) struct PyFullLoopControl(pub(super) api::FullLoopControl);
#[pymethods]
impl PyFullLoopControl {
    /// Creates idle one-shot shared progress/cancellation storage without starting a worker.
    /// # Returns
    /// Fresh directional control; the actual worker acknowledges safe-boundary pauses.
    #[new]
    fn new() -> Self {
        Self(api::FullLoopControl::new())
    }
    /// Measures the existing Rust control without advancing its lifecycle.
    ///
    /// # Returns
    /// Immutable payload/entry/root quantities including private shared storage.
    /// Releases the interpreter; no worker, clock or scientific result is queried.
    /// Python projections and private allocator/refcount bookkeeping are excluded.
    ///
    /// # Errors
    /// Preserves Rust ResourceLimit on checked retained_footprint overflow.
    fn retained_footprint(
        &self,
        py: Python<'_>,
    ) -> PyResult<crate::retention::PyRetainedFootprint> {
        py.detach(|| self.0.retained_footprint())
            .map(crate::retention::PyRetainedFootprint)
            .map_err(error)
    }

    /// Requests pause before dispatch or at the next completed scientific step.
    /// # Returns
    /// Unit; poll state to observe actual pause acknowledgement.
    /// # Errors
    /// Busy after terminal completion; existing accepted results remain valid.
    fn pause(&self) -> PyResult<()> {
        self.0.pause().map_err(error)
    }
    /// Releases the pause gate; existing source/RNG/receiver state continues unchanged.
    /// # Returns
    /// Unit; snapshot reports Running when the worker leaves its wait.
    fn resume(&self) {
        self.0.resume();
    }
    /// Requests one-way cancellation, including during an acknowledged pause.
    /// # Returns
    /// Unit; completed FEC comparisons remain in the real partial result.
    fn cancel(&self) {
        self.0.cancel();
    }
    /// Copies constant-size lifecycle, requested flags and actual last-step progress.
    /// # Returns
    /// Owned dictionary available while native execution releases the GIL.
    /// # Errors
    /// Preserves structured transport and Python allocation failures.
    fn snapshot(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let text = serde_json::to_string(&self.0.snapshot())
            .map_err(transport_error)
            .map_err(error)?;
        object_from_json(py, &text)
    }

    /// Reads age of the actual backend notification independently of UI/Python polling.
    ///
    /// # Returns
    /// None before progress, otherwise monotonic elapsed whole milliseconds.
    /// Age continues through paused/terminal states; the native u64 representation
    /// saturates at its maximum. No Python clock, callback or scientific result is used.
    fn last_progress_age_ms(&self) -> Option<u64> {
        self.0.last_progress_age_ms()
    }
}
