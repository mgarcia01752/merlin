/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit piecewise-constant measurement epochs; no inferred temporal correlation.
use crate::{signal_processing::channel::impairments::ChannelImpairments, types::CaptureTime};
use serde::{Deserialize, Serialize};
/// A physical channel change relative to the first payload-observation symbol.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImpairmentEpoch {
    pub first_observation_symbol: u64,
    /// Original measurement epoch; never rewritten to the simulated symbol clock.
    pub capture_time: CaptureTime,
    pub impairments: ChannelImpairments,
}
