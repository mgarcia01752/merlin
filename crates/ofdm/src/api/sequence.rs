/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Immutable measured-sequence admission over existing residual-power planning.
mod queries;
use super::{
    ApiError, ApiResult, AssociationTolerance, CaptureProvenance, ChannelEstimate, Dpd, ErrorCode,
    MeasurementView, MeasurementViewport, ObservationDuration, RxMer, TransmissionConfiguration,
    Weight,
};
use crate::{
    scenario::{measurement_view::measurement_view, FrequencyGrid},
    signal_processing::noise::residual_power_mer,
    simulation::channel::measurement_playback::{
        validate_response, MeasurementPlayback, MAX_PLAYBACK_FRAMES, MAX_PLAYBACK_VALUES,
    },
};
use serde::Serialize;
use std::{collections::BTreeSet, sync::Arc};

pub use crate::simulation::channel::measurement_playback::PlaybackMode as MeasurementMode;

/// Maximum original captures, shared with the existing playback owner.
pub const MAX_SEQUENCE_CAPTURES: usize = MAX_PLAYBACK_FRAMES;

/// Maximum original residual values admitted before frame cloning.
pub const MAX_SEQUENCE_VALUES: usize = MAX_PLAYBACK_VALUES;

/// Existing desktop aggregate original-byte ceiling.
pub const MAX_SEQUENCE_ORIGINAL_BYTES: usize = 96 * 1024 * 1024;

/// Explicit synthetic playback clock; original capture epochs remain unchanged.
#[derive(Clone, Copy, Serialize)]
pub struct MeasurementSequenceOptions {
    /// Single, equal-capture-weight power average, hold or power interpolation.
    pub mode: MeasurementMode,
    /// Physical OFDM-symbol interval; temporal modes require a positive value.
    pub interval_symbols: u64,
    /// Maximum original epoch distance from the accepted profile.
    pub tolerance: AssociationTolerance,
}

/// One actual contributor to an immutable residual snapshot.
#[derive(Serialize)]
pub struct MeasurementContribution {
    /// Original zero-based position in this sequence's admitted order.
    pub index: usize,
    /// Dimensionless linear-power weight, never a dB weight.
    pub weight: Weight,
    /// Exact original source identity, epoch and byte/hash ownership.
    pub provenance: CaptureProvenance,
}

/// One equivalent residual query; no fabricated combined capture epoch.
#[derive(Serialize)]
pub struct MeasurementResidual {
    /// Requested zero-based synthetic observation symbol.
    pub symbol: u64,
    /// Per-original-carrier normalized residual powers, with explicit missing values.
    pub normalized_power: Vec<Option<f64>>,
    /// Every actual contributor and its linear-power weight.
    pub contributors: Vec<MeasurementContribution>,
}

/// Bounded equivalent RxMER view, distinct from a captured RxMER observation.
#[derive(Serialize)]
pub struct MeasurementSequencePreview {
    /// Existing bounded min/max view with explicit dB units and missing samples.
    pub view: MeasurementView,
    /// Unchanged original carrier grid.
    pub grid: FrequencyGrid,
    /// Requested synthetic symbol, independent of original epochs.
    pub symbol: u64,
    /// Original contributors without a fabricated average timestamp.
    pub contributors: Vec<MeasurementContribution>,
}

/// Accepted clock metadata; durations use the existing integer sample cadence.
#[derive(Serialize)]
pub struct MeasurementSequenceInfo {
    pub mode: MeasurementMode,
    pub source_count: usize,
    pub interval_symbols: u64,
    pub interval_milliseconds: f64,
    pub last_transition_symbol: u64,
    pub last_transition_milliseconds: f64,
}

/// Frozen reusable measurements; no live source, RNG, simulation job or UI state.
#[derive(Clone)]
pub struct MeasurementSequence {
    captures: Arc<Vec<RxMer>>,
    plan: MeasurementPlayback,
    transmission: TransmissionConfiguration,
    options: MeasurementSequenceOptions,
}

