/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable capture ownership and explicit JSON/typed conversion boundaries.
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
mod profile_capture;
use super::*;
/// Original channel observation DTO, distinct from an inverse/equalizer response.
/// Its scientific source metadata remains decoded-frame provenance; original bytes
/// and decoder lineage are retained separately by ChannelEstimate.provenance.
pub use crate::scenario::ChannelFrame as ChannelEstimateFrame;
/// Original measured RxMER DTO used by existing frame/from_frame operations.
/// This nominal re-export adds no observation, algorithm or mutation to an input.
pub use crate::scenario::RxMerFrame;
use crate::{
    profile::candidate::{CandidateProfile, CapturedProfile},
    scenario::{
        pnm_json::{self, DecoderContract, JsonCapture},
        ChannelFrame, FrequencyGrid,
    },
};
pub use profile_capture::*;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::sync::Arc;
/// Per-input bound, shared with decoder JSON expansion.
pub const MAX_CAPTURE_BYTES: usize = 16 * 1024 * 1024;
/// Original bytes and display identity; extensions never determine content.
#[derive(Debug, Clone)]
pub struct NamedCapture {
    pub(crate) name: String,
    pub(crate) bytes: Arc<[u8]>,
}
impl NamedCapture {
    /// Owns bounded bytes for import, preserving them unchanged.
    /// # Arguments
    /// * `name` - Nonempty display identity, at most 4096 bytes.
    /// * `bytes` - Original input, at most 16 MiB.
    /// # Returns
    /// Immutable shared input; no simulation or filesystem activity.
    /// # Errors
    /// Rejects excessive sizes or invalid names before retaining input.
    pub fn new(name: impl Into<String>, bytes: Vec<u8>) -> ApiResult<Self> {
        let name = name.into();
        if name.is_empty() || name.len() > 4096 || bytes.len() > MAX_CAPTURE_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Capture name or size exceeds input bounds",
            ));
        }
        Ok(Self {
            name,
            bytes: bytes.into(),
        })
    }
    /// Borrows exact original bytes without copying.
    /// # Returns
    /// Read-only bytes tied to this input.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Borrows the caller's original display identity.
    /// # Returns
    /// Name, never a format hint.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }
}
/// Original representation, distinct from decoded intermediate JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CaptureKind {
    BinaryPnm,
    DecodedJson,
}
/// Immutable provenance with exact bytes retained separately from decoded hashes.
#[derive(Debug, Clone, Serialize)]
pub struct CaptureProvenance {
    original_name: String,
    original_sha256: String,
    original_kind: CaptureKind,
    original_epoch: CaptureTime,
    decoded_sha256: String,
    decoder_contract: Option<DecoderContract>,
    #[serde(skip)]
    pub(crate) original: Arc<[u8]>,
    #[serde(skip)]
    pub(crate) decoded: Arc<[u8]>,
}
impl CaptureProvenance {
    /// Borrows original identity and unchanged bytes.
    /// # Returns
    /// Shared bytes retained for this provenance lifetime.
    pub fn original_bytes(&self) -> &[u8] {
        &self.original
    }
    /// Borrows exact decoded intermediate bytes.
    /// # Returns
    /// JSON bytes distinct from original binary input.
    pub fn decoded_bytes(&self) -> &[u8] {
        &self.decoded
    }
    /// Reads the original epoch without rewriting it for association.
    /// # Returns
    /// Whole epoch seconds as a validated domain value.
    pub fn original_epoch(&self) -> CaptureTime {
        self.original_epoch
    }
    /// Borrows the exact original input digest.
    /// # Returns
    /// Lowercase SHA-256 hex digest.
    pub fn original_sha256(&self) -> &str {
        &self.original_sha256
    }
    /// Borrows the exact intermediate input digest.
    /// # Returns
    /// Lowercase SHA-256 hex digest.
    pub fn decoded_sha256(&self) -> &str {
        &self.decoded_sha256
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }
}
/// Profile source classification; created/edited profiles are hypotheses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProfileOrigin {
    Captured,
    Created,
    Edited,
}
/// Supported payload modulation. Actual engine admission still runs in preflight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DataModulation {
    Qam16,
    Qam64,
    Qam128,
    Qam256,
    Qam512,
    Qam1024,
    Qam2048,
    Qam4096,
    Qam8192,
    Qam16384,
}
impl DataModulation {
    pub(crate) fn bits(self) -> u8 {
        match self {
            Self::Qam16 => 4,
            Self::Qam64 => 6,
            Self::Qam128 => 7,
            Self::Qam256 => 8,
            Self::Qam512 => 9,
            Self::Qam1024 => 10,
            Self::Qam2048 => 11,
            Self::Qam4096 => 12,
            Self::Qam8192 => 13,
            Self::Qam16384 => 14,
        }
    }
}
/// Selected profile with immutable original sources and explicit hypothesis origin.
#[derive(Debug, Clone)]
pub struct Dpd {
    pub(crate) profile: CapturedProfile,
    profile_ids: Vec<ProfileId>,
    pub(crate) origin: ProfileOrigin,
    pub(crate) sources: Vec<CaptureProvenance>,
    pub(crate) bandwidth: Option<BandwidthHz>,
}
/// Original full-resolution RxMER with exact identity and nulls.
#[derive(Debug, Clone, Serialize)]
pub struct RxMer {
    pub(crate) frame: RxMerFrame,
    pub(crate) provenance: CaptureProvenance,
}
/// Original complex response H; not reciprocal equalizer gain.
#[derive(Debug, Clone, Serialize)]
pub struct ChannelEstimate {
    pub(crate) frame: ChannelFrame,
    pub(crate) provenance: CaptureProvenance,
}
fn json(input: &NamedCapture) -> ApiResult<&[u8]> {
    use crate::input_format::{detect, json_bytes, InputFormat};
    match detect(input.bytes()) {
        InputFormat::JsonText => Ok(json_bytes(input.bytes())),
        InputFormat::BinaryPnm => Err(ApiError::new(
            ErrorCode::UnsupportedCapability,
            "Binary input requires pnm-input CaptureLoader",
        )),
        _ => Err(ApiError::new(
            ErrorCode::UnsupportedFormat,
            "Expected supported decoded JSON or binary PNM",
        )),
    }
}
fn provenance(
    original: &NamedCapture,
    decoded: &NamedCapture,
    epoch: u32,
    contract: Option<DecoderContract>,
) -> ApiResult<CaptureProvenance> {
    if let Some(contract) = &contract {
        contract
            .validate()
            .map_err(|e| ApiError::new(ErrorCode::UnsupportedDecoderContract, e.to_string()))?;
    }
    let binary = crate::input_format::detect(original.bytes())
        == crate::input_format::InputFormat::BinaryPnm;
    Ok(CaptureProvenance {
        original_name: original.name.clone(),
        original_sha256: format!("{:x}", Sha256::digest(original.bytes())),
        original_kind: if binary {
            CaptureKind::BinaryPnm
        } else {
            CaptureKind::DecodedJson
        },
        original_epoch: CaptureTime(epoch),
        decoded_sha256: format!("{:x}", Sha256::digest(decoded.bytes())),
        decoder_contract: contract,
        original: original.bytes.clone(),
        decoded: decoded.bytes.clone(),
    })
}
impl Dpd {
    /// Admits one checked simulation identity while retaining the complete original source bytes.
    /// # Arguments
    /// * `capture` - Immutable complete byte-sized source catalog.
    /// * `id` - Existing checked run identity; unrelated source IDs remain inspection-only.
    /// # Returns
    /// Original selected loading/provenance with the catalog's supported run-ID subset.
    /// # Errors
    /// Rejects an absent selected source identity; source and legacy loaders remain unchanged.
    pub(super) fn from_profile_capture(capture: &ProfileCapture, id: ProfileId) -> ApiResult<Self> {
        let profile = capture.profile_ref(id.value())?.clone();
        let profile_ids = capture
            .profile_ids()
            .iter()
            .copied()
            .filter_map(|raw| ProfileId::new(raw).ok())
            .collect();
        Ok(Self {
            profile,
            profile_ids,
            origin: ProfileOrigin::Captured,
            sources: vec![capture.provenance().clone()],
            bandwidth: None,
        })
    }

