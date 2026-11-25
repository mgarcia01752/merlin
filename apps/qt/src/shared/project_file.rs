/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded atomic byte writing shared by native project adapters.
//!
//! Extracted from downstream `Project::write` at MERLIN revision 4b18caf84d27.
//! Scientific validation, serialization and the host-wide original-input registry
//! guard remain caller responsibilities and must complete before this writer runs.

use std::{fs, io::Write, path::Path};

/// Existing desktop project transport ceiling: 144 MiB, with one MiB = 1024² bytes.
/// This bounds serialized project output, independently of scientific validation.
pub(crate) const MAX_PROJECT_BYTES: usize = 144 * 1024 * 1024;

/// Writes already-validated project bytes through an exclusive sibling file.
///
/// The caller must first validate the complete scientific project and reject
/// destinations protected by the host-wide original-input registry. This helper
/// neither parses bytes nor has access to that registry. Callers retain ownership
/// of the borrowed bytes throughout this synchronous operation.
///
/// # Arguments
/// * `path` - Explicit user-selected project destination, checked by the caller's
///   original-input guard before invocation.
/// * `bytes` - Complete serialized project bytes, at most `MAX_PROJECT_BYTES`.
///
/// # Returns
/// Unit after writing, syncing the sibling file and atomically renaming it to the
/// destination. Existing regular contents are untouched until rename succeeds.
/// The parent directory is not separately synced, matching the existing writer.
///
/// # Errors
/// Rejects oversized output and existing nonregular destinations, including
/// symlinks and dangling symlinks. Returns clock/filesystem errors as strings.
/// Failed writes remove only the sibling created by this call; a pre-existing
/// sibling is never removed. Failed rename preserves the previous destination.
pub(crate) fn write_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    write_bytes_with_limit(path, bytes, MAX_PROJECT_BYTES)
}

fn write_bytes_with_limit(path: &Path, bytes: &[u8], maximum_bytes: usize) -> Result<(), String> {
    if bytes.len() > maximum_bytes {
        return Err("Workbench exceeds 144 MiB.".into());
    }
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            if !metadata.file_type().is_file() {
                return Err("Choose a regular project destination.".into());
            }
        }
        Err(error) => {
            if error.kind() != std::io::ErrorKind::NotFound {
                return Err(error.to_string());
            }
        }
    }
    let temporary = path.with_file_name(format!(
        ".merlin-save-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|error| error.to_string())?
            .as_nanos()
    ));
    write_sibling(path, &temporary, bytes)
}

fn write_sibling(path: &Path, temporary: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temporary)
        .map_err(|error| error.to_string())?;
    let result = (|| -> Result<(), String> {
        file.write_all(bytes).map_err(|error| error.to_string())?;
        file.sync_all().map_err(|error| error.to_string())?;
        fs::rename(temporary, path).map_err(|error| error.to_string())?;
        Ok(())
    })();
    drop(file);
    if result.is_err() {
        let _ = fs::remove_file(temporary);
    }
    result
}

#[cfg(test)]
#[path = "project_file_tests.rs"]
mod tests;
