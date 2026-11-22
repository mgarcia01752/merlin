/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable complete source-profile inspection, separate from simulation identity admission.
mod models;
mod views;
use super::*;
use crate::scenario::pnm_json::{import_profile_capture, profile_metadata::ProfileCaptureMetadata};
pub use models::*;

/// Validated original capture/catalog; byte-sized source IDs never silently become run IDs.
#[derive(Clone)]
pub struct ProfileCapture {
    profiles: Arc<Vec<CapturedProfile>>,
    source: Arc<ProfileCaptureMetadata>,
    ids: Vec<u8>,
    summaries: Vec<ProfileSummary>,
    provenance: CaptureProvenance,
}

impl ProfileCapture {
    pub(crate) fn profile_ref(&self, profile_id: u8) -> ApiResult<&CapturedProfile> {
        self.profiles
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidIdentifier,
                    "Source profile identity is unavailable",
                )
            })
    }
    /// Admits decoded source JSON without a simulation or implicit profile selection.
    /// # Arguments
    /// * `input` - Exact bounded named JSON; binary uses the optional native CaptureLoader.
    /// # Returns
    /// Immutable complete source catalog, including source IDs outside legacy run admission.
    /// # Errors
    /// Preserves existing format/profile/scheme/grid/resource errors.
    pub fn from_bytes(input: &NamedCapture) -> ApiResult<Self> {
        Self::from_decoded(input, input, None)
    }

    /// Admits one decoder intermediate while retaining exact original source provenance.
    /// # Arguments
    /// * `original`, `decoded` - Bounded original bytes and validated-format decoder JSON.
    /// * `contract` - Optional accepted native decoder declaration.
    /// # Returns
    /// Owned read-only profiles/header/schemes; no PHY execution occurs.
    /// # Errors
    /// Preserves decoder-contract, schema, profile, grid and work-limit errors.
    pub fn from_decoded(
        original: &NamedCapture,
        decoded: &NamedCapture,
        contract: Option<DecoderContract>,
    ) -> ApiResult<Self> {
        if let Some(contract) = &contract {
            contract.validate().map_err(|cause| {
                ApiError::new(ErrorCode::UnsupportedDecoderContract, cause.to_string())
            })?;
        }
        let (profiles, source) = import_profile_capture(JsonCapture {
            name: decoded.name(),
            bytes: json(decoded)?,
        })?;
        let mut summaries = profiles
            .iter()
            .map(|profile| views::summary(profile, ProfileOrigin::Captured))
            .collect::<ApiResult<Vec<_>>>()?;
        summaries.sort_by_key(|profile| profile.profile_id);
        let ids = summaries.iter().map(|profile| profile.profile_id).collect();
        let provenance = provenance(original, decoded, source.header.capture_time, contract)?;
        Ok(Self {
            profiles: Arc::new(profiles),
            source: Arc::new(source),
            ids,
            summaries,
            provenance,
        })
    }

    /// Selects original loading for the existing simulation ProfileId domain.
    /// # Arguments
    /// * `id` - Checked 0..15 identity present in this complete source catalog.
    /// # Returns
    /// Immutable captured Dpd preserving selected assignments and exact original provenance.
    /// Its profile_ids lists only supported run IDs; this capture retains all raw source IDs.
    /// No source, transmitter, channel or receiver starts.
    /// # Errors
    /// InvalidIdentifier for an absent selected source; unrelated raw IDs are preserved.
    pub fn simulation_profile(&self, id: ProfileId) -> ApiResult<Dpd> {
        Dpd::from_profile_capture(self, id)
    }
    /// Borrows every original encoded source identity, sorted independently of source order.
    /// # Returns
    /// Complete byte-sized catalog; this does not widen Simulation ProfileId.
    pub fn profile_ids(&self) -> &[u8] {
        &self.ids
    }

    /// Reads original header/grid/catalog metadata without exposing unbounded scheme arrays.
    /// # Returns
    /// Owned small snapshot with sorted IDs and the original source profile order.
    pub fn metadata(&self) -> ProfileCaptureInfo {
        ProfileCaptureInfo {
            header: self.source.header.clone(),
            key: self.source.key.clone(),
            profile_ids: self.ids.clone(),
            source_profile_order: self
                .profiles
                .iter()
                .map(|profile| profile.profile_id)
                .collect(),
            original_kind: self.provenance.original_kind,
        }
    }

    /// Borrows original/intermediate byte and decoder provenance without reparsing.
    /// # Returns
    /// Immutable provenance tied to this capture's lifetime.
    pub fn provenance(&self) -> &CaptureProvenance {
        &self.provenance
    }

    /// Reads a scientific summary by its original identity, never an array position.
    /// # Arguments
    /// * `profile_id` - Original encoded byte-sized identity.
    /// # Returns
    /// Owned role counts and uncoded useful-symbol capacity; no receiver/net-rate claim.
    /// # Errors
    /// InvalidIdentifier for an unavailable source identity.
    pub fn summary(&self, profile_id: u8) -> ApiResult<ProfileSummary> {
        self.summaries
            .iter()
            .find(|profile| profile.profile_id == profile_id)
            .cloned()
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::InvalidIdentifier,
                    "Source profile identity is unavailable",
                )
            })
    }

    /// Returns a bounded page of validated original compact schemes and physical coordinates.
    /// # Arguments
    /// * `profile_id` - Original source identity.
    /// * `start` - Zero-based original scheme position.
    /// * `limit` - Maximum returned rows, 1 through 256.
    /// # Returns
    /// Original ordered schemes, raw codes, known roles/constellations and ideal bounds.
    /// # Errors
    /// Rejects unavailable identities, invalid limits and positions beyond the source.
    pub fn schemes(
        &self,
        profile_id: u8,
        start: usize,
        limit: usize,
    ) -> ApiResult<ProfileSchemeWindow> {
        if !(1..=MAX_PROFILE_SCHEME_ROWS).contains(&limit) {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Profile scheme page exceeds row bounds",
            ));
        }
        let schemes = self.source.schemes.get(&profile_id).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Source profile identity is unavailable",
            )
        })?;
        let selected = schemes.get(start..).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidInput,
                "Scheme position is outside the source",
            )
        })?;
        Ok(ProfileSchemeWindow {
            profile_id,
            start,
            total: schemes.len(),
            schemes: selected.iter().take(limit).cloned().collect(),
        })
    }

    /// Builds bounded authoritative multi-profile metrics on the exact original grid.
    /// # Arguments
    /// * `request` - Explicit source selection, role filter, metric, viewport and total budget.
    /// # Returns
    /// Owned raw extrema/role support/provenance; no source or simulation state changes.
    /// # Errors
    /// Rejects invalid IDs/windows/resources before constructing output arrays.
    pub fn inspect(&self, request: &ProfileViewRequest) -> ApiResult<ProfileInspection> {
        views::inspect(self, request)
    }
}

impl Dpd {
    /// Summarizes accepted assignments using the same source-profile numerical owner.
    /// # Returns
    /// Original/hypothesis origin, role counts and uncoded useful-symbol capacity.
    /// # Errors
    /// Preserves checked rate/size failures without invoking a simulation.
    pub fn summary(&self) -> ApiResult<ProfileSummary> {
        views::summary(&self.profile, self.origin)
    }

    /// Views this one accepted profile through the same bounded metric owner.
    /// # Arguments
    /// * `request` - Original viewport/roles/metric; explicit IDs must name this profile.
    /// # Returns
    /// Owned view, preserving captured/created/edited origin and every retained source.
    /// # Errors
    /// Preserves identity/window/budget errors without loading another profile or running.
    pub fn view(&self, request: &ProfileViewRequest) -> ApiResult<ProfileInspection> {
        let summary = self.summary()?;
        views::inspect_profiles(
            std::slice::from_ref(&self.profile),
            std::slice::from_ref(&summary),
            &self.profile.key,
            self.sources.clone(),
            request,
        )
    }
}
