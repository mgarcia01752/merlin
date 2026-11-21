/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! One-way cooperative cancellation without exposing mutable flag handles.
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Read-only stop query usable by existing atomic callers and public controls.
pub trait Cancellation {
    /// Reads a cooperative stop request without changing caller or worker state.
    ///
    /// # Returns
    /// True after cancellation; checking never resets the request.
    fn is_cancelled(&self) -> bool;
}

/// Cloneable Send+Sync stop token, independent of directional worker locking.
///
/// Clones share one request. This owns no worker, pause gate or progress model.
#[derive(Debug, Clone, Default)]
pub struct RunControl {
    cancel: Arc<AtomicBool>,
}

impl RunControl {
    /// Creates a fresh uncancelled control without starting a worker.
    ///
    /// # Returns
    /// An owned shareable token; cancellation also affects later dispatches.
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cooperative stop at the next bounded engine boundary.
    ///
    /// # Returns
    /// Unit; completed scientific results remain owned by the directional engine.
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }

    /// Reads the shared request without exposing its mutable atomic storage.
    ///
    /// # Returns
    /// True after any clone requests cancellation, with the original relaxed load.
    pub fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

impl Cancellation for RunControl {
    /// Reads the same one-way request used by the public control.
    ///
    /// # Returns
    /// The current shared cancellation state; no synchronization state is reset.
    fn is_cancelled(&self) -> bool {
        self.is_cancelled()
    }
}

impl Cancellation for AtomicBool {
    /// Preserves the existing downstream atomic cancellation query semantics.
    ///
    /// # Returns
    /// The caller-owned atomic value using relaxed ordering.
    fn is_cancelled(&self) -> bool {
        self.load(Ordering::Relaxed)
    }
}
