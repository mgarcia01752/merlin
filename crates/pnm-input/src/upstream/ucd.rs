/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Offline UCD inspection: MULPI I25 sections 6.2.1.2, 6.4.1 and 6.4.3.
//!
//! Bytes start at DA and stop before CRC. Structural admission does not prove
//! channel conformance, associate a CM or supply an executable configuration.
use super::{
    bounded_json,
    constants::{MAX_CAPTURE_SOURCE_BYTES, UCD_LENGTH_ORIGIN},
    ApiError, ApiResult, CaptureFailure,
};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;

// Figure 30: DA + SA + length precede the message-length-covered region.
const LENGTH_ORIGIN: usize = UCD_LENGTH_ORIGIN;
const HEADER_BYTES: usize = 20;
const UCD_FIXED_END: usize = HEADER_BYTES + 4;
const MAX_CAPTURE_BYTES: usize = MAX_CAPTURE_SOURCE_BYTES;
// Application allocation policy, counting burst wrappers and nested attributes.
const MAX_TLVS: usize = 1024;
const OFDMA_BURST_TYPE: u8 = 23;

/// Raw management/fixed UCD fields; borrowed from an immutable capture snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UcdHeader {
    /// Management destination, not an inferred measured-CM identity.
    pub destination: [u8; 6],
    /// Management sender MAC-domain address.
    pub source: [u8; 6],
    /// Original big-endian length from DSAP through management payload.
    pub message_length: u16,
    /// Admitted management version, currently exactly 5.
    pub version: u8,
    /// Admitted message type, currently exactly 51.
    pub message_type: u8,
    /// Original multipart byte, currently exactly zero.
    pub multipart: u8,
    /// Raw UCID; zero remains reserved and is not made executable here.
    pub upstream_channel_id: u8,
    /// Original wrapping descriptor-change count.
    pub configuration_change_count: u8,
    /// Raw field; it has no timing effect for OFDMA.
    pub minislot_size_ticks: u8,
    /// Raw downstream channel ID, not an association with a captured CM.
    pub downstream_channel_id: u8,
}

/// One ordered raw TLV; no unknown or duplicate value is silently discarded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UcdTlv {
    /// Absolute zero-based offset of the type octet in the admitted envelope.
    pub offset: usize,
    /// Original one-octet type, interpreted only within its parent scope.
    pub kind: u8,
    /// Exact value octets, excluding type/length; an empty unknown is retained.
    pub value: Vec<u8>,
}

/// Type23 wrapper and its ordered nested attributes; semantic validation is later.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct UcdBurst {
    /// Absolute wrapper-type offset, preserving the source location.
    pub offset: usize,
    /// Low-nibble source IUC; this is not a QAM order or vendor preset number.
    pub iuc: u8,
    /// Ordered raw burst attributes, retaining unknowns and duplicates.
    pub attributes: Vec<UcdTlv>,
}

/// Immutable bounded structural inspection; cloning shares the exact original.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UcdCapture {
    original: Arc<[u8]>,
    header: UcdHeader,
    channel_parameters: Vec<UcdTlv>,
    bursts: Vec<UcdBurst>,
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

// The cursor only advances after both header/value and cumulative work validate.
fn next_tlv(
    bytes: &[u8],
    cursor: &mut usize,
    end: usize,
    count: &mut usize,
) -> ApiResult<(usize, u8, std::ops::Range<usize>)> {
    let offset = *cursor;
    if end - offset < 2 {
        return Err(invalid("ucd.tlv_header", offset + 1));
    }
    if *count == MAX_TLVS {
        return Err(failure(
            CaptureFailure::ResourceLimit,
            "ucd.tlv_count",
            offset,
        ));
    }
    let value_start = offset + 2;
    let value_end = value_start + usize::from(bytes[offset + 1]);
    if value_end > end {
        return Err(invalid("ucd.tlv_length", offset + 1));
    }
    *count += 1;
    *cursor = value_end;
    Ok((offset, bytes[offset], value_start..value_end))
}

