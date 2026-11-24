/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Table 28 OFDMA-applicable fields; non-OFDMA rows are explicitly out of domain.
use super::BurstDescriptor;
use crate::{
    configuration::{Minislot, Numerology, Profile, SyntheticGrant, UsConfiguration, Waveform},
    error::invalid,
    PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Inclusive physical carrier band exactly matching source start/end fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Band {
    /// Inclusive first physical index (u16 source field).
    pub first: u16,
    /// Inclusive last physical index; first==last represents one carrier.
    pub last: u16,
}
/// The 9-byte OFDMA timestamp snapshot, unpacked without invented timing policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimestampSnapshot {
    /// Upper four reserved bits; zero in this pin.
    pub reserved: u8,
    /// DOCSIS 3.0-corresponding 32-bit timestamp portion.
    pub docsis_timestamp: u32,
    /// Four most-significant divide-by-20 bits, 0..15.
    pub divide_by_20_msb: u8,
    /// Lowest-frequency minislot count at the sampled frame boundary.
    pub minislot_count: u32,
}
/// Generic source configuration, independent of RF truth and simulation budgets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OfdmaChannel {
    /// MAC management message type 51; other UCD families are inapplicable.
    pub message_type: u8,
    /// DOCSIS 3.1 management version 5.
    pub management_version: u8,
    /// Nonzero upstream channel identifier, distinct from a CCAP ifIndex.
    pub upstream_channel_id: u8,
    /// Source modulo-256 configuration count; snapshot changes alone do not increment.
    pub configuration_change_count: u8,
    /// Retained one-byte fixed field; has no OFDMA timing effect.
    pub minislot_size_ticks: u8,
    /// Nonzero downstream management-channel association, not an OFDM profile ID.
    pub downstream_channel_id: u8,
    /// Channel TLV 3, MSB-first superstring bytes, optional outside ranging.
    #[serde(
        default,
        deserialize_with = "crate::configuration::serde_limits::optional_vec::<_, _, 128>"
    )]
    pub preamble_pattern: Option<Vec<u8>>,
    /// Channel TLV 16, 0..3; omission means no ranging required.
    pub ranging_required: Option<u8>,
    /// Channel TLV 18: classes in bits 0..4, extension 16..31; 5..15 reserved.
    pub ranging_holdoff_priority: Option<u32>,
    /// Channel TLV 19: same class/extension domains; bits 5..15 reserved.
    pub channel_class_id: Option<u32>,
    /// Required TLV 24; only low 11 change indicators are defined.
    pub change_indicator_bitmask: u16,
    /// Required TLV 25 snapshot, distinct from simulator sample epoch.
    pub timestamp_snapshot: TimestampSnapshot,
    /// TLV 26 decoded CP sample count at 102.4 MHz.
    pub cyclic_prefix_samples: usize,
    /// TLV 27 decoded RP sample count, except separate initial-ranging waveform.
    pub rolloff_samples: usize,
    /// TLV 28 decoded 25 or 50 kHz spacing, not the wire enum number.
    pub subcarrier_spacing_khz: u8,
    /// TLV 29, physical carrier-zero center frequency in Hz, spacing aligned.
    pub subcarrier_zero_hz: u32,
    /// TLV 30 inclusive exclusion bands, including mandatory outer guards.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 63>")]
    pub excluded_bands: Vec<Band>,
    /// TLV 31 inclusive unused bands; all remaining bins map unambiguously to slots.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 63>")]
    pub unused_bands: Vec<Band>,
    /// TLV 32, integer physical-symbol count subject to PHY bandwidth limits.
    pub frame_symbols: usize,
    /// TLV 33, right-justified raw 23-bit channel randomization register seed.
    pub randomization_seed: u32,
    /// Required physical geometry input from PHY; not a separate Table 28 TLV.
    pub encompassed_bandwidth_hz: u32,
    /// Ordered Type-23 descriptor inventory; all IUC identities stay discoverable.
    #[serde(deserialize_with = "crate::configuration::serde_limits::bounded_vec::<_, _, 11>")]
    pub bursts: Vec<BurstDescriptor>,
}
impl OfdmaChannel {
    /// Validates all source/domain fields and derives the exact physical slot map.
    ///
    /// Returns first-bin indices without packing holes; rejects reserved fields,
    /// ambiguous/overlapping bands, missing IUC13, conditional attributes and PHY limits.
    pub fn validate(&self) -> PhyResult<Vec<usize>> {
        if self.message_type != 51
            || self.management_version != 5
            || self.upstream_channel_id == 0
            || self.downstream_channel_id == 0
            || self.ranging_required.is_some_and(|value| value > 3)
            || self.change_indicator_bitmask & !0x7ff != 0
            || self
                .ranging_holdoff_priority
                .is_some_and(|mask| mask & 0x0000_ffe0 != 0)
            || self
                .channel_class_id
                .is_some_and(|mask| mask & 0x0000_ffe0 != 0)
            || self.timestamp_snapshot.reserved != 0
            || self.timestamp_snapshot.divide_by_20_msb > 15
            || self.randomization_seed > 0x7fffff
            || self
                .preamble_pattern
                .as_ref()
                .is_some_and(|value| value.is_empty() || value.len() > 128)
        {
            return Err(invalid("ucd.header_or_channel"));
        }
        let geometry = self.waveform()?;
        let mode = geometry.numerology();
        let n = mode.fft_len();
        let q = mode.carriers_per_minislot();
        let spacing = u32::from(self.subcarrier_spacing_khz) * 1000;
        if self.subcarrier_zero_hz % spacing != 0 {
            return Err(invalid("ucd.frequency_alignment"));
        }
        let excluded = mask(&self.excluded_bands, n)?;
        let unused = mask(&self.unused_bands, n)?;
        let guard = if n == 2048 { 74 } else { 148 };
        if (0..guard).chain(n - guard..n).any(|i| !excluded[i])
            || (0..n).any(|i| excluded[i] && unused[i])
        {
            return Err(invalid("ucd.exclusion_or_unused"));
        }
        let mut slots = Vec::new();
        let mut bin = 0;
        while bin < n {
            if excluded[bin] || unused[bin] {
                bin += 1;
                continue;
            }
            if bin + q > n || (bin..bin + q).any(|i| excluded[i] || unused[i]) {
                return Err(invalid("ucd.ambiguous_minislot"));
            }
            slots.push(bin);
            bin += q;
        }
        if slots.is_empty() || slots.len() > 237 {
            return Err(invalid("ucd.minislots"));
        }
        let first = excluded
            .iter()
            .position(|x| !*x)
            .ok_or_else(|| invalid("ucd.encompassed"))?;
        let last = excluded
            .iter()
            .rposition(|x| !*x)
            .ok_or_else(|| invalid("ucd.encompassed"))?;
        if (last - first + 1) as u64 * u64::from(spacing) > u64::from(self.encompassed_bandwidth_hz)
        {
            return Err(invalid("ucd.encompassed_bandwidth"));
        }
        if self.bursts.is_empty() || self.bursts.len() > 11 {
            return Err(invalid("ucd.bursts"));
        }
        let mut iucs = BTreeSet::new();
        let bits = self
            .preamble_pattern
            .as_ref()
            .map_or(0, |bytes| bytes.len() * 8);
        for burst in &self.bursts {
            if !iucs.insert(burst.iuc) {
                return Err(invalid("ucd.duplicate_iuc"));
            }
            burst.validate(q, bits, slots.len())?;
        }
        if !iucs.contains(&13) {
            return Err(invalid("ucd.required_iuc13"));
        }
        // Shared PHY admission additionally enforces channel-wide pilot family.
        self.project_internal(
            SyntheticGrant {
                id: 0,
                profile_id: 13,
                epoch_sample: 0,
                first_frame: 0,
                frames: 1,
                minislots: vec![0],
            },
            &slots,
            &excluded,
        )?;
        Ok(slots)
    }
    /// Builds source-checked shared PHY geometry without simulation execution.
    ///
    /// Returns a waveform or a structured spacing/CP/RP/frame/bandwidth error.
    pub fn waveform(&self) -> PhyResult<Waveform> {
        let mode = match self.subcarrier_spacing_khz {
            25 => Numerology::Khz25,
            50 => Numerology::Khz50,
            _ => return Err(invalid("ucd.spacing")),
        };
        Waveform::new(
            mode,
            self.cyclic_prefix_samples,
            self.rolloff_samples,
            self.frame_symbols,
            self.encompassed_bandwidth_hz,
        )
    }
    /// Projects one explicit synthetic `grant` through all shared admission rules.
    ///
    /// Returns legitimate immutable endpoint configuration; selected non-data IUCs
    /// fail before any source consumption. Profile IDs equal their normative IUCs.
    pub fn project(&self, grant: SyntheticGrant) -> PhyResult<UsConfiguration> {
        let slots = self.validate()?;
        let excluded = mask(
            &self.excluded_bands,
            self.waveform()?.numerology().fft_len(),
        )?;
        self.project_internal(grant, &slots, &excluded)
    }
    fn project_internal(
        &self,
        grant: SyntheticGrant,
        slots: &[usize],
        excluded: &[bool],
    ) -> PhyResult<UsConfiguration> {
        let mut profiles = Vec::new();
        for burst in &self.bursts {
            if let Some(groups) = &burst.data_profile {
                let mut cells = Vec::new();
                let mut index = 0;
                for group in groups {
                    for _ in 0..=group.additional_minislots {
                        cells.push(Minislot {
                            first_carrier: slots[index],
                            loading: group.modulation_index,
                            pilot_pattern: group.pilot_pattern,
                        });
                        index += 1;
                    }
                }
                profiles.push(Profile::new(
                    u16::from(burst.iuc),
                    burst.iuc,
                    self.randomization_seed,
                    cells,
                )?);
            }
        }
        UsConfiguration::new(
            self.waveform()?,
            profiles,
            grant,
            excluded
                .iter()
                .enumerate()
                .filter_map(|(i, yes)| yes.then_some(i))
                .collect(),
        )
    }
}
fn mask(bands: &[Band], n: usize) -> PhyResult<Vec<bool>> {
    if bands.len() > 63 {
        return Err(invalid("ucd.band_count"));
    }
    let mut mask = vec![false; n];
    for band in bands {
        let first = usize::from(band.first);
        let last = usize::from(band.last);
        if first > last || last >= n || mask[first..=last].iter().any(|set| *set) {
            return Err(invalid("ucd.band_range"));
        }
        mask[first..=last].fill(true);
    }
    Ok(mask)
}

