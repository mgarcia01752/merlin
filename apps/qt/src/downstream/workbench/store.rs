/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit shared application storage; handles never select a direction implicitly.
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

/// One owner-scoped allocation, transferable with independently prepared models.
pub(crate) struct Store<T>(Arc<Mutex<T>>);

/// Read-only view that keeps its original store locked for the borrow lifetime.
pub(crate) struct ReadGuard<'store, T>(MutexGuard<'store, T>);

impl<T: Default> Default for Store<T> {
    /// Creates independent default application storage, without admitting science.
    /// # Returns
    /// A new allocation; cloning this handle subsequently shares that allocation.
    fn default() -> Self {
        Self(Arc::new(Mutex::new(T::default())))
    }
}

impl<T> Clone for Store<T> {
    /// Shares the exact original owner without copying its retained payload.
    /// # Returns
    /// Another handle whose lifetime keeps the same allocation alive.
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> Store<T> {
    /// Applies a scoped operation to this exact store without a registry lookup.
    /// # Arguments
    /// * `operation` - Accesses this owner through its read/write guards.
    /// # Returns
    /// The operation's owned result; guards cannot outlive this store borrow.
    pub(crate) fn with<R>(&self, operation: impl FnOnce(&Self) -> R) -> R {
        operation(self)
    }

    /// Borrows the current payload without permitting mutation through this guard.
    /// # Returns
    /// A read-only guard; recover the original payload after mutex poisoning.
    pub(crate) fn borrow(&self) -> ReadGuard<'_, T> {
        ReadGuard(self.borrow_mut())
    }

    /// Borrows this owner's payload exclusively for an application state change.
    /// # Returns
    /// A write guard; no selected-tab lookup, cloning or scientific work occurs.
    pub(crate) fn borrow_mut(&self) -> MutexGuard<'_, T> {
        self.0.lock().unwrap_or_else(|cause| cause.into_inner())
    }

    /// Observes destruction without retaining application payloads in a test.
    /// # Returns
    /// A weak allocation reference; this operation adds no production interface.
    #[cfg(test)]
    pub(crate) fn downgrade(&self) -> std::sync::Weak<Mutex<T>> {
        Arc::downgrade(&self.0)
    }
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    /// Borrows the locked payload for the lifetime of this read-only guard.
    /// # Returns
    /// An immutable reference; releasing the guard releases the original lock.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