    /// Lists validated captured profile identities from bounded decoded JSON.
    ///
    /// # Arguments
    /// * `input` - Named decoded JSON; binary PNM needs the optional input adapter.
    ///
    /// # Returns
    /// Owned identities sorted by captured ID. All profiles are decoded and
    /// validated before returning; no controller is created or simulation run.
    /// Original bytes are unchanged. IDs describe the capture, not wire mapping.
    ///
    /// # Errors
    /// Rejects malformed, empty, duplicate or unsupported profile catalogs and
    /// identities outside the public ProfileId domain, preserving input limits.
    pub fn profile_ids_from_bytes(input: &NamedCapture) -> ApiResult<Vec<ProfileId>> {
        let profiles = pnm_json::import_profiles(JsonCapture {
            name: input.name(),
            bytes: json(input)?,
        })?;
        let mut ids = profiles
            .into_iter()
            .map(|profile| ProfileId::new(profile.profile_id))
            .collect::<ApiResult<Vec<_>>>()?;
        ids.sort_by_key(|id| id.value());
        Ok(ids)
    }
    /// Imports an explicitly selected profile from bounded decoded JSON.
    /// # Arguments
    /// * `input` - Original named JSON bytes; binary needs the optional adapter.
    /// * `id` - Captured profile ID to select, even for multi-profile documents.
    /// # Returns
    /// Immutable imported profile; no simulation runs.
    /// # Errors
    /// Rejects malformed JSON, unsupported kind or absent profile ID.
    pub fn from_bytes(input: &NamedCapture, id: ProfileId) -> ApiResult<Self> {
        Self::from_decoded(input, input, id, None)
    }
    /// Admits decoder output while preserving its original input separately.
    /// # Arguments
    /// * `original`, `decoded` - Original bytes and bounded verified JSON intermediate.
    /// * `id` - Explicit captured identity.
    /// * `contract` - Optional declared producer, checked before JSON parsing.
    /// # Returns
    /// Immutable selected profile retaining both source representations.
    /// # Errors
    /// Rejects unsupported producer, malformed profile or missing identity.
    pub fn from_decoded(
        original: &NamedCapture,
        decoded: &NamedCapture,
        id: ProfileId,
        contract: Option<DecoderContract>,
    ) -> ApiResult<Self> {
        if let Some(c) = &contract {
            c.validate()
                .map_err(|e| ApiError::new(ErrorCode::UnsupportedDecoderContract, e.to_string()))?;
        }
        let profiles = pnm_json::import_profiles(JsonCapture {
            name: decoded.name(),
            bytes: json(decoded)?,
        })?;
        let mut profile_ids = profiles
            .iter()
            .map(|profile| ProfileId::new(profile.profile_id))
            .collect::<ApiResult<Vec<_>>>()?;
        profile_ids.sort_by_key(|id| id.value());
        let profile = profiles
            .into_iter()
            .find(|p| p.profile_id == id.0)
            .ok_or_else(|| super::error::invalid("Selected profile ID is absent"))?;
        let source = provenance(original, decoded, profile.capture_time, contract)?;
        Ok(Self {
            profile,
            profile_ids,
            origin: ProfileOrigin::Captured,
            sources: vec![source],
            bandwidth: None,
        })
    }
    /// Reads captured identity independently of internal wire-profile mapping.
    /// # Returns
    /// Selected original profile ID.
    pub fn profile_id(&self) -> ProfileId {
        ProfileId(self.profile.profile_id)
    }
    /// Borrows the validated profile catalog retained when this DPD was loaded.
    ///
    /// # Returns
    /// Captured IDs in ascending order, independent of the selected profile.
    /// Reading performs no file access, decoding or simulation. Created profiles
    /// contain one ID; edited profiles preserve their original source catalog.
    /// Values remain valid for this immutable DPD's lifetime.
    pub fn profile_ids(&self) -> &[ProfileId] {
        &self.profile_ids
    }
    /// Borrows the exact original frequency grid.
    /// # Returns
    /// Immutable validated grid; no resampling.
    pub fn grid(&self) -> &FrequencyGrid {
        &self.profile.key.grid
    }
    /// Borrows exact original device/channel/grid identity for explicit association.
    /// # Returns
    /// Immutable identity tied to this profile; no decoding or execution occurs.
    pub fn key(&self) -> &crate::scenario::ScenarioKey {
        &self.profile.key
    }
    /// Borrows preserved sources for captured and edited profiles.
    /// # Returns
    /// Empty for a newly created hypothesis.
    pub fn sources(&self) -> &[CaptureProvenance] {
        &self.sources
    }
    /// Reads captured/created/edited provenance classification.
    /// # Returns
    /// A hypothesis is never labeled measured.
    pub fn origin(&self) -> ProfileOrigin {
        self.origin
    }
    /// Creates a uniform data hypothesis against an already validated OCD.
    /// # Arguments
    /// * `ocd` - Exact geometry, identity and protected roles to preserve.
    /// * `id` - Requested profile identity.
    /// * `modulation` - Payload loading only; protected bins stay unchanged.
    /// # Returns
    /// New synthetic profile with no captured source hashes or measured epoch.
    /// # Errors
    /// Rejects geometry not supported by the existing engine.
    pub fn from_ocd(ocd: &Ocd, id: ProfileId, modulation: DataModulation) -> ApiResult<Self> {
        use crate::{
            profile::candidate::{AssignmentOrigin, CarrierAssignment},
            transmission::layout::{CarrierLayout, PhysicalCarrierRole},
            types::modulation::CarrierType,
        };
        let layout = CarrierLayout::new(&ocd.transmission, &ocd.layout)?;
        let assignments = (0..ocd.key.grid.sample_count)
            .map(|i| {
                let role =
                    layout.static_roles()[usize::from(ocd.key.grid.first_subcarrier_index) + i];
                let (carrier_type, bit_loading) = match role {
                    PhysicalCarrierRole::Data => (CarrierType::Data, Some(modulation.bits())),
                    PhysicalCarrierRole::Plc => (CarrierType::Plc, Some(4)),
                    PhysicalCarrierRole::ContinuousPilot => (CarrierType::ContinuousPilot, Some(1)),
                    _ => (CarrierType::Exclusion, None),
                };
                CarrierAssignment {
                    carrier_type,
                    bit_loading,
                    origin: AssignmentOrigin::Proposed,
                }
            })
            .collect();
        // Private compatibility metadata for the existing layout validator only.
        // Public origin/sources explicitly identify this as a hypothesis.
        let profile = CapturedProfile {
            key: ocd.key.clone(),
            source: crate::scenario::SourceProvenance {
                name: "created hypothesis".into(),
                sha256: "0".repeat(64),
                byte_length: 0,
                capture_time: Some(0),
                file_kind: "internal_hypothesis".into(),
                format_major: 0,
                format_minor: 0,
            },
            capture_time: 0,
            profile_id: id.0,
            assignments,
        };
        Ok(Self {
            profile,
            profile_ids: vec![id],
            origin: ProfileOrigin::Created,
            sources: vec![],
            bandwidth: Some(*ocd.configuration.bandwidth().value()),
        })
    }
    /// Returns an edited hypothesis while preserving original captures and roles.
    /// # Arguments
    /// * `range` - Half-open physical-bin range within this original grid.
    /// * `modulation` - New loading for data carriers only.
    /// # Returns
    /// Independent edited profile; original and all protected assignments unchanged.
    /// # Errors
    /// Rejects invalid range or unsupported candidate assignments atomically.
    pub fn with_modulation(
        &self,
        range: CarrierRange,
        modulation: DataModulation,
    ) -> ApiResult<Self> {
        let start = range
            .first
            .0
            .checked_sub(self.grid().first_subcarrier_index)
            .ok_or_else(|| super::error::invalid("Range begins before captured grid"))?
            as usize;
        let count = usize::from(range.end.0 - range.first.0);
        let mut candidate = CandidateProfile {
            schema_version: 1,
            name: "API hypothesis".into(),
            protocol: ProtocolScope::Docsis31,
            key: self.profile.key.clone(),
            profile_id: Some(self.profile.profile_id),
            assignments: self.profile.assignments.clone(),
            sources: vec![],
            roles_verified: false,
        };
        candidate.edit_range(start, count, Some(modulation.bits()))?;
        let mut edited = self.clone();
        edited.profile.assignments = candidate.assignments;
        edited.origin = ProfileOrigin::Edited;
        Ok(edited)
    }
}
/// Checked half-open physical-bin range for data-only edits.
#[derive(Debug, Clone, Copy)]
pub struct CarrierRange {
    first: SubcarrierIndex,
    end: SubcarrierIndex,
}
impl CarrierRange {
    /// Validates an increasing physical-bin edit range.
    /// # Arguments
    /// * `first`, `end` - Inclusive start and exclusive end.
    /// # Returns
    /// Checked range; profile bounds checked when editing.
    /// # Errors
    /// Rejects empty or reversed ranges.
    pub fn new(first: SubcarrierIndex, end: SubcarrierIndex) -> ApiResult<Self> {
        if first.0 >= end.0 {
            return Err(super::error::invalid("Empty carrier range"));
        }
        Ok(Self { first, end })
    }
}
macro_rules! measurement {
    ($ty:ident,$import:ident,$field:ident,$frame:ty) => {
        impl $ty {
            /// Imports bounded decoded JSON without simulation or implicit pairing.
            /// # Arguments
            /// * `input` - Original JSON bytes and identity.
            /// # Returns
            /// Immutable exact-grid observation preserving nulls and epochs.
            /// # Errors
            /// Rejects malformed, wrong-kind or unsupported content.
            pub fn from_bytes(input: &NamedCapture) -> ApiResult<Self> {
                Self::from_decoded(input, input, None)
            }
            /// Admits a decoded intermediate while retaining original bytes.
            /// # Arguments
            /// * `original`, `decoded` - Original input and supported JSON output.
            /// * `contract` - Explicit optional producer declaration.
            /// # Returns
            /// Immutable observation with original and intermediate provenance.
            /// # Errors
            /// Rejects unsupported contracts, kinds, grids and numeric values.
            pub fn from_decoded(
                original: &NamedCapture,
                decoded: &NamedCapture,
                contract: Option<DecoderContract>,
            ) -> ApiResult<Self> {
                if let Some(c) = &contract {
                    c.validate().map_err(|e| {
                        ApiError::new(ErrorCode::UnsupportedDecoderContract, e.to_string())
                    })?;
                }
                let frame = pnm_json::$import(JsonCapture {
                    name: decoded.name(),
                    bytes: json(decoded)?,
                })?;
                let source = provenance(original, decoded, frame.capture_time, contract)?;
                Ok(Self {
                    frame,
                    provenance: source,
                })
            }
            /// Borrows original provenance and retained bytes.
            /// # Returns
            /// Immutable source identity.
            pub fn provenance(&self) -> &CaptureProvenance {
                &self.provenance
            }
            /// Borrows the original scientific source-kind identifier without reclassification.
            /// # Returns
            /// Exact backend source kind, including synthetic/designed measurement identifiers.
            pub fn source_kind(&self) -> &str {
                &self.frame.source.file_kind
            }
            /// Borrows the original validated frequency grid.
            /// # Returns
            /// Exact grid without resampling.
            pub fn grid(&self) -> &FrequencyGrid {
                &self.frame.key.grid
            }
        }
    };
}
measurement!(RxMer, import_rxmer, mer_db, RxMerFrame);
measurement!(ChannelEstimate, import_channel, iq, ChannelFrame);
impl RxMer {
    /// Admits a materialized observation while retaining its original recipe or capture.
    /// # Arguments
    /// * `original` - Bounded immutable source bytes retained without rewriting.
    /// * `frame` - Owned exact-grid observation with validated source metadata.
    /// # Returns
    /// Idle immutable input with separate source and serialized-frame provenance.
    /// # Errors
    /// Rejects invalid frames or oversized serialized intermediates.
    pub fn from_frame(original: &NamedCapture, frame: RxMerFrame) -> ApiResult<Self> {
        frame.validate()?;
        let bytes = serde_json::to_vec(&frame)
            .map_err(|error| ApiError::new(ErrorCode::InvalidInput, error.to_string()))?;
        let decoded = NamedCapture::new(original.name(), bytes)?;
        let source = provenance(original, &decoded, frame.capture_time, None)?;
        Ok(Self {
            frame,
            provenance: source,
        })
    }

