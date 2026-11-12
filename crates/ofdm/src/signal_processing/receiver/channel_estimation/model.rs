/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::types::{IqPair, SubcarrierIdx};
use serde::{Deserialize, Serialize};
/// Explicit receiver correction source; pilot mode never consumes supplied H.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverMode {
    #[default]
    SuppliedChannel,
    PilotEstimated,
    /// Pilot correction and received-only reliability; supplied H/noise never enter RX.
    MerlinEstimated,
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
