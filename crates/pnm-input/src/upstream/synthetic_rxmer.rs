/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Offline CCAP-OSSI I18 synthetic output, never a field-measurement claim.
use super::{bounded_json, rxmer_format::*, ApiError, ApiResult, RxmerCapture};
use crate::{
    common::rxmer::{quantize, MISSING_VALUE},
    rxmer_export::{ExportSummary, RangePolicy},
};
use docsis_ofdma::api::PhyError;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

// Application JSON budgets; binary output separately retains the decoder's cap.
/// Application request-transport budget in bytes, independent of wire file size.
pub const MAX_SYNTHETIC_RXMER_REQUEST_BYTES: usize = 256 * 1024;
const MAX_ARTIFACT_BYTES: usize = 1024 * 1024;
const ARTIFACT_SCHEMA_VERSION: u32 = 1;
// Request and result formats evolve independently, even when their versions match.
const REQUEST_SCHEMA_VERSION: u32 = 1;
const ARTIFACT_FORMAT: &str = "merlin.upstream.synthetic-rxmer";
const REQUEST_FORMAT: &str = "merlin.upstream.synthetic-rxmer-request";
const NORMATIVE_SOURCE: &str = "CM-SP-CCAP-OSSIv3.1-I18-200610 Table 410 / 7.3.2.5";

/// Requested synthetic values, before wire rounding and explicit range policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SyntheticRxmerValues {
    /// One flat level with unavailable positions specified as physical carriers.
    Flat {
        /// Requested finite RxMER in dB, not a measured or simulated result.
        mer_db: f64,
        /// Nonzero contiguous extent, admitted before allocating the sequence.
        count: usize,
        /// Unique original physical indices within that extent; no compaction.
        #[serde(
            default,
            deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, MAX_DATA_BYTES>"
        )]
        missing_subcarriers: Vec<u16>,
    },
    /// Explicit contiguous readings, with null entries retaining unavailable bins.
    Samples {
        /// Finite dB or unavailable, in original physical carrier order.
        #[serde(
            deserialize_with = "docsis_ofdma::configuration::serde_limits::bounded_vec::<_, _, MAX_DATA_BYTES>"
        )]
        readings_db: Vec<Option<f64>>,
    },
}

/// Explicit file metadata and synthetic values; no clock, channel or device lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SyntheticRxmerRequest {
    /// Exactly merlin.upstream.synthetic-rxmer-request; not captured input JSON.
    pub format: String,
    /// Exactly one; unsupported request versions reject explicitly.
    pub schema_version: u32,
    /// Caller-selected unsigned Unix epoch, preserved even when zero.
    pub capture_time_epoch_seconds: u32,
    /// Explicit upstream interface identity, never derived from channel ID.
    pub if_index: u32,
    /// Nonempty identifier, at most 255 UTF-8 octets without embedded NUL.
    pub ccap_id: String,
    /// Explicit RPD identity; all zeros represent an integrated CCAP.
    pub rpd_id: [u8; RPD_ID_BYTES],
    /// Explicit RPD upstream port; zero for an integrated CCAP.
    pub rpd_port: u8,
    /// Explicit measured-CM identity; examples use a locally administered address.
    pub cm_mac: [u8; CM_MAC_BYTES],
    /// Positive Table 409 unsigned-byte metadata, written into two file octets.
    pub number_of_averages: u8,
    /// Explicit object-model Boolean profile; does not apply transmit correction.
    pub pre_eq_enabled: bool,
    /// Physical subcarrier-zero center frequency, in hertz.
    pub subcarrier_zero_hz: u32,
    /// First physical carrier of the contiguous extent.
    pub first_active_subcarrier: u16,
    /// Exactly 25 or 50 kHz; unsupported modes reject.
    pub subcarrier_spacing_khz: u8,
    /// Flat or explicit values; generation never runs a noise model.
    pub values: SyntheticRxmerValues,
    /// Reject finite out-of-range inputs by default, or explicitly clamp them.
    #[serde(default)]
    pub range_policy: RangePolicy,
}