    /// Borrows the validated original-grid frame for native Rust adapters.
    /// # Returns
    /// Immutable complete identity, samples and source provenance for this
    /// capture's lifetime. No parsing, simulation or source mutation occurs.
    pub fn frame(&self) -> &RxMerFrame {
        &self.frame
    }
    /// Reads full-resolution values with explicit dB units and missing samples.
    /// # Returns
    /// Owned values; original storage is unchanged.
    pub fn values(&self) -> Vec<Option<MerDb>> {
        self.frame.mer_db.iter().map(|v| v.map(MerDb)).collect()
    }
}

impl serde::Serialize for Dpd {
    /// Serializes public identity and provenance, omitting private engine compatibility metadata.
    /// # Arguments
    /// * `serializer` - Caller-selected output serializer.
    /// # Returns
    /// Original profile identity, grid, roles, origin and original sources.
    /// # Errors
    /// Propagates serializer failures without altering the profile.
    fn serialize<Serializer: serde::Serializer>(
        &self,
        serializer: Serializer,
    ) -> Result<Serializer::Ok, Serializer::Error> {
        serde_json::json!({"profile_id":self.profile_id(),"profile_ids":self.profile_ids(),"grid":self.grid(),
            "carriers":self.profile.assignments,"origin":self.origin,
            "sources":self.sources,"established_bandwidth_hz":self.bandwidth})
        .serialize(serializer)
    }
}

