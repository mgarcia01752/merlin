/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! CM-OSSI I27 Tables 77/78 immutable applied-bank and last-update observations.
use super::{bounded_json, ApiError, ApiResult, CaptureFailure, Iq};
use pnm::pnm::{
    lib::fixed_point_decoder::{decode_iq, QFormat},
    parser::{cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, parse_limits::ParseLimits},
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

const HEADER_BYTES: usize = 34;
const MAX_BYTES: usize = HEADER_BYTES + 4096 * 4;

/// Source file kind determines scale, never the missing RNG-RSP operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PreEqCaptureKind {
    /// Applied coefficients: signed two's-complement s2.13.
    Applied,
    /// Most recent update: s1.14, requires explicit Set/Adjust and probe context.
    LastUpdate,
}
/// Exact source header, independent of simulation association and activation time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PreEqHeader {
    /// Source major version, exactly one in this adapter profile.
    pub major_version: u8,
    /// Opaque vendor revision, not a guessed extension indicator.
    pub minor_version: u8,
    /// Original Unix epoch seconds.
    pub capture_time_epoch_seconds: u32,
    /// Original upstream channel ID, not an interface index.
    pub upstream_channel_id: u8,
    /// Original measured CM identity.
    pub cm_mac: [u8; 6],
    /// Downstream MAC receiving MAPs/UCDs, not an invented CCAP ID.
    pub cmts_mac: [u8; 6],
    /// Physical bin-zero frequency in hertz.
    pub subcarrier_zero_hz: u32,
    /// First physical index in the retained contiguous extent.
    pub first_active_subcarrier: u16,
    /// Exactly 25 or 50 kHz in this profile.
    pub subcarrier_spacing_khz: u8,
    /// Exact declared data bytes, a multiple of four.
    pub data_length_bytes: u32,
}
/// Immutable lossless capture; coefficients never imply injected channel truth.
#[derive(Debug, Clone, PartialEq)]
pub struct PreEqCapture {
    original: Arc<[u8]>,
    kind: PreEqCaptureKind,
    header: PreEqHeader,
    coefficients: Vec<Iq>,
}
fn failure(category: CaptureFailure, field: &'static str, offset: usize) -> ApiError {
    ApiError::Capture {
        category,
        field,
        offset,
    }
}
fn invalid(field: &'static str, offset: usize) -> ApiError {
    failure(CaptureFailure::InvalidInput, field, offset)
}
fn u32_at(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}
fn decoder_error(error: pnm::diagnostic::ParseError) -> ApiError {
    use pnm::diagnostic::ErrorCode;
    let category = match error.code {
        ErrorCode::LimitExceeded | ErrorCode::ArithmeticOverflow => CaptureFailure::ResourceLimit,
        ErrorCode::UnsupportedType
        | ErrorCode::UnsupportedVersion
        | ErrorCode::UnsupportedLayout
        | ErrorCode::UnsupportedDecoder => CaptureFailure::Unsupported,
        ErrorCode::Truncated | ErrorCode::LengthMismatch => CaptureFailure::InvalidInput,
    };
    failure(category, error.field, error.offset)
}
impl PreEqCapture {
    /// Decodes one exact major1 bank/update in the explicit MS-octet-first profile.
    ///
    /// Returns immutable originals/header/IQ. Input is at most 16418 bytes and
    /// 4096 complex coefficients; wrong type/version/spacing returns Unsupported,
    /// malformed lengths/grids retain byte offsets. Empty files stay inspectable.
    ///
    /// # Arguments
    /// * `bytes` - Complete original PNM file, borrowed only for admission.
    ///
    /// # Returns
    /// An owned immutable capture containing exact originals and decoded observations.
    ///
    /// # Errors
    /// Rejects unsupported formats, malformed extents, invalid field values or exceeded resource limits with field/offset diagnostics.
    pub fn from_pnm(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_BYTES {
            return Err(failure(CaptureFailure::ResourceLimit, "pre_eq.bytes", 0));
        }
        if bytes.len() < HEADER_BYTES {
            return Err(invalid("pre_eq.header", bytes.len()));
        }
        let kind = match bytes[..4] {
            [0x50, 0x4e, 0x4e, 6] => PreEqCaptureKind::Applied,
            [0x50, 0x4e, 0x4e, 7] => PreEqCaptureKind::LastUpdate,
            _ => return Err(failure(CaptureFailure::Unsupported, "pre_eq.file_type", 0)),
        };
        if bytes[4] != 1 {
            return Err(failure(
                CaptureFailure::Unsupported,
                "pre_eq.major_version",
                4,
            ));
        }
        let n = match bytes[29] {
            25 => 4096,
            50 => 2048,
            _ => return Err(failure(CaptureFailure::Unsupported, "pre_eq.spacing", 29)),
        };
        let length = u32_at(bytes, 30);
        if length > 4096 * 4 || length % 4 != 0 || HEADER_BYTES + length as usize != bytes.len() {
            return Err(invalid("pre_eq.data_length", 30));
        }
        let first = u16::from_be_bytes([bytes[27], bytes[28]]);
        if usize::from(first) >= n || usize::from(first) + length as usize / 4 > n {
            return Err(invalid("pre_eq.subcarrier_grid", 27));
        }
        let coefficients = if bytes[5] == 0 {
            // The exact pin accepts version 1.0. Never falsify an opaque minor
            // to make its header parser accept another original file.
            let decoded = CmUsOfdmaPreEq::parse(bytes, ParseLimits { max_samples: 4096 })
                .map_err(decoder_error)?;
            decoded
                .iq()
                .map(|pair| {
                    pair.map(|[re, im]| Iq { re, im })
                        .ok_or(ApiError::AdapterFailure("pre_eq.unexpected_excluded_pair"))
                })
                .collect::<ApiResult<Vec<_>>>()?
        } else {
            let format = match kind {
                PreEqCaptureKind::Applied => QFormat::S2_13,
                PreEqCaptureKind::LastUpdate => QFormat::S1_14,
            };
            bytes[HEADER_BYTES..]
                .chunks_exact(4)
                .map(|pair| {
                    let [re, im] = decode_iq(
                        i16::from_be_bytes([pair[0], pair[1]]),
                        i16::from_be_bytes([pair[2], pair[3]]),
                        format,
                    );
                    Iq { re, im }
                })
                .collect()
        };
        Ok(Self {
            original: Arc::from(bytes),
            kind,
            coefficients,
            header: PreEqHeader {
                major_version: bytes[4],
                minor_version: bytes[5],
                capture_time_epoch_seconds: u32_at(bytes, 6),
                upstream_channel_id: bytes[10],
                cm_mac: bytes[11..17]
                    .try_into()
                    .map_err(|_| invalid("pre_eq.cm_mac", 11))?,
                cmts_mac: bytes[17..23]
                    .try_into()
                    .map_err(|_| invalid("pre_eq.cmts_mac", 17))?,
                subcarrier_zero_hz: u32_at(bytes, 23),
                first_active_subcarrier: first,
                subcarrier_spacing_khz: bytes[29],
                data_length_bytes: length,
            },
        })
    }
    /// Imports a lossless source envelope through the original binary decoder.
    ///
    /// Loading never associates or executes a capture. The returned snapshot owns
    /// the original binary bytes; callers retain outer delivery JSON separately.
    ///
    /// # Arguments
    /// * `bytes` - Complete UTF-8 capture-source JSON, at most 1 MiB.
    ///
    /// # Returns
    /// An immutable capture with the same original bytes and observations as
    /// direct binary admission, independent of the caller's input buffer.
    ///
    /// # Errors
    /// Rejects malformed/versioned/wrong-family envelopes, resource excess and
    /// count/hash mismatches at the envelope boundary. Binary errors retain their
    /// original category, field and offset. Inspection-only JSON is not accepted.
    pub fn from_source_json(bytes: &[u8]) -> ApiResult<Self> {
        let original = super::source_json::decode(bytes, super::source_json::CaptureKind::PreEq)?;
        Self::from_pnm(&original)
    }

