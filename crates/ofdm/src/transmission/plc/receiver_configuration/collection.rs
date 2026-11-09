/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded fixed-session descriptor admission; no version-equality shortcut.
use crate::{
    error::invalid,
    transmission::plc::{
        descriptors::{self, ParsedDescriptor},
        mac_packets::ManagementPacket,
    },
    types::{ChannelId, MacAddress},
    LabError,
};
use serde::Serialize;
/// Required components still absent; missing does not mean zero/default configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct MissingDescriptors {
    pub ocd: bool,
    pub profile_a: bool,
    pub ncp: bool,
}
#[derive(Clone)]
pub(super) struct CollectedDescriptor {
    pub packet: ManagementPacket,
    pub parsed: ParsedDescriptor,
}
/// At most one accepted packet per required role; all mutable state stays private.
#[derive(Clone, Default)]
pub(super) struct DescriptorCollection {
    identity: Option<(ChannelId, MacAddress)>,
    pub ocd: Option<CollectedDescriptor>,
    pub profile_a: Option<CollectedDescriptor>,
    pub ncp: Option<CollectedDescriptor>,
}
impl DescriptorCollection {
    /// Reports absent components without mutating admission state.
    /// # Returns
    /// Three explicit missing flags, independent of the last admission outcome.
    pub(super) fn missing(&self) -> MissingDescriptors {
        MissingDescriptors {
            ocd: self.ocd.is_none(),
            profile_a: self.profile_a.is_none(),
            ncp: self.ncp.is_none(),
        }
    }
    /// Admits a validated packet into one fixed session without replacing conflicts.
    /// # Arguments
    /// * `packet` - Complete CRC-validated single-part acquisition management packet.
    /// # Returns
    /// True for a newly collected role, false for an identical repeat.
    /// # Errors
    /// Rejects semantic parse errors, source/channel mismatch or changed role content.
    /// Callers use a candidate copy to preserve all previous state on later failure.
    pub(super) fn admit(&mut self, packet: &ManagementPacket) -> Result<bool, LabError> {
        let parsed = descriptors::parse(packet)?;
        let identity = (parsed.channel_id, packet.source_address());
        if self.identity.is_some_and(|expected| expected != identity) {
            return Err(invalid(
                "Received descriptors disagree on channel or CMTS source.",
            ));
        }
        let destination = match parsed.profile_id {
            None => &mut self.ocd,
            Some(0) => &mut self.profile_a,
            Some(255) => &mut self.ncp,
            _ => return Err(invalid("Unsupported acquisition profile identity.")),
        };
        if let Some(previous) = destination {
            if previous.packet == *packet {
                return Ok(false);
            }
            return Err(LabError::new(
                "ReceiverSessionResetRequired",
                "Changed descriptor content requires a new fixed acquisition session.",
            ));
        }
        *destination = Some(CollectedDescriptor {
            packet: packet.clone(),
            parsed,
        });
        self.identity = Some(identity);
        Ok(true)
    }
}