/// Immutable generated binary plus request, conversion and synthetic provenance.
#[derive(Debug, Clone)]
pub struct SyntheticRxmerCapture {
    request: SyntheticRxmerRequest,
    capture: RxmerCapture,
    conversion: ExportSummary,
}

fn invalid(field: &'static str) -> ApiError {
    PhyError::InvalidInput { field }.into()
}

fn limit(field: &'static str) -> ApiError {
    PhyError::ResourceLimit { field }.into()
}

impl SyntheticRxmerCapture {
    /// Generates one immutable synthetic file using the existing decoder profile.
    ///
    /// # Arguments
    /// * `request` - Explicit metadata, physical-grid values and conversion policy.
    ///
    /// # Returns
    /// Owned binary and provenance. The input is unchanged; no I/O, clock lookup,
    /// device probing, averaging, association or simulation occurs.
    ///
    /// # Errors
    /// Rejects nonfinite/invalid values, metadata, grid, missing positions and
    /// exceeded input limits before cloning or building the output.
    pub fn generate(request: &SyntheticRxmerRequest) -> ApiResult<Self> {
        if request.format != REQUEST_FORMAT || request.schema_version != REQUEST_SCHEMA_VERSION {
            return Err(PhyError::Unsupported {
                field: "synthetic_rxmer.request_version",
            }
            .into());
        }
        if request.ccap_id.is_empty() || request.ccap_id.as_bytes().contains(&0) {
            return Err(invalid("synthetic_rxmer.ccap_id"));
        }
        if request.ccap_id.len() >= CCAP_ID_BYTES {
            return Err(limit("synthetic_rxmer.ccap_id"));
        }
        if request.number_of_averages == 0 {
            return Err(invalid("synthetic_rxmer.number_of_averages"));
        }
        if request.rpd_id == [0; RPD_ID_BYTES] && request.rpd_port != 0 {
            return Err(invalid("synthetic_rxmer.rpd_port"));
        }
        let fft = fft_len(request.subcarrier_spacing_khz).ok_or_else(|| {
            ApiError::from(PhyError::Unsupported {
                field: "synthetic_rxmer.subcarrier_spacing",
            })
        })?;
        let (count, supplied_missing) = match &request.values {
            SyntheticRxmerValues::Flat {
                count,
                missing_subcarriers,
                ..
            } => (*count, missing_subcarriers.as_slice()),
            SyntheticRxmerValues::Samples { readings_db } => (readings_db.len(), &[][..]),
        };
        if count > MAX_DATA_BYTES || supplied_missing.len() > MAX_DATA_BYTES {
            return Err(limit("synthetic_rxmer.readings"));
        }
        let first = usize::from(request.first_active_subcarrier);
        if count == 0 || first >= fft || count > fft - first {
            return Err(invalid("synthetic_rxmer.grid"));
        }
        let mut missing = BTreeSet::new();
        for &carrier in supplied_missing {
            let physical = usize::from(carrier);
            if physical < first || physical >= first + count || !missing.insert(physical) {
                return Err(invalid("synthetic_rxmer.missing_subcarriers"));
            }
        }
        if let SyntheticRxmerValues::Flat { mer_db, .. } = &request.values {
            quantize(*mer_db, request.range_policy)
                .map_err(|_| invalid("synthetic_rxmer.readings_db"))?;
        }
        let mut bytes = Vec::with_capacity(HEADER_BYTES + count);
        bytes.extend_from_slice(&FILE_TYPE);
        bytes.extend_from_slice(&[MAJOR_VERSION, MINOR_VERSION]);
        bytes.extend_from_slice(&request.capture_time_epoch_seconds.to_be_bytes());
        bytes.extend_from_slice(&request.if_index.to_be_bytes());
        bytes.extend_from_slice(request.ccap_id.as_bytes());
        bytes.resize(CCAP_ID_END, 0);
        bytes.extend_from_slice(&request.rpd_id);
        bytes.push(request.rpd_port);
        bytes.extend_from_slice(&request.cm_mac);
        bytes.extend_from_slice(&u16::from(request.number_of_averages).to_be_bytes());
        bytes.push(u8::from(request.pre_eq_enabled));
        bytes.extend_from_slice(&request.subcarrier_zero_hz.to_be_bytes());
        bytes.extend_from_slice(&request.first_active_subcarrier.to_be_bytes());
        bytes.push(request.subcarrier_spacing_khz);
        bytes.extend_from_slice(&(count as u32).to_be_bytes());
        let mut conversion = ExportSummary {
            captures: 1,
            samples: count,
            bytes: HEADER_BYTES + count,
            ..Default::default()
        };
        for offset in 0..count {
            let value = match &request.values {
                SyntheticRxmerValues::Flat { mer_db, .. } => {
                    if missing.contains(&(first + offset)) {
                        None
                    } else {
                        Some(*mer_db)
                    }
                }
                SyntheticRxmerValues::Samples { readings_db } => readings_db[offset],
            };
            let Some(value) = value else {
                bytes.push(MISSING_VALUE);
                conversion.missing += 1;
                continue;
            };
            let encoded = quantize(value, request.range_policy)
                .map_err(|_| invalid("synthetic_rxmer.readings_db"))?;
            bytes.push(encoded.byte);
            conversion.clipped += usize::from(value != encoded.bounded_db);
            let rounding = (encoded.decoded_db - encoded.bounded_db).abs();
            conversion.rounded += usize::from(rounding > 0.0);
            conversion.maximum_rounding_db = conversion.maximum_rounding_db.max(rounding);
            conversion.maximum_change_db = conversion
                .maximum_change_db
                .max((encoded.decoded_db - value).abs());
        }
        Ok(Self {
            request: request.clone(),
            capture: RxmerCapture::from_pnm(&bytes)?,
            conversion,
        })
    }

