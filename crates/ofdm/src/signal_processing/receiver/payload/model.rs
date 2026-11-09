/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual received decisions and intermediate soft values, independent of TX truth.
use crate::{transmission::ncp::DecodedNcpField, types::ProfileId, LabError};
use fec_codec::concatenated::DecodedBlock;
/// One control-hypothesis cell, preserving ascending data-position identity despite reversed field order.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoftNcpCell {
    pub data_position: usize,
    pub llrs: Vec<f64>,
}
/// Actual payload cell after accepted NCP selection and inverse randomization.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoftDataCell {
    pub data_position: usize,
    pub llrs: [f64; crate::transmission::symbols::PAYLOAD_BITS],
}
/// Complete recovered short8/9 word with genuine decoder flags/iterations and no TX ID.
pub struct RecoveredPayload {
    pub receive_sequence: u64,
    pub profile_id: ProfileId,
    pub first_source_symbol: u64,
    pub last_source_symbol: u64,
    pub cell_llrs: Vec<f64>,
    pub ldpc_llrs: Vec<f64>,
    pub decoded: DecodedBlock,
}
/// Completed source-symbol processing, including a terminal rejection diagnostic.
pub struct PayloadSymbolResult {
    pub source_symbol_index: u64,
    pub ncp: Option<DecodedNcpField>,
    /// High-to-low hypothesis order; each record retains its original data-position rank.
    pub ncp_soft: Vec<SoftNcpCell>,
    pub data_soft: Vec<SoftDataCell>,
    pub recovered: Vec<RecoveredPayload>,
    pub pending_payload_bits: usize,
    pub error: Option<LabError>,
}
