/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared, versioned scenario/candidate models. No inferred device control state.
use super::{invalid, LabError};
use crate::types::{
    modulation::CarrierType, BitsPerSymbol, CaptureTime, ChannelId, FrequencyHz, IqPair,
    MacAddress, ProfileId, SubcarrierIdx,
};
use serde::{Deserialize, Serialize};

/// Application safety limit, not an invented PHY or encoded-format limit.
pub const MAX_LAB_CARRIERS: usize = 65_536;
/// Version of the internal candidate file; never a vendor deployment format.
pub const CANDIDATE_SCHEMA_VERSION: u32 = 1;
/// Grid origin/index and count retained exactly from source headers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencyGrid {
    pub zero_frequency_hz: FrequencyHz,
    pub first_subcarrier_index: SubcarrierIdx,
    pub spacing_hz: FrequencyHz,
    pub sample_count: usize,
}
impl FrequencyGrid {
    /// Validates bounds and the final original carrier center without resampling.
    /// # Returns
    /// Unit on success.
    /// # Errors
    /// Rejects empty/oversized, zero-spacing and overflowing frequency grids.
    pub fn validate(&self) -> Result<(), LabError> {
        if self.sample_count == 0 || self.sample_count > MAX_LAB_CARRIERS || self.spacing_hz == 0 {
            return Err(invalid(
                "A bounded nonempty positive-spacing grid is required.",
            ));
        }
        self.frequency(self.sample_count - 1)?;
        Ok(())
    }
    /// Returns an original carrier center, never a decimated-bin center.
    /// # Arguments
    /// * `offset` - Zero-based offset within the captured grid.
    /// # Returns
    /// Frequency in the repository's u32 Hz domain.
    /// # Errors
    /// Rejects out-of-range positions and checked-arithmetic overflow.
    pub fn frequency(&self, offset: usize) -> Result<FrequencyHz, LabError> {
        if offset >= self.sample_count {
            return Err(invalid("Carrier offset is outside the source grid."));
        }
        let index = u32::from(self.first_subcarrier_index)
            .checked_add(u32::try_from(offset).map_err(|_| invalid("Carrier index overflow."))?)
            .ok_or_else(|| invalid("Carrier index overflow."))?;
        index
            .checked_mul(self.spacing_hz)
            .and_then(|offset_hz| self.zero_frequency_hz.checked_add(offset_hz))
            .ok_or_else(|| invalid("Frequency exceeds the supported Hz range."))
    }
}
/// Exact association identity. File format revisions remain source-specific.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioKey {
    pub mac_address: MacAddress,
    pub channel_id: ChannelId,
    pub grid: FrequencyGrid,
}
/// Supported interpretation is chosen explicitly; file versions cannot infer it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtocolScope {
    Docsis31,
    Docsis40Fdd,
}
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
    pub fn validate(&self) -> Result<(), LabError> {
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
    pub fn validate(&self) -> Result<(), LabError> {
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
/// Optional equalizer coefficient observation. None preserves an excluded carrier.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChannelFrame {
    pub key: ScenarioKey,
    pub source: SourceProvenance,
    pub capture_time: CaptureTime,
    pub iq: Vec<Option<IqPair>>,
}
/// One decoded captured profile, retaining source and exact frequency grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapturedProfile {
    pub key: ScenarioKey,
    pub source: SourceProvenance,
    pub capture_time: CaptureTime,
    pub profile_id: ProfileId,
    pub assignments: Vec<CarrierAssignment>,
}
/// Declared source of an assignment, distinct from whether its threshold passes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssignmentOrigin {
    Captured,
    Proposed,
}
/// One source-position assignment; unknown roles and missing bit loading persist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierAssignment {
    pub carrier_type: CarrierType,
    pub bit_loading: Option<BitsPerSymbol>,
    pub origin: AssignmentOrigin,
}
/// Editable hypothesis. Protected captured roles are immutable through range edits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateProfile {
    pub schema_version: u32,
    pub name: String,
    pub protocol: ProtocolScope,
    pub key: ScenarioKey,
    pub profile_id: Option<ProfileId>,
    pub assignments: Vec<CarrierAssignment>,
    pub sources: Vec<SourceProvenance>,
    pub roles_verified: bool,
}
impl CandidateProfile {
    /// Validates imported candidate structure before evaluation or mutation.
    /// # Returns
    /// Unit for a supported, bounded candidate with valid data modulations.
    /// # Errors
    /// Rejects schema drift, malformed assignments/provenance or resource limits.
    pub fn validate(&self) -> Result<(), LabError> {
        self.key.grid.validate()?;
        if self.schema_version != CANDIDATE_SCHEMA_VERSION
            || self.name.is_empty()
            || self.name.len() > 256
            || self.assignments.len() != self.key.grid.sample_count
            || self.sources.len() > 4096
        {
            return Err(invalid("Unsupported or malformed candidate profile."));
        }
        for source in &self.sources {
            source.validate()?;
        }
        for assignment in &self.assignments {
            if self.protocol == ProtocolScope::Docsis31
                && assignment.carrier_type == CarrierType::Data
                && assignment.bit_loading.is_some_and(|bits| bits > 13)
            {
                return Err(invalid("DOCSIS3.1 data assignments cannot exceed8192-QAM."));
            }
            let valid = match assignment.carrier_type {
                CarrierType::Data => assignment
                    .bit_loading
                    .is_none_or(crate::math::bit_loading::is_data_bit_loading),
                CarrierType::Plc => assignment.bit_loading == Some(4),
                CarrierType::ContinuousPilot => assignment.bit_loading == Some(1),
                CarrierType::ZeroBitLoaded => assignment.bit_loading == Some(0),
                CarrierType::Exclusion | CarrierType::Unknown | CarrierType::Reserved => {
                    assignment.bit_loading.is_none()
                }
            };
            if !valid {
                return Err(invalid(
                    "Candidate contains an unsupported role or modulation.",
                ));
            }
        }
        Ok(())
    }
    /// Edits a range of existing data carriers, preserving all special/unknown roles.
    /// # Arguments
    /// * `first`, `count` - Source-offset range, not physical subcarrier numbers.
    /// * `bits` - Verified registry data modulation; None leaves loading unresolved.
    /// # Returns
    /// Number of data carriers edited; captured special roles remain intact.
    /// # Errors
    /// Rejects out-of-bounds ranges or invalid modulation before changing anything.
    pub fn edit_range(
        &mut self,
        first: usize,
        count: usize,
        bits: Option<BitsPerSymbol>,
    ) -> Result<usize, LabError> {
        self.validate()?;
        let end = first
            .checked_add(count)
            .filter(|end| count > 0 && *end <= self.assignments.len())
            .ok_or_else(|| invalid("Edit range is outside the candidate grid."))?;
        if bits.is_some_and(|bits| !crate::math::bit_loading::is_data_bit_loading(bits)) {
            return Err(invalid("Unsupported candidate data modulation."));
        }
        let mut edited = 0;
        for assignment in &mut self.assignments[first..end] {
            if assignment.carrier_type == CarrierType::Data {
                assignment.bit_loading = bits;
                assignment.origin = AssignmentOrigin::Proposed;
                edited += 1;
            }
        }
        Ok(edited)
    }
}
