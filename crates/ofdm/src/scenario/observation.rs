/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable full-resolution measurement observations and source metadata.
use super::grid::ScenarioKey;
use crate::{
    error::{invalid, SimulationError},
    types::{CaptureTime, IqPair},
};
use serde::{Deserialize, Serialize};
/// Immutable full-file identity; original bytes are owned once by the application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceProvenance {
    pub name: String,
    pub sha256: String,
    pub byte_length: usize,
    pub capture_time: Option<CaptureTime>,
    pub file_kind: String,
    pub format_major: u8,
    pub format_minor: u8,
}
impl SourceProvenance {
    /// Validates bounded identity metadata without claiming source authentication.
    /// # Returns
    /// Unit for a bounded name, hexadecimal SHA-256 and file kind.
    /// # Errors
    /// Rejects malformed hashes, excessive names and impossible source sizes.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if self.name.is_empty()
            || self.name.len() > 4096
            || self.sha256.len() != 64
            || !self.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            || self.byte_length > 64 * 1024 * 1024
            || self.file_kind.is_empty()
            || self.file_kind.len() > 128
        {
            return Err(invalid("Invalid source provenance metadata."));
        }
        Ok(())
    }
}
/// One measured RxMER frame, before any averaging or chart decimation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RxMerFrame {
    pub key: ScenarioKey,
    pub source: SourceProvenance,
    pub capture_time: CaptureTime,
    pub mer_db: Vec<Option<f64>>,
}
impl RxMerFrame {
    /// Checks exact sample count and finite present values.
    /// # Returns
    /// Unit for a valid measured frame; null readings remain null.
    /// # Errors
    /// Rejects inconsistent length, time identity or nonfinite present values.
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.key.grid.validate()?;
        self.source.validate()?;
        if self.mer_db.len() != self.key.grid.sample_count
            || self.source.capture_time != Some(self.capture_time)
            || self.mer_db.iter().flatten().any(|value| !value.is_finite())
        {
            return Err(invalid(
                "RxMER samples or source timestamp are inconsistent.",
            ));
        }
        Ok(())
    }
}
/// Optional channel response H observation; the reciprocal forms a known-reference equalizer.
/// None preserves an excluded/unavailable carrier, never a fabricated unity response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelFrame {
    pub key: ScenarioKey,
    pub source: SourceProvenance,
    pub capture_time: CaptureTime,
    pub iq: Vec<Option<IqPair>>,
}