/// Finite real component of a dimensionless complex channel response.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct ResponseReal(f64);
/// Finite imaginary component of a dimensionless complex channel response.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(transparent)]
pub struct ResponseImaginary(f64);
/// Named finite complex H components, distinct from reciprocal equalizer gain.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct ComplexResponse {
    real: ResponseReal,
    imaginary: ResponseImaginary,
}
impl ResponseReal {
    /// Reads the dimensionless real response component.
    /// # Returns
    /// Original finite value without calibration or inversion.
    pub fn value(self) -> f64 {
        self.0
    }
}
impl ResponseImaginary {
    /// Reads the dimensionless imaginary response component.
    /// # Returns
    /// Original finite value without phase normalization.
    pub fn value(self) -> f64 {
        self.0
    }
}
impl ComplexResponse {
    /// Reads the real component with its response-domain identity.
    /// # Returns
    /// Finite original real value.
    pub fn real(self) -> ResponseReal {
        self.real
    }
    /// Reads the imaginary component with its response-domain identity.
    /// # Returns
    /// Finite original imaginary value.
    pub fn imaginary(self) -> ResponseImaginary {
        self.imaginary
    }
}
impl ChannelEstimate {
    /// Borrows the exact admitted channel frame without deriving an inverse response.
    /// # Returns
    /// Read-only original key, epoch, decoded-source provenance and I/Q/nulls tied
    /// to this input's lifetime. No clone, parsing, simulation or source mutation.
    /// Original capture bytes/hash and decoder lineage remain in provenance().
    pub fn frame(&self) -> &ChannelEstimateFrame {
        &self.frame
    }

