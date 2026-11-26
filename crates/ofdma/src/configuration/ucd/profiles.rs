/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Table 29 applicable rows and 6.4.3.4 required per-IUC attributes.
use crate::{error::invalid, PhyResult};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};

/// One source Table 29.21 two-byte group, not a vendor profile template.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileGroup {
    /// High nibble: 0 or 2..12; 1 and 13..15 are reserved.
    pub modulation_index: u8,
    /// Low nibble: zero iff no loading; otherwise numerology-specific 1..14.
    pub pilot_pattern: u8,
    /// Second byte, additional identical minislots after the first (0..255).
    pub additional_minislots: u8,
}
/// Optional broadcast initial-ranging power control (23.22), inspection only.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IrPowerControl {
    /// Unsigned quarter-dBmV per 1.6 MHz starting level.
    pub starting_power_quarter_dbmv: u8,
    /// Unsigned quarter-dB retry increment.
    pub step_quarter_db: u8,
}
/// Type-23 descriptor with every applicable source attribute represented.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BurstDescriptor {
    /// Low four-bit IUC; 1/2, 3, 4 and data IUCs have distinct attribute rules.
    pub iuc: u8,
    /// Type 3, bits, at most 512; required only for IUC 3/4.
    pub preamble_length_bits: Option<u16>,
    /// Type 4, bit offset into the MSB-first preamble superstring.
    pub preamble_offset_bits: Option<u16>,
    /// Type 19, positive even Nir; required only for IUC 3.
    pub initial_ranging_subcarriers: Option<u16>,
    /// Type 20, positive even Nfr; required only for IUC 4.
    pub fine_ranging_subcarriers: Option<u16>,
    /// Type 21, 1..126 consecutive groups, required only for data IUCs.
    #[serde(
        default,
        deserialize_with = "crate::configuration::serde_limits::optional_vec::<_, _, 126>"
    )]
    pub data_profile: Option<Vec<ProfileGroup>>,
    /// Type 22, optional for IUC 3 only, never scheduled-data settings.
    pub ir_power_control: Option<IrPowerControl>,
}
impl BurstDescriptor {
    pub(crate) fn validate(&self, q: usize, preamble_bits: usize, slots: usize) -> PhyResult<()> {
        if super::data_iuc(self.iuc) {
            if self.preamble_length_bits.is_some()
                || self.preamble_offset_bits.is_some()
                || self.initial_ranging_subcarriers.is_some()
                || self.fine_ranging_subcarriers.is_some()
                || self.ir_power_control.is_some()
            {
                return Err(invalid("ucd.data_attributes"));
            }
            let groups = self
                .data_profile
                .as_ref()
                .ok_or_else(|| invalid("ucd.data_profile"))?;
            if groups.is_empty() || groups.len() > 126 {
                return Err(invalid("ucd.profile_groups"));
            }
            let mut count = 0_usize;
            for group in groups {
                if group.modulation_index == 0 {
                    if group.pilot_pattern != 0 {
                        return Err(invalid("ucd.zero_pattern"));
                    }
                } else if !(2..=12).contains(&group.modulation_index)
                    || !(if q == 8 { 1..=7 } else { 8..=14 }).contains(&group.pilot_pattern)
                {
                    return Err(invalid("ucd.group_loading_pattern"));
                }
                count += 1 + usize::from(group.additional_minislots);
            }
            if count != slots || count > 237 {
                return Err(invalid("ucd.group_coverage"));
            }
        } else if matches!(self.iuc, 3 | 4) {
            let length = usize::from(
                self.preamble_length_bits
                    .ok_or_else(|| invalid("ucd.preamble_length"))?,
            );
            let offset = usize::from(
                self.preamble_offset_bits
                    .ok_or_else(|| invalid("ucd.preamble_offset"))?,
            );
            if self.data_profile.is_some()
                || length == 0
                || length > 512
                || offset + length > preamble_bits
            {
                return Err(invalid("ucd.ranging_preamble"));
            }
            if self.iuc == 3 {
                let nir = usize::from(
                    self.initial_ranging_subcarriers
                        .ok_or_else(|| invalid("ucd.nir"))?,
                );
                let maximum = if q == 8 { 64 } else { 128 };
                if self.fine_ranging_subcarriers.is_some()
                    || nir == 0
                    || nir % 2 != 0
                    || nir > maximum
                    || length % nir != 0
                    || !(1..=8).contains(&(length / nir))
                {
                    return Err(invalid("ucd.nir"));
                }
            } else {
                let nfr = usize::from(
                    self.fine_ranging_subcarriers
                        .ok_or_else(|| invalid("ucd.nfr"))?,
                );
                let maximum = if q == 8 { 256 } else { 512 };
                if self.initial_ranging_subcarriers.is_some()
                    || self.ir_power_control.is_some()
                    || nfr == 0
                    || nfr % 2 != 0
                    || nfr > maximum
                    || length != nfr
                    || offset % 2 != 0
                {
                    return Err(invalid("ucd.nfr"));
                }
            }
        } else if !matches!(self.iuc, 1 | 2)
            || self.preamble_length_bits.is_some()
            || self.preamble_offset_bits.is_some()
            || self.initial_ranging_subcarriers.is_some()
            || self.fine_ranging_subcarriers.is_some()
            || self.data_profile.is_some()
            || self.ir_power_control.is_some()
        {
            return Err(invalid("ucd.burst_iuc"));
        }
        Ok(())
    }
}

impl RetainedPayload for ProfileGroup {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            modulation_index,
            pilot_pattern,
            additional_minislots,
        } = self;
        sum([
            modulation_index.retained_children(),
            pilot_pattern.retained_children(),
            additional_minislots.retained_children(),
        ])
    }
}

impl RetainedPayload for IrPowerControl {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            starting_power_quarter_dbmv,
            step_quarter_db,
        } = self;
        sum([
            starting_power_quarter_dbmv.retained_children(),
            step_quarter_db.retained_children(),
        ])
    }
}

impl RetainedPayload for BurstDescriptor {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            iuc,
            preamble_length_bits,
            preamble_offset_bits,
            initial_ranging_subcarriers,
            fine_ranging_subcarriers,
            data_profile,
            ir_power_control,
        } = self;
        sum([
            iuc.retained_children(),
            preamble_length_bits.retained_children(),
            preamble_offset_bits.retained_children(),
            initial_ranging_subcarriers.retained_children(),
            fine_ranging_subcarriers.retained_children(),
            data_profile.retained_children(),
            ir_power_control.retained_children(),
        ])
    }
}