    /// Parses a bounded request and invokes the same typed generator.
    ///
    /// # Arguments
    /// * `input` - UTF-8 request JSON of at most 256 KiB; unknown fields reject.
    ///
    /// # Returns
    /// An owned synthetic artifact, without filesystem or simulation operations.
    ///
    /// # Errors
    /// Returns shared structured errors for size, syntax, schema or domain failures.
    pub fn from_json(input: &str) -> ApiResult<Self> {
        if input.len() > MAX_SYNTHETIC_RXMER_REQUEST_BYTES {
            return Err(limit("synthetic_rxmer.request_bytes"));
        }
        Self::generate(&serde_json::from_str(input)?)
    }

    /// Borrows the generated file's validated observation without losing provenance.
    ///
    /// # Returns
    /// A capture borrowed for this artifact's lifetime. Keep the artifact/sidecar
    /// with exported bytes; the standard binary format carries no origin marker.
    pub fn capture(&self) -> &RxmerCapture {
        &self.capture
    }

    /// Serializes bounded synthetic provenance and the exact generated observations.
    ///
    /// # Returns
    /// Owned schema-versioned JSON with request, header, values, conversion, binary
    /// hash and explicit synthetic origin. This is not a captured-source document.
    ///
    /// # Errors
    /// Returns shared serialization/resource errors without modifying the artifact.
    pub fn to_json(&self) -> ApiResult<String> {
        bounded_json(
            &serde_json::json!({
                "format": ARTIFACT_FORMAT,
                "schema_version": ARTIFACT_SCHEMA_VERSION,
                "origin": "synthetic",
                "normative_source": NORMATIVE_SOURCE,
                "numeric_encoding": "explicit-most-significant-octet-first-profile",
                "measurement_performed": false,
                "request": self.request,
                "header": self.capture.header(),
                "raw_readings": self.capture.raw_readings(),
                "readings_db": self.capture.readings_db(),
                "binary_sha256": self.capture.original_sha256(),
                "binary_byte_count": self.capture.original_bytes().len(),
                "conversion": self.conversion,
            }),
            MAX_ARTIFACT_BYTES,
        )
    }
}
