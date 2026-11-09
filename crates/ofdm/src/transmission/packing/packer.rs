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

/// Stateful bounded label/coded-bit packer, independent of FEC and waveform code.
///
/// Inputs are already bit-processed labels; output cells are not yet randomized.
/// Owns labels, carries partial words across symbols and never resets a randomizer.
pub struct SymbolPacker {
    settings: PackingSettings,
    bit_loading: Option<Vec<u8>>,
    coded_bits: usize,
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
            coded_bits: super::model::QAM256_CODEWORD_CELLS * 8,
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
        let bit_order =
            crate::signal_processing::modulation::interleaving::mixed::PayloadBitOrder::new(
                &bit_loading,
            )?;
        let mut packer = Self::new(settings)?;
        packer.coded_bits = bit_order.permutation().len();
        packer.bit_loading = Some(bit_loading);
        Ok(packer)
    }
    /// Changes data coding only after all previously queued source words are placed.
    /// # Arguments
    /// * `profile_id` - Next data profile identity0..15.
    /// * `loading` - Next validated source-rank loading of unchanged NI.
    /// # Returns
    /// Unit preserving word IDs, symbol count and rolling start limits.
    /// # Errors
    /// Rejects pending words or invalid loading/settings without changing state.
    #[cfg(feature = "simulation")]
    pub fn activate_profile(
        &mut self,
        profile_id: crate::types::ProfileId,
        loading: Vec<u8>,
    ) -> Result<(), SimulationError> {
        if !self.queue.is_empty() || loading.len() != self.settings.interleaver_cells {
            return Err(invalid(
                "Activate data profiles only with an empty source queue and unchanged NI.",
            ));
        }
        let candidate = PackingSettings {
            data_profile_id: profile_id,
            ..self.settings
        };
        candidate.validate()?;
        let order =
            crate::signal_processing::modulation::interleaving::mixed::PayloadBitOrder::new(
                &loading,
            )?;
        self.coded_bits = order.permutation().len();
        self.bit_loading = Some(loading);
        self.settings = candidate;
        Ok(())
    }
    /// Changes uniform NCP modulation without discarding pending data or start history.
    /// # Arguments
    /// * `modulation` - Generation selected by a separately validated announcement.
    /// # Returns
    /// Unit with subsequent reservations using the new field cell count.
    /// # Errors
    /// Rejects insufficient NI without changing state.
    pub fn activate_ncp(
        &mut self,
        modulation: super::NcpModulation,
    ) -> Result<(), SimulationError> {
        let candidate = PackingSettings {
            ncp_modulation: modulation,
            ..self.settings
        };
        candidate.validate()?;
        self.settings = candidate;
        Ok(())
    }
    /// Queues an encoded, bit-interleaved full word for variable-width grouping.
    /// # Arguments
    /// * `bits` - Exactly the selected coded/padded bit count of binary values; final-cell padding is added during packing.
    /// # Returns
    /// Diagnostic identity; input bits do not enter the receiver as truth.
    /// # Errors
    /// Rejects non-bit mode, malformed bits, queue limit or exhausted IDs atomically.
    #[cfg(feature = "simulation")]
    pub fn enqueue_bits(&mut self, bits: Vec<u8>) -> Result<CodewordId, SimulationError> {
        if self.bit_loading.is_none()
            || bits.len() != self.coded_bits
            || bits.iter().any(|bit| *bit > 1)
            || self.queue.len() >= MAX_QUEUED_CODEWORDS
        {
            return Err(invalid(
                "Expected one correctly sized binary coded word in variable-width mode.",
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
    /// Queues an already bit-processed shortened word for sequential profile grouping.
    /// # Arguments
    /// * `bits` - Binary K+1968 mapper-order bits, with no final-cell padding yet.
    /// # Returns
    /// Job-local diagnostic ID; no length is passed through the channel to RX.
    /// # Errors
    /// Rejects non-bit mode, invalid byte length, nonbinary data, full queue or exhausted IDs.
    #[cfg(feature = "simulation")]
    pub fn enqueue_shortened_bits(&mut self, bits: Vec<u8>) -> Result<CodewordId, SimulationError> {
        let payload = bits
            .len()
            .checked_sub(fec_codec::parameters::PAYLOAD_PARITY_BITS)
            .ok_or_else(|| invalid("Shortened word is shorter than parity."))?;
        let geometry = fec_codec::shortening::Shortening::new(payload).map_err(invalid)?;
        super::shortening::validate_shortened_transport(geometry)?;
        if self.bit_loading.is_none()
            || bits.iter().any(|bit| *bit > 1)
            || self.queue.len() >= MAX_QUEUED_CODEWORDS
        {
            return Err(invalid("Invalid shortened bit queue input or queue full."));
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
