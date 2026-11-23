/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application byte accounting that follows accepted, queued and old-run ownership.
use std::sync::{Arc, Mutex, MutexGuard};

/// Conservative upper bound in bytes for reachable retained payload allocations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedBytes(pub usize);

/// Failure to reserve application memory without changing existing ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetentionError {
    ArithmeticOverflow,
    LimitExceeded {
        maximum: RetainedBytes,
        requested: RetainedBytes,
    },
}

#[derive(Debug)]
struct Accounting {
    maximum: RetainedBytes,
    used: RetainedBytes,
}

/// Shared application policy; cloning shares one counter rather than a new budget.
#[derive(Clone, Debug)]
pub struct RetentionBudget(Arc<Mutex<Accounting>>);

/// Reserved bytes, released on drop unless transferred into a retained payload.
#[derive(Debug)]
pub struct RetentionReservation {
    budget: RetentionBudget,
    bytes: RetainedBytes,
}

/// Exclusively owned payload carrying its aggregate application byte reservation.
///
/// Reserve before allocation. The charge must cover all reachable owned data;
/// external aliases/clones must carry their own conservative reservation. This
/// wrapper does not inspect heap allocations or impose backend runtime limits.
#[derive(Debug)]
pub struct Retained<T> {
    // Drop data before releasing its reservation, including user-defined Drop.
    value: T,
    reservation: RetentionReservation,
}

impl RetentionBudget {
    /// Creates shared accounting with an explicit application byte ceiling.
    ///
    /// # Arguments
    /// * `maximum` - Inclusive policy ceiling; zero admits only zero-byte payloads.
    ///
    /// # Returns
    /// An empty budget; no scientific or platform default is assumed.
    pub fn new(maximum: RetainedBytes) -> Self {
        Self(Arc::new(Mutex::new(Accounting {
            maximum,
            used: RetainedBytes(0),
        })))
    }

    /// Reserves bytes before creating or cloning retained payload allocations.
    ///
    /// # Arguments
    /// * `bytes` - Conservative additional ownership bound in bytes.
    ///
    /// # Returns
    /// An owned reservation that releases its charge on drop.
    ///
    /// # Errors
    /// Returns overflow or a policy-limit error without changing the counter.
    pub fn reserve(&self, bytes: RetainedBytes) -> Result<RetentionReservation, RetentionError> {
        let mut accounting = lock(&self.0);
        let requested = accounting
            .used
            .0
            .checked_add(bytes.0)
            .ok_or(RetentionError::ArithmeticOverflow)?;
        if requested > accounting.maximum.0 {
            return Err(RetentionError::LimitExceeded {
                maximum: accounting.maximum,
                requested: RetainedBytes(requested),
            });
        }
        accounting.used = RetainedBytes(requested);
        Ok(RetentionReservation {
            budget: self.clone(),
            bytes,
        })
    }

    /// Reads current reserved ownership, including queued and old payloads.
    ///
    /// # Returns
    /// The aggregate charge in bytes across every clone of this budget.
    pub fn used(&self) -> RetainedBytes {
        lock(&self.0).used
    }

    /// Checks whether a payload participates in this exact shared accounting owner.
    ///
    /// # Arguments
    /// * `T` - Candidate payload type; no scientific inspection is performed.
    /// * `payload` - Candidate retained payload whose reservation is inspected.
    ///
    /// # Returns
    /// True only for the same budget allocation, not merely an equal byte ceiling.
    pub(super) fn owns<T>(&self, payload: &Retained<T>) -> bool {
        Arc::ptr_eq(&self.0, &payload.reservation.budget.0)
    }
}

impl RetentionReservation {
    /// Attaches this reservation to an exclusively owned payload.
    ///
    /// # Arguments
    /// * `T` - Exclusively owned payload type.
    /// * `value` - Data allocated within the previously reserved byte bound.
    ///
    /// # Returns
    /// Immutable retained ownership, charged until the payload is destroyed.
    pub fn retain<T>(self, value: T) -> Retained<T> {
        Retained {
            value,
            reservation: self,
        }
    }
}

impl<T> Retained<T> {
    /// Borrows retained data without transferring or cloning its allocation.
    ///
    /// # Returns
    /// A shared reference limited to this retained owner's lifetime.
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl Drop for RetentionReservation {
    /// Releases exactly this reservation after its retained payload has dropped.
    ///
    /// No worker or payload allocation is inspected or waited for here.
    fn drop(&mut self) {
        lock(&self.budget.0).used.0 -= self.bytes.0;
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
