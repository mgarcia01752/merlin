/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared measurement models and independent project restoration.
pub(crate) mod document;
pub(crate) mod model;
use docsis_ofdm_fec::transmission::TransmissionSettings;
use model::Model;
use serde_json::Value;
/// Independent accepted measurements reconstructed before project adoption.
pub(crate) struct RestoredMeasurements {
    pub playback:
        Option<docsis_ofdm_fec::simulation::channel::measurement_playback::PlaybackConfiguration>,
    pub response: Option<docsis_ofdm_fec::scenario::ChannelFrame>,
}
/// Decodes all saved originals into an independent accepted measurement snapshot.
/// # Arguments
/// * `value` - Self-contained saved measurement section.
/// * `_settings` - OCD; cross-input mismatches remain editable readiness errors.
/// * `profile` - Current DPD or None for incomplete projects.
/// # Returns
/// Typed playback/response inputs, preserving all original bytes in the editor model.
/// # Errors
/// Reports malformed originals, unknown fields or exceeded resource limits.
pub(crate) fn restore_measurements(
    value: &Value,
    _settings: &TransmissionSettings,
    profile: Option<&docsis_ofdm_fec::profile::candidate::CapturedProfile>,
) -> Result<RestoredMeasurements, String> {
    Model::from_saved(value)?.published(profile)
}
