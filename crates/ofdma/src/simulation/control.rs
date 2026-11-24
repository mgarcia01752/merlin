/*
 * Copyright 2026 MERLIN contributors
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Cooperative cancellation and small owned progress snapshots, no callbacks.
use crate::{PhyError, PhyResult};
use merlin_api::{
    retention_support::{self as retention, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
};
use std::time::{Duration, Instant};

/// Actual bounded-run processing state; recovery success is a separate result.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize)]
pub enum RunStatus {
    /// No job has claimed this control.
    Idle,
    /// Frozen work is executing.
    Running,
    /// Worker acknowledged a pause at a safe boundary; science is retained.
    Paused,
    /// Every scheduled grant reached its normal processing end.
    Completed,
    /// Explicit cooperative cancellation retained the completed prefix.
    Cancelled,
    /// Monotonic execution budget expired at a safe boundary.
    TimeLimit,
    /// Scientific processing failed after the job began; prior science survives.
    Failed,
}

/// Small owned lifecycle progress, safe to poll while a session is busy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProgressSnapshot {
    /// Run lifecycle state, independent of BER/convergence.
    pub status: RunStatus,
    /// Unique run identity, absent before a job starts.
    pub run: Option<u64>,
    /// Fully processed grants, including any failed source recovery.
    pub completed_grants: usize,
    /// Actually emitted transmitter samples, including terminal tails.
    pub emitted_samples: usize,
    /// Source decisions actually compared, including failed decoder decisions.
    pub compared_bits: usize,
    /// Monotonic wall time used by this execution, in milliseconds.
    pub elapsed_ms: u64,
}

/// Test-only unset cancellation threshold; production controls have no hook field.
#[cfg(test)]
const TEST_GRANT_CANCELLATION_UNSET: usize = usize::MAX;

struct State {
    cancel: AtomicBool,
    #[cfg(test)]
    cancel_after_completed_grants: std::sync::atomic::AtomicUsize,
    shared_cancellation: Option<merlin_api::RunControl>,
    claimed: AtomicBool,
    progress: Mutex<ProgressSnapshot>,
    pause: Mutex<PauseState>,
    wake: Condvar,
}

struct PauseState {
    requested: bool,
    since: Option<Instant>,
    accumulated: Duration,
}

/// Cloneable cancellation/progress handle independent of an exclusively held API.
#[derive(Clone)]
pub struct RunControl {
    inner: Arc<State>,
}

impl Default for RunControl {
    /// Constructs a fresh idle, uncancelled control handle.
    ///
    /// No worker starts and no global state is changed.
    fn default() -> Self {
        Self::new()
    }
}

impl RunControl {
    /// Constructs fresh shared cancellation and progress state without a thread.
    ///
    /// Returns an idle handle; cloning shares one request, not another worker.
    pub fn new() -> Self {
        Self::create(None)
    }
    /// Creates directional lifecycle state sharing a common stop request.
    ///
    /// # Arguments
    /// * `cancellation` - Owned common token; its clones can stop this worker.
    ///
    /// # Returns
    /// An idle control with independent upstream pause/progress/Busy state.
    /// Cancelling this control also cancels the shared token. A paused worker
    /// checks externally requested cancellation at most every 10ms of waiting.
    pub fn with_cancellation(cancellation: merlin_api::RunControl) -> Self {
        Self::create(Some(cancellation))
    }
    fn create(shared_cancellation: Option<merlin_api::RunControl>) -> Self {
        Self {
            inner: Arc::new(State {
                cancel: AtomicBool::new(false),
                #[cfg(test)]
                cancel_after_completed_grants: std::sync::atomic::AtomicUsize::new(
                    TEST_GRANT_CANCELLATION_UNSET,
                ),
                shared_cancellation,
                claimed: AtomicBool::new(false),
                pause: Mutex::new(PauseState {
                    requested: false,
                    since: None,
                    accumulated: Duration::ZERO,
                }),
                wake: Condvar::new(),
                progress: Mutex::new(ProgressSnapshot {
                    status: RunStatus::Idle,
                    run: None,
                    completed_grants: 0,
                    emitted_samples: 0,
                    compared_bits: 0,
                    elapsed_ms: 0,
                }),
            }),
        }
    }
    /// Measures current shared control storage without advancing execution state.
    ///
    /// # Returns
    /// Exact root/State payload layouts with one shared root and zero ordered
    /// entries; an optional common cancellation adds its atomic and one root.
    /// Clones report the same conservative footprint independently. Excludes
    /// private allocator/refcount bookkeeping and temporary execution storage.
    /// Progress and pause locks are short and separate, with poison recovery.
    /// No clock is read, job claimed, Condvar waited/notified or atomic changed.
    ///
    /// # Errors
    /// ResourceLimit with field `retained_footprint` on checked arithmetic overflow.
    pub fn retained_footprint(&self) -> PhyResult<RetainedFootprint> {
        retention::retained_footprint(self).ok_or_else(|| crate::error::limit("retained_footprint"))
    }

    /// Requests cooperative stop from any thread, including before execution.
    ///
    /// Returns immediately; the engine preserves completed scientific outcomes.
    pub fn cancel(&self) {
        let _pause = self.inner.pause.lock().unwrap_or_else(|p| p.into_inner());
        self.inner.cancel.store(true, Ordering::Release);
        if let Some(shared) = &self.inner.shared_cancellation {
            shared.cancel();
        }
        self.inner.wake.notify_all();
    }
    /// Requests pause at the next safe worker boundary without blocking callers.
    ///
    /// Acknowledgement appears as `Paused` in progress. Pipeline state stays on
    /// the Rust worker stack; a pre-start request pauses before source consumption.
    pub fn pause(&self) {
        self.inner
            .pause
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .requested = true;
    }
    /// Releases a pending or acknowledged pause, retaining all scientific state.
    ///
    /// Returns immediately; cancellation also wakes a paused worker. Runtime
    /// budgets exclude acknowledged paused time, including a cancelled pause.
    pub fn resume(&self) {
        let mut state = self.inner.pause.lock().unwrap_or_else(|p| p.into_inner());
        state.requested = false;
        self.inner.wake.notify_all();
    }
    pub(crate) fn checkpoint(&self) {
        let mut state = self.inner.pause.lock().unwrap_or_else(|p| p.into_inner());
        if state.requested && !self.is_cancelled() {
            state.since = Some(Instant::now());
            self.inner
                .progress
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .status = RunStatus::Paused;
            while state.requested && !self.is_cancelled() {
                if self.inner.shared_cancellation.is_some() {
                    state = self
                        .inner
                        .wake
                        .wait_timeout(state, Duration::from_millis(10))
                        .unwrap_or_else(|p| p.into_inner())
                        .0;
                } else {
                    state = self
                        .inner
                        .wake
                        .wait(state)
                        .unwrap_or_else(|p| p.into_inner());
                }
            }
            if let Some(since) = state.since.take() {
                state.accumulated += since.elapsed();
            }
            self.inner
                .progress
                .lock()
                .unwrap_or_else(|p| p.into_inner())
                .status = RunStatus::Running;
        }
    }
    pub(crate) fn elapsed(&self, started: Instant) -> Duration {
        let state = self.inner.pause.lock().unwrap_or_else(|p| p.into_inner());
        let paused =
            state.accumulated + state.since.map_or(Duration::ZERO, |since| since.elapsed());
        started.elapsed().saturating_sub(paused)
    }
    /// Reads the shared stop request without changing it.
    ///
    /// True remains true; create another handle for a fresh uncancelled request.
    pub fn is_cancelled(&self) -> bool {
        self.inner.cancel.load(Ordering::Acquire)
            || self
                .inner
                .shared_cancellation
                .as_ref()
                .is_some_and(merlin_api::RunControl::is_cancelled)
    }
    /// Copies current bounded progress independently of a running session.
    ///
    /// Returned snapshots remain owned and unchanged across later polling/runs.
    pub fn progress(&self) -> ProgressSnapshot {
        self.inner
            .progress
            .lock()
            .unwrap_or_else(|poison| poison.into_inner())
            .clone()
    }
    /// Arms deterministic owner-test cancellation at a completed-grant publication.
    ///
    /// # Arguments
    /// * `completed_grants` - Completed publication count triggering cooperative stop.
    ///
    /// # Returns
    /// Unit; this private test-only seam does not exist in production controls.
    #[cfg(test)]
    pub(crate) fn cancel_after_completed_grants_for_test(&self, completed_grants: usize) {
        self.inner
            .cancel_after_completed_grants
            .store(completed_grants, Ordering::Release);
    }

    pub(crate) fn publish(&self, progress: ProgressSnapshot) {
        #[cfg(test)]
        let cancel_at_boundary = {
            let threshold = self
                .inner
                .cancel_after_completed_grants
                .load(Ordering::Acquire);
            threshold != TEST_GRANT_CANCELLATION_UNSET && progress.completed_grants >= threshold
        };
        *self
            .inner
            .progress
            .lock()
            .unwrap_or_else(|poison| poison.into_inner()) = progress;
        // The progress MutexGuard from the assignment is released before cancel
        // takes the pause lock; test cancellation cannot introduce a lock cycle.
        #[cfg(test)]
        if cancel_at_boundary {
            self.cancel();
        }
    }
    pub(crate) fn claim(&self) -> PhyResult<Lease> {
        self.inner
            .claimed
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| PhyError::Association {
                field: "control.busy",
            })?;
        self.inner
            .pause
            .lock()
            .unwrap_or_else(|p| p.into_inner())
            .accumulated = Duration::ZERO;
        Ok(Lease(self.clone()))
    }
}

impl merlin_api::Cancellation for RunControl {
    /// Reads directional and optional common cancellation without changing state.
    ///
    /// # Returns
    /// True after either shared request is cancelled; pause is a separate state.
    fn is_cancelled(&self) -> bool {
        self.is_cancelled()
    }
}

pub(crate) struct Lease(RunControl);
impl Drop for Lease {
    fn drop(&mut self) {
        self.0.inner.claimed.store(false, Ordering::Release);
    }
}

impl RetainedPayload for RunStatus {
    /// Audits every inline variant without counting borrowed or fixed data twice.
    ///
    /// # Returns
    /// Zero dynamic children; the enclosing root/container owns the fixed bytes.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Idle
            | Self::Running
            | Self::Paused
            | Self::Completed
            | Self::Cancelled
            | Self::TimeLimit
            | Self::Failed => Some(RetainedFootprint::default()),
        }
    }
}

impl RetainedPayload for RunControl {
    /// Visits the directional shared State beyond the enclosing fixed handle.
    ///
    /// # Returns
    /// Complete private child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { inner } = self;
        inner.retained_children()
    }
}

impl RetainedPayload for State {
    /// Audits private control ownership with no nested progress/pause lock order.
    ///
    /// # Returns
    /// Optional common cancellation and scalar-state children, or None on overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            cancel,
            #[cfg(test)]
            cancel_after_completed_grants,
            shared_cancellation,
            claimed,
            progress,
            pause,
            wake,
        } = self;
        let _: (&AtomicBool, &AtomicBool, &Condvar) = (cancel, claimed, wake);
        #[cfg(test)]
        let _: &std::sync::atomic::AtomicUsize = cancel_after_completed_grants;
        let progress = {
            let progress = progress.lock().unwrap_or_else(|cause| cause.into_inner());
            progress_children(&progress)
        };
        let pause = {
            let pause = pause.lock().unwrap_or_else(|cause| cause.into_inner());
            pause_children(&pause)
        };
        retention::sum([shared_cancellation.retained_children(), progress, pause])
    }
}

fn progress_children(value: &ProgressSnapshot) -> Option<RetainedFootprint> {
    let ProgressSnapshot {
        status,
        run,
        completed_grants,
        emitted_samples,
        compared_bits,
        elapsed_ms,
    } = value;
    retention::sum([
        status.retained_children(),
        run.retained_children(),
        completed_grants.retained_children(),
        emitted_samples.retained_children(),
        compared_bits.retained_children(),
        elapsed_ms.retained_children(),
    ])
}

fn pause_children(value: &PauseState) -> Option<RetainedFootprint> {
    let PauseState {
        requested,
        since,
        accumulated,
    } = value;
    retention::sum([
        requested.retained_children(),
        retention::fixed_children(since),
        retention::fixed_children(accumulated),
    ])
}

#[cfg(test)]
mod retention_tests;
