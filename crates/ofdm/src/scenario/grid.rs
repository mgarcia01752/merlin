/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original carrier grid and exact spatial scenario identity.
use crate::{
    error::{invalid, SimulationError},
    types::{ChannelId, FrequencyHz, MacAddress, SubcarrierIdx},
};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};
/// Application safety limit, not an invented PHY or encoded-format limit.
pub const MAX_LAB_CARRIERS: usize = 65_536;
/// Grid origin/index and count retained exactly from source headers.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FrequencyGrid {
    pub zero_frequency_hz: FrequencyHz,
    pub first_subcarrier_index: SubcarrierIdx,
    pub spacing_hz: FrequencyHz,
    pub sample_count: usize,
}
impl FrequencyGrid {
    /// Validates bounds and the final original carrier center without resampling.
    /// # Returns
    /// Unit on success.
    /// # Errors
    /// Rejects empty/oversized, zero-spacing and overflowing frequency grids.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if self.sample_count == 0 || self.sample_count > MAX_LAB_CARRIERS || self.spacing_hz == 0 {
            return Err(invalid(
                "A bounded nonempty positive-spacing grid is required.",
            ));
        }
        self.frequency(self.sample_count - 1)?;
        Ok(())
    }
    /// Returns an original carrier center, never a decimated-bin center.
    /// # Arguments
    /// * `offset` - Zero-based offset within the captured grid.
    /// # Returns
    /// Frequency in the repository's u32 Hz domain.
    /// # Errors
    /// Rejects out-of-range positions and checked-arithmetic overflow.
    pub fn frequency(&self, offset: usize) -> Result<FrequencyHz, SimulationError> {
        if offset >= self.sample_count {
            return Err(invalid("Carrier offset is outside the source grid."));
        }
        let index = u32::from(self.first_subcarrier_index)
            .checked_add(u32::try_from(offset).map_err(|_| invalid("Carrier index overflow."))?)
            .ok_or_else(|| invalid("Carrier index overflow."))?;
        index
            .checked_mul(self.spacing_hz)
            .and_then(|offset_hz| self.zero_frequency_hz.checked_add(offset_hz))
            .ok_or_else(|| invalid("Frequency exceeds the supported Hz range."))
    }
}
/// Exact association identity. File format revisions remain source-specific.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScenarioKey {
    pub mac_address: MacAddress,
    pub channel_id: ChannelId,
    pub grid: FrequencyGrid,
}

impl RetainedPayload for FrequencyGrid {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            zero_frequency_hz,
            first_subcarrier_index,
            spacing_hz,
            sample_count,
        } = self;
        merlin_api::retention_support::sum([
            zero_frequency_hz.retained_children(),
            first_subcarrier_index.retained_children(),
            spacing_hz.retained_children(),
            sample_count.retained_children(),
        ])
    }
}

impl RetainedPayload for ScenarioKey {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            mac_address,
            channel_id,
            grid,
        } = self;
        merlin_api::retention_support::sum([
            mac_address.retained_children(),
            channel_id.retained_children(),
            grid.retained_children(),
        ])
    }
}
