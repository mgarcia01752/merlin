/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit configured-channel inputs and bounded before-receiver observations.
use super::*;
pub use crate::signal_processing::channel::spectrum::{ChannelSpectrum, ChannelSpectrumBucket};
use serde::{Deserialize, Serialize};

pub use crate::signal_processing::channel::{
    additional_loss::{AdditionalLoss, CableModel, LengthUnit},
    filter::{ChannelFilterSettings, EchoPath, FilterDiagnostics},
    power::PowerConditioning,
};

/// Existing directional channel settings; defaults retain unity/relative/diagonal behavior.
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ChannelConfigurationInput {
    pub additional_loss: AdditionalLoss,
    pub power: PowerConditioning,
    pub filter: ChannelFilterSettings,
}

/// Configured electrical/gain reference, independent of a measured original H.
#[derive(Serialize)]
pub struct ChannelPowerReference {
    pub amplifier_gain_db: f64,
    pub cpa_attenuation_db: f64,
    pub net_amplitude: f64,
    pub millivolts_squared_per_unit: Option<f64>,
    pub receiver_noise_power: f64,
}

/// One actual active physical carrier sampled from the configured transfer.
#[derive(Serialize)]
pub struct ChannelResponsePoint {
    pub physical_carrier: u16,
    pub frequency_hz: u32,
    pub additional_gain_db: f64,
    pub effective_gain_db: Option<f64>,
}

/// Owned configured transfer before receiver correction; measured inputs stay separate.
#[derive(Serialize)]
pub struct ChannelPreview {
    pub reference_plane: &'static str,
    pub profile_id: u8,
    pub key: ScenarioKey,
    pub sources: Vec<CaptureProvenance>,
    /// Original full source-grid response; excluded carriers remain explicit gaps.
    pub effective_response: Vec<Option<[f64; 2]>>,
    pub active_start_offset: usize,
    pub active_count: usize,
    pub active_low_frequency_hz: u32,
    pub active_high_frequency_hz: u32,
    /// At most 1024 original-grid extrema groups over the active physical band.
    pub gain: MeasurementView,
    /// At most 128 actual active carriers, including both band endpoints.
    pub additional_points: Vec<ChannelResponsePoint>,
    pub additional_low_gain_db: f64,
    pub additional_high_gain_db: f64,
    pub power: ChannelPowerReference,
    /// Actual selected continuous-filter plan, separate from original input H.
    pub filter_coefficients: Vec<[f64; 2]>,
    pub filter_diagnostics: Option<FilterDiagnostics>,
}
