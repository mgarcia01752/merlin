/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! One-shot preview control; no worker, global result store or interpreter callback.
use super::*;
use merlin_api::{
    retention_support::{self as retention, RetainedPayload},
    RetainedFootprint,
};
use serde::Serialize;
use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

/// Preview lifecycle, independent of the full-loop simulation controller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TransmitterPreviewState {
    Idle,
    Running,
    Completed,
    Cancelled,
    Failed,
}

/// Bounded scalar progress; completed/total counts describe TX symbols only.
#[derive(Debug, Serialize)]
pub struct TransmitterPreviewProgress {
    pub state: TransmitterPreviewState,
    pub completed_symbols: usize,
    pub total_symbols: usize,
    pub cancellation_requested: bool,
}

#[derive(Default)]
struct Shared {
    stop: RunControl,
    state: AtomicU8,
    completed: AtomicUsize,
    total: AtomicUsize,
}

/// Cloneable Send+Sync one-shot token. It starts no thread and cannot be reset/reused.
#[derive(Clone, Default)]
pub struct TransmitterPreviewControl(Arc<Shared>);

impl TransmitterPreviewControl {
    /// Creates idle uncancelled progress/cancellation storage without starting work.
    /// # Returns
    /// A fresh control; clones share the same single generation admission.
    pub fn new() -> Self {
        Self::default()
    }

    /// Measures current preview-control storage without claiming or advancing it.
    ///
    /// # Returns
    /// Exact root/Shared/common-stop payload layouts, two shared roots and zero
    /// ordered entries. Clones report the same conservative footprint independently.
    /// Excludes private allocator/refcount bookkeeping; no atomic is read or stored,
    /// no worker starts and existing progress/cancellation remains unchanged.
    ///
    /// # Errors
    /// ResourceLimit if checked retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<RetainedFootprint> {
        retention::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }

    /// Requests cooperative cancellation at the existing bounded TX/analysis boundaries.
    /// # Returns
    /// Unit; completed previously returned results remain usable. No pause is supported.
    pub fn cancel(&self) {
        self.0.stop.cancel();
    }

    /// Reads scalar progress without taking scientific result locks.
    /// # Returns
    /// Current lifecycle/counts; a pre-start stop request remains idle until dispatch.
    pub fn snapshot(&self) -> TransmitterPreviewProgress {
        let state = match self.0.state.load(Ordering::Acquire) {
            0 => TransmitterPreviewState::Idle,
            1 => TransmitterPreviewState::Running,
            2 => TransmitterPreviewState::Completed,
            3 => TransmitterPreviewState::Cancelled,
            _ => TransmitterPreviewState::Failed,
        };
        TransmitterPreviewProgress {
            state,
            completed_symbols: self.0.completed.load(Ordering::Relaxed),
            total_symbols: self.0.total.load(Ordering::Relaxed),
            cancellation_requested: self.0.stop.is_cancelled(),
        }
    }

    pub(super) fn begin(&self) -> ApiResult<()> {
        self.0
            .state
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| ApiError::new(ErrorCode::Busy, "TX preview control is already used"))?;
        Ok(())
    }
    pub(super) fn stop(&self) -> RunControl {
        self.0.stop.clone()
    }
    pub(super) fn progress(&self, done: usize, total: usize) {
        self.0.total.store(total, Ordering::Relaxed);
        self.0.completed.store(done, Ordering::Relaxed);
    }
    pub(super) fn finish(&self, success: bool) {
        self.0.state.store(
            if self.0.stop.is_cancelled() {
                3
            } else if success {
                2
            } else {
                4
            },
            Ordering::Release,
        );
    }
}

impl RetainedPayload for TransmitterPreviewControl {
    /// Visits its shared preview owner beyond the enclosing fixed handle.
    ///
    /// # Returns
    /// Complete private child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self(shared) = self;
        shared.retained_children()
    }
}

impl RetainedPayload for Shared {
    /// Audits the private atomics and the separate common cancellation child.
    ///
    /// # Returns
    /// Common stop child quantities; all scalar synchronization storage is inline.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            stop,
            state,
            completed,
            total,
        } = self;
        let _: (&AtomicU8, &AtomicUsize, &AtomicUsize) = (state, completed, total);
        stop.retained_children()
    }
}

#[cfg(test)]
mod retention_tests;
