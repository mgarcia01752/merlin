/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual received decisions and intermediate soft values, independent of TX truth.
use crate::{transmission::ncp::DecodedNcpField, types::ProfileId, SimulationError};
use fec_codec::concatenated::DecodedBlock;
/// One control-hypothesis cell, preserving ascending data-position identity despite reversed field order.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoftNcpCell {
    pub data_position: usize,
    pub llrs: Vec<f64>,
}
/// Actual payload cell after NCP selection/inverse randomization; zero loading has no LLRs.
#[derive(Debug, Clone, serde::Serialize)]
pub struct SoftDataCell {
    pub data_position: usize,
    pub llrs: Vec<f64>,
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
    /// Present for opt-in boundary-discovered transport, including its full-word case.
    pub shortening: Option<fec_codec::shortening::Shortening>,
    pub known_zeros_consistent: bool,
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
    pub error: Option<SimulationError>,
}

impl RecoveredPayload {
    /// Borrows original-length payload decisions, including failures, without parity.
    /// # Returns
    /// K received information bits; default legacy words retain14232-bit behavior.
    /// # Errors
    /// Rejects caller-replaced malformed public BCH decision storage.
    pub fn payload(&self) -> Result<&[u8], SimulationError> {
        let omitted = self
            .shortening
            .map_or(0, |geometry| geometry.omitted_bits());
        self.decoded
            .bch
            .codeword
            .get(omitted..fec_codec::parameters::BCH_INPUT_BITS)
            .ok_or_else(|| crate::error::invalid("Malformed retained BCH payload decisions."))
    }
}
