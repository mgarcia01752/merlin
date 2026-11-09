/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Symbol-stream convolutional delays, including conceptual end-of-symbol dummies.
use super::{InterleaverDirection, InterleaverSettings};
use crate::{error::invalid, SimulationError};
use std::collections::VecDeque;
/// Bounded generic convolutional delay lines; None marks startup/drain, never zero.
pub struct TimeInterleaver<CellValue> {
    settings: InterleaverSettings,
    branches: Vec<VecDeque<Option<CellValue>>>,
    next_symbol: u64,
}
impl<CellValue: Clone> TimeInterleaver<CellValue> {
    /// Allocates fixed branch buffers for one independent TX or RX stream.
    /// # Arguments
    /// * `settings` - Validated NI/depth; branch cells are clocked at padded JM length.
    /// * `direction` - TX branch b delays bJ cells; RX delays (M-1-b)J cells.
    /// # Returns
    /// Empty-history stream expecting symbol0. CellValue is cloned, never numerically changed.
    pub fn new(settings: InterleaverSettings, direction: InterleaverDirection) -> Self {
        let stride = settings.cells().div_ceil(settings.depth());
        let branches = (0..settings.depth())
            .map(|branch| {
                let delay = match direction {
                    InterleaverDirection::Transmit => branch,
                    InterleaverDirection::Receive => settings.depth() - 1 - branch,
                };
                std::iter::repeat_with(|| None)
                    .take(delay * stride)
                    .collect()
            })
            .collect();
        Self {
            settings,
            branches,
            next_symbol: 0,
        }
    }
    /// Advances one complete symbol while retaining delay history.
    /// # Arguments
    /// * `symbol_index` - Consecutive index beginning at0; no per-codeword reset.
    /// * `input` - Exactly NI borrowed cells; None denotes startup/drain gaps only.
    /// # Returns
    /// Owned NI cells, including None where delay history has not been filled.
    /// Conceptual dummy positions are clocked but never exported.
    /// # Errors
    /// Rejects invalid length/index or index overflow before changing any history.
    pub fn process_symbol(
        &mut self,
        symbol_index: u64,
        input: &[Option<CellValue>],
    ) -> Result<Vec<Option<CellValue>>, SimulationError> {
        let next = symbol_index
            .checked_add(1)
            .ok_or_else(|| invalid("Interleaver symbol index exhausted."))?;
        if symbol_index != self.next_symbol || input.len() != self.settings.cells() {
            return Err(invalid(
                "Time interleaver requires consecutive symbols and fixed NI.",
            ));
        }
        let padded = self.settings.cells().div_ceil(self.settings.depth()) * self.settings.depth();
        let mut output = Vec::with_capacity(input.len());
        for position in 0..padded {
            let incoming = input.get(position).cloned().unwrap_or(None);
            let branch = &mut self.branches[position % self.settings.depth()];
            let outgoing = if branch.is_empty() {
                incoming
            } else {
                let outgoing = branch.pop_front().flatten();
                branch.push_back(incoming);
                outgoing
            };
            if position < input.len() {
                output.push(outgoing);
            }
        }
        self.next_symbol = next;
        Ok(output)
    }
}
