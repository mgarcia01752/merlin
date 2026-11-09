/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in received-boundary collector for full and shortened frozen-profile words.
use super::model::{RecoveredPayload, SoftDataCell};
use crate::{
    error::invalid,
    signal_processing::modulation::interleaving::{
        mixed::PayloadBitOrder, shortened::ShortenedBitOrder,
    },
    transmission::{
        ncp::NcpDataSpan,
        packing::shortening::{discover_payload_length, MAX_RECEIVED_CODEWORD_BITS},
    },
    SimulationError,
};
use fec_codec::{
    concatenated::{ConcatenatedCodec, DecodedBlock},
    parameters::BCH_INPUT_BITS,
    shortening::ShortenedCodec,
};
/// Bounded receiver state; boundaries come solely from accepted NCP start/filler spans.
pub struct VariablePayloadCollector {
    full: ConcatenatedCodec,
    full_bits: PayloadBitOrder,
    pending: Vec<f64>,
    first_source: Option<u64>,
    last_source: u64,
    sequence: u64,
    last_cell_bits: usize,
}
impl VariablePayloadCollector {
    /// Constructs a receiver with independently acquired data loading.
    /// # Arguments
    /// * `loading` - Valid source-rank bit loading; no TX identities or word lengths.
    /// # Returns
    /// Empty bounded collection state, reusable across successive original lengths.
    /// # Errors
    /// Rejects loading or mother codec setup failures.
    pub fn new(loading: &[u8]) -> Result<Self, SimulationError> {
        Ok(Self {
            full: ConcatenatedCodec::new().map_err(invalid)?,
            full_bits: PayloadBitOrder::new(loading)?,
            pending: Vec::new(),
            first_source: None,
            last_source: 0,
            sequence: 0,
            last_cell_bits: 0,
        })
    }
    /// Reports retained soft coordinates, including any unclassified final-cell bits.
    /// # Returns
    /// At most16213 finite values; no incomplete word is counted as recovered.
    pub fn pending_bits(&self) -> usize {
        self.pending.len()
    }
    /// Accepts a span, finalizing a prior word only on a received start or filler boundary.
    /// # Arguments
    /// * `span` - CRC-accepted NCP interval; source end alone is not a word boundary.
    /// * `cells` - Exactly the interval's soft cells, empty for zero-loaded filler.
    /// * `source_symbol` - Recovered source phase, never a TX word identity.
    /// * `max_iterations` - Actual LDPC cap1..100.
    /// # Returns
    /// Previous completed word at a new boundary, or None; the current word remains pending.
    /// Failure decisions are retained. No flush may invent an absent terminal pointer.
    /// # Errors
    /// Rejects malformed spans, orphan continuations, profile mismatch, overfull words
    /// or impossible lengths. A caller must close the session after rejection.
    pub fn accept_span(
        &mut self,
        span: &NcpDataSpan,
        cells: &[SoftDataCell],
        source_symbol: u64,
        max_iterations: usize,
    ) -> Result<Option<RecoveredPayload>, SimulationError> {
        if span.start >= span.end_exclusive
            || span.end_exclusive
                > crate::transmission::SubcarrierSpacing::Khz25.max_interleaver_cells()
            || !(1..=100).contains(&max_iterations)
        {
            return Err(invalid("Invalid variable payload span or iteration limit."));
        }
        if span.zero_loaded {
            if !cells.is_empty() {
                return Err(invalid("Filler cannot contain payload soft cells."));
            }
        } else if cells.len() != span.end_exclusive - span.start
            || cells.iter().enumerate().any(|(offset, cell)| {
                cell.data_position != span.start + offset
                    || !matches!(cell.llrs.len(), 0 | 4 | 6..=14)
                    || cell.llrs.iter().any(|value| !value.is_finite())
            })
        {
            return Err(invalid(
                "Soft cells do not match the variable payload span.",
            ));
        }
        if !span.zero_loaded
            && (span.starts_codeword
                && span.profile_id != Some(crate::transmission::symbols::PAYLOAD_PROFILE_ID)
                || span.profile_id.is_some_and(|profile| {
                    profile != crate::transmission::symbols::PAYLOAD_PROFILE_ID
                }))
        {
            return Err(invalid("Unsupported variable payload profile."));
        }
        let completed = if (span.starts_codeword || span.zero_loaded) && self.first_source.is_some()
        {
            Some(self.finish(max_iterations)?)
        } else {
            None
        };
        if span.zero_loaded {
            return Ok(completed);
        }
        if span.starts_codeword {
            self.first_source = Some(source_symbol);
        } else if self.first_source.is_none() {
            return Err(invalid("Variable continuation has no accepted start."));
        }
        let added: usize = cells.iter().map(|cell| cell.llrs.len()).sum();
        if self.pending.len() + added > MAX_RECEIVED_CODEWORD_BITS {
            return Err(invalid("Variable payload exceeds bounded wire dimensions."));
        }
        self.pending
            .extend(cells.iter().flat_map(|cell| cell.llrs.iter().copied()));
        if let Some(cell) = cells.iter().rfind(|cell| !cell.llrs.is_empty()) {
            self.last_cell_bits = cell.llrs.len();
            self.last_source = source_symbol;
        }
        Ok(completed)
    }
    fn finish(&mut self, max_iterations: usize) -> Result<RecoveredPayload, SimulationError> {
        let geometry = discover_payload_length(self.pending.len())?;
        let (coded_count, ldpc_llrs, decoded, consistent) =
            if geometry.payload_bits() == BCH_INPUT_BITS {
                let count = self.full_bits.permutation().len();
                if self.pending.len() < count {
                    return Err(invalid(
                        "Full uniform word is missing prescribed interleaver padding.",
                    ));
                }
                let llrs = self.full_bits.decode(&self.pending[..count])?;
                let decoded = self.full.decode(&llrs, max_iterations).map_err(invalid)?;
                (count, llrs, decoded, true)
            } else {
                let count = geometry.transmitted_bits();
                let bits = ShortenedBitOrder::new(geometry)?;
                let llrs = bits.decode(&self.pending[..count])?;
                let mut codec = ShortenedCodec::new(geometry).map_err(invalid)?;
                let result = codec.decode(&llrs, max_iterations).map_err(invalid)?;
                (
                    count,
                    result.restored_llrs,
                    DecodedBlock {
                        ldpc: result.ldpc,
                        bch: result.bch,
                    },
                    result.known_zeros_consistent,
                )
            };
        if self.pending.len() - coded_count >= self.last_cell_bits {
            return Err(invalid(
                "Received boundary extends beyond the word's final data cell.",
            ));
        }
        let next = self
            .sequence
            .checked_add(1)
            .ok_or_else(|| invalid("Receive sequence exhausted."))?;
        let result = RecoveredPayload {
            receive_sequence: self.sequence,
            profile_id: crate::transmission::symbols::PAYLOAD_PROFILE_ID,
            first_source_symbol: self
                .first_source
                .ok_or_else(|| invalid("Missing received start."))?,
            last_source_symbol: self.last_source,
            cell_llrs: self.pending[..coded_count].to_vec(),
            ldpc_llrs,
            decoded,
            shortening: Some(geometry),
            known_zeros_consistent: consistent,
        };
        self.pending.clear();
        self.first_source = None;
        self.last_cell_bits = 0;
        self.sequence = next;
        Ok(result)
    }
}
