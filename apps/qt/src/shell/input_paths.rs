/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! One canonical host's guarded original paths and direction-local protection.
use super::{
    constants::MAX_ORIGINAL_SOURCE_PATHS, Direction, Retained, RetainedResources, RetentionBudget,
    RetentionError, RetentionReservation,
};
use std::{
    borrow::Borrow,
    cell::Cell,
    cmp::Ordering,
    collections::BTreeSet,
    mem::size_of,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, MutexGuard},
};

// Exactly one shared mutex/set pointee belongs to each canonical host.
const REGISTRY_SHARED_ROOTS: usize = 1;

// Each distinct record is one ordered entry; its path and guard are exclusive.
const PATH_ORDERED_ENTRIES: usize = 1;

type Storage = Retained<Mutex<BTreeSet<Record>>>;

#[derive(Clone, Copy, Default)]
struct Owners {
    downstream: bool,
    upstream: bool,
}

impl Owners {
    fn with(mut self, direction: Direction, present: bool) -> Self {
        match direction {
            Direction::Downstream => self.downstream = present,
            Direction::Upstream => self.upstream = present,
        }
        self
    }

    fn any(self) -> bool {
        self.downstream || self.upstream
    }
}

struct Record {
    path: PathBuf,
    owners: Cell<Owners>,

    // Path payload drops before its reservation, independent of tree node order.
    _guard: RetentionReservation,
}

impl Borrow<Path> for Record {
    /// Borrows the sole immutable ordering key without allocating another path.
    /// # Returns
    /// The original canonical path; ownership flags and guard are not part of it.
    fn borrow(&self) -> &Path {
        &self.path
    }
}

impl PartialEq for Record {
    /// Compares only immutable paths, never mutable flags or budget counters.
    /// # Arguments
    /// * `other` - Another admitted original record.
    /// # Returns
    /// True exactly when borrowed canonical lookup addresses the same path.
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
impl Eq for Record {}
impl PartialOrd for Record {
    /// Uses the same immutable ordering as the complete path comparison.
    /// # Arguments
    /// * `other` - Another admitted original record.
    /// # Returns
    /// Always Some; records have a total path order.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Record {
    /// Orders records by canonical path alone, matching Borrow<Path> exactly.
    /// # Arguments
    /// * `other` - Another admitted original record.
    /// # Returns
    /// Stable ordering unaffected by direction ownership or reservation usage.
    fn cmp(&self, other: &Self) -> Ordering {
        self.path.cmp(&other.path)
    }
}

/// Shared registry handle; clones retain one admitted root and the same records.
#[derive(Clone)]
pub(super) struct InputPaths {
    storage: Arc<Storage>,
}

// The key's path is immutable and is its only Borrow/Eq/Ord input. The Cell flags
// and reservation's accounting mutex never participate in lookup or ordering.
#[allow(clippy::mutable_key_type)]
impl InputPaths {
    /// Admits the exact empty registry before allocating its shared pointee.
    /// # Arguments
    /// * `budget` - Existing canonical counter; generic counter defaults stay unchanged.
    /// # Returns
    /// One shared guarded mutex/set, with no Context or host back-reference.
    /// # Errors
    /// Fixed payload or shared-root admission failure, with no partial registry.
    pub(super) fn new(budget: &RetentionBudget) -> Result<Self, RetentionError> {
        let reservation = budget.reserve_resources(RetainedResources {
            payload_bytes: size_of::<Storage>(),
            ordered_entries: 0,
            shared_roots: REGISTRY_SHARED_ROOTS,
            ..Default::default()
        })?;
        Ok(Self {
            storage: Arc::new(reservation.retain(Mutex::new(BTreeSet::new()))),
        })
    }

    /// Protects one successfully read original before editor/scientific staging.
    /// # Arguments
    /// * `budget` - Same host counter that admitted this registry.
    /// * `direction` - Explicit importing owner, independent of tab selection.
    /// * `path` - Actual canonical PathBuf; its requested capacity is measured as owned.
    /// # Returns
    /// Unit after admission/insertion or adding an existing direction flag. A duplicate
    /// never adds another record/charge, even at the distinct-path or host limit.
    /// # Errors
    /// Foreign counter, checked overflow, aggregate byte/entry admission or path cap.
    /// Rejection preserves every earlier record and all ownership flags.
    pub(super) fn register(
        &self,
        budget: &RetentionBudget,
        direction: Direction,
        path: PathBuf,
    ) -> Result<(), String> {
        if !budget.owns(self.storage.as_ref()) {
            return Err("Original source paths belong to another application host.".into());
        }
        let mut records = self.records();
        if let Some(record) = records.get(path.as_path()) {
            record.owners.set(record.owners.get().with(direction, true));
            return Ok(());
        }
        if records.len() >= MAX_ORIGINAL_SOURCE_PATHS {
            return Err(
                "Too many original source paths tracked; reopen a saved workbench to release them."
                    .into(),
            );
        }
        let resources =
            record_resources(&path).ok_or("Original source path ownership size overflowed.")?;
        let guard = budget
            .reserve_resources(resources)
            .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
        records.insert(Record {
            path,
            owners: Cell::new(Owners::default().with(direction, true)),
            _guard: guard,
        });
        Ok(())
    }

    /// Checks the host-wide union without copying a retained path.
    /// # Arguments
    /// * `path` - Borrowed canonical destination, normalized by the filesystem adapter.
    /// # Returns
    /// True while either direction protects that original on this host.
    pub(super) fn contains(&self, path: &Path) -> bool {
        self.records().contains(path)
    }

    /// Releases only one direction after its guarded self-contained Open succeeds.
    /// # Arguments
    /// * `direction` - Successfully replaced owner; the other direction stays protected.
    /// # Returns
    /// Unit after dropping records with no remaining owner. Data drops before guards.
    pub(super) fn clear(&self, direction: Direction) {
        self.records().retain(|record| {
            let owners = record.owners.get().with(direction, false);
            record.owners.set(owners);
            owners.any()
        });
    }

    fn records(&self) -> MutexGuard<'_, BTreeSet<Record>> {
        self.storage
            .value()
            .lock()
            .unwrap_or_else(|cause| cause.into_inner())
    }
}

fn record_resources(path: &PathBuf) -> Option<RetainedResources> {
    Some(RetainedResources {
        payload_bytes: size_of::<Record>().checked_add(path.capacity())?,
        ordered_entries: PATH_ORDERED_ENTRIES,
        shared_roots: 0,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests;