    /// Reads original full-resolution complex H with explicit missing samples.
    /// # Returns
    /// Owned immutable-value snapshot; original storage remains unchanged.
    pub fn response(&self) -> Vec<Option<ComplexResponse>> {
        self.frame
            .iq
            .iter()
            .map(|point| {
                point.map(|[real, imaginary]| ComplexResponse {
                    real: ResponseReal(real),
                    imaginary: ResponseImaginary(imaginary),
                })
            })
            .collect()
    }

    /// Measures this complete immutable root and all reachable retained payloads.
    ///
    /// Includes requested spare buffer capacity, private caches and each owned
    /// Arc pointee/edge, conservatively repeating aliases. Ordered entries and
    /// shared roots are separate from payload bytes; allocator, tree-node and
    /// refcount bookkeeping are excluded. No source or numerical state advances.
    ///
    /// # Returns
    /// Passive current-root metadata including its fixed value exactly once.
    /// This does not reserve future output, construction scratch or copied views.
    ///
    /// # Errors
    /// Returns ResourceLimit if retained_footprint arithmetic overflows.
    pub fn retained_footprint(&self) -> ApiResult<merlin_api::RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| ApiError::new(ErrorCode::ResourceLimit, "retained_footprint overflow"))
    }
}
impl Dpd {
    /// Borrows exact captured or explicitly edited assignments in source-grid order.
    /// # Returns
    /// Existing domain-role models, including zero loading and unknown roles.
    pub fn carriers(&self) -> &[crate::profile::candidate::CarrierAssignment] {
        &self.profile.assignments
    }
}

