/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Figure92/Table53, PDF180–181; bit6 of byte1 is U, not old reserved R.
use crate::{error::invalid, types::ProfileId, LabError};
use serde::Serialize;
/// Wire pointer rank including the NULL sentinel; not a physical FFT-bin index.
pub type NcpStartPointer = u16;
/// Maximum active codeword starts in one8K symbol or two consecutive4K symbols.
/// PHY I20 section8.3.4.3; terminal Z/NULL is not an active start.
pub const MAX_CODEWORD_STARTS: usize = 10;
/// Reserved13-bit pointer value indicating no codeword start.
pub const NULL_POINTER: NcpStartPointer = 0x1fff;
/// Unencoded three-byte NCP message. Flags describe wire fields, not CMTS control.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct NcpMessage {
    pub profile_id: ProfileId,
    pub zero_loaded: bool,
    pub data_profile_odd: bool,
    pub ncp_profile_odd: bool,
    pub last: bool,
    pub tagged: bool,
    pub ncp_profile_update: bool,
    /// Data-position rank excluding scattered pilots;0x1fff denotes NULL.
    pub pointer: NcpStartPointer,
}
impl NcpMessage {
    /// Serializes validated fields without changing them.
    /// # Returns
    /// Bytes0..2 with reserved R=0, per the current Figure92 and Table53.
    /// # Errors
    /// Rejects profile IDs above15, pointers above13 bits and Z+NULL together.
    pub fn to_bytes(&self) -> Result<[u8; 3], LabError> {
        if self.profile_id > 15
            || self.pointer > NULL_POINTER
            || (self.zero_loaded && self.pointer == NULL_POINTER)
        {
            return Err(invalid(
                "Invalid NCP profile, pointer or Z+NULL combination.",
            ));
        }
        Ok([
            (self.profile_id << 4)
                | (u8::from(self.zero_loaded) << 3)
                | (u8::from(self.data_profile_odd) << 2)
                | (u8::from(self.ncp_profile_odd) << 1)
                | u8::from(self.last),
            (u8::from(self.tagged) << 7)
                | (u8::from(self.ncp_profile_update) << 6)
                | ((self.pointer >> 8) as u8),
            self.pointer as u8,
        ])
    }
    /// Parses a borrowed three-byte message, preserving control flags.
    /// # Arguments
    /// * `bytes` - Unencoded message in MSB-first byte order.
    /// # Returns
    /// Owned fields with no inferred modem state or dynamic profile scheduling.
    /// # Errors
    /// Rejects reserved R=1 or an invalid Z+NULL combination as a strict lab policy.
    pub fn from_bytes(bytes: &[u8; 3]) -> Result<Self, LabError> {
        if bytes[1] & 0x20 != 0 {
            return Err(invalid(
                "Reserved NCP R bit must be zero in this experiment.",
            ));
        }
        let message = Self {
            profile_id: bytes[0] >> 4,
            zero_loaded: bytes[0] & 8 != 0,
            data_profile_odd: bytes[0] & 4 != 0,
            ncp_profile_odd: bytes[0] & 2 != 0,
            last: bytes[0] & 1 != 0,
            tagged: bytes[1] & 0x80 != 0,
            ncp_profile_update: bytes[1] & 0x40 != 0,
            pointer: (u16::from(bytes[1] & 0x1f) << 8) | u16::from(bytes[2]),
        };
        message.to_bytes()?;
        Ok(message)
    }
}

#[cfg(feature = "simulation")]
pub(super) const MAX_MESSAGES: usize = MAX_CODEWORD_STARTS + 1;
/// Validates one symbol's message order independently of FEC and profile state.
///
/// # Arguments
/// * `messages` - Borrowed unencoded messages in descending field-assignment order.
/// # Returns
/// Unit for a bounded chain with increasing pointers and a valid final marker.
/// # Errors
/// Rejects bad fields, nonterminal NULL/Z, misplaced L, inconsistent N/U or
/// more than10 active starts. Rolling4K limits require external session state.
#[cfg(feature = "simulation")]
pub(super) fn validate_chain(messages: &[NcpMessage]) -> Result<(), LabError> {
    if messages.is_empty() || messages.len() > MAX_MESSAGES {
        return Err(invalid("NCP chain requires1..11 messages before its CRC."));
    }
    let mut previous = None;
    let mut active = 0;
    for (index, message) in messages.iter().enumerate() {
        message.to_bytes()?;
        let terminal = message.zero_loaded || message.pointer == NULL_POINTER;
        if message.last != (index + 1 == messages.len())
            || (terminal && !message.last)
            || message.ncp_profile_odd != messages[0].ncp_profile_odd
            || message.ncp_profile_update != messages[0].ncp_profile_update
        {
            return Err(invalid(
                "Invalid NCP last/terminal marker or inconsistent N/U flags.",
            ));
        }
        if message.pointer != NULL_POINTER {
            if previous.is_some_and(|pointer| message.pointer <= pointer) {
                return Err(invalid("NCP start pointers must increase strictly."));
            }
            previous = Some(message.pointer);
        }
        if !terminal {
            active += 1;
        }
    }
    if active > MAX_CODEWORD_STARTS {
        return Err(invalid("At most10 active NCP starts per symbol."));
    }
    Ok(())
}
