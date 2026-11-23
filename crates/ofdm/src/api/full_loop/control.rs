/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direction-specific one-shot control; acknowledged pause is separate from requested pause.
use super::*;
pub use crate::simulation::channel::{ChannelJobStatus, ChannelPhase, ChannelProgress};
use serde::Serialize;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex, MutexGuard,
    },
    time::Instant,
};

/// Actual controlled-job lifecycle; a pause request is acknowledged only at a safe boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FullLoopState {
    Idle,
    Running,
    Paused,
    Completed,
    Cancelled,
    RuntimeExceeded,
    Failed,
}
/// Constant-size last progress notification and explicit requested control flags.
#[derive(Clone, Serialize)]
pub struct FullLoopProgress {
    pub state: FullLoopState,
    pub pause_requested: bool,
    pub cancellation_requested: bool,
    pub progress: Option<ChannelProgress>,
}
struct ProgressState {
    snapshot: FullLoopProgress,
    last_notification: Option<Instant>,
}
struct Shared {
    stop: RunControl,
    paused: AtomicBool,
    state: Mutex<ProgressState>,
}

/// Cloneable Send+Sync one-shot execution token; no worker or result storage is owned here.
#[derive(Clone)]
pub struct FullLoopControl(Arc<Shared>);
impl Default for FullLoopControl {
    /// Creates idle scalar state with no cancellation or pause request.
    /// # Returns
    /// Fresh shared one-shot control storage, independent of all other jobs.
    fn default() -> Self {
        Self(Arc::new(Shared {
            stop: RunControl::new(),
            paused: AtomicBool::new(false),
            state: Mutex::new(ProgressState {
                snapshot: FullLoopProgress {
                    state: FullLoopState::Idle,
                    pause_requested: false,
                    cancellation_requested: false,
                    progress: None,
                },
                last_notification: None,
            }),
        }))
    }
}
impl FullLoopControl {
    /// Creates a fresh idle one-shot control without starting a worker.
    /// # Returns
    /// Shareable token; clones observe/control the same single job.
    pub fn new() -> Self {
        Self::default()
    }
    /// Requests pause before dispatch or at the next completed scientific step.
    /// # Returns
    /// Unit; snapshot distinguishes requested pause from acknowledged Paused state.
    /// # Errors
    /// Busy after terminal completion; accepted results and scientific state remain intact.
    pub fn pause(&self) -> ApiResult<()> {
        let mut state = self.state();
        if !matches!(
            state.snapshot.state,
            FullLoopState::Idle | FullLoopState::Running | FullLoopState::Paused
        ) {
            return Err(ApiError::new(
                ErrorCode::Busy,
                "Full-loop control has finished",
            ));
        }
        self.0.paused.store(true, Ordering::Relaxed);
        state.snapshot.pause_requested = true;
        Ok(())
    }
    /// Releases a requested/acknowledged pause without reconstructing or reseeding the job.
    /// # Returns
    /// Unit; the actual worker reports Running when it leaves the safe-boundary wait.
    pub fn resume(&self) {
        self.0.paused.store(false, Ordering::Relaxed);
        self.state().snapshot.pause_requested = false;
    }
    /// Requests one-way stop, also honored during a paused safe-boundary wait.
    /// # Returns
    /// Unit; already completed scientific comparisons remain in the returned partial report.
    pub fn cancel(&self) {
        self.0.stop.cancel();
    }
    /// Reads bounded scalar progress without locking scientific result data.
    /// # Returns
    /// Owned last-step/lifecycle snapshot with actual requested control flags.
    pub fn snapshot(&self) -> FullLoopProgress {
        let mut result = self.state().snapshot.clone();
        result.pause_requested = self.0.paused.load(Ordering::Relaxed);
        result.cancellation_requested = self.0.stop.is_cancelled();
        result
    }

    /// Reads monotonic wall time since the backend's last completed-step notification.
    ///
    /// # Returns
    /// None before any progress notification, otherwise elapsed whole milliseconds
    /// saturated at u64::MAX. Age continues through pauses and terminal states;
    /// it is separate from simulation time and the runner's active host-runtime budget.
    /// Reads the scalar timestamp under the progress mutex without changing the job,
    /// reporting a new notification or accessing scientific results.
    pub fn last_progress_age_ms(&self) -> Option<u64> {
        self.state()
            .last_notification
            .map(|notified| u64::try_from(notified.elapsed().as_millis()).unwrap_or(u64::MAX))
    }

    fn state(&self) -> MutexGuard<'_, ProgressState> {
        self.0
            .state
            .lock()
            .unwrap_or_else(|cause| cause.into_inner())
    }

    pub(super) fn begin(&self) -> ApiResult<()> {
        let mut state = self.state();
        if state.snapshot.state != FullLoopState::Idle {
            return Err(ApiError::new(
                ErrorCode::Busy,
                "Full-loop control is already used",
            ));
        }
        state.snapshot.state = FullLoopState::Running;
        Ok(())
    }
    pub(super) fn stop(&self) -> RunControl {
        self.0.stop.clone()
    }
    pub(super) fn pause_gate(&self) -> &AtomicBool {
        &self.0.paused
    }
    pub(super) fn progress(&self, progress: ChannelProgress) {
        let mut state = self.state();
        state.snapshot.progress = Some(progress);
        state.last_notification = Some(Instant::now());
    }
    pub(super) fn acknowledge_pause(&self, paused: bool) {
        self.state().snapshot.state = if paused {
            FullLoopState::Paused
        } else {
            FullLoopState::Running
        };
    }
    pub(super) fn finish(&self, job: Option<&crate::simulation::channel::ChannelJobResult>) {
        let mut state = self.state();
        self.0.paused.store(false, Ordering::Relaxed);
        state.snapshot.pause_requested = false;
        state.snapshot.state = match job.map(|job| job.status) {
            Some(ChannelJobStatus::Complete) => FullLoopState::Completed,
            Some(ChannelJobStatus::Cancelled) => FullLoopState::Cancelled,
            Some(ChannelJobStatus::RuntimeExceeded) => FullLoopState::RuntimeExceeded,
            _ => FullLoopState::Failed,
        };
    }
}
