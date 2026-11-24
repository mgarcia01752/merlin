/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit shared application storage; handles never select a direction implicitly.
use crate::shell::{Retained, RetainedResources, RetentionBudget, RetentionError};
use std::{
    mem::size_of,
    ops::Deref,
    sync::{Arc, Mutex, MutexGuard},
};

/// One owner-scoped allocation, transferable with independently prepared models.
pub(crate) struct Store<T>(Arc<Retained<Mutex<T>>>);

// Each Store owns one shared pointee; clones reuse its original reservation.
const STORE_SHARED_ROOTS: usize = 1;

/// Read-only view that keeps its original store locked for the borrow lifetime.
pub(crate) struct ReadGuard<'store, T>(MutexGuard<'store, T>);

impl<T> Store<T> {
    /// Admits one fixed shared Store allocation before allocating its Arc pointee.
    /// # Arguments
    /// * `value` - Initial payload; its dynamic children require separate admission.
    /// * `budget` - Exact canonical counter also covering old and candidate owners.
    /// # Returns
    /// A shared mutex whose data drops before its last covering reservation.
    /// The containing Store handle belongs to its enclosing owner's fixed layout.
    /// # Errors
    /// Rejects checked accounting overflow or any insufficient byte/root limit
    /// without allocating the Store pointee or changing existing owners.
    pub(crate) fn new(value: T, budget: &RetentionBudget) -> Result<Self, RetentionError> {
        let reservation = budget.reserve_resources(RetainedResources {
            payload_bytes: size_of::<Retained<Mutex<T>>>(),
            ordered_entries: 0,
            shared_roots: STORE_SHARED_ROOTS,
            ..Default::default()
        })?;
        Ok(Self(Arc::new(reservation.retain(Mutex::new(value)))))
    }

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
        self.0
            .value()
            .lock()
            .unwrap_or_else(|cause| cause.into_inner())
    }

    /// Observes destruction without retaining application payloads in a test.
    /// # Returns
    /// A weak allocation reference; this operation adds no production interface.
    #[cfg(test)]
    pub(crate) fn downgrade(&self) -> std::sync::Weak<Retained<Mutex<T>>> {
        Arc::downgrade(&self.0)
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

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;

    /// Borrows the locked payload for the lifetime of this read-only guard.
    /// # Returns
    /// An immutable reference; releasing the guard releases the original lock.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod retention_tests;
