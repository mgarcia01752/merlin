/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! New-file publication with owned staging and explicit provenance sidecars.
use super::{
    encode_format, failure, preflight_format, ExportError, ExportFormat, ExportSummary, RangePolicy,
};
use docsis_ofdm_fec::scenario::RxMerFrame;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
};
/// Caller-selected new destination; neither mode overwrites an existing path.
pub enum ExportDestination<'path> {
    Single(&'path Path),
    Collection(&'path Path),
}
fn io(error: std::io::Error) -> ExportError {
    failure("io", error.to_string())
}
fn cancelled(cancel: &AtomicBool) -> Result<(), ExportError> {
    if cancel.load(Ordering::Relaxed) {
        return Err(failure("cancelled", "PNM export cancelled."));
    }
    Ok(())
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<(), ExportError> {
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(io)?;
    file.write_all(bytes).map_err(io)?;
    file.sync_all().map_err(io)
}
fn publish_directory(source: &Path, destination: &Path) -> Result<(), ExportError> {
    #[cfg(target_os = "linux")]
    {
        rustix::fs::renameat_with(
            rustix::fs::CWD,
            source,
            rustix::fs::CWD,
            destination,
            rustix::fs::RenameFlags::NOREPLACE,
        )
        .map_err(|error| io(error.into()))
    }
    #[cfg(target_os = "windows")]
    {
        std::fs::rename(source, destination).map_err(io)
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        let _ = (source, destination);
        Err(failure(
            "unsupported_platform",
            "Atomic new-directory export is supported on Linux and Windows.",
        ))
    }
}
/// Encodes and saves new PNM files plus a provenance manifest with cancellation.
///
/// # Arguments
/// * `destination` - New `.bin` file or new collection directory on the same volume.
/// * `frames` - Full immutable source-grid RxMER snapshots, revalidated here.
/// * `policy` - Explicit range conversion; no implicit clipping.
/// * `context` - Bounded caller provenance/recipe JSON, not embedded in PNM headers.
/// * `cancel` - Cooperative cancellation checked before each file and publication.
/// * `completed` - Actual completed-capture progress, reset when writing starts.
///
/// # Returns
/// Owned aggregate summary. Single-file output includes `<filename>.merlin.json`;
/// collection output contains numbered `.bin` files and `merlin-export.json`.
/// Staging uses the destination volume. A collection publishes in one no-replace
/// directory rename. A single-file sidecar publishes first, binary last; failure
/// removes only the sidecar created by this call. Source inputs stay unchanged.
///
/// # Errors
/// Preserves validation/capture/carrier context, cancellation, existing-path and
/// filesystem errors. Incomplete owned staging is removed automatically.
pub fn save(
    destination: ExportDestination<'_>,
    frames: &[RxMerFrame],
    policy: RangePolicy,
    context: &Value,
    cancel: &AtomicBool,
    completed: &AtomicUsize,
) -> Result<ExportSummary, ExportError> {
    save_with_format(
        destination,
        frames,
        policy,
        context,
        cancel,
        completed,
        ExportFormat::Pnm,
    )
}
/// Saves binary or lossless decoded JSON with the same guarded new-file publication.
/// # Arguments
/// * `destination` - New file or new collection directory; existing paths fail.
/// * `frames` - Immutable complete-grid capture snapshots.
/// * `policy` - Binary range conversion; ignored for lossless JSON.
/// * `context` - Bounded caller recipe and provenance included in the sidecar.
/// * `cancel` - Cooperative cancellation before each file and publication.
/// * `completed` - Actual completed-capture progress, reset before encoding.
/// * `format` - Binary PNM 1.0 or decoded JSON preserving finite samples and nulls.
/// # Returns
/// Aggregate conversion summary; a companion manifest records format and hashes.
/// # Errors
/// Rejects invalid input, excessive work, cancellation and existing destinations.
pub fn save_with_format(
    destination: ExportDestination<'_>,
    frames: &[RxMerFrame],
    policy: RangePolicy,
    context: &Value,
    cancel: &AtomicBool,
    completed: &AtomicUsize,
    format: ExportFormat,
) -> Result<ExportSummary, ExportError> {
    cancelled(cancel)?;
    if serde_json::to_vec(context)
        .map_err(|error| failure("context", error.to_string()))?
        .len()
        > 128 * 1024
    {
        return Err(failure("limit", "Export provenance exceeds 128 KiB."));
    }
    let summary = preflight_format(frames, policy, format)?;
    cancelled(cancel)?;
    let (destination, single) = match destination {
        ExportDestination::Single(path) => (path, true),
        ExportDestination::Collection(path) => (path, false),
    };
    if single && frames.len() != 1 {
        return Err(failure(
            "selection",
            "Single-file export requires exactly one capture.",
        ));
    }
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    if destination.file_name().is_none() {
        return Err(failure(
            "destination",
            "Choose a new file or collection name.",
        ));
    }
    let staging = tempfile::Builder::new()
        .prefix(".merlin-pnm-")
        .tempdir_in(parent)
        .map_err(io)?;
    let extension = format.extension();
    let staging_name = format!("capture.{extension}");
    let mut entries = Vec::new();
    completed.store(0, Ordering::Relaxed);
    for (index, frame) in frames.iter().enumerate() {
        cancelled(cancel)?;
        let encoded = encode_format(frame, policy, format).map_err(|mut error| {
            error.capture = Some(index + 1);
            error
        })?;
        let name = if single {
            staging_name.clone()
        } else {
            format!("merlin_rxmer_designed_{:04}.{extension}", index + 1)
        };
        write_new(&staging.path().join(&name), &encoded.bytes)?;
        let output_name = if single {
            destination
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        } else {
            name
        };
        entries.push(json!({"file":output_name,"sha256":format!("{:x}",Sha256::digest(&encoded.bytes)),"key":frame.key,"capture_time":frame.capture_time,"source":frame.source,"conversion":encoded.summary}));
        completed.store(index + 1, Ordering::Relaxed);
    }
    let manifest = json!({"merlin_pnm_export":1,"created_by":"MERLIN","modem_measurement":false,"output_format":format,"pnm_format":"1.0","range_policy":if matches!(format, ExportFormat::Pnm) {json!(policy)} else {Value::Null},"summary":summary,"context":context,"captures":entries});
    let manifest = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| failure("manifest", error.to_string()))?;
    write_new(&staging.path().join("merlin-export.json"), &manifest)?;
    cancelled(cancel)?;
    if single {
        let mut sidecar_name = destination.as_os_str().to_os_string();
        sidecar_name.push(".merlin.json");
        let sidecar = PathBuf::from(sidecar_name);
        std::fs::hard_link(staging.path().join("merlin-export.json"), &sidecar).map_err(io)?;
        if let Err(error) = cancelled(cancel).and_then(|()| {
            std::fs::hard_link(staging.path().join(&staging_name), destination).map_err(io)
        }) {
            let _ = std::fs::remove_file(sidecar);
            return Err(error);
        }
    } else {
        publish_directory(staging.path(), destination)?;
    }
    Ok(summary)
}