    /// Exports the exact binary snapshot in a versioned lossless source envelope.
    ///
    /// The existing inspection-only `to_json` output remains separate and unchanged.
    /// No configuration, association or scientific result is fabricated.
    ///
    /// # Returns
    /// Owned JSON text containing the original bytes, byte count and SHA-256.
    ///
    /// # Errors
    /// Returns a structured capture error if transport bounds or serialization fail.
    pub fn to_source_json(&self) -> ApiResult<String> {
        super::source_json::encode(
            self.original_bytes(),
            super::source_json::CaptureKind::PreEq,
        )
    }

    /// Borrows exact source bytes for this immutable capture's lifetime.
    ///
    /// Includes opaque vendor metadata and original signed coefficient encodings.
    ///
    /// # Returns
    /// The exact admitted byte slice, borrowed from this capture.
    pub fn original_bytes(&self) -> &[u8] {
        &self.original
    }
    /// Hashes exact originals into owned lowercase SHA-256 text.
    ///
    /// Exporting or cloning never modifies the source bytes.
    ///
    /// # Returns
    /// An owned lowercase SHA-256 fingerprint of the original bytes.
    pub fn original_sha256(&self) -> String {
        format!("{:x}", Sha256::digest(&self.original))
    }
    /// Returns source kind and therefore its fixed-point scale.
    ///
    /// LastUpdate does not determine initialization versus adjustment.
    ///
    /// # Returns
    /// The admitted applied-bank or last-update file kind.
    pub fn kind(&self) -> PreEqCaptureKind {
        self.kind
    }
    /// Borrows source metadata without synthesizing missing control context.
    ///
    /// All original times and identities retain their separate meanings.
    ///
    /// # Returns
    /// The original decoded header, borrowed from this capture.
    pub fn header(&self) -> &PreEqHeader {
        &self.header
    }
    /// Borrows exact scaled I/Q pairs in physical subgrid order.
    ///
    /// No normalization, inverse channel inference or gap filling occurs here.
    ///
    /// # Returns
    /// The immutable scaled complex coefficients on the original subgrid.
    pub fn coefficients(&self) -> &[Iq] {
        &self.coefficients
    }
    /// Exports bounded schema1 observation JSON, independent of configurations.
    ///
    /// Returns owned text; original bytes remain separately available.
    ///
    /// # Returns
    /// An owned bounded UTF-8 inspection document.
    ///
    /// # Errors
    /// Returns a structured error if serialization fails or the output exceeds its byte budget.
    pub fn to_json(&self) -> ApiResult<String> {
        bounded_json(
            &serde_json::json!({
                "format":"merlin.upstream.pre-eq-capture", "schema_version":1,
                "semantics":"decoded-observation-only", "numeric_encoding":"explicit-most-significant-octet-first-profile",
                "kind":self.kind, "header":self.header, "coefficients":self.coefficients,
                "original_byte_count":self.original.len(), "original_sha256":self.original_sha256(),
            }),
            1024 * 1024,
        )
    }
}
