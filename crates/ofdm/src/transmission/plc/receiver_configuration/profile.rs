/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Received range assignments with order-independent default/specific precedence.
use super::channel::ReceivedChannel;
use crate::{
    error::invalid,
    transmission::{layout::PhysicalCarrierRole, plc::descriptors::ParsedDescriptor},
    types::{BitsPerSymbol, ProfileId},
    SimulationError,
};
use serde::Serialize;
const ASSIGNMENT_TLV: u8 = 5;
const DEFAULT_FLAG: u8 = 0x20;
/// Effective received physical-bin loading. Protected OCD bins remain None.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReceivedProfile {
    profile_id: ProfileId,
    change_count: u8,
    bit_loading: Vec<Option<BitsPerSymbol>>,
}
impl ReceivedProfile {
    /// Returns the received data-profile identity or 255 for the NCP profile.
    /// # Returns
    ///0..15 for data or255 for NCP; no inferred activation selection.
    pub fn profile_id(&self) -> ProfileId {
        self.profile_id
    }
    /// Returns this profile's independent configuration change count.
    /// # Returns
    /// Received byte; it need not equal the OCD or another DPD count.
    pub fn change_count(&self) -> u8 {
        self.change_count
    }
    /// Borrows effective loading in increasing physical FFT-bin order.
    /// # Returns
    /// None for excluded/PLC/continuous pilots, Some(0) for explicit zero loading,
    /// otherwise the assigned bit count. Scattered pilots override data per symbol.
    pub fn bit_loading(&self) -> &[Option<BitsPerSymbol>] {
        &self.bit_loading
    }
}
/// Applies a structurally validated received DPD to received OCD geometry.
/// # Arguments
/// * `descriptor` - Parsed immutable collector-owned range TLVs for ProfileA or NCP.
/// * `channel` - Geometry decoded from this session's OCD.
/// # Returns
/// Complete effective data-bin loading with OCD protected roles taking precedence.
/// # Errors
/// Rejects wrong profile, out-of-grid ranges, duplicate defaults/specifics or missing
/// assignments. Semantic bit values are checked by the packet descriptor parser.
pub(super) fn build(
    descriptor: &ParsedDescriptor,
    channel: &ReceivedChannel,
) -> Result<ReceivedProfile, SimulationError> {
    let profile_id = descriptor
        .profile_id
        .ok_or_else(|| invalid("Expected a received DPD profile."))?;
    let layout = channel.physical_layout()?;
    let roles = layout.static_roles();
    let mut defaults = vec![None; roles.len()];
    let mut specifics = vec![None; roles.len()];
    for tlv in &descriptor.tlvs {
        if tlv.kind != ASSIGNMENT_TLV || tlv.value.len() != 5 {
            return Err(invalid("Unsupported received profile assignment."));
        }
        let start = usize::from(u16::from_be_bytes([tlv.value[1], tlv.value[2]]));
        let end = usize::from(u16::from_be_bytes([tlv.value[3], tlv.value[4]]));
        if start > end || end >= roles.len() {
            return Err(invalid("Received profile range exceeds its OCD FFT grid."));
        }
        let destination = if tlv.value[0] & DEFAULT_FLAG != 0 {
            &mut defaults
        } else {
            &mut specifics
        };
        for value in &mut destination[start..=end] {
            if value.is_some() {
                return Err(invalid("Duplicate default or specific DPD assignment."));
            }
            *value = Some(tlv.value[0] & 15);
        }
    }
    let mut bit_loading = Vec::with_capacity(roles.len());
    for (bin, role) in roles.iter().enumerate() {
        if *role == PhysicalCarrierRole::Data {
            let bits = specifics[bin]
                .or(defaults[bin])
                .ok_or_else(|| invalid("Received DPD leaves an active data carrier unassigned."))?;
            bit_loading.push(Some(bits));
        } else {
            bit_loading.push(None);
        }
    }
    Ok(ReceivedProfile {
        profile_id,
        change_count: descriptor.change_count,
        bit_loading,
    })
}
