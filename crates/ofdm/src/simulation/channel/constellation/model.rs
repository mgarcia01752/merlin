/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared finite scientific observations; CLI and future presentation serialize these unchanged.
use super::ConstellationSelection;
use crate::types::{FrequencyHz, IqPair, ProfileId, SubcarrierIdx};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::Serialize;
/// Actual receive-stage samples for the same physical cell, before any display normalization.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConstellationPair {
    pub channel_symbol_index: u64,
    pub source_symbol_index: u64,
    pub physical_bin: SubcarrierIdx,
    pub frequency_hz: FrequencyHz,
    pub data_position: usize,
    pub profile_id: ProfileId,
    pub modulation_order: u16,
    pub before_equalization: IqPair,
    pub after_equalization: IqPair,
    /// Matched physical TX symbol for diagnostics only; never an RX input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_iq: Option<IqPair>,
    /// Actual post-descrambling receiver cell LLRs, before inverse bit processing.
    pub llrs: Vec<f64>,
}
/// One CRC-classified NCP cell with matched physical receive coordinates.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NcpConstellationPair {
    pub channel_symbol_index: u64,
    pub source_symbol_index: u64,
    pub physical_bin: SubcarrierIdx,
    pub frequency_hz: FrequencyHz,
    pub data_position: usize,
    pub modulation_order: u16,
    pub before_equalization: IqPair,
    pub after_equalization: IqPair,
    /// Matched physical TX symbol for diagnostics only; never an RX input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_iq: Option<IqPair>,
    /// Actual NCP-hypothesis LLRs for the accepted field, after descrambling.
    pub llrs: Vec<f64>,
}
/// Capture completion is independent of the enclosing scientific job/FEC status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstellationStatus {
    Unavailable,
    Partial,
    Complete,
}
/// Bounded paired data plus its selection and explicit scientific reference plane.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConstellationResult {
    pub schema_version: u16,
    pub selection: ConstellationSelection,
    pub status: ConstellationStatus,
    pub collected_pairs: usize,
    pub reference: &'static str,
    pub channel_coefficients: &'static str,
    pub pairs: Vec<ConstellationPair>,
    /// Empty unless explicitly enabled; bounded independently of payload retention.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ncp_pairs: Vec<NcpConstellationPair>,
}

impl RetainedPayload for ConstellationResult {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            schema_version,
            selection,
            status,
            collected_pairs,
            reference: _,
            channel_coefficients: _,
            pairs,
            ncp_pairs,
        } = self;
        merlin_api::retention_support::sum([
            schema_version.retained_children(),
            selection.retained_children(),
            status.retained_children(),
            collected_pairs.retained_children(),
            pairs.retained_children(),
            ncp_pairs.retained_children(),
        ])
    }
}

impl RetainedPayload for ConstellationPair {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            channel_symbol_index,
            source_symbol_index,
            physical_bin,
            frequency_hz,
            data_position,
            profile_id,
            modulation_order,
            before_equalization,
            after_equalization,
            reference_iq,
            llrs,
        } = self;
        merlin_api::retention_support::sum([
            channel_symbol_index.retained_children(),
            source_symbol_index.retained_children(),
            physical_bin.retained_children(),
            frequency_hz.retained_children(),
            data_position.retained_children(),
            profile_id.retained_children(),
            modulation_order.retained_children(),
            before_equalization.retained_children(),
            after_equalization.retained_children(),
            reference_iq.retained_children(),
            llrs.retained_children(),
        ])
    }
}

impl RetainedPayload for NcpConstellationPair {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            channel_symbol_index,
            source_symbol_index,
            physical_bin,
            frequency_hz,
            data_position,
            modulation_order,
            before_equalization,
            after_equalization,
            reference_iq,
            llrs,
        } = self;
        merlin_api::retention_support::sum([
            channel_symbol_index.retained_children(),
            source_symbol_index.retained_children(),
            physical_bin.retained_children(),
            frequency_hz.retained_children(),
            data_position.retained_children(),
            modulation_order.retained_children(),
            before_equalization.retained_children(),
            after_equalization.retained_children(),
            reference_iq.retained_children(),
            llrs.retained_children(),
        ])
    }
}

impl RetainedPayload for ConstellationStatus {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::Unavailable => Some(RetainedFootprint::default()),
            Self::Partial => Some(RetainedFootprint::default()),
            Self::Complete => Some(RetainedFootprint::default()),
        }
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use super::{
        ConstellationPair, ConstellationResult, ConstellationStatus, NcpConstellationPair,
    };
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn constellation_counts_both_pair_buffers_and_each_actual_llr_capacity() {
        // Storage fixture only: the numerical values do not claim demapping validity.
        const PAYLOAD_PAIR_SLOTS: usize = 3;
        const NCP_PAIR_SLOTS: usize = 5;
        const PAYLOAD_LLR_SLOTS: usize = 7;
        const NCP_LLR_SLOTS: usize = 11;
        let mut pairs = Vec::with_capacity(PAYLOAD_PAIR_SLOTS);
        let payload_llrs = Vec::with_capacity(PAYLOAD_LLR_SLOTS);
        let mut ncp_pairs = Vec::with_capacity(NCP_PAIR_SLOTS);
        let ncp_llrs = Vec::with_capacity(NCP_LLR_SLOTS);
        let llr_bytes = (payload_llrs.capacity() + ncp_llrs.capacity()) * size_of::<f64>();
        pairs.push(ConstellationPair {
            channel_symbol_index: 0,
            source_symbol_index: 0,
            physical_bin: 0,
            frequency_hz: 0,
            data_position: 0,
            profile_id: 0,
            modulation_order: 2, // BPSK label; this fixture runs no receiver.
            before_equalization: [0.0; 2],
            after_equalization: [0.0; 2],
            reference_iq: None,
            llrs: payload_llrs,
        });
        ncp_pairs.push(NcpConstellationPair {
            channel_symbol_index: 0,
            source_symbol_index: 0,
            physical_bin: 0,
            frequency_hz: 0,
            data_position: 0,
            modulation_order: 4, // QPSK label; this fixture runs no receiver.
            before_equalization: [0.0; 2],
            after_equalization: [0.0; 2],
            reference_iq: None,
            llrs: ncp_llrs,
        });
        let expected = size_of::<ConstellationResult>()
            + pairs.capacity() * size_of::<ConstellationPair>()
            + ncp_pairs.capacity() * size_of::<NcpConstellationPair>()
            + llr_bytes;
        let report = ConstellationResult {
            schema_version: 1,
            selection: Default::default(),
            status: ConstellationStatus::Partial,
            collected_pairs: pairs.len(),
            reference: "borrowed static fixture reference",
            channel_coefficients: "borrowed static fixture coefficients",
            pairs,
            ncp_pairs,
        };
        let serialized = serde_json::to_value(&report).unwrap();
        let footprint = retained_footprint(&report).unwrap();
        assert_eq!(footprint.payload_bytes, expected);
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(footprint.shared_roots, 0);
        assert_eq!(serde_json::to_value(&report).unwrap(), serialized);
    }
}