impl RetainedPayload for Band {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self { first, last } = self;
        sum([first.retained_children(), last.retained_children()])
    }
}

impl RetainedPayload for TimestampSnapshot {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            reserved,
            docsis_timestamp,
            divide_by_20_msb,
            minislot_count,
        } = self;
        sum([
            reserved.retained_children(),
            docsis_timestamp.retained_children(),
            divide_by_20_msb.retained_children(),
            minislot_count.retained_children(),
        ])
    }
}

impl RetainedPayload for OfdmaChannel {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            message_type,
            management_version,
            upstream_channel_id,
            configuration_change_count,
            minislot_size_ticks,
            downstream_channel_id,
            preamble_pattern,
            ranging_required,
            ranging_holdoff_priority,
            channel_class_id,
            change_indicator_bitmask,
            timestamp_snapshot,
            cyclic_prefix_samples,
            rolloff_samples,
            subcarrier_spacing_khz,
            subcarrier_zero_hz,
            excluded_bands,
            unused_bands,
            frame_symbols,
            randomization_seed,
            encompassed_bandwidth_hz,
            bursts,
        } = self;
        sum([
            message_type.retained_children(),
            management_version.retained_children(),
            upstream_channel_id.retained_children(),
            configuration_change_count.retained_children(),
            minislot_size_ticks.retained_children(),
            downstream_channel_id.retained_children(),
            preamble_pattern.retained_children(),
            ranging_required.retained_children(),
            ranging_holdoff_priority.retained_children(),
            channel_class_id.retained_children(),
            change_indicator_bitmask.retained_children(),
            timestamp_snapshot.retained_children(),
            cyclic_prefix_samples.retained_children(),
            rolloff_samples.retained_children(),
            subcarrier_spacing_khz.retained_children(),
            subcarrier_zero_hz.retained_children(),
            excluded_bands.retained_children(),
            unused_bands.retained_children(),
            frame_symbols.retained_children(),
            randomization_seed.retained_children(),
            encompassed_bandwidth_hz.retained_children(),
            bursts.retained_children(),
        ])
    }
}
