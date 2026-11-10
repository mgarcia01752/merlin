/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Validated finite selection; no waveform or scientific computation is duplicated.
use crate::{
    error::invalid,
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        symbols::PAYLOAD_PROFILE_ID,
    },
    types::{FrequencyHz, ProfileId},
    SimulationError,
};
use serde::{Deserialize, Serialize};
/// Default number of actual paired QAM observations, not full OFDM symbols.
pub const DEFAULT_CONSTELLATION_PAIRS: usize = 8_192;
/// Finite application memory limit; output adapters retain their own byte limit.
pub const MAX_CONSTELLATION_PAIRS: usize = 65_536;
/// Profile and optional inclusive physical carrier-center bounds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ConstellationSelection {
    pub requested_pairs: usize,
    pub profile_id: ProfileId,
    pub start_hz: Option<FrequencyHz>,
    pub stop_hz: Option<FrequencyHz>,
    /// Retain CRC-classified matched NCP cells using the same coordinate inverse.
    pub capture_ncp: bool,
}
impl Default for ConstellationSelection {
    /// Selects8192 observations across the supported payload profile.
    /// # Returns
    /// Bounded full-profile selection; collecting remains opt-in.
    fn default() -> Self {
        Self {
            requested_pairs: DEFAULT_CONSTELLATION_PAIRS,
            profile_id: PAYLOAD_PROFILE_ID,
            start_hz: None,
            stop_hz: None,
            capture_ncp: false,
        }
    }
}
impl ConstellationSelection {
    /// Checks count, supported profile and inclusive frequency bounds before a run.
    /// # Arguments
    /// * `layout` - Validated channel geometry; protected bins remain ineligible.
    /// # Returns
    /// Unit for a finite selection containing at least one static data carrier.
    /// # Errors
    /// Rejects unsupported profiles, invalid counts, partial/reversed/outside bounds or no data.
    pub fn validate(&self, layout: &CarrierLayout) -> Result<(), SimulationError> {
        if !(1..=MAX_CONSTELLATION_PAIRS).contains(&self.requested_pairs)
            || self.profile_id != PAYLOAD_PROFILE_ID
        {
            return Err(invalid(
                "Constellation requires1..65536 pairs and supported payload profile0.",
            ));
        }
        let data: Vec<_> = layout
            .static_roles()
            .iter()
            .enumerate()
            .filter_map(|(bin, role)| {
                (*role == PhysicalCarrierRole::Data)
                    .then(|| layout.frequency_hz(bin as u16))
                    .flatten()
            })
            .collect();
        let first = *data
            .first()
            .ok_or_else(|| invalid("No eligible constellation data carriers."))?;
        let last = *data
            .last()
            .ok_or_else(|| invalid("No eligible constellation data carriers."))?;
        match (self.start_hz, self.stop_hz) {
            (None, None) => Ok(()),
            (Some(start), Some(stop))
                if start <= stop
                    && start >= first
                    && stop <= last
                    && data
                        .iter()
                        .any(|frequency| (start..=stop).contains(frequency)) =>
            {
                Ok(())
            }
            _ => Err(invalid(
                "Constellation requires both inclusive bounds inside the data-carrier envelope.",
            )),
        }
    }
    /// Checks whether a carrier center falls inside the validated inclusive selection.
    /// # Arguments
    /// * `frequency` - Actual physical carrier center in Hz.
    /// # Returns
    /// True for unbounded selection or an included center.
    pub(super) fn contains(&self, frequency: FrequencyHz) -> bool {
        self.start_hz.is_none_or(|start| frequency >= start)
            && self.stop_hz.is_none_or(|stop| frequency <= stop)
    }
}
