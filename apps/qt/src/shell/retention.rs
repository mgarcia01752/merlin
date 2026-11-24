/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application resource accounting for accepted, queued and old-run ownership.
use std::{
    mem::size_of,
    sync::{Arc, Mutex, MutexGuard},
};

/// Retained payload bytes, including requested capacity and nested cached data.
///
/// Excludes allocator, private tree-node and reference-count bookkeeping; not RSS
/// or total heap allocation bytes. Shared payloads may be conservatively repeated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetainedBytes(pub usize);

/// Independent aggregate quantities under the approved retention contract.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RetainedResources {
    /// Fixed owned values and reachable payload storage, including spare capacity.
    pub payload_bytes: usize,
    /// Entries in ordered containers whose private node bookkeeping is excluded.
    pub ordered_entries: usize,
    /// Shared allocation owner edges whose private refcount headers are excluded.
    pub shared_roots: usize,
    /// Live bounded channel owners; private std synchronization is excluded.
    pub channel_count: usize,
    /// Configured bounded channel slots, independent of buffered message data.
    pub slot_count: usize,
}

/// Failure to reserve application memory without changing existing ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetentionError {
    ArithmeticOverflow,
    LimitExceeded {
        maximum: RetainedBytes,
        requested: RetainedBytes,
    },
    OrderedEntriesExceeded {
        maximum: usize,
        requested: usize,
    },
    SharedRootsExceeded {
        maximum: usize,
        requested: usize,
    },
    ChannelsExceeded {
        maximum: usize,
        requested: usize,
    },
    SlotsExceeded {
        maximum: usize,
        requested: usize,
    },
}

#[derive(Debug)]
struct Accounting {
    maximum: RetainedResources,
    used: RetainedResources,
}

// The application counter owns one shared payload allocation for its entire life.
const APPLICATION_ACCOUNTING_ROOTS: usize = 1;

/// Shared application policy; cloning shares one counter rather than a new budget.
#[derive(Clone, Debug)]
pub struct RetentionBudget(Arc<Mutex<Accounting>>);

/// Reserved quantities, released on drop unless transferred into a retained payload.
#[derive(Debug)]
pub struct RetentionReservation {
    budget: RetentionBudget,
    resources: RetainedResources,
}

/// Exclusively owned payload carrying its aggregate application reservation.
///
/// Reserve before allocation. The charge must cover all reachable payload data
/// and container/shared-root counts under the approved contract;
/// aliases of `Arc<Retained<T>>` share the charge until final drop. Independently
/// retained payload clones or aliases escaping the wrapper require their own
/// conservative reservations. This wrapper does not inspect heap allocations or
/// impose backend runtime limits.
#[derive(Debug)]
pub struct Retained<T> {
    // Drop data before releasing its reservation, including user-defined Drop.
    value: T,
    reservation: RetentionReservation,
}

impl RetentionBudget {
    /// Creates bytes-only accounting with zero container/shared-root admission.
    ///
    /// # Arguments
    /// * `maximum` - Inclusive policy ceiling; zero admits only zero-byte payloads.
    ///
    /// # Returns
    /// An empty budget; use `with_limits` for multidimensional backend ownership.
    pub fn new(maximum: RetainedBytes) -> Self {
        Self::with_limits(RetainedResources {
            payload_bytes: maximum.0,
            ..RetainedResources::default()
        })
    }

    /// Creates shared accounting with five explicit independent ceilings.
    ///
    /// # Arguments
    /// * `maximum` - Inclusive application policies; zero admits only zero usage
    ///   in that dimension. No scientific or platform default is assumed.
    ///
    /// # Returns
    /// An empty shared budget enforcing every dimension atomically.
    pub fn with_limits(maximum: RetainedResources) -> Self {
        Self(Arc::new(Mutex::new(Accounting {
            maximum,
            used: RetainedResources::default(),
        })))
    }

