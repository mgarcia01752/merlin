/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Private bounded update/restart transport, separate from scientific projects.
//!
//! Project documents remain exact bytes. Callers prepare/admit both directions,
//! bound execution scratch and reserve retained ownership, and commit the pair through
//! the application transaction. This codec performs no scientific parsing or I/O.

use crate::{shared::project_file::MAX_PROJECT_BYTES, shell::constants::MAX_ORIGINAL_SOURCE_PATHS};
use base64::{engine::general_purpose::STANDARD, Engine};
use merlin::common::ProjectDirection;
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    collections::HashSet,
    io::{self, Write},
    path::{Path, PathBuf},
};

const RECOVERY_APPLICATION: &str = "MERLIN";
const RECOVERY_FORMAT: &str = "merlin.qt.update-recovery";

/// Private restart transport version; unrelated to public project/editor schemas.
const RECOVERY_VERSION: u32 = 1;
const RECOVERY_DIRECTION_COUNT: usize = 2;
const BASE64_INPUT_GROUP_BYTES: usize = 3;
const BASE64_OUTPUT_GROUP_BYTES: usize = 4;
const BASE64_PADDING_BYTE: u8 = b'=';
const MAX_BASE64_PROJECT_BYTES: usize =
    MAX_PROJECT_BYTES.div_ceil(BASE64_INPUT_GROUP_BYTES) * BASE64_OUTPUT_GROUP_BYTES;

/// Independent metadata policy: 16 MiB, where one MiB is 1024² bytes.
/// Counts serialized destinations, original-path records and envelope overhead,
/// replacing each present document's base64 text with the empty string. This is
/// an application transport policy, not an OS path or scientific limit.
pub(crate) const MAX_RECOVERY_METADATA_BYTES: usize = 16 * 1024 * 1024;

/// Complete private recovery-file ceiling, including both base64 project bodies.
/// Worker file reads must enforce this before parsing. JSON parser scratch is
/// finite under this input ceiling; these transport limits are not an RSS claim.
pub(crate) const MAX_RECOVERY_BYTES: usize =
    RECOVERY_DIRECTION_COUNT * MAX_BASE64_PROJECT_BYTES + MAX_RECOVERY_METADATA_BYTES;

const TRANSPORT_LIMIT_MESSAGE: &str = "Update recovery exceeds its byte limit.";
const METADATA_LIMIT_MESSAGE: &str = "Update recovery metadata exceeds its byte limit.";
const PROJECT_LIMIT_MESSAGE: &str = "Update recovery project exceeds its byte limit.";
const BASE64_MESSAGE: &str = "Update recovery contains invalid base64 project bytes.";

/// One independently retained direction's exact project bytes and normal path.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ProjectState {
    /// Complete existing project transport; upstream `None` denotes blank state.
    pub(crate) document: Option<Vec<u8>>,
    /// Normal Save destination, never the private recovery-file destination.
    pub(crate) path: Option<PathBuf>,
    /// Actual native dirty revision; recovery never acknowledges normal Save.
    pub(crate) dirty: bool,
}

/// One actual original-source protection record, possibly owned by both PHYs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OriginalPath {
    /// Previously admitted native path; no canonicalization or I/O occurs here.
    pub(crate) path: PathBuf,
    /// Whether the downstream project owns this protection record.
    pub(crate) downstream: bool,
    /// Whether the upstream project owns this protection record.
    pub(crate) upstream: bool,
}

/// Fixed two-direction restart candidate, prepared before application adoption.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Bundle {
    /// Required downstream project, including valid legacy partial editor state.
    pub(crate) downstream: ProjectState,
    /// Independent upstream project, or explicit blank state with no destination.
    pub(crate) upstream: ProjectState,
    /// Native selected direction, independent of document availability.
    pub(crate) selected: ProjectDirection,
    /// Existing application theme choice.
    pub(crate) dark: bool,
    /// Exact original-input path inventory with directional ownership preserved.
    pub(crate) original_paths: Vec<OriginalPath>,
}

#[derive(Serialize)]
struct Wire<'bundle> {
    application: &'static str,
    format: &'static str,
    version: u32,
    downstream: WireProject<'bundle>,
    upstream: WireProject<'bundle>,
    selected: ProjectDirection,
    dark: bool,
    original_paths: &'bundle [OriginalPath],
}

