/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated immutable configured-channel admission; scientific composition keeps existing owners.
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
mod models;
mod preview;
use super::{
    ApiError, ApiResult, AssociationTolerance, CaptureProvenance, ChannelEstimate, Dpd, ErrorCode,
    MeasurementView, ProfileCapture, ScenarioKey, TransmissionConfiguration,
};
pub use models::*;
use serde::Serialize;

/// Application transport ceiling for channel settings; independent of waveform/PHY limits.
pub const MAX_CHANNEL_CONFIGURATION_BYTES: usize = 64 * 1024;

/// Immutable settings snapshot; admission/preview creates no source, filter stream or PHY run.
#[derive(Clone, Serialize)]
#[serde(transparent)]
pub struct ChannelConfiguration {
    settings: ChannelConfigurationInput,
}

impl ChannelConfiguration {
    /// Validates existing loss, electrical-reference and continuous-filter parameters.
    /// # Arguments
    /// * `settings` - Owned typed directional channel configuration.
    /// # Returns
    /// Immutable validated settings, separate from profile/H compatibility.
    /// # Errors
    /// Preserves existing finite parameter, electrical reference and echo/FIR bounds.
    pub fn new(settings: ChannelConfigurationInput) -> ApiResult<Self> {
        settings.additional_loss.validate()?;
        settings.power.validate()?;
        settings.filter.validate()?;
        Ok(Self { settings })
    }

    /// Admits bounded strict JSON through the same typed scientific parameter validator.
    /// # Arguments
    /// * `bytes` - At most 64 KiB JSON settings; unknown fields reject.
    /// # Returns
    /// Fresh immutable snapshot; reading starts no run or source stream.
    /// # Errors
    /// Rejects transport limits, malformed/unknown fields and existing parameter errors.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > MAX_CHANNEL_CONFIGURATION_BYTES {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Channel configuration exceeds 64 KiB",
            ));
        }
        let settings = serde_json::from_slice(bytes)
            .map_err(|cause| ApiError::new(ErrorCode::InvalidInput, cause.to_string()))?;
        Self::new(settings)
    }

    /// Borrows exact validated settings without changing live sessions or captures.
    /// # Returns
    /// Immutable typed settings valid for this snapshot's lifetime.
    pub fn settings(&self) -> &ChannelConfigurationInput {
        &self.settings
    }

    /// Resolves configured gain/calibration/noise through the existing power owner.
    /// # Returns
    /// Explicit configured electrical reference; absent calibration preserves relative units.
    pub fn power_reference(&self) -> ChannelPowerReference {
        let power = &self.settings.power;
        ChannelPowerReference {
            amplifier_gain_db: power.gain_db(),
            cpa_attenuation_db: power.cpa_attenuation_db,
            net_amplitude: power.net_amplitude(),
            millivolts_squared_per_unit: power.millivolts_squared_per_unit(),
            receiver_noise_power: power.receiver_noise_power(),
        }
    }

    /// Previews the configured total transfer using an accepted immutable Dpd.
    /// # Arguments
    /// * `transmission` - Accepted numerology.
    /// * `profile` - Accepted original/hypothesis profile and physical roles.
    /// * `response` - Original H, or None for explicit synthetic unity.
    /// * `tolerance` - Original H/profile epoch association window.
    /// # Returns
    /// Owned bounded before-receiver transfer; exact original inputs remain unchanged.
    /// # Errors
    /// Preserves profile/H/epoch/geometry/fit/null/unsupported-composition errors.
    pub fn preview(
        &self,
        transmission: &TransmissionConfiguration,
        profile: &Dpd,
        response: Option<&ChannelEstimate>,
        tolerance: AssociationTolerance,
    ) -> ApiResult<ChannelPreview> {
        preview::prepare(
            self,
            transmission,
            &profile.profile,
            profile.sources.clone(),
            response,
            tolerance,
        )
    }

    /// Previews a selected encoded source profile without widening legacy simulation IDs.
    /// # Arguments
    /// * `transmission` - Accepted numerology.
    /// * `capture` - Validated immutable complete source catalog.
    /// * `profile_id` - Original byte-sized source identity.
    /// * `response` - Original H, or None for explicit synthetic unity.
    /// * `tolerance` - Original H/profile epoch association window.
    /// # Returns
    /// Bounded configured transfer; success establishes channel planning, not a full PHY run.
    /// # Errors
    /// Preserves source identity, geometry, response/epoch and composition admission.
    pub fn preview_capture(
        &self,
        transmission: &TransmissionConfiguration,
        capture: &ProfileCapture,
        profile_id: u8,
        response: Option<&ChannelEstimate>,
        tolerance: AssociationTolerance,
    ) -> ApiResult<ChannelPreview> {
        preview::prepare(
            self,
            transmission,
            capture.profile_ref(profile_id)?,
            vec![capture.provenance().clone()],
            response,
            tolerance,
        )
    }
}

impl RetainedPayload for ChannelConfiguration {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { settings } = self;
        merlin_api::retention_support::sum([settings.retained_children()])
    }
}