impl RetainedPayload for Dpd {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            profile,
            profile_ids,
            origin,
            sources,
            bandwidth,
        } = self;
        merlin_api::retention_support::sum([
            profile.retained_children(),
            profile_ids.retained_children(),
            origin.retained_children(),
            sources.retained_children(),
            bandwidth.retained_children(),
        ])
    }
}

impl RetainedPayload for RxMer {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { frame, provenance } = self;
        merlin_api::retention_support::sum([
            frame.retained_children(),
            provenance.retained_children(),
        ])
    }
}

impl RetainedPayload for ChannelEstimate {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { frame, provenance } = self;
        merlin_api::retention_support::sum([
            frame.retained_children(),
            provenance.retained_children(),
        ])
    }
}

impl RetainedPayload for CaptureProvenance {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            original_name,
            original_sha256,
            original_kind,
            original_epoch,
            decoded_sha256,
            decoder_contract,
            original,
            decoded,
        } = self;
        merlin_api::retention_support::sum([
            original_name.retained_children(),
            original_sha256.retained_children(),
            original_kind.retained_children(),
            original_epoch.retained_children(),
            decoded_sha256.retained_children(),
            decoder_contract.retained_children(),
            original.retained_children(),
            decoded.retained_children(),
        ])
    }
}

