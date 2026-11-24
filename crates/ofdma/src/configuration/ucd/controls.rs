/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Applicable MAP/P-MAP and per-CM assignment representation, without MAC execution.
use super::OfdmaChannel;
use crate::{error::invalid, PhyResult};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// One-CM assignment context; Annex C.1.5.1.11/12 and 10.5.1.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CmAssignment {
    /// Explicit CM identity, independent from a file name or a SID.
    pub cm_mac: [u8; 6],
    /// Registration state selects pre-registration IUC13 versus assigned profiles.
    pub registered: bool,
    /// One or two data IUCs after registration; empty before registration.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 2>")]
    pub assigned_iucs: Vec<u8>,
    /// Unicast ranging SID, 1..0x3dff; separate from a testing SID.
    pub ranging_sid: u16,
    /// Optional lower-14-bit OUDP testing SID, distinct from ordinary SID cluster.
    pub testing_sid: Option<u16>,
    /// Existing one-CM SID cluster used to reject a testing SID collision.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 256>")]
    pub sid_cluster: Vec<u16>,
}
impl CmAssignment {
    /// Checks assignment count, supported data IUCs and explicit SID references.
    ///
    /// Returns unit; no registration/DBC transaction or scheduler is executed.
    pub fn validate(&self, channel: &OfdmaChannel) -> PhyResult<()> {
        if self.cm_mac == [0; 6]
            || self.cm_mac[0] & 1 != 0
            || !unicast(self.ranging_sid)
            || self.sid_cluster.len() > 256
            || self.sid_cluster.iter().any(|sid| !unicast(*sid))
            || self.testing_sid.is_some_and(|sid| {
                !unicast(sid) || sid == self.ranging_sid || self.sid_cluster.contains(&sid)
            })
        {
            return Err(invalid("assignment.identities"));
        }
        let assigned: BTreeSet<_> = self.assigned_iucs.iter().copied().collect();
        if assigned.len() != self.assigned_iucs.len()
            || (self.registered && !(1..=2).contains(&assigned.len()))
            || (!self.registered && !assigned.is_empty())
            || assigned.iter().any(|iuc| {
                !super::data_iuc(*iuc) || !channel.bursts.iter().any(|burst| burst.iuc == *iuc)
            })
        {
            return Err(invalid("assignment.iucs"));
        }
        Ok(())
    }
    /// Checks whether selected data IUC is permitted by this frozen CM context.
    ///
    /// Returns true for IUC13 before registration or explicit assigned IUCs after it.
    pub fn permits(&self, iuc: u8) -> bool {
        if self.registered {
            self.assigned_iucs.contains(&iuc)
        } else {
            iuc == 13
        }
    }
}
fn unicast(sid: u16) -> bool {
    (1..0x3e00).contains(&sid)
}