#[derive(Serialize)]
struct WireProject<'bundle> {
    document: Option<&'bundle str>,
    path: Option<&'bundle Path>,
    dirty: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DecodedWire {
    application: String,
    format: String,
    version: u32,
    downstream: DecodedProject,
    upstream: DecodedProject,
    selected: ProjectDirection,
    dark: bool,
    original_paths: Vec<OriginalPath>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct DecodedProject {
    #[serde(deserialize_with = "required_option")]
    document: Option<String>,
    #[serde(deserialize_with = "required_option")]
    path: Option<PathBuf>,
    dirty: bool,
}

// The private seam gives boundary tests small limits without allocating full
// project-sized fixtures. Production always uses the existing application caps.
#[derive(Clone, Copy)]
struct Limits {
    project_bytes: usize,
    metadata_bytes: usize,
    original_paths: usize,
}

struct Output {
    bytes: Option<Vec<u8>>,
    length: usize,
    maximum: usize,
    limit_message: &'static str,
}

impl Bundle {
    /// Decodes one complete private restart candidate without admitting science.
    ///
    /// # Arguments
    /// * `bytes` - Complete private recovery JSON, within `MAX_RECOVERY_BYTES`.
    ///
    /// # Returns
    /// Exact decoded project bytes, independent paths/dirty flags, cursor/theme
    /// and original protections. No caller state or filesystem is changed.
    ///
    /// # Errors
    /// Rejects unsupported envelope tags/version, malformed/trailing/duplicate or
    /// unknown fields, invalid paths/base64, missing downstream, a blank upstream
    /// with a destination, duplicate/unowned/excess original records and all byte
    /// limits. Decoded project lengths are checked before either buffer allocation.
    /// JSON parser scratch is bounded by the complete input transport ceiling;
    /// callers govern that finite construction scratch through the execution limits.
    pub(crate) fn decode(bytes: &[u8]) -> Result<Self, String> {
        decode_with_limits(bytes, Limits::production())
    }

    /// Encodes exact projects and native metadata into private version-1 JSON.
    ///
    /// # Returns
    /// Owned bounded UTF-8 recovery bytes. Project bytes are base64 transport,
    /// never reserialized or interpreted as scientific documents.
    ///
    /// # Errors
    /// Rejects project/metadata/complete byte limits, missing downstream, blank
    /// upstream with a destination, invalid/non-UTF-8 paths, duplicate/unowned or
    /// excessive original records, arithmetic overflow and allocation failures.
    /// Metadata is counted before any base64 buffers are allocated. Caller-owned
    /// documents, paths and dirty flags remain unchanged on every outcome.
    pub(crate) fn encode(&self) -> Result<Vec<u8>, String> {
        encode_with_limits(self, Limits::production())
    }
}

impl Limits {
    fn production() -> Self {
        Self {
            project_bytes: MAX_PROJECT_BYTES,
            metadata_bytes: MAX_RECOVERY_METADATA_BYTES,
            original_paths: MAX_ORIGINAL_SOURCE_PATHS,
        }
    }

    fn transport_bytes(self) -> Result<usize, String> {
        padded_length(self.project_bytes)?
            .checked_mul(RECOVERY_DIRECTION_COUNT)
            .and_then(|documents| documents.checked_add(self.metadata_bytes))
            .ok_or_else(|| TRANSPORT_LIMIT_MESSAGE.into())
    }
}

impl Output {
    fn counter(maximum: usize) -> Self {
        Self {
            bytes: None,
            length: 0,
            maximum,
            limit_message: METADATA_LIMIT_MESSAGE,
        }
    }

    fn buffer(maximum: usize) -> Self {
        Self {
            bytes: Some(Vec::new()),
            length: 0,
            maximum,
            limit_message: TRANSPORT_LIMIT_MESSAGE,
        }
    }
}

fn encode_with_limits(bundle: &Bundle, limits: Limits) -> Result<Vec<u8>, String> {
    let maximum = limits.transport_bytes()?;
    validate_states(
        bundle.downstream.document.is_some(),
        bundle.upstream.document.is_none(),
        bundle.upstream.path.as_deref(),
    )?;
    validate_originals(&bundle.original_paths, limits.original_paths)?;
    for project in [&bundle.downstream, &bundle.upstream] {
        if project
            .document
            .as_ref()
            .is_some_and(|document| document.len() > limits.project_bytes)
        {
            return Err(PROJECT_LIMIT_MESSAGE.into());
        }
    }
    let mut wire = Wire {
        application: RECOVERY_APPLICATION,
        format: RECOVERY_FORMAT,
        version: RECOVERY_VERSION,
        downstream: WireProject {
            document: bundle.downstream.document.as_ref().map(|_| ""),
            path: bundle.downstream.path.as_deref(),
            dirty: bundle.downstream.dirty,
        },
        upstream: WireProject {
            document: bundle.upstream.document.as_ref().map(|_| ""),
            path: bundle.upstream.path.as_deref(),
            dirty: bundle.upstream.dirty,
        },
        selected: bundle.selected,
        dark: bundle.dark,
        original_paths: &bundle.original_paths,
    };
    count_metadata(&wire, limits.metadata_bytes)?;
    let downstream = bundle
        .downstream
        .document
        .as_deref()
        .map(encode_document)
        .transpose()?;
    let upstream = bundle
        .upstream
        .document
        .as_deref()
        .map(encode_document)
        .transpose()?;
    wire.downstream.document = downstream.as_deref();
    wire.upstream.document = upstream.as_deref();
    let mut output = Output::buffer(maximum);
    serde_json::to_writer(&mut output, &wire).map_err(|error| error.to_string())?;
    output.bytes.ok_or_else(|| TRANSPORT_LIMIT_MESSAGE.into())
}

fn decode_with_limits(bytes: &[u8], limits: Limits) -> Result<Bundle, String> {
    if bytes.len() > limits.transport_bytes()? {
        return Err(TRANSPORT_LIMIT_MESSAGE.into());
    }
    let wire: DecodedWire = serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
    if wire.application != RECOVERY_APPLICATION
        || wire.format != RECOVERY_FORMAT
        || wire.version != RECOVERY_VERSION
    {
        return Err("Unsupported update recovery application, format or version.".into());
    }
    validate_states(
        wire.downstream.document.is_some(),
        wire.upstream.document.is_none(),
        wire.upstream.path.as_deref(),
    )?;
    validate_originals(&wire.original_paths, limits.original_paths)?;
    let metadata = Wire {
        application: RECOVERY_APPLICATION,
        format: RECOVERY_FORMAT,
        version: RECOVERY_VERSION,
        downstream: WireProject {
            document: wire.downstream.document.as_ref().map(|_| ""),
            path: wire.downstream.path.as_deref(),
            dirty: wire.downstream.dirty,
        },
        upstream: WireProject {
            document: wire.upstream.document.as_ref().map(|_| ""),
            path: wire.upstream.path.as_deref(),
            dirty: wire.upstream.dirty,
        },
        selected: wire.selected,
        dark: wire.dark,
        original_paths: &wire.original_paths,
    };
    count_metadata(&metadata, limits.metadata_bytes)?;
    let downstream_length = wire
        .downstream
        .document
        .as_deref()
        .map(|document| decoded_length(document, limits.project_bytes))
        .transpose()?;
    let upstream_length = wire
        .upstream
        .document
        .as_deref()
        .map(|document| decoded_length(document, limits.project_bytes))
        .transpose()?;
    let downstream = decode_project(wire.downstream, downstream_length)?;
    let upstream = decode_project(wire.upstream, upstream_length)?;
    Ok(Bundle {
        downstream,
        upstream,
        selected: wire.selected,
        dark: wire.dark,
        original_paths: wire.original_paths,
    })
}

fn validate_states(
    downstream_present: bool,
    upstream_blank: bool,
    upstream_path: Option<&Path>,
) -> Result<(), String> {
    if !downstream_present {
        return Err("Update recovery requires a downstream project document.".into());
    }
    if upstream_blank && upstream_path.is_some() {
        return Err("A blank upstream recovery project cannot have a Save destination.".into());
    }
    Ok(())
}

fn validate_originals(paths: &[OriginalPath], maximum: usize) -> Result<(), String> {
    if paths.len() > maximum {
        return Err("Update recovery exceeds the original-source path count limit.".into());
    }
    let mut seen = HashSet::new();
    seen.try_reserve(paths.len())
        .map_err(|error| error.to_string())?;
    for original in paths {
        if !original.downstream && !original.upstream {
            return Err("Update recovery original path has no owning direction.".into());
        }
        if !seen.insert(original.path.as_path()) {
            return Err("Update recovery contains duplicate original paths.".into());
        }
    }
    Ok(())
}

fn count_metadata(wire: &Wire<'_>, maximum: usize) -> Result<usize, String> {
    let mut counter = Output::counter(maximum);
    serde_json::to_writer(&mut counter, wire).map_err(|error| error.to_string())?;
    Ok(counter.length)
}

fn padded_length(decoded: usize) -> Result<usize, String> {
    let groups = decoded / BASE64_INPUT_GROUP_BYTES;
    let final_group = usize::from(decoded % BASE64_INPUT_GROUP_BYTES != 0);
    groups
        .checked_add(final_group)
        .and_then(|groups| groups.checked_mul(BASE64_OUTPUT_GROUP_BYTES))
        .ok_or_else(|| PROJECT_LIMIT_MESSAGE.into())
}

fn encode_document(document: &[u8]) -> Result<String, String> {
    let length = padded_length(document.len())?;
    let mut encoded = Vec::new();
    encoded
        .try_reserve_exact(length)
        .map_err(|error| error.to_string())?;
    encoded.resize(length, 0);
    let written = STANDARD
        .encode_slice(document, &mut encoded)
        .map_err(|error| error.to_string())?;
    if written != length {
        return Err(BASE64_MESSAGE.into());
    }
    String::from_utf8(encoded).map_err(|error| error.to_string())
}

fn decoded_length(document: &str, maximum: usize) -> Result<usize, String> {
    let encoded = document.as_bytes();
    if encoded.len() > padded_length(maximum)? {
        return Err(PROJECT_LIMIT_MESSAGE.into());
    }
    if encoded.len() % BASE64_OUTPUT_GROUP_BYTES != 0 {
        return Err(BASE64_MESSAGE.into());
    }
    let padding = encoded
        .iter()
        .rev()
        .take(BASE64_INPUT_GROUP_BYTES - 1)
        .take_while(|byte| **byte == BASE64_PADDING_BYTE)
        .count();
    let length = (encoded.len() / BASE64_OUTPUT_GROUP_BYTES)
        .checked_mul(BASE64_INPUT_GROUP_BYTES)
        .and_then(|length| length.checked_sub(padding))
        .ok_or(BASE64_MESSAGE)?;
    if length > maximum {
        return Err(PROJECT_LIMIT_MESSAGE.into());
    }
    Ok(length)
}

fn decode_project(project: DecodedProject, length: Option<usize>) -> Result<ProjectState, String> {
    let document = match (project.document, length) {
        (Some(encoded), Some(length)) => {
            let mut decoded = Vec::new();
            decoded
                .try_reserve_exact(length)
                .map_err(|error| error.to_string())?;
            decoded.resize(length, 0);
            let written = STANDARD
                .decode_slice(encoded, &mut decoded)
                .map_err(|error| error.to_string())?;
            if written != length {
                return Err(BASE64_MESSAGE.into());
            }
            Some(decoded)
        }
        (None, None) => None,
        _ => return Err(BASE64_MESSAGE.into()),
    };
    Ok(ProjectState {
        document,
        path: project.path,
        dirty: project.dirty,
    })
}

fn required_option<'input, Decoder, Value>(
    decoder: Decoder,
) -> Result<Option<Value>, Decoder::Error>
where
    Decoder: Deserializer<'input>,
    Value: Deserialize<'input>,
{
    Option::deserialize(decoder)
}

impl Write for Output {
    /// Counts/appends a complete JSON fragment only after checked byte admission.
    ///
    /// # Arguments
    /// * `bytes` - Next serializer fragment; counted identically for both modes.
    ///
    /// # Returns
    /// Complete fragment length; counting retains no fragment or document copy.
    ///
    /// # Errors
    /// Rejects length overflow, the selected metadata/transport ceiling or output
    /// allocation failure before changing the current buffer/count.
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let length = self
            .length
            .checked_add(bytes.len())
            .filter(|length| *length <= self.maximum)
            .ok_or_else(|| io::Error::other(self.limit_message))?;
        if let Some(output) = &mut self.bytes {
            output
                .try_reserve_exact(bytes.len())
                .map_err(|error| io::Error::other(error.to_string()))?;
            output.extend_from_slice(bytes);
        }
        self.length = length;
        Ok(bytes.len())
    }

    /// Completes in-memory serialization without filesystem side effects.
    ///
    /// # Returns
    /// Unit; counting and retained bytes are already current.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[cfg(test)]
#[path = "update_recovery_tests.rs"]
mod tests;