impl RetainedPayload for CaptureKind {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::BinaryPnm => Some(RetainedFootprint::default()),
            Self::DecodedJson => Some(RetainedFootprint::default()),
        }
    }
}

impl RetainedPayload for ProfileOrigin {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Captured => Some(RetainedFootprint::default()),
            Self::Created => Some(RetainedFootprint::default()),
            Self::Edited => Some(RetainedFootprint::default()),
        }
    }
}

impl RetainedPayload for NamedCapture {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { name, bytes } = self;
        merlin_api::retention_support::sum([name.retained_children(), bytes.retained_children()])
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use super::{CaptureKind, CaptureProvenance, CaptureTime, DecoderContract};
    use merlin_api::retention_support::retained_footprint;
    use std::{mem::size_of, sync::Arc};

    #[test]
    fn provenance_counts_spare_strings_and_repeated_original_canonical_arc() {
        // Extra UTF-8 storage tests capacity rather than serialized text length.
        const SPARE_NAME_BYTES: usize = 13;
        let spare = |text: &str| {
            let mut value = String::with_capacity(text.len() + SPARE_NAME_BYTES);
            value.push_str(text);
            value
        };
        let bytes: Arc<[u8]> = Arc::from(&b"original capture fixture"[..]);
        let provenance = CaptureProvenance {
            original_name: spare("capture"),
            original_sha256: spare("original hash fixture"),
            original_kind: CaptureKind::DecodedJson,
            original_epoch: CaptureTime::new(0).unwrap(), // Epoch anchor only.
            decoded_sha256: spare("canonical hash fixture"),
            decoder_contract: Some(DecoderContract {
                revision: 1, // Storage fixture lineage revision, no import occurs.
                producer: spare("fixture"),
                producer_version: spare("fixture version"),
            }),
            original: Arc::clone(&bytes),
            decoded: Arc::clone(&bytes),
        };
        let contract = provenance.decoder_contract.as_ref().unwrap();
        let expected = size_of::<CaptureProvenance>()
            + provenance.original_name.capacity()
            + provenance.original_sha256.capacity()
            + provenance.decoded_sha256.capacity()
            + contract.producer.capacity()
            + contract.producer_version.capacity()
            + bytes.len() * 2; // Both retained edges count the same byte payload.
        let serialized = serde_json::to_value(&provenance).unwrap();
        let owners = Arc::strong_count(&bytes);
        let footprint = retained_footprint(&provenance).unwrap();
        assert_eq!(footprint.payload_bytes, expected);
        assert_eq!(footprint.shared_roots, 2);
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(Arc::strong_count(&bytes), owners);
        assert_eq!(serde_json::to_value(&provenance).unwrap(), serialized);
    }
}