    /// Creates application accounting that includes its own fixed allocation.
    ///
    /// # Arguments
    /// * `maximum` - Inclusive host policy, including this exact counter baseline.
    ///
    /// # Returns
    /// A counter initially charging its Mutex<Accounting> layout and one root,
    /// with zero ordered entries. Clones share this baseline until final drop.
    /// No reservation points back to its own counter; generic constructors retain
    /// their original zero baseline.
    ///
    /// # Errors
    /// Rejects an insufficient payload or root ceiling before allocating the Arc.
    pub(super) fn for_application(maximum: RetainedResources) -> Result<Self, RetentionError> {
        let baseline = RetainedResources {
            payload_bytes: size_of::<Mutex<Accounting>>(),
            ordered_entries: 0,
            shared_roots: APPLICATION_ACCOUNTING_ROOTS,
            ..RetainedResources::default()
        };
        if baseline.payload_bytes > maximum.payload_bytes {
            return Err(RetentionError::LimitExceeded {
                maximum: RetainedBytes(maximum.payload_bytes),
                requested: RetainedBytes(baseline.payload_bytes),
            });
        }
        if baseline.shared_roots > maximum.shared_roots {
            return Err(RetentionError::SharedRootsExceeded {
                maximum: maximum.shared_roots,
                requested: baseline.shared_roots,
            });
        }
        Ok(Self(Arc::new(Mutex::new(Accounting {
            maximum,
            used: baseline,
        }))))
    }

    /// Reserves bytes before creating or cloning retained payload allocations.
    ///
    /// # Arguments
    /// * `bytes` - Additional payload bytes; claims zero entries/shared roots.
    ///
    /// # Returns
    /// An owned reservation that releases its charge on drop.
    ///
    /// # Errors
    /// Returns overflow or a policy-limit error without changing the counter.
    pub fn reserve(&self, bytes: RetainedBytes) -> Result<RetentionReservation, RetentionError> {
        self.reserve_resources(RetainedResources {
            payload_bytes: bytes.0,
            ..RetainedResources::default()
        })
    }

    /// Reserves all quantities before creating or cloning retained ownership.
    ///
    /// # Arguments
    /// * `resources` - Independently audited additional payload and population
    ///   bounds. This primitive cannot inspect or validate a scientific footprint.
    ///
    /// # Returns
    /// One reservation shared across no other independently retained roots.
    ///
    /// # Errors
    /// Returns arithmetic overflow or a dimension-specific limit error without
    /// changing any counter, even when another dimension would fit.
    pub fn reserve_resources(
        &self,
        resources: RetainedResources,
    ) -> Result<RetentionReservation, RetentionError> {
        let mut accounting = lock(&self.0);
        let requested = accounting.used.checked_add(resources)?;
        if requested.payload_bytes > accounting.maximum.payload_bytes {
            return Err(RetentionError::LimitExceeded {
                maximum: RetainedBytes(accounting.maximum.payload_bytes),
                requested: RetainedBytes(requested.payload_bytes),
            });
        }
        if requested.ordered_entries > accounting.maximum.ordered_entries {
            return Err(RetentionError::OrderedEntriesExceeded {
                maximum: accounting.maximum.ordered_entries,
                requested: requested.ordered_entries,
            });
        }
        if requested.shared_roots > accounting.maximum.shared_roots {
            return Err(RetentionError::SharedRootsExceeded {
                maximum: accounting.maximum.shared_roots,
                requested: requested.shared_roots,
            });
        }
        if requested.channel_count > accounting.maximum.channel_count {
            return Err(RetentionError::ChannelsExceeded {
                maximum: accounting.maximum.channel_count,
                requested: requested.channel_count,
            });
        }
        if requested.slot_count > accounting.maximum.slot_count {
            return Err(RetentionError::SlotsExceeded {
                maximum: accounting.maximum.slot_count,
                requested: requested.slot_count,
            });
        }
        accounting.used = requested;
        Ok(RetentionReservation {
            budget: self.clone(),
            resources,
        })
    }

    /// Reads current reserved ownership, including queued and old payloads.
    ///
    /// # Returns
    /// The aggregate charge in bytes across every clone of this budget.
    pub fn used(&self) -> RetainedBytes {
        RetainedBytes(self.usage().payload_bytes)
    }