impl MeasurementSequence {
    /// Admits exact ordered originals through the existing physical measurement planner.
    ///
    /// # Arguments
    /// * `profile` - Accepted device/channel/grid and required carrier roles.
    /// * `transmission` - Immutable admitted numerology.
    /// * `captures` - Unique original identities in explicit caller-selected order.
    /// * `options` - Mode, synthetic interval and original association tolerance.
    ///
    /// # Returns
    /// Immutable sequence; originals stay unchanged and no simulation starts.
    ///
    /// # Errors
    /// Rejects duplicate hashes, aggregate limits and existing grid/epoch/coverage/mode errors.
    pub fn new(
        profile: &Dpd,
        transmission: &TransmissionConfiguration,
        captures: Vec<RxMer>,
        options: MeasurementSequenceOptions,
    ) -> ApiResult<Self> {
        if captures.is_empty() || captures.len() > MAX_PLAYBACK_FRAMES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Measurement capture count exceeds bounds",
            ));
        }
        let values = captures
            .iter()
            .map(|v| v.frame.mer_db.len())
            .try_fold(0usize, usize::checked_add);
        let bytes = captures
            .iter()
            .map(|v| v.provenance.original_bytes().len())
            .try_fold(0usize, usize::checked_add);
        if values.is_none_or(|n| n > MAX_PLAYBACK_VALUES)
            || bytes.is_none_or(|n| n > MAX_SEQUENCE_ORIGINAL_BYTES)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Measurement sequence exceeds original/value/capture bounds",
            ));
        }
        let identities = captures
            .iter()
            .map(|v| v.provenance.original_sha256())
            .collect::<BTreeSet<_>>();
        if identities.len() != captures.len() {
            return Err(ApiError::new(
                ErrorCode::InvalidInput,
                "Duplicate original measurement identity",
            ));
        }
        transmission.check_profile(profile)?;
        let plan = MeasurementPlayback::new(
            transmission.settings(),
            &profile.profile,
            captures.iter().map(|v| v.frame.clone()).collect(),
            options.mode,
            options.interval_symbols,
            options.tolerance.value(),
        )?;
        Ok(Self {
            captures: Arc::new(captures),
            plan,
            transmission: transmission.clone(),
            options,
        })
    }

    /// Borrows an exact original without rewriting epochs or advancing playback.
    /// # Arguments
    /// * `index` - Zero-based position in the admitted order.
    /// # Returns
    /// Immutable capture tied to this sequence's lifetime.
    /// # Errors
    /// InvalidIdentifier for a position outside the retained collection.
    pub fn capture(&self, index: usize) -> ApiResult<&RxMer> {
        self.captures.get(index).ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Measurement position is outside the sequence",
            )
        })
    }
}

impl TransmissionConfiguration {
    /// Validates explicit unity or original H with profile/grid/epoch/coverage checks.
    /// # Arguments
    /// * `profile` - Accepted spatial identity and physical carrier roles.
    /// * `response` - Original H, or None for explicit synthetic unity.
    /// * `tolerance` - Maximum original epoch distance from the profile.
    /// # Returns
    /// Unit on compatibility without starting a job or inferring equalizer gain.
    /// # Errors
    /// Preserves physical admission and returns AssociationFailed for epoch mismatch.
    pub fn check_response(
        &self,
        profile: &Dpd,
        response: Option<&ChannelEstimate>,
        tolerance: AssociationTolerance,
    ) -> ApiResult<()> {
        self.check_profile(profile)?;
        if response.is_some_and(|v| {
            v.frame.capture_time.abs_diff(profile.profile.capture_time) > tolerance.value()
        }) {
            return Err(ApiError::new(
                ErrorCode::AssociationFailed,
                "Channel response exceeds profile epoch tolerance",
            ));
        }
        Ok(validate_response(
            self.settings(),
            &profile.profile,
            response.map(|v| &v.frame),
        )?)
    }
}
