/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ordered TX/RX composition, independent of payload types and FEC.
use super::{FrequencyInterleaver, InterleaverDirection, InterleaverSettings, TimeInterleaver};
use crate::LabError;
/// One stream's ordered time/frequency state; instantiate separate TX and RX objects.
pub struct SymbolInterleaver<CellValue> {
    direction: InterleaverDirection,
    time: TimeInterleaver<CellValue>,
    frequency: FrequencyInterleaver,
}
impl<CellValue: Clone> SymbolInterleaver<CellValue> {
    /// Creates a bounded stream using validated fixed channel geometry.
    /// # Arguments
    /// * `settings` - Same settings required at both ends of the matched pair.
    /// * `direction` - TX time→frequency or RX inverse frequency→complementary time.
    /// # Returns
    /// Owned independent history expecting symbol0; no input/sample ownership leaks.
    /// # Errors
    /// Reports frequency permutation setup failures.
    pub fn new(
        settings: InterleaverSettings,
        direction: InterleaverDirection,
    ) -> Result<Self, LabError> {
        Ok(Self {
            direction,
            time: TimeInterleaver::new(settings, direction),
            frequency: FrequencyInterleaver::new(settings.spacing(), settings.cells())?,
        })
    }
    /// Advances one symbol through the selected ordered interleaver path.
    /// # Arguments
    /// * `symbol_index` - Consecutive0-based stream symbol, including warm-up/drain.
    /// * `input` - Exactly NI optional cells; None marks unavailable history/drain.
    /// # Returns
    /// Owned NI output. A matched pair restores each input after M-1 symbols.
    /// # Errors
    /// Rejects invalid length/sequence/overflow without advancing stream history.
    pub fn process_symbol(
        &mut self,
        symbol_index: u64,
        input: &[Option<CellValue>],
    ) -> Result<Vec<Option<CellValue>>, LabError> {
        match self.direction {
            InterleaverDirection::Transmit => self
                .frequency
                .interleave(&self.time.process_symbol(symbol_index, input)?),
            InterleaverDirection::Receive => self
                .time
                .process_symbol(symbol_index, &self.frequency.deinterleave(input)?),
        }
    }
}