    /// Reads all reserved quantities, including queued, old and aliased owners.
    ///
    /// # Returns
    /// One consistent aggregate snapshot across every clone of this budget.
    /// Application accounting includes its permanent fixed baseline; generic
    /// constructors begin at zero and charge only their explicit reservations.
    pub fn usage(&self) -> RetainedResources {
        lock(&self.0).used
    }

    /// Checks whether a payload participates in this exact shared accounting owner.
    ///
    /// # Arguments
    /// * `T` - Candidate payload type; no scientific inspection is performed.
    /// * `payload` - Candidate retained payload whose reservation is inspected.
    ///
    /// # Returns
    /// True only for the same budget allocation, not merely equal ceilings.
    pub(crate) fn owns<T>(&self, payload: &Retained<T>) -> bool {
        Arc::ptr_eq(&self.0, &payload.reservation.budget.0)
    }
}

impl RetentionReservation {
    /// Reads this actual reservation for independent fixture ledger assertions.
    ///
    /// # Returns
    /// Exactly the held quantities, without copying a payload or changing charge.
    #[cfg(test)]
    pub(crate) fn fixture_reserved_resources(&self) -> RetainedResources {
        self.resources
    }

    /// Attaches this reservation to an exclusively owned payload.
    ///
    /// # Arguments
    /// * `T` - Exclusively owned payload type.
    /// * `value` - Data allocated within every previously reserved quantity.
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

    /// Transfers an exclusive queued payload and its live reservation to an editor.
    ///
    /// # Returns
    /// The same payload and guard, without cloning data or releasing the charge.
    /// The caller must store the payload before the guard so data drops first.
    pub(crate) fn into_parts(self) -> (T, RetentionReservation) {
        (self.value, self.reservation)
    }

    /// Reads the assigned reservation for surrounding ownership fixtures only.
    ///
    /// # Returns
    /// Exact ledger quantities, not an independent measurement of the payload.
    /// Production admission and scientific footprint validation do not use this hook.
    #[cfg(test)]
    pub(crate) fn fixture_reserved_resources(&self) -> RetainedResources {
        self.reservation.resources
    }
}

impl Drop for RetentionReservation {
    /// Releases exactly this reservation after its retained payload has dropped.
    ///
    /// No worker or payload allocation is inspected or waited for here.
    fn drop(&mut self) {
        let mut accounting = lock(&self.budget.0);
        accounting.used.payload_bytes -= self.resources.payload_bytes;
        accounting.used.ordered_entries -= self.resources.ordered_entries;
        accounting.used.shared_roots -= self.resources.shared_roots;
        accounting.used.channel_count -= self.resources.channel_count;
        accounting.used.slot_count -= self.resources.slot_count;
    }
}

impl RetainedResources {
    /// Adds audited quantities without changing any application counter.
    ///
    /// # Arguments
    /// * `self` - Existing payload, ordered-entry and shared-root quantities.
    /// * `additional` - Independent quantities to include in the same aggregate.
    ///
    /// # Returns
    /// Checked totals for every dimension; admission remains the budget's job.
    ///
    /// # Errors
    /// Returns ArithmeticOverflow when any sum exceeds usize.
    pub(crate) fn checked_add(self, additional: Self) -> Result<Self, RetentionError> {
        Ok(Self {
            payload_bytes: self
                .payload_bytes
                .checked_add(additional.payload_bytes)
                .ok_or(RetentionError::ArithmeticOverflow)?,
            ordered_entries: self
                .ordered_entries
                .checked_add(additional.ordered_entries)
                .ok_or(RetentionError::ArithmeticOverflow)?,
            shared_roots: self
                .shared_roots
                .checked_add(additional.shared_roots)
                .ok_or(RetentionError::ArithmeticOverflow)?,
            channel_count: self
                .channel_count
                .checked_add(additional.channel_count)
                .ok_or(RetentionError::ArithmeticOverflow)?,
            slot_count: self
                .slot_count
                .checked_add(additional.slot_count)
                .ok_or(RetentionError::ArithmeticOverflow)?,
        })
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod application_tests;

#[cfg(test)]
#[path = "retention_tests.rs"]
mod tests;
