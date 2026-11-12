/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared finite scientific observations; CLI and future presentation serialize these unchanged.
use super::ConstellationSelection;
use crate::types::{FrequencyHz, IqPair, ProfileId, SubcarrierIdx};
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
