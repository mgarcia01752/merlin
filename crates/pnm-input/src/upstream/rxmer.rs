/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! CCAP-OSSI I18 Table 410 / section 7.3.2.5 upstream RxMER observation decoder.
//! Original bytes and interpretation limits remain distinct from model readiness.
use super::{bounded_json, ApiError, ApiResult, CaptureFailure};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

const FILE_TYPE: [u8; 4] = [0x50, 0x4e, 0x4e, 0x69];
// Table 410: fixed major2 header, not a legacy unversioned PNM header.
const HEADER_BYTES: usize = 297;
const CCAP_ID_START: usize = 14;
const CCAP_ID_END: usize = 270;
const SPACING_OFFSET: usize = 292;
const DATA_LENGTH_OFFSET: usize = 293;
// Application geometry/budget, not a universal PNM file-size requirement.
const MAX_DATA_BYTES: usize = 4096;
const MAX_FILE_BYTES: usize = HEADER_BYTES + MAX_DATA_BYTES;

/// Original major2 file fields, borrowed immutably from a decoded observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RxmerHeader {
    /// Exact supported header major version, currently 2.
    pub major_version: u8,
    /// Vendor-assigned minor byte, preserved without guessing extensions.
    pub minor_version: u8,
    /// Original unsigned Unix epoch seconds; zero is never replaced with now.
    pub capture_time_epoch_seconds: u32,
    /// Original upstream interface index, not a guessed channel ID.
    pub if_index: u32,
    /// All 256 CCAP-ID octets, including terminator and opaque padding.
    pub ccap_id_raw: Vec<u8>,
    /// Exact six-octet RPD identifier, including integrated-CCAP zeros.
    pub rpd_id: [u8; 6],
    /// Raw one-octet RPD port value.
    pub rpd_port: u8,
    /// Exact measured CM identity, distinct from a management-frame destination.
    pub cm_mac: [u8; 6],
    /// Raw two-octet number-of-averages field; no new averaging is performed.
    pub number_of_averages: u16,
    /// Original flag octet; optional rendering never replaces this value.
    pub pre_eq_on_off: u8,
    /// Original physical subcarrier-zero center frequency in hertz.
    pub subcarrier_zero_hz: u32,
    /// First physical subcarrier index in the retained contiguous data extent.
    pub first_active_subcarrier: u16,
    /// Raw spacing in kHz, admitted as 25 or 50 in this decoder profile.
    pub subcarrier_spacing_khz: u8,
    /// Original declared count of one-octet RxMER readings.
    pub data_length_bytes: u32,
}

/// Immutable lossless decoded upstream capture; no session/model association.
#[derive(Debug, Clone, PartialEq)]
pub struct RxmerCapture {
    original: Arc<[u8]>,
    header: RxmerHeader,
    readings_db: Vec<Option<f64>>,
}