/// One Table 32 interval including the explicitly represented expansion case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapElement {
    /// 14-bit SID, or expanded IUC value for IUC15.
    pub sid: u16,
    /// Four-bit Table 32 usage; 0,8,14 are reserved.
    pub iuc: u8,
    /// Fourteen-bit allocation offset, or number of additional words for IUC15.
    pub minislot_offset: u16,
    /// Exact additional expansion words only for IUC15; no invented semantics.
    #[serde(
        default,
        deserialize_with = "crate::configuration::serde_limits::optional_vec::<_, _, 490>"
    )]
    pub expansion_words: Option<Vec<u32>>,
}
/// Version-5 CAT0 MAP controls retained for inspection, not a synthetic scheduler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapControl {
    /// Version 5; version 1 applies to non-OFDMA channels.
    pub version: u8,
    /// Explicit UCD upstream channel identity.
    pub upstream_channel_id: u8,
    /// Explicit matching UCD configuration count.
    pub ucd_count: u8,
    /// Nine-bit source count, equal to elements.len(), at most 490.
    pub number_of_elements: u16,
    /// Three reserved bits, zero in this edition.
    pub reserved: u8,
    /// Four-bit CAT, exactly zero for this non-probe structure.
    pub channel_allocation_type: u8,
    /// Source 32-bit allocation start in minislots.
    pub allocation_start: u32,
    /// Source latest processed minislot count, not a sample index.
    pub acknowledgement_time: u32,
    /// Ranging contention exponent, 0..15.
    pub ranging_backoff_start: u8,
    /// Ranging contention final exponent, 0..15.
    pub ranging_backoff_end: u8,
    /// Request/data contention exponent, 0..15.
    pub data_backoff_start: u8,
    /// Request/data contention final exponent, 0..15.
    pub data_backoff_end: u8,
    /// Exact source-ordered information elements, including final null IE.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 490>")]
    pub elements: Vec<MapElement>,
}
impl MapControl {
    /// Validates source widths, UCD association, IUC/SID roles and ordered offsets.
    ///
    /// Returns unit; expansion words are retained but never interpreted as data.
    /// This validator does not implement contention, requests or MAC scheduling.
    pub fn validate(&self, channel: &OfdmaChannel) -> PhyResult<()> {
        if self.version != 5
            || self.upstream_channel_id != channel.upstream_channel_id
            || self.ucd_count != channel.configuration_change_count
            || self.reserved != 0
            || self.channel_allocation_type != 0
            || self.elements.len() > 490
            || usize::from(self.number_of_elements) != self.elements.len()
            || [
                self.ranging_backoff_start,
                self.ranging_backoff_end,
                self.data_backoff_start,
                self.data_backoff_end,
            ]
            .iter()
            .any(|value| *value > 15)
        {
            return Err(invalid("map.header"));
        }
        let mut previous = 0;
        for element in &self.elements {
            if element.sid > 0x3fff
                || element.minislot_offset > 0x3fff
                || !matches!(element.iuc,1..=7|9..=13|15)
            {
                return Err(invalid("map.element_widths"));
            }
            if element.iuc == 15 {
                if element.expansion_words.as_ref().is_none_or(|words| {
                    words.len() != usize::from(element.minislot_offset) || words.len() > 490
                }) {
                    return Err(invalid("map.expansion"));
                }
                continue;
            }
            if element.expansion_words.is_some() || element.minislot_offset < previous {
                return Err(invalid("map.offsets"));
            }
            previous = element.minislot_offset;
            let valid_sid = match element.iuc {
                1 => true,
                2 => element.sid == 0x3ff0,
                3 => unicast(element.sid) || element.sid == 0x3fff,
                4 => unicast(element.sid),
                7 => element.sid == 0,
                _ => unicast(element.sid),
            };
            if !valid_sid
                || (element.iuc != 7
                    && !channel.bursts.iter().any(|burst| burst.iuc == element.iuc))
            {
                return Err(invalid("map.iuc_sid"));
            }
        }
        if self.elements.last().is_some_and(|element| element.iuc != 7) {
            return Err(invalid("map.final_null"));
        }
        Ok(())
    }
}
/// Table 33 P-IE fields, each represented in its original domain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeElement {
    /// 14-bit unicast ranging SID.
    pub sid: u16,
    /// MER bit: CMTS measurement request, ignored by CM.
    pub measure_rxmer: bool,
    /// PW bit: alternate power using starting-subcarrier value when true.
    pub alternate_power: bool,
    /// EQ bit, true disables the transmitter equalizer (source polarity retained).
    pub equalization_disabled: bool,
    /// St bit: repeat skip+1 shifted patterns, possibly across a frame boundary.
    pub stagger: bool,
    /// Two-bit probe frame offset 0..3.
    pub frame: u8,
    /// Six-bit symbol offset, additionally restricted to 0..K-1.
    pub symbol: u8,
    /// Three-bit start; PW=false requires start<=skip; PW=true uses start%(skip+1).
    pub start_subcarrier: u8,
    /// Three-bit skipped bins between pilots, 0..7.
    pub subcarrier_skip: u8,
}
impl ProbeElement {
    /// Returns the source-defined relative power when PW is enabled.
    ///
    /// Values 0..7 give -2..-9 dB; normal power returns None, not a measured value.
    pub fn alternate_power_db(&self) -> PhyResult<Option<i8>> {
        if self.start_subcarrier > 7 {
            return Err(invalid("probe.start_subcarrier"));
        }
        Ok(self
            .alternate_power
            .then_some(-2 - self.start_subcarrier as i8))
    }
}
/// Version-5 CAT1 P-MAP inspection configuration, distinct from a waveform mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeControl {
    /// Version 5, required by OFDMA.
    pub version: u8,
    /// Explicit associated upstream channel.
    pub upstream_channel_id: u8,
    /// Matching accepted UCD change count.
    pub ucd_count: u8,
    /// Nine-bit source count, equal to elements.len(), maximum 128.
    pub number_of_elements: u16,
    /// Three reserved bits, zero.
    pub reserved: u8,
    /// Four-bit CAT, exactly one for probes.
    pub channel_allocation_type: u8,
    /// First probe-frame minislot count, never inferred from wall-clock time.
    pub allocation_start: u32,
    /// Source time/subcarrier ordering, at most K per one-CM channel.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 128>")]
    pub elements: Vec<ProbeElement>,
}
impl ProbeControl {
    /// Validates all Table 33 widths, source order, PW and one-CM outstanding limit.
    ///
    /// Returns unit; stagger may cross frames, and unallocated symbols stay absent.
    /// No probe/RxMER capture or live ranging exchange is executed.
    pub fn validate(&self, channel: &OfdmaChannel, assignment: &CmAssignment) -> PhyResult<()> {
        if self.version != 5
            || self.upstream_channel_id != channel.upstream_channel_id
            || self.ucd_count != channel.configuration_change_count
            || self.reserved != 0
            || self.channel_allocation_type != 1
            || self.elements.len() > 128
            || usize::from(self.number_of_elements) != self.elements.len()
            || self.elements.len() > channel.frame_symbols
        {
            return Err(invalid("probe.header"));
        }
        let mut previous = None;
        for element in &self.elements {
            if element.sid != assignment.ranging_sid
                || element.frame > 3
                || usize::from(element.symbol) >= channel.frame_symbols
                || element.start_subcarrier > 7
                || element.subcarrier_skip > 7
                || (!element.alternate_power && element.start_subcarrier > element.subcarrier_skip)
            {
                return Err(invalid("probe.element"));
            }
            let actual_start = element.start_subcarrier % (element.subcarrier_skip + 1);
            let position = (element.frame, element.symbol, actual_start);
            if previous.is_some_and(|old| old > position) {
                return Err(invalid("probe.order"));
            }
            previous = Some(position);
        }
        Ok(())
    }
}

