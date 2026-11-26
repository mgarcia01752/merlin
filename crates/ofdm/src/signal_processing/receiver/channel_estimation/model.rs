/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::types::{IqPair, SubcarrierIdx};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};
/// Explicit receiver correction source; pilot mode never consumes supplied H.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverMode {
    /// Correct using the supplied CM channel response selected for the channel.
    #[default]
    #[serde(rename = "supplied_channel", alias = "cm_channel_estimated")]
    CmChannelEstimated,
    PilotEstimated,
    /// Pilot correction and received-only reliability; supplied H/noise never enter RX.
    #[serde(rename = "merlin_estimated", alias = "merlin_channel_estimated")]
    MerlinChannelEstimated,
}
impl ReceiverMode {
    /// Compatibility name for callers using the previous supplied-channel mode.
    #[allow(non_upper_case_globals)]
    pub const SuppliedChannel: Self = Self::CmChannelEstimated;
}
/// One least-squares pilot estimate and its pre-update independent prediction.
#[derive(Debug, Clone, Serialize)]
pub struct PilotEstimate {
    pub bin: SubcarrierIdx,
    pub scattered: bool,
    pub reference: IqPair,
    pub received: IqPair,
    pub response: IqPair,
    /// Previous-symbol/history prediction, not the same observation's fitted value.
    pub prior_prediction: Option<IqPair>,
}
/// Finite CH-EST-RX output at one physical symbol; coefficients retain actual gain.
#[derive(Debug, Clone, Serialize)]
pub struct ChannelEstimate {
    pub channel_symbol_index: u64,
    pub ready: bool,
    pub observed_bins: usize,
    pub maximum_age_symbols: u64,
    pub phase_slope_radians_per_bin: f64,
    pub pilots: Vec<PilotEstimate>,
    /// Full physical grid; excluded bins carry unity solely for shape consistency.
    pub response: Vec<IqPair>,
}

impl RetainedPayload for PilotEstimate {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            bin,
            scattered,
            reference,
            received,
            response,
            prior_prediction,
        } = self;
        merlin_api::retention_support::sum([
            bin.retained_children(),
            scattered.retained_children(),
            reference.retained_children(),
            received.retained_children(),
            response.retained_children(),
            prior_prediction.retained_children(),
        ])
    }
}

impl RetainedPayload for ChannelEstimate {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            channel_symbol_index,
            ready,
            observed_bins,
            maximum_age_symbols,
            phase_slope_radians_per_bin,
            pilots,
            response,
        } = self;
        merlin_api::retention_support::sum([
            channel_symbol_index.retained_children(),
            ready.retained_children(),
            observed_bins.retained_children(),
            maximum_age_symbols.retained_children(),
            phase_slope_radians_per_bin.retained_children(),
            pilots.retained_children(),
            response.retained_children(),
        ])
    }
}

impl RetainedPayload for ReceiverMode {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::CmChannelEstimated => Some(RetainedFootprint::default()),
            Self::PilotEstimated => Some(RetainedFootprint::default()),
            Self::MerlinChannelEstimated => Some(RetainedFootprint::default()),
        }
    }
}
