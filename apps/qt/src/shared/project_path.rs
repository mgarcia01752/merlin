/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual project-path ownership shared by startup, the loader and Save.
use crate::shell::{Retained, RetainedResources, RetentionBudget};
use std::{mem::size_of, path::PathBuf};

/// Exclusive original path and its exact retained wrapper/capacity reservation.
pub(crate) type ProjectPath = Box<Retained<PathBuf>>;

/// Admits one actual project path before persistent storage or worker capture.
///
/// # Arguments
/// * `path` - Actual owned path; spare capacity is retained and charged unchanged.
/// * `budget` - Canonical host covering the previous path and this candidate.
///
/// # Returns
/// The same path in a guarded Box, with no shared root or ordered entry claim.
///
/// # Errors
/// Checked layout or aggregate byte rejection before Box allocation and adoption.
pub(crate) fn admit(path: PathBuf, budget: &RetentionBudget) -> Result<ProjectPath, String> {
    let payload_bytes = size_of::<Retained<PathBuf>>()
        .checked_add(path.capacity())
        .ok_or("Project path ownership size overflowed.")?;
    let reservation = budget
        .reserve_resources(RetainedResources {
            payload_bytes,
            ..Default::default()
        })
        .map_err(|_| "The session retained-data limit would be exceeded.".to_owned())?;
    Ok(Box::new(reservation.retain(path)))
}

#[cfg(test)]
mod tests;
