/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Lossless source transport; scientific admission stays with each binary decoder.
use super::{
    constants::{
        CAPTURE_SOURCE_JSON_FORMAT, CAPTURE_SOURCE_JSON_SCHEMA_VERSION, MAX_CAPTURE_SOURCE_BYTES,
        MAX_CAPTURE_SOURCE_JSON_BYTES,
    },
    ApiError, ApiResult, CaptureFailure,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Source-family identity chosen by the concrete typed capture owner.
pub(super) enum CaptureKind {
    Ucd,
    Rxmer,
    PreEq,
}

impl CaptureKind {
    fn name(&self) -> &'static str {
        match self {
            Self::Ucd => "ucd",
            Self::Rxmer => "rxmer",
            Self::PreEq => "pre_eq",
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceEnvelope {
    format: String,
    schema_version: u32,
    capture_kind: String,
    original_byte_count: usize,
    original_sha256: String,
    original_bytes: Vec<u8>,
}

#[derive(Serialize)]
struct SourceEnvelopeRef<'source> {
    format: &'static str,
    schema_version: u32,
    capture_kind: &'static str,
    original_byte_count: usize,
    original_sha256: String,
    original_bytes: &'source [u8],
}

fn failure(category: CaptureFailure, field: &'static str) -> ApiError {
    // Offset zero names the envelope boundary, never a guessed binary/JSON offset.
    ApiError::Capture {
        category,
        field,
        offset: 0,
    }
}

fn invalid(field: &'static str) -> ApiError {
    failure(CaptureFailure::InvalidInput, field)
}

/// Validates a bounded source envelope and returns its unchanged binary payload.
///
/// # Arguments
/// * `bytes` - Complete UTF-8 source JSON, at most the transport text policy.
/// * `kind` - Exact capture-family identity required by the typed caller.
///
/// # Returns
/// Owned original capture bytes; the caller performs authoritative binary admission.
///
/// # Errors
/// Rejects malformed/unknown/duplicate fields, identities, counts, hashes or bounds.
pub(super) fn decode(bytes: &[u8], kind: CaptureKind) -> ApiResult<Vec<u8>> {
    if bytes.len() > MAX_CAPTURE_SOURCE_JSON_BYTES {
        return Err(failure(
            CaptureFailure::ResourceLimit,
            "capture_source_json.bytes",
        ));
    }
    let source: SourceEnvelope =
        serde_json::from_slice(bytes).map_err(|_| invalid("capture_source_json.document"))?;
    if source.original_bytes.len() > MAX_CAPTURE_SOURCE_BYTES {
        return Err(failure(
            CaptureFailure::ResourceLimit,
            "capture_source_json.original_bytes",
        ));
    }
    if source.format != CAPTURE_SOURCE_JSON_FORMAT {
        return Err(invalid("capture_source_json.format"));
    }
    if source.schema_version != CAPTURE_SOURCE_JSON_SCHEMA_VERSION {
        return Err(invalid("capture_source_json.schema_version"));
    }
    if source.capture_kind != kind.name() {
        return Err(invalid("capture_source_json.capture_kind"));
    }
    if source.original_byte_count != source.original_bytes.len() {
        return Err(invalid("capture_source_json.original_byte_count"));
    }
    if source.original_sha256 != format!("{:x}", Sha256::digest(&source.original_bytes)) {
        return Err(invalid("capture_source_json.original_sha256"));
    }
    Ok(source.original_bytes)
}

/// Exports an already-admitted immutable binary capture as lossless source JSON.
///
/// # Arguments
/// * `bytes` - Original binary snapshot, bounded by the existing capture families.
/// * `kind` - Exact identity of the calling capture type.
///
/// # Returns
/// Owned versioned JSON text; binary bytes and their SHA-256 remain unchanged.
///
/// # Errors
/// Rejects source/text resource excess or serialization failure at the envelope.
pub(super) fn encode(bytes: &[u8], kind: CaptureKind) -> ApiResult<String> {
    if bytes.len() > MAX_CAPTURE_SOURCE_BYTES {
        return Err(failure(
            CaptureFailure::ResourceLimit,
            "capture_source_json.original_bytes",
        ));
    }
    let source = SourceEnvelopeRef {
        format: CAPTURE_SOURCE_JSON_FORMAT,
        schema_version: CAPTURE_SOURCE_JSON_SCHEMA_VERSION,
        capture_kind: kind.name(),
        original_byte_count: bytes.len(),
        original_sha256: format!("{:x}", Sha256::digest(bytes)),
        original_bytes: bytes,
    };
    let json =
        serde_json::to_string(&source).map_err(|_| invalid("capture_source_json.document"))?;
    if json.len() > MAX_CAPTURE_SOURCE_JSON_BYTES {
        return Err(failure(
            CaptureFailure::ResourceLimit,
            "capture_source_json.bytes",
        ));
    }
    Ok(json)
}
