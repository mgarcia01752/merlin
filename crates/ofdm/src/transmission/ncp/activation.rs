/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 §8.3.4.1 Table53:128-symbol NCP update announcement.
use crate::{error::invalid, SimulationError};
/// MSB-first U pattern in the128 symbols immediately preceding an NCP change.
pub const NCP_UPDATE_PATTERN: u128 = 0xBCB240898BAD833539ED0ABE946E3F85;
/// Per-field controls for a packer whose starts all select the same data generation.
#[derive(Debug, Clone, Copy, Default)]
pub struct NcpFieldFlags {
    /// C: least significant bit of the selected data DPD counter.
    pub data_profile_odd: bool,
    /// N: least significant bit of the current NCP DPD counter.
    pub ncp_profile_odd: bool,
    /// U: one bit of the128-symbol update announcement.
    pub ncp_profile_update: bool,
}
/// Immutable TX announcement timing; it does not clock or reset cell state.
#[derive(Debug, Clone, Copy)]
pub struct NcpTransitionSchedule {
    activation_symbol: u64,
    previous_odd: bool,
}
impl NcpTransitionSchedule {
    /// Schedules one change after a complete announcement interval.
    /// # Arguments
    /// * `next_symbol` - Next source symbol which will be transmitted.
    /// * `activation_symbol` - First symbol using the other NCP generation.
    /// * `previous_odd` - Current N field.
    /// # Returns
    /// Valid schedule; the caller must deliver the new DPD before activation.
    /// # Errors
    /// Rejects less than128 symbols of announcement lead time or overflow.
    pub fn new(
        next_symbol: u64,
        activation_symbol: u64,
        previous_odd: bool,
    ) -> Result<Self, SimulationError> {
        if next_symbol
            .checked_add(128)
            .is_none_or(|minimum| activation_symbol < minimum)
        {
            return Err(invalid(
                "NCP change requires128 complete announcement symbols.",
            ));
        }
        Ok(Self {
            activation_symbol,
            previous_odd,
        })
    }
    /// Returns generation and U bit at an absolute source-symbol coordinate.
    /// # Arguments
    /// * `symbol` - Source-order symbol; channel interleaving may delay delivery.
    /// # Returns
    /// N and U bits, with U zero outside the announcement interval.
    pub fn flags(&self, symbol: u64) -> (bool, bool) {
        let beginning = self.activation_symbol - 128;
        let update = if (beginning..self.activation_symbol).contains(&symbol) {
            NCP_UPDATE_PATTERN & (1u128 << (127 - (symbol - beginning))) != 0
        } else {
            false
        };
        (
            self.previous_odd ^ (symbol >= self.activation_symbol),
            update,
        )
    }
}
/// Received-only N/U recognizer; call only with a whole CRC-accepted NCP field.
#[derive(Debug, Clone)]
pub struct NcpTransitionReceiver {
    current_odd: bool,
    recent: u128,
    received: usize,
    armed: bool,
}
impl NcpTransitionReceiver {
    /// Starts from an acquired NCP DPD generation.
    /// # Arguments
    /// * `odd` - Least significant bit of the acquired change count.
    /// # Returns
    /// Empty announcement history; no transmitter schedule is accepted.
    pub fn new(odd: bool) -> Self {
        Self {
            current_odd: odd,
            recent: 0,
            received: 0,
            armed: false,
        }
    }
    /// Selects the NCP generation needed to demap the next source-order field.
    /// # Returns
    /// Opposite parity only immediately after the complete received U pattern.
    pub fn expected_odd(&self) -> bool {
        self.current_odd ^ self.armed
    }
    /// Admits one CRC-validated field's uniform N/U bits.
    /// # Arguments
    /// * `odd` - Received N bit.
    /// * `update` - Received U bit.
    /// # Returns
    /// Unit after advancing exactly one source-symbol observation.
    /// # Errors
    /// Rejects an unannounced/missing generation change without mutating state.
    pub fn accept(&mut self, odd: bool, update: bool) -> Result<(), SimulationError> {
        if odd != self.expected_odd() {
            return Err(SimulationError::new(
                "NcpActivationMismatch",
                "NCP generation does not match the received128-symbol announcement.",
            ));
        }
        self.current_odd = odd;
        self.recent = (self.recent << 1) | u128::from(update);
        self.received = (self.received + 1).min(128);
        self.armed = self.received == 128 && self.recent == NCP_UPDATE_PATTERN;
        Ok(())
    }
}
