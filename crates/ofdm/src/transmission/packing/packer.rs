/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded owned queue and atomic per-symbol commit.
use super::{
    model::{MAX_ACTIVE_STARTS, MAX_QUEUED_CODEWORDS, QAM256_CODEWORD_CELLS},
    planner, CodewordId, PackedSymbol, PackingSettings,
};
use crate::{
    error::invalid,
    signal_processing::modulation::SymbolLabel,
    transmission::{layout::ncp::PreInterleavingRole, SubcarrierSpacing},
    SimulationError,
};
use std::collections::VecDeque;

pub(super) struct QueuedWord {
    pub id: CodewordId,
    pub labels: Vec<SymbolLabel>,
    pub offset: usize,
}

/// Stateful bounded uniform256-QAM packer, independent of FEC and waveform code.
///
/// Inputs are already bit-processed labels; output cells are not yet randomized.
/// Owns labels, carries partial words across symbols and never resets a randomizer.
pub struct SymbolPacker {
    settings: PackingSettings,
    bit_loading: Option<Vec<u8>>,
    queue: VecDeque<QueuedWord>,
    next_word_id: CodewordId,
    next_symbol: u64,
    previous_starts: usize,
}
impl SymbolPacker {
    /// Creates one fixed-NI packing job.
    /// # Arguments
    /// * `settings` - Fixed domain size, spacing, data profile and NCP modulation.
    /// # Returns
    /// Empty bounded queue starting at symbol0 and codeword ID0.
    /// # Errors
    /// Rejects invalid settings before allocating queued codewords.
    pub fn new(settings: PackingSettings) -> Result<Self, SimulationError> {
        settings.validate()?;
        Ok(Self {
            settings,
            bit_loading: None,
            queue: VecDeque::new(),
            next_word_id: 0,
            next_symbol: 0,
            previous_starts: 0,
        })
    }

    /// Creates a variable-width coded-bit packer with a validated source-rank profile.
    /// # Arguments
    /// * `settings` - NI, NCP and profile identity.
    /// * `bit_loading` - Exactly NI bit counts in source order, not physical order.
    /// # Returns
    /// Empty queue whose offsets count encoded bits instead of fixed-size cells.
    /// # Errors
    /// Rejects shape, invalid/all-zero loading and unsupported uniform coding modes.
    #[cfg(feature = "simulation")]
    pub fn with_bit_loading(
        settings: PackingSettings,
        bit_loading: Vec<u8>,
    ) -> Result<Self, SimulationError> {
        if bit_loading.len() != settings.interleaver_cells {
            return Err(invalid("Profile does not match packer NI."));
        }
        crate::signal_processing::modulation::interleaving::mixed::PayloadBitOrder::new(
            &bit_loading,
        )?;
        let mut packer = Self::new(settings)?;
        packer.bit_loading = Some(bit_loading);
        Ok(packer)
    }
    /// Queues an encoded, bit-interleaved full word for variable-width grouping.
    /// # Arguments
    /// * `bits` - Exactly16200 binary values; final-cell padding is added during packing.
    /// # Returns
    /// Diagnostic identity; input bits do not enter the receiver as truth.
    /// # Errors
    /// Rejects non-bit mode, malformed bits, queue limit or exhausted IDs atomically.
    #[cfg(feature = "simulation")]
    pub fn enqueue_bits(&mut self, bits: Vec<u8>) -> Result<CodewordId, SimulationError> {
        if self.bit_loading.is_none()
            || bits.len() != fec_codec::parameters::LDPC_CODEWORD_BITS
            || bits.iter().any(|bit| *bit > 1)
            || self.queue.len() >= MAX_QUEUED_CODEWORDS
        {
            return Err(invalid(
                "Expected one binary16200-bit word in variable-width mode.",
            ));
        }
        let next = self
            .next_word_id
            .checked_add(1)
            .ok_or_else(|| invalid("Codeword IDs exhausted."))?;
        let id = self.next_word_id;
        self.queue.push_back(QueuedWord {
            id,
            labels: bits.into_iter().map(u16::from).collect(),
            offset: 0,
        });
        self.next_word_id = next;
        Ok(id)
    }
    /// Enqueues an owned complete codeword's post-bit-processing cell labels.
    /// # Arguments
    /// * `labels` - Exactly2025 labels in0..256. Ownership transfers on call;
    ///   rejected values are dropped while existing queued data remains unchanged.
    /// # Returns
    /// Monotonic job-local identity, not a wire pointer or profile ID.
    /// # Errors
    /// Rejects invalid labels, full queue or exhausted IDs without changing state.
    pub fn enqueue(&mut self, labels: Vec<SymbolLabel>) -> Result<CodewordId, SimulationError> {
        if self.bit_loading.is_some()
            || self.queue.len() >= MAX_QUEUED_CODEWORDS
            || labels.len() != QAM256_CODEWORD_CELLS
            || labels.iter().any(|label| *label >= 256)
        {
            return Err(invalid("Queue full or expected2025 labels in0..256."));
        }
        let next = self
            .next_word_id
            .checked_add(1)
            .ok_or_else(|| invalid("Codeword IDs exhausted."))?;
        let id = self.next_word_id;
        self.queue.push_back(QueuedWord {
            id,
            labels,
            offset: 0,
        });
        self.next_word_id = next;
        Ok(id)
    }

    /// Returns queued words, counting a partial head word until its final cell.
    /// # Returns
    /// Count bounded by16; no data is copied or consumed.
    pub fn queued_codewords(&self) -> usize {
        self.queue.len()
    }

    /// Plans and commits the next symbol without changing supplied roles.
    /// # Arguments
    /// * `roles` - ExactlyNI pre-interleaving data/pilot roles for this symbol;
    ///   physical-bin pilot locations must first be inverse-interleaved externally.
    /// # Returns
    /// Owned cells, planned NCP meanings and exact NCP+CRC cell reservation.
    /// Empty input queues produce tagged zero-loaded filler, not fake payload.
    /// # Errors
    /// Rejects invalid roles/capacity or exhausted symbol indices atomically.
    pub fn pack_symbol(
        &mut self,
        roles: &[PreInterleavingRole],
    ) -> Result<PackedSymbol, SimulationError> {
        let next = self
            .next_symbol
            .checked_add(1)
            .ok_or_else(|| invalid("Symbol indices exhausted."))?;
        let budget = match self.settings.subcarrier_spacing {
            SubcarrierSpacing::Khz25 => MAX_ACTIVE_STARTS,
            SubcarrierSpacing::Khz50 => MAX_ACTIVE_STARTS - self.previous_starts,
        };
        let preview = planner::plan(
            &self.settings,
            roles,
            &self.queue,
            budget,
            self.bit_loading.as_deref(),
        )?;
        let mut remaining = preview.consumed;
        while remaining > 0 {
            let word = self
                .queue
                .front_mut()
                .expect("Validated plan consumes queued cells only");
            let take = remaining.min(word.labels.len() - word.offset);
            word.offset += take;
            remaining -= take;
            if word.offset == word.labels.len() {
                self.queue.pop_front();
            }
        }
        let result = PackedSymbol {
            symbol_index: self.next_symbol,
            cells: preview.cells,
            ncp_entries: preview.entries,
            ncp_modulation: self.settings.ncp_modulation,
            ncp_reserved_cells: preview.reserved,
            queued_codewords_after: self.queue.len(),
        };
        self.next_symbol = next;
        self.previous_starts = preview.starts;
        Ok(result)
    }
}
