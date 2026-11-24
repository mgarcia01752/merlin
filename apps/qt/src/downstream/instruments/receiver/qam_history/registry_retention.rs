/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Passive storage metadata for the QAM registry outside its charged Snapshots.
use super::{Entry, Snapshot};
use crate::shell::{RetainedResources, RetentionError};
use std::{mem::size_of, sync::Arc};

/// Measures every actual registry slot and each live entry's owned status buffer.
///
/// # Arguments
/// * `entries` - Exact retained registry, including uninitialized spare Vec slots.
///
/// # Returns
/// Requested buffer bytes only, with zero ordered entries and shared roots.
/// The containing Vec handle belongs to the separate Store infrastructure owner.
/// Each Snapshot already retains its own complete archive/wrapper/root charge;
/// this visitor borrows its existing Arc without cloning or counting it again.
///
/// # Errors
/// Checked slot multiplication or status-capacity addition overflow. No owner,
/// reservation, capacity, scientific value or alias is changed.
pub(super) fn resources(entries: &Vec<Entry>) -> Result<RetainedResources, RetentionError> {
    let mut payload_bytes = entries
        .capacity()
        .checked_mul(size_of::<Entry>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    for entry in entries {
        let Entry {
            id,
            status,
            snapshot,
        } = entry;
        let _: u8 = *id;

        // The inline optional Arc handle is in its Vec slot. Its pointee and
        // outer shared root remain covered by Snapshot's existing last guard.
        let _: &Option<Arc<Snapshot>> = snapshot;
        payload_bytes = payload_bytes
            .checked_add(status.capacity())
            .ok_or(RetentionError::ArithmeticOverflow)?;
    }
    Ok(RetainedResources {
        payload_bytes,
        ordered_entries: 0,
        shared_roots: 0,
    })
}

#[cfg(test)]
mod tests;
