/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded RxMER 1.0 output. I27 D.2.8 p172 / Table68 p176 text evidence;
//! big-endian fields follow pinned Apache PNM 5fbd556 reader implementation.
//! Rounding/clamping are explicit export policies, never changes to source data.
mod files;
use crate::common::rxmer::{quantize, MISSING_VALUE};
use docsis_ofdm_fec::scenario::RxMerFrame;
pub use files::{save, save_with_format, ExportDestination};
use serde::{Deserialize, Serialize};
use std::fmt;
const HEADER_LENGTH: usize = 28;
const HEADER_PREFIX: [u8; 6] = [0x50, 0x4e, 0x4e, 0x04, 1, 0];
/// Maximum collection captures; application limit, not an encoded PNM limit.
pub const MAX_EXPORT_CAPTURES: usize = 256;
/// Maximum total original-grid samples admitted by one export.
pub const MAX_EXPORT_SAMPLES: usize = 4_000_000;
/// Output representation; JSON retains finite readings without quarter-dB conversion.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    Pnm,
    Json,
}
impl ExportFormat {
    /// Returns the conventional filename extension for this representation.
    /// # Returns
    /// A static extension without a leading dot.
    pub fn extension(self) -> &'static str {
        match self {
            Self::Pnm => "bin",
            Self::Json => "json",
        }
    }
}
/// Out-of-range conversion policy; nonfinite present readings always fail.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RangePolicy {
    #[default]
    Reject,
    Clamp,
}
/// Structured export failure, independent of display text.
#[derive(Debug, Clone, Serialize)]
pub struct ExportError {
    pub code: &'static str,
    pub capture: Option<usize>,
    pub subcarrier: Option<usize>,
    pub message: String,
}
impl fmt::Display for ExportError {
    /// Formats capture/carrier context for an export diagnostic.
    /// # Arguments
    /// * `formatter` - Caller-owned display destination.
    /// # Returns
    /// The destination formatting result; the structured fields remain available.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(capture) = self.capture {
            write!(formatter, "Capture {capture}: ")?;
        }
        if let Some(carrier) = self.subcarrier {
            write!(formatter, "carrier {carrier}: ")?;
        }
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for ExportError {}
pub(super) fn failure(code: &'static str, message: impl Into<String>) -> ExportError {
    ExportError {
        code,
        capture: None,
        subcarrier: None,
        message: message.into(),
    }
}
/// Conversion summary computed from full-resolution input, never the viewport.
#[derive(Debug, Default, Clone, Serialize)]
pub struct ExportSummary {
    pub captures: usize,
    pub samples: usize,
    pub missing: usize,
    pub rounded: usize,
    pub clipped: usize,
    pub maximum_rounding_db: f64,
    pub maximum_change_db: f64,
    pub bytes: usize,
}
impl ExportSummary {
    pub(super) fn add(&mut self, other: &Self) {
        self.captures += other.captures;
        self.samples += other.samples;
        self.missing += other.missing;
        self.rounded += other.rounded;
        self.clipped += other.clipped;
        self.maximum_rounding_db = self.maximum_rounding_db.max(other.maximum_rounding_db);
        self.maximum_change_db = self.maximum_change_db.max(other.maximum_change_db);
        self.bytes += other.bytes;
    }
}
/// Encoded owned bytes and their conversion summary; source inputs remain intact.
#[derive(Debug)]
pub struct EncodedRxmer {
    pub bytes: Vec<u8>,
    pub summary: ExportSummary,
}
/// Encodes a complete original-grid RxMER frame using explicit range conversion.
///
/// # Arguments
/// * `frame` - Existing validated Rust frame with identity, epoch and full samples.
/// * `policy` - Reject out-of-range values by default, or explicitly clamp them.
///
/// # Returns
/// Owned PNM 1.0 bytes and a summary. In-range values round to the nearest quarter
/// dB, positive half-ties up; None becomes 0xFF. No simulation or I/O occurs.
///
/// # Errors
/// Rejects invalid metadata/grid/count, nonfinite readings and values outside
/// 0..63.5 under Reject. Diagnostics identify physical subcarriers when possible.
pub fn encode(frame: &RxMerFrame, policy: RangePolicy) -> Result<EncodedRxmer, ExportError> {
    // Reject malformed/oversized caller-owned buffers before scanning readings.
    frame
        .key
        .grid
        .validate()
        .map_err(|error| failure("invalid_frame", error.to_string()))?;
    if frame.mer_db.len() != frame.key.grid.sample_count {
        return Err(failure(
            "invalid_frame",
            "RxMER sample count differs from the bounded grid.",
        ));
    }
    if let Some((index, _)) = frame
        .mer_db
        .iter()
        .enumerate()
        .find(|(_, value)| value.is_some_and(|value| !value.is_finite()))
    {
        let mut error = failure(
            "nonfinite",
            "Present RxMER values must be finite; use None for missing data.",
        );
        error.subcarrier = Some(usize::from(frame.key.grid.first_subcarrier_index) + index);
        return Err(error);
    }
    frame
        .validate()
        .map_err(|error| failure("invalid_frame", error.to_string()))?;
    let grid = &frame.key.grid;
    if !matches!(grid.spacing_hz, 25_000 | 50_000) {
        return Err(failure(
            "unsupported_spacing",
            "PNM export requires 25 or 50 kHz spacing.",
        ));
    }
    let length = u32::try_from(frame.mer_db.len())
        .map_err(|_| failure("limit", "PNM sample count exceeds u32."))?;
    let mut bytes = Vec::with_capacity(HEADER_LENGTH + frame.mer_db.len());
    bytes.extend_from_slice(&HEADER_PREFIX);
    bytes.extend_from_slice(&frame.capture_time.to_be_bytes());
    bytes.push(frame.key.channel_id);
    bytes.extend_from_slice(&frame.key.mac_address);
    bytes.extend_from_slice(&grid.zero_frequency_hz.to_be_bytes());
    bytes.extend_from_slice(&grid.first_subcarrier_index.to_be_bytes());
    bytes.push((grid.spacing_hz / 1000) as u8);
    bytes.extend_from_slice(&length.to_be_bytes());
    let mut summary = ExportSummary {
        captures: 1,
        samples: frame.mer_db.len(),
        bytes: HEADER_LENGTH + frame.mer_db.len(),
        ..Default::default()
    };
    for (index, sample) in frame.mer_db.iter().enumerate() {
        let Some(value) = sample else {
            bytes.push(MISSING_VALUE);
            summary.missing += 1;
            continue;
        };
        let quantized = quantize(*value, policy).map_err(|_| {
            let mut error = failure("out_of_range", format!("{value:.6} dB is outside PNM range 0–63.5 dB. Adjust the recipe or explicitly enable Clamp to PNM range."));
            error.subcarrier = Some(usize::from(grid.first_subcarrier_index) + index);
            error
        })?;
        let bounded = quantized.bounded_db;
        if *value != bounded {
            summary.clipped += 1;
        }
        let encoded = quantized.byte;
        let decoded = quantized.decoded_db;
        let rounding = (decoded - bounded).abs();
        if rounding > 0.0 {
            summary.rounded += 1;
        }
        summary.maximum_rounding_db = summary.maximum_rounding_db.max(rounding);
        summary.maximum_change_db = summary.maximum_change_db.max((decoded - value).abs());
        bytes.push(encoded);
    }
    Ok(EncodedRxmer { bytes, summary })
}
/// Serializes a full-grid capture into the existing decoded PNM JSON contract.
/// # Arguments
/// * `frame` - Immutable RxMER frame with complete grid, identity and provenance.
/// # Returns
/// Owned UTF-8 JSON, unchanged finite sample values and nulls, and a lossless summary.
/// Extra MERLIN provenance explicitly identifies generated data, not a modem capture.
/// # Errors
/// Rejects invalid frames and nonfinite present values rather than serializing nulls.
pub fn encode_json(frame: &RxMerFrame) -> Result<EncodedRxmer, ExportError> {
    frame
        .validate()
        .map_err(|error| failure("invalid_frame", error.to_string()))?;
    let grid = &frame.key.grid;
    let mac = frame
        .key
        .mac_address
        .iter()
        .map(|octet| format!("{octet:02x}"))
        .collect::<Vec<_>>()
        .join(":");
    let document = serde_json::json!({
        "pnm_header":{"file_type":"PNN","file_type_version":4,"major_version":1,"minor_version":0,"capture_time":frame.capture_time},
        "mac_address":mac,"channel_id":frame.key.channel_id,
        "subcarrier_zero_frequency":grid.zero_frequency_hz,
        "first_active_subcarrier_index":grid.first_subcarrier_index,
        "subcarrier_spacing":grid.spacing_hz,"data_length":frame.mer_db.len(),
        "value_units":"dB","values":frame.mer_db,
        "merlin_export":{"created_by":"MERLIN","modem_measurement":false,"source":frame.source}
    });
    let bytes =
        serde_json::to_vec_pretty(&document).map_err(|error| failure("json", error.to_string()))?;
    let summary = ExportSummary {
        captures: 1,
        samples: frame.mer_db.len(),
        missing: frame.mer_db.iter().filter(|value| value.is_none()).count(),
        bytes: bytes.len(),
        ..Default::default()
    };
    Ok(EncodedRxmer { bytes, summary })
}
pub(super) fn encode_format(
    frame: &RxMerFrame,
    policy: RangePolicy,
    format: ExportFormat,
) -> Result<EncodedRxmer, ExportError> {
    match format {
        ExportFormat::Pnm => encode(frame, policy),
        ExportFormat::Json => encode_json(frame),
    }
}
/// Validates a bounded collection for the selected output representation.
/// # Arguments
/// * `frames` - Complete immutable capture snapshots.
/// * `policy` - Explicit binary range policy; JSON never clamps or rounds.
/// * `format` - PNM binary or decoded JSON.
/// # Returns
/// Aggregate summary without writing or changing captures.
/// # Errors
/// Rejects excessive work or invalid capture metadata/samples.
pub fn preflight_format(
    frames: &[RxMerFrame],
    policy: RangePolicy,
    format: ExportFormat,
) -> Result<ExportSummary, ExportError> {
    bounded(frames)?;
    let mut summary = ExportSummary::default();
    for (index, frame) in frames.iter().enumerate() {
        let encoded = encode_format(frame, policy, format).map_err(|mut error| {
            error.capture = Some(index + 1);
            error
        })?;
        summary.add(&encoded.summary);
    }
    Ok(summary)
}
/// Checks a bounded collection using the same encoder invoked at the write boundary.
///
/// # Arguments
/// * `frames` - Immutable complete-grid captures; source identities may differ.
/// * `policy` - Explicit conversion policy for every frame.
///
/// # Returns
/// Aggregate conversion summary. No files, accepted settings or samples change.
///
/// # Errors
/// Rejects excessive collection work or the first invalid capture/carrier.
pub fn preflight(frames: &[RxMerFrame], policy: RangePolicy) -> Result<ExportSummary, ExportError> {
    preflight_format(frames, policy, ExportFormat::Pnm)
}
fn bounded(frames: &[RxMerFrame]) -> Result<(), ExportError> {
    if frames.is_empty()
        || frames.len() > MAX_EXPORT_CAPTURES
        || frames
            .iter()
            .try_fold(0usize, |count, frame| count.checked_add(frame.mer_db.len()))
            .is_none_or(|count| count > MAX_EXPORT_SAMPLES)
    {
        return Err(failure(
            "limit",
            "Export requires 1–256 captures and at most 4,000,000 samples.",
        ));
    }
    Ok(())
}
