/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Pre-copy admission of one detached archive and its actual application wrapper.
use super::Snapshot;
use crate::shell::{RetainedResources, RetentionBudget, RetentionError};
use merlin::{
    common::RetainedFootprint,
    downstream::{
        ApiError, FullLoopReceiverArchive, FullLoopReceiverArchivePolicy, FullLoopResult,
    },
};
use std::{mem::size_of, sync::Arc};

// The backend bound already includes its own detached Arc; Snapshot adds one actual Arc.
const SNAPSHOT_SHARED_ROOTS: usize = 1;

#[derive(Debug)]
pub(super) enum ArchiveError {
    Backend(ApiError),
    Retention(RetentionError),
}

impl ArchiveError {
    /// Preserves original backend diagnostics or formats the fixed host rejection.
    ///
    /// # Returns
    /// Existing public error text or bounded application wording. Error String,
    /// Entry status/catalog and outer registry storage remain separate consumers.
    pub(super) fn into_message(self) -> String {
        match self {
            Self::Backend(error) => error.to_string(),
            Self::Retention(
                RetentionError::ArithmeticOverflow
                | RetentionError::LimitExceeded { .. }
                | RetentionError::OrderedEntriesExceeded { .. }
                | RetentionError::SharedRootsExceeded { .. }
                | RetentionError::ChannelsExceeded { .. }
                | RetentionError::SlotsExceeded { .. }
                | RetentionError::PresentationUtf16BytesExceeded { .. }
                | RetentionError::PresentationEntriesExceeded { .. }
                | RetentionError::RetainedViewsExceeded { .. }
                | RetentionError::OpenViewsExceeded { .. },
            ) => "The session retained-data limit would be exceeded.".into(),
        }
    }
}

/// Pre-admits the complete target and application wrapper before copying observations.
///
/// # Arguments
/// * `id` - Exact original application profile identity, independent of wire profile IDs.
/// * `result` - Independently charged immutable complete/partial source, kept alive here.
/// * `policy` - Existing validated role, decoder-prefix and logical-byte policy.
/// * `budget` - Canonical host counter also covering source, old and aliased histories.
///
/// # Returns
/// A shared immutable Snapshot; its sole archive is moved, never scientifically cloned.
/// The embedded guard follows every registry/current/old alias until final drop.
///
/// # Errors
/// Original bound/copy errors or checked aggregate rejection before target production.
pub(super) fn snapshot(
    id: u8,
    result: &FullLoopResult,
    policy: FullLoopReceiverArchivePolicy,
    budget: &RetentionBudget,
) -> Result<Arc<Snapshot>, ArchiveError> {
    let limit = result
        .receiver_archive_retained_bound(&policy)
        .map_err(ArchiveError::Backend)?;
    let reservation = budget
        .reserve_resources(resources(limit).map_err(ArchiveError::Retention)?)
        .map_err(ArchiveError::Retention)?;
    let backend = result
        .receiver_archive_bounded(policy, limit)
        .map_err(ArchiveError::Backend)?;
    Ok(Arc::new(Snapshot {
        id,
        backend,
        _retained: reservation.retain(()),
    }))
}

/// Replaces the backend's fixed handle once with the full actual Snapshot layout.
///
/// # Arguments
/// * `scientific` - Existing complete public detached target bound, not historical retained_bytes.
///
/// # Returns
/// All backend payload/entry/root quantities plus the concrete wrapper and outer Arc edge.
/// Allocator/refcount bookkeeping is excluded; shell accounting metadata is a separate owner.
///
/// # Errors
/// Checked fixed-root replacement, payload or shared-root overflow.
pub(super) fn resources(
    scientific: RetainedFootprint,
) -> Result<RetainedResources, RetentionError> {
    let children = scientific
        .payload_bytes
        .checked_sub(size_of::<FullLoopReceiverArchive>())
        .ok_or(RetentionError::ArithmeticOverflow)?;
    Ok(RetainedResources {
        payload_bytes: children
            .checked_add(size_of::<Snapshot>())
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ordered_entries: scientific.ordered_entries,
        shared_roots: scientific
            .shared_roots
            .checked_add(SNAPSHOT_SHARED_ROOTS)
            .ok_or(RetentionError::ArithmeticOverflow)?,
        ..Default::default()
    })
}

/// Admits an already-produced archive only for the original independent query fixture.
///
/// # Arguments
/// * `id` - Original fixture identity.
/// * `backend` - Existing actual detached fixture; production always reserves before copy.
///
/// # Returns
/// Query-compatible charged wrapper with independent exact fixture policy.
///
/// # Panics
/// Panics on fixture metadata/layout/admission failure.
#[cfg(test)]
pub(super) fn retain_fixture(id: u8, backend: FullLoopReceiverArchive) -> Arc<Snapshot> {
    let required = resources(
        backend
            .retained_footprint()
            .expect("Actual fixture archive metadata"),
    )
    .expect("Actual fixture Snapshot layout");
    let budget = RetentionBudget::with_limits(required);
    let reservation = budget
        .reserve_resources(required)
        .expect("Independent exact fixture policy");
    Arc::new(Snapshot {
        id,
        backend,
        _retained: reservation.retain(()),
    })
}