impl UcdCapture {
    /// Inspects one unfragmented Type51/version5 message from DA through payload.
    ///
    /// `bytes` excludes common MAC framing/CRC/transport containers. Returns an
    /// owned immutable original and raw inventory. Length/LLC/TLV/IUC/order errors
    /// retain exact offsets; unsupported multipart/version/type remain distinct.
    /// At most 65549 input bytes and 1024 total TLVs are copied; nothing executes.
    ///
    /// # Arguments
    /// * `bytes` - Complete management envelope starting at DA and excluding CRC.
    ///
    /// # Returns
    /// An owned immutable capture containing exact originals and ordered raw fields.
    ///
    /// # Errors
    /// Rejects unsupported formats, malformed extents, invalid field values or exceeded resource limits with field/offset diagnostics.
    pub fn from_mac_management(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_CAPTURE_BYTES {
            return Err(failure(CaptureFailure::ResourceLimit, "ucd.bytes", 0));
        }
        if bytes.len() < UCD_FIXED_END {
            return Err(invalid("ucd.header", bytes.len()));
        }
        let message_length = u16::from_be_bytes([bytes[12], bytes[13]]);
        if usize::from(message_length) + LENGTH_ORIGIN != bytes.len() {
            return Err(invalid("ucd.message_length", 12));
        }
        for (offset, expected) in [(14, 0), (15, 0), (16, 3)] {
            if bytes[offset] != expected {
                return Err(invalid("ucd.llc", offset));
            }
        }
        if bytes[17] != 5 {
            return Err(failure(CaptureFailure::Unsupported, "ucd.version", 17));
        }
        if bytes[18] != 51 {
            return Err(failure(CaptureFailure::Unsupported, "ucd.message_type", 18));
        }
        let last_fragment = bytes[19] >> 4;
        let fragment_sequence = bytes[19] & 15;
        if fragment_sequence > last_fragment {
            return Err(invalid("ucd.fragment_sequence", 19));
        }
        if last_fragment != 0 {
            return Err(failure(CaptureFailure::Unsupported, "ucd.multipart", 19));
        }
        let header = UcdHeader {
            destination: bytes[..6].try_into().map_err(|_| invalid("ucd.da", 0))?,
            source: bytes[6..12].try_into().map_err(|_| invalid("ucd.sa", 6))?,
            message_length,
            version: bytes[17],
            message_type: bytes[18],
            multipart: bytes[19],
            upstream_channel_id: bytes[20],
            configuration_change_count: bytes[21],
            minislot_size_ticks: bytes[22],
            downstream_channel_id: bytes[23],
        };
        let mut channel_parameters = Vec::new();
        let mut bursts = Vec::new();
        let mut cursor = UCD_FIXED_END;
        let mut count = 0;
        while cursor < bytes.len() {
            let (offset, kind, value) = next_tlv(bytes, &mut cursor, bytes.len(), &mut count)?;
            if matches!(kind, 4 | 5) {
                return Err(invalid("ucd.burst_type", offset));
            }
            if kind == OFDMA_BURST_TYPE {
                if value.is_empty() {
                    return Err(invalid("ucd.burst_iuc", value.start));
                }
                let iuc = bytes[value.start];
                if iuc & 0xf0 != 0 {
                    return Err(invalid("ucd.burst_iuc", value.start));
                }
                let mut attributes = Vec::new();
                let mut attribute_cursor = value.start + 1;
                while attribute_cursor < value.end {
                    let (offset, kind, value) =
                        next_tlv(bytes, &mut attribute_cursor, value.end, &mut count)?;
                    attributes.push(UcdTlv {
                        offset,
                        kind,
                        value: bytes[value].to_vec(),
                    });
                }
                bursts.push(UcdBurst {
                    offset,
                    iuc,
                    attributes,
                });
            } else {
                if !bursts.is_empty() {
                    return Err(invalid("ucd.channel_order", offset));
                }
                channel_parameters.push(UcdTlv {
                    offset,
                    kind,
                    value: bytes[value].to_vec(),
                });
            }
        }
        Ok(Self {
            original: Arc::from(bytes),
            header,
            channel_parameters,
            bursts,
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
        let original = super::source_json::decode(bytes, super::source_json::CaptureKind::Ucd)?;
        Self::from_mac_management(&original)
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
        super::source_json::encode(self.original_bytes(), super::source_json::CaptureKind::Ucd)
    }

    /// Borrows exact original management-message bytes for this snapshot's life.
    ///
    /// Returns the admitted envelope including all unknown/duplicate values;
    /// serialization and later caller-input mutation never rewrite it.
    ///
    /// # Returns
    /// The exact admitted byte slice, borrowed from this capture.
    pub fn original_bytes(&self) -> &[u8] {
        &self.original
    }

    /// Hashes exact original bytes with SHA-256, distinct from inspection JSON.
    ///
    /// Returns an owned lowercase hexadecimal fingerprint without changing state.
    ///
    /// # Returns
    /// An owned lowercase SHA-256 fingerprint of the original bytes.
    pub fn original_sha256(&self) -> String {
        format!("{:x}", Sha256::digest(&self.original))
    }

    /// Borrows original management and fixed UCD fields without interpretation.
    ///
    /// Returns an immutable view tied to this snapshot, including reserved IDs.
    ///
    /// # Returns
    /// The original decoded header, borrowed from this capture.
    pub fn header(&self) -> &UcdHeader {
        &self.header
    }

    /// Borrows all ordered channel TLVs with absolute offsets and raw values.
    ///
    /// Unknown/duplicate values remain present and do not imply channel validity.
    ///
    /// # Returns
    /// The ordered raw channel TLVs, borrowed from this capture.
    pub fn channel_parameters(&self) -> &[UcdTlv] {
        &self.channel_parameters
    }

    /// Borrows ordered IUC wrappers and raw attribute inventories.
    ///
    /// Returns an immutable view; inspection does not select a modulation profile.
    ///
    /// # Returns
    /// The ordered burst descriptors, borrowed from this capture.
    pub fn bursts(&self) -> &[UcdBurst] {
        &self.bursts
    }

    /// Exports versioned structural-inspection JSON, bounded to one MiB.
    ///
    /// Returns owned header/TLV/offset/hash data. Original bytes remain separately
    /// available; no missing measurement time or executable settings are invented.
    ///
    /// # Returns
    /// An owned bounded UTF-8 inspection document.
    ///
    /// # Errors
    /// Returns a structured error if serialization fails or the output exceeds its byte budget.
    pub fn to_json(&self) -> ApiResult<String> {
        #[derive(Serialize)]
        struct Inspection<'a> {
            format: &'static str,
            schema_version: u32,
            semantics: &'static str,
            original_byte_count: usize,
            original_sha256: String,
            header: &'a UcdHeader,
            channel_parameters: &'a [UcdTlv],
            bursts: &'a [UcdBurst],
        }
        bounded_json(
            &Inspection {
                format: "merlin.upstream.ucd-inspection",
                schema_version: 1,
                semantics: "structural-inspection-only",
                original_byte_count: self.original.len(),
                original_sha256: self.original_sha256(),
                header: &self.header,
                channel_parameters: &self.channel_parameters,
                bursts: &self.bursts,
            },
            1024 * 1024,
        )
    }
}
