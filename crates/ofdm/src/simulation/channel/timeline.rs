/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit piecewise-constant measurement epochs; no inferred temporal correlation.
use crate::{signal_processing::channel::impairments::ChannelImpairments, types::CaptureTime};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
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

impl RetainedPayload for ImpairmentEpoch {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            first_observation_symbol,
            capture_time,
            impairments,
        } = self;
        merlin_api::retention_support::sum([
            first_observation_symbol.retained_children(),
            capture_time.retained_children(),
            impairments.retained_children(),
        ])
    }
}