fn error(category: CaptureFailure, field: &'static str, offset: usize) -> ApiError {
    ApiError::Capture {
        category,
        field,
        offset,
    }
}
fn invalid(field: &'static str, offset: usize) -> ApiError {
    error(CaptureFailure::InvalidInput, field, offset)
}
// Callers first admit the complete fixed header, so every numeric read is bounded.
fn unsigned_short(bytes: &[u8], offset: usize) -> u16 {
    u16::from_be_bytes([bytes[offset], bytes[offset + 1]])
}
fn unsigned_int(bytes: &[u8], offset: usize) -> u32 {
    u32::from_be_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

impl RxmerCapture {
    /// Decodes one exact major2 upstream 0x504e4e69 file without executing science.
    ///
    /// `bytes` starts at file type and ends at the declared data EOF. Returns an
    /// immutable original/header/dB view. Type/major/spacing outside this profile
    /// return unsupported; invalid lengths/ID/subgrids retain byte offsets.
    /// At most 4393 source bytes and 4096 readings are allocated. Empty data and
    /// uninterpreted metadata stay inspectable, never simulation-ready by default.
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
        if bytes.len() > MAX_FILE_BYTES {
            return Err(error(CaptureFailure::ResourceLimit, "rxmer.bytes", 0));
        }
        if bytes.len() < HEADER_BYTES {
            return Err(invalid("rxmer.header", bytes.len()));
        }
        if bytes[..4] != FILE_TYPE {
            return Err(error(CaptureFailure::Unsupported, "rxmer.file_type", 0));
        }
        if bytes[4] != 2 {
            return Err(error(CaptureFailure::Unsupported, "rxmer.major_version", 4));
        }
        let fft_len = match bytes[SPACING_OFFSET] {
            25 => 4096,
            50 => 2048,
            _ => {
                return Err(error(
                    CaptureFailure::Unsupported,
                    "rxmer.subcarrier_spacing",
                    SPACING_OFFSET,
                ))
            }
        };
        let declared = unsigned_int(bytes, DATA_LENGTH_OFFSET);
        let count = usize::try_from(declared).map_err(|_| {
            error(
                CaptureFailure::ResourceLimit,
                "rxmer.data_length",
                DATA_LENGTH_OFFSET,
            )
        })?;
        if count > MAX_DATA_BYTES {
            return Err(error(
                CaptureFailure::ResourceLimit,
                "rxmer.data_length",
                DATA_LENGTH_OFFSET,
            ));
        }
        // Count is already bounded, so this addition cannot overflow usize.
        if HEADER_BYTES + count != bytes.len() {
            return Err(invalid("rxmer.data_length", DATA_LENGTH_OFFSET));
        }
        let first = unsigned_short(bytes, 290);
        if usize::from(first) >= fft_len || usize::from(first) + count > fft_len {
            return Err(invalid("rxmer.subcarrier_grid", 290));
        }
        if !bytes[CCAP_ID_START..CCAP_ID_END].contains(&0) {
            return Err(invalid("rxmer.ccap_id_terminator", CCAP_ID_START));
        }
        let header = RxmerHeader {
            major_version: bytes[4],
            minor_version: bytes[5],
            capture_time_epoch_seconds: unsigned_int(bytes, 6),
            if_index: unsigned_int(bytes, 10),
            ccap_id_raw: bytes[CCAP_ID_START..CCAP_ID_END].to_vec(),
            rpd_id: bytes[270..276]
                .try_into()
                .map_err(|_| invalid("rxmer.rpd_id", 270))?,
            rpd_port: bytes[276],
            cm_mac: bytes[277..283]
                .try_into()
                .map_err(|_| invalid("rxmer.cm_mac", 277))?,
            number_of_averages: unsigned_short(bytes, 283),
            pre_eq_on_off: bytes[285],
            subcarrier_zero_hz: unsigned_int(bytes, 286),
            first_active_subcarrier: first,
            subcarrier_spacing_khz: bytes[SPACING_OFFSET],
            data_length_bytes: declared,
        };
        let readings_db = bytes[HEADER_BYTES..]
            .iter()
            .map(|value| match value {
                255 => None,
                value => Some(f64::from(*value) * 0.25),
            })
            .collect();
        Ok(Self {
            original: Arc::from(bytes),
            header,
            readings_db,
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
        let original = super::source_json::decode(bytes, super::source_json::CaptureKind::Rxmer)?;
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
            super::source_json::CaptureKind::Rxmer,
        )
    }

    /// Borrows the exact originally admitted file bytes for this capture's life.
    ///
    /// Unknown metadata, padding, null sentinels and original epoch are unchanged.
    ///
    /// # Returns
    /// The exact admitted byte slice, borrowed from this capture.
    pub fn original_bytes(&self) -> &[u8] {
        &self.original
    }

    /// Hashes exact original bytes with SHA-256, independently from JSON export.
    ///
    /// Returns owned lowercase hexadecimal; cloning/export never rewrites input.
    ///
    /// # Returns
    /// An owned lowercase SHA-256 fingerprint of the original bytes.
    pub fn original_sha256(&self) -> String {
        format!("{:x}", Sha256::digest(&self.original))
    }

    /// Borrows the exact decoded raw header with explicit units and identities.
    ///
    /// Returns an immutable view; raw vendor/flag/ID bytes remain authoritative.
    ///
    /// # Returns
    /// The original decoded header, borrowed from this capture.
    pub fn header(&self) -> &RxmerHeader {
        &self.header
    }

    /// Borrows all original data octets in their contiguous physical-grid order.
    ///
    /// Includes every 0xff missing reading; no frequency position is removed.
    ///
    /// # Returns
    /// All original reading bytes, including missing-value sentinels.
    pub fn raw_readings(&self) -> &[u8] {
        &self.original[HEADER_BYTES..]
    }

    /// Borrows quarter-dB-decoded readings at first index plus each array offset.
    ///
    /// Returns exact finite values 0..63.5 or None for 0xff, including interior
    /// gaps. Empty data stays empty; this does not create a noise-power model.
    ///
    /// # Returns
    /// The fixed-grid optional dB readings, borrowed from this capture.
    pub fn readings_db(&self) -> &[Option<f64>] {
        &self.readings_db
    }

    /// Renders valid UTF-8 before the first CCAP-ID NUL without modifying raw bytes.
    ///
    /// Returns None for non-UTF-8 text; padding is never trimmed from the original.
    /// Text is presentation only, not validated DNS/IP or association identity.
    ///
    /// # Returns
    /// Borrowed UTF-8 text before NUL, or None when unavailable.
    pub fn ccap_id_text(&self) -> Option<&str> {
        let end = self.header.ccap_id_raw.iter().position(|byte| *byte == 0)?;
        std::str::from_utf8(&self.header.ccap_id_raw[..end]).ok()
    }

    /// Optionally renders object-model 0/1 convention, retaining the raw file flag.
    ///
    /// Returns false/true for 0/1, None otherwise. This is not a claim that the
    /// file independently defines Boolean wire encoding or that a model is ready.
    ///
    /// # Returns
    /// Some(false/true) for raw 0/1, or None for other raw values.
    pub fn pre_eq_enabled(&self) -> Option<bool> {
        match self.header.pre_eq_on_off {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    /// Exports independently versioned decoded-observation JSON, bounded to 1 MiB.
    ///
    /// Returns header/raw/optional dB readings, provenance and explicit encoding
    /// limits; exact original bytes remain separately available. Never associates
    /// a CM/channel/time, selects a residual model or configures a simulation.
    ///
    /// # Returns
    /// An owned bounded UTF-8 inspection document.
    ///
    /// # Errors
    /// Returns a structured error if serialization fails or the output exceeds its byte budget.
    pub fn to_json(&self) -> ApiResult<String> {
        #[derive(Serialize)]
        struct Observation<'a> {
            format: &'static str,
            schema_version: u32,
            semantics: &'static str,
            reference_plane: &'static str,
            numeric_encoding: &'static str,
            pre_eq_rendering: &'static str,
            original_byte_count: usize,
            original_sha256: String,
            header: &'a RxmerHeader,
            ccap_id_text: Option<&'a str>,
            pre_eq_enabled: Option<bool>,
            raw_readings: &'a [u8],
            readings_db: &'a [Option<f64>],
        }
        bounded_json(
            &Observation {
                format: "merlin.upstream.rxmer-capture",
                schema_version: 1,
                semantics: "decoded-observation-only",
                reference_plane: "cmts-equalized-upstream-probe-relative-to-ideal-bpsk",
                numeric_encoding: "explicit-most-significant-octet-first-profile",
                pre_eq_rendering: "optional-object-model-0-1-convention-raw-flag-authoritative",
                original_byte_count: self.original.len(),
                original_sha256: self.original_sha256(),
                header: &self.header,
                ccap_id_text: self.ccap_id_text(),
                pre_eq_enabled: self.pre_eq_enabled(),
                raw_readings: self.raw_readings(),
                readings_db: self.readings_db(),
            },
            1024 * 1024,
        )
    }
}
