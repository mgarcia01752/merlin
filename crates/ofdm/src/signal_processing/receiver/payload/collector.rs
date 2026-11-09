/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded cross-symbol codeword collection, driven only by received NCP spans.
use super::model::{RecoveredPayload, SoftDataCell};
use crate::{
    error::invalid, signal_processing::modulation::interleaving::mixed::PayloadBitOrder,
    transmission::ncp::NcpDataSpan, SimulationError,
};
use fec_codec::concatenated::ConcatenatedCodec;
/// At most one pending word; completed results are returned to the caller, never retained in history.
pub struct PayloadCollector {
    codec: ConcatenatedCodec,
    bits: PayloadBitOrder,
    pending: Vec<f64>,
    first_source: Option<u64>,
    next_sequence: u64,
}
impl PayloadCollector {
    /// Builds the supported256-QAM short8/9 collector/decoder.
    /// # Returns
    /// Independent codec and empty pending history; no source truth is accepted.
    /// # Errors
    /// Propagates codec/permutation setup failure.
    pub fn new() -> Result<Self, SimulationError> {
        Ok(Self {
            codec: ConcatenatedCodec::new().map_err(invalid)?,
            bits: PayloadBitOrder::new(&[8])?,
            pending: Vec::new(),
            first_source: None,
            next_sequence: 0,
        })
    }
    /// Creates a collector using the coding rules of an independently received profile.
    /// # Arguments
    /// * `loading` - Valid source-rank data profile; zeros do not imply mixed loading.
    /// # Returns
    /// Empty pending history and the corresponding uniform/mixed bit permutation.
    /// # Errors
    /// Rejects unsupported loading or codec initialization failures.
    pub fn with_bit_loading(loading: &[u8]) -> Result<Self, SimulationError> {
        let mut collector = Self::new()?;
        collector.bits = PayloadBitOrder::new(loading)?;
        Ok(collector)
    }
    /// Reports incomplete received coded bits.
    /// # Returns
    /// Zero to the selected padded bit count (at most16212); incomplete bits are not payload.
    pub fn pending_bits(&self) -> usize {
        self.pending.len()
    }
    /// Collects one received NCP span and decodes only a complete word.
    /// # Arguments
    /// * `span` - CRC-accepted pointer interval; profile0 is the supported data mode.
    /// * `cells` - Actual inverse-randomized cells for exactly this interval; empty for filler.
    /// * `source_symbol` - Original pre-interleaving absolute symbol identity.
    /// * `max_iterations` - Positive payload LDPC work bound, at most100.
    /// # Returns
    /// A completed word with actual LDPC/BCH outcomes, or None for continuation/filler.
    /// # Errors
    /// Rejects orphan continuation, premature start/filler, unsupported profile,
    /// inconsistent interval or unannounced extra data. A caller closes the session after failure.
    pub fn accept_span(
        &mut self,
        span: &NcpDataSpan,
        cells: &[SoftDataCell],
        source_symbol: u64,
        max_iterations: usize,
    ) -> Result<Option<RecoveredPayload>, SimulationError> {
        if span.end_exclusive
            > crate::transmission::SubcarrierSpacing::Khz25.max_interleaver_cells()
            || span.start >= span.end_exclusive
            || cells.len() > crate::transmission::SubcarrierSpacing::Khz25.max_interleaver_cells()
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid("Invalid received payload span or iteration limit."));
        }
        if span.zero_loaded {
            if !self.pending.is_empty() || !cells.is_empty() {
                return Err(invalid(
                    "Zero-loaded span interrupts an incomplete payload.",
                ));
            }
            return Ok(None);
        }
        if cells.len() != span.end_exclusive - span.start
            || cells.iter().enumerate().any(|(offset, cell)| {
                cell.data_position != span.start + offset
                    || !matches!(cell.llrs.len(), 0 | 4 | 6..=14)
                    || cell.llrs.iter().any(|value| !value.is_finite())
            })
        {
            return Err(invalid(
                "Payload soft cells do not match the received pointer interval.",
            ));
        }
        if !span.starts_codeword
            && span
                .profile_id
                .is_some_and(|profile| profile != crate::transmission::symbols::PAYLOAD_PROFILE_ID)
        {
            return Err(SimulationError::new(
                "UnsupportedPayloadProfile",
                "Continuation selects an unsupported profile.",
            ));
        }
        if span.starts_codeword {
            if !self.pending.is_empty() || self.first_source.is_some() {
                return Err(invalid("New NCP start interrupts an incomplete codeword."));
            }
            if span.profile_id != Some(crate::transmission::symbols::PAYLOAD_PROFILE_ID) {
                return Err(SimulationError::new(
                    "UnsupportedPayloadProfile",
                    "Only fixed ProfileA is supported for payload decoding.",
                ));
            }
        } else if self.first_source.is_none() {
            return Err(invalid(
                "Received continuation has no accepted codeword start.",
            ));
        }
        let coded_bits = self.bits.permutation().len();
        let added: usize = cells.iter().map(|cell| cell.llrs.len()).sum();
        let total = self.pending.len() + added;
        if total > coded_bits
            && (cells.is_empty()
                || total - cells.last().map_or(0, |cell| cell.llrs.len()) >= coded_bits)
        {
            return Err(invalid(
                "Received interval exceeds one codeword plus its final-cell padding.",
            ));
        }
        if span.starts_codeword {
            self.first_source = Some(source_symbol);
        }
        let remaining = coded_bits - self.pending.len();
        self.pending.extend(
            cells
                .iter()
                .flat_map(|cell| cell.llrs.iter().copied())
                .take(remaining),
        );
        if self.pending.len() != coded_bits {
            return Ok(None);
        }
        let next_sequence = self
            .next_sequence
            .checked_add(1)
            .ok_or_else(|| invalid("Receive codeword sequence exhausted."))?;
        let ldpc_llrs = self.bits.decode(&self.pending)?;
        let decoded = self
            .codec
            .decode(&ldpc_llrs, max_iterations)
            .map_err(invalid)?;
        let first_source_symbol = self
            .first_source
            .take()
            .ok_or_else(|| invalid("Missing accepted codeword source identity."))?;
        let result = RecoveredPayload {
            receive_sequence: self.next_sequence,
            profile_id: crate::transmission::symbols::PAYLOAD_PROFILE_ID,
            first_source_symbol,
            last_source_symbol: source_symbol,
            cell_llrs: std::mem::take(&mut self.pending),
            ldpc_llrs,
            decoded,
            shortening: None,
            known_zeros_consistent: true,
        };
        self.next_sequence = next_sequence;
        Ok(Some(result))
    }
}