impl RetainedPayload for CmAssignment {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            cm_mac,
            registered,
            assigned_iucs,
            ranging_sid,
            testing_sid,
            sid_cluster,
        } = self;
        sum([
            cm_mac.retained_children(),
            registered.retained_children(),
            assigned_iucs.retained_children(),
            ranging_sid.retained_children(),
            testing_sid.retained_children(),
            sid_cluster.retained_children(),
        ])
    }
}

impl RetainedPayload for MapElement {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            sid,
            iuc,
            minislot_offset,
            expansion_words,
        } = self;
        sum([
            sid.retained_children(),
            iuc.retained_children(),
            minislot_offset.retained_children(),
            expansion_words.retained_children(),
        ])
    }
}

impl RetainedPayload for MapControl {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            version,
            upstream_channel_id,
            ucd_count,
            number_of_elements,
            reserved,
            channel_allocation_type,
            allocation_start,
            acknowledgement_time,
            ranging_backoff_start,
            ranging_backoff_end,
            data_backoff_start,
            data_backoff_end,
            elements,
        } = self;
        sum([
            version.retained_children(),
            upstream_channel_id.retained_children(),
            ucd_count.retained_children(),
            number_of_elements.retained_children(),
            reserved.retained_children(),
            channel_allocation_type.retained_children(),
            allocation_start.retained_children(),
            acknowledgement_time.retained_children(),
            ranging_backoff_start.retained_children(),
            ranging_backoff_end.retained_children(),
            data_backoff_start.retained_children(),
            data_backoff_end.retained_children(),
            elements.retained_children(),
        ])
    }
}

impl RetainedPayload for ProbeElement {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            sid,
            measure_rxmer,
            alternate_power,
            equalization_disabled,
            stagger,
            frame,
            symbol,
            start_subcarrier,
            subcarrier_skip,
        } = self;
        sum([
            sid.retained_children(),
            measure_rxmer.retained_children(),
            alternate_power.retained_children(),
            equalization_disabled.retained_children(),
            stagger.retained_children(),
            frame.retained_children(),
            symbol.retained_children(),
            start_subcarrier.retained_children(),
            subcarrier_skip.retained_children(),
        ])
    }
}

impl RetainedPayload for ProbeControl {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            version,
            upstream_channel_id,
            ucd_count,
            number_of_elements,
            reserved,
            channel_allocation_type,
            allocation_start,
            elements,
        } = self;
        sum([
            version.retained_children(),
            upstream_channel_id.retained_children(),
            ucd_count.retained_children(),
            number_of_elements.retained_children(),
            reserved.retained_children(),
            channel_allocation_type.retained_children(),
            allocation_start.retained_children(),
            elements.retained_children(),
        ])
    }
}
