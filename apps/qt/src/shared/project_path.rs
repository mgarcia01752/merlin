/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Actual project-path ownership shared by startup, the loader and Save.
use crate::shell::{Retained, RetainedResources, RetentionBudget};
use std::{mem::size_of, path::PathBuf};

/// Exclusive original path and its exact retained wrapper/capacity reservation.
pub(crate) type ProjectPath = Box<Retained<PathBuf>>;

/// Shared native project filename contract, independent of schema versions.
pub(crate) const WORKBENCH_FILE_SUFFIX: &str = ".merlin.json";

/// Normalizes a native Save destination without confirming implicit replacement.
/// # Arguments
/// * `destination` - Local path selected by the user, possibly without the suffix.
/// # Returns
/// Original full filename or a new suffixed path.
/// # Errors
/// An existing implicitly suffixed destination requires explicit full-name selection.
pub(crate) fn normalize_destination(mut destination: PathBuf) -> Result<PathBuf, String> {
    if !destination
        .to_string_lossy()
        .ends_with(WORKBENCH_FILE_SUFFIX)
    {
        destination = append_destination_suffix(&destination)?;
    }
    Ok(destination)
}

/// Adds the shared project suffix only when that implicit destination is absent.
/// # Arguments
/// * `destination` - Native chooser path whose filename lacks the suffix.
/// # Returns
/// New complete project path; the source path is never changed.
/// # Errors
/// Existing destination requires user confirmation through the full filename.
pub(crate) fn append_destination_suffix(destination: &std::path::Path) -> Result<PathBuf, String> {
    let destination = PathBuf::from(format!("{}{WORKBENCH_FILE_SUFFIX}", destination.display()));
    if std::fs::symlink_metadata(&destination).is_ok() {
        return Err(
            "Choose the full .merlin.json filename to confirm replacement of that project.".into(),
        );
    }
    Ok(destination)
}

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
