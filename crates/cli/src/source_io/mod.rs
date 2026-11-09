/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! OS-file adaptation only; source selection, fill and verification stay in core.
use docsis_ofdm_fec::simulation::source::file::MAX_FILE_BYTES;
use std::{
    io::{Read, Write},
    path::Path,
};
/// Reads exact bounded bytes from a regular payload file without interpreting text.
/// # Arguments
/// * `path` - User-selected local file, unrelated to PNM measurement inputs.
/// # Returns
/// Exact bytes, preserving zeros, BOM and line endings.
/// # Errors
/// Reports I/O, nonregular files or input exceeding the core16MiB bound, including growth.
pub(crate) fn read_payload(path: &Path) -> Result<Vec<u8>, String> {
    // Reject nonregular inputs before opening, then check the opened descriptor too.
    if !std::fs::metadata(path)
        .map_err(|error| error.to_string())?
        .is_file()
    {
        return Err("Payload must be a regular file.".into());
    }
    let file =
        std::fs::File::open(path).map_err(|error| format!("payload-input-open-failed: {error}"))?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() > MAX_FILE_BYTES as u64 {
        return Err("Payload must be a regular file no larger than16MiB.".into());
    }
    let mut bytes = Vec::new();
    file.take(MAX_FILE_BYTES as u64 + 1)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > MAX_FILE_BYTES {
        return Err("Payload file exceeds16MiB.".into());
    }
    Ok(bytes)
}
/// Exports recovered original-file bytes with create-new semantics.
/// # Arguments
/// * `path` - New destination; existing files are never replaced.
/// * `bytes` - Core-provided whole-byte prefix, excluding fill and coding padding.
/// # Returns
/// Unit after syncing output; incomplete/error-containing recovery remains labeled in JSON.
/// # Errors
/// Reports creation/write/sync failure and cleans only this call's incomplete output.
pub(crate) fn write_recovered(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|error| error.to_string())?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        drop(file);
        if let Err(cleanup) = std::fs::remove_file(path) {
            log::error!("recovered-output-cleanup-failed error={cleanup}");
        }
        return Err(format!("recovered-output-write-failed: {error}"));
    }
    Ok(())
}
