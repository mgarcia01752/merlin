/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded JSON persistence; no scientific post-processing.
use std::{
    io::{Read, Write},
    path::Path,
};
const MAX_JSON_BYTES: u64 = 16 * 1024 * 1024;
/// Reads one bounded JSON document from a local path.
/// # Arguments
/// * `path` - Source selected by the command.
/// # Returns
/// An owned deserialized document, without scientific interpretation.
/// # Errors
/// Rejects I/O, byte-limit and schema failures.
pub(crate) fn read_json<Document: serde::de::DeserializeOwned>(
    path: &Path,
) -> Result<Document, String> {
    serde_json::from_slice(&read_bytes(path)?).map_err(|error| error.to_string())
}
/// Reads exact bounded JSON bytes for schema adaptation and provenance hashing.
/// # Arguments
/// * `path` - Caller-selected source.
/// # Returns
/// Owned original bytes without rewriting whitespace or scientific fields.
/// # Errors
/// Rejects I/O or input exceeding16MiB, including growing files.
pub(crate) fn read_bytes(path: &Path) -> Result<Vec<u8>, String> {
    let file =
        std::fs::File::open(path).map_err(|error| format!("json-input-open-failed: {error}"))?;
    let mut bytes = vec![];
    file.take(MAX_JSON_BYTES + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() as u64 > MAX_JSON_BYTES {
        return Err("Input exceeds16MiB.".into());
    }
    Ok(bytes)
}
/// Serializes a bounded document before creating a new destination.
/// # Arguments
/// * `document` - Complete typed Rust result.
/// * `path` - New destination; an existing file is never replaced.
/// # Returns
/// Unit after syncing output; partial writes are cleaned up on failure.
/// # Errors
/// Reports serialization limits, existing paths and I/O failures.
pub(crate) fn write_json(document: &impl serde::Serialize, path: &Path) -> Result<(), String> {
    // Serialization is bounded before opening a destination, preserving transactional failure.
    struct Bounded(Vec<u8>);
    impl Write for Bounded {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self.0.len() + bytes.len() > MAX_JSON_BYTES as usize {
                return Err(std::io::Error::other("Output exceeds16MiB."));
            }
            self.0.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut bytes = Bounded(vec![]);
    serde_json::to_writer_pretty(&mut bytes, document).map_err(|error| error.to_string())?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file.write_all(&bytes.0).and_then(|()| file.sync_all()) {
        drop(file);
        if let Err(cleanup_error) = std::fs::remove_file(path) {
            log::error!("json-output-cleanup-failed error={cleanup_error}");
        }
        return Err(format!("json-output-write-failed: {error}"));
    }
    Ok(())
}
