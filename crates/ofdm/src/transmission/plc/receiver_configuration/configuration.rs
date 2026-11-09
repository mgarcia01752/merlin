/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Complete immutable data-plane configuration assembled only from received descriptors.
use super::{
    channel::{self, ReceivedChannel},
    collection::DescriptorCollection,
    profile::{self, ReceivedProfile},
};
use crate::{
    error::invalid,
    signal_processing::interleaving::InterleaverSettings,
    transmission::{layout::PhysicalCarrierRole, packing::NcpModulation, SubcarrierSpacing},
    types::{ChannelId, MacAddress, SubcarrierIdx},
    SimulationError,
};
use serde::Serialize;
/// Validated fixed receiver configuration. Construction is private to the admission gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ReceiverConfiguration {
    channel_id: ChannelId,
    source_mac: MacAddress,
    ocd_change_count: u8,
    channel: ReceivedChannel,
    profile_a: ReceivedProfile,
    ncp: ReceivedProfile,
    ncp_modulation: NcpModulation,
    reduced_bins: Vec<SubcarrierIdx>,
}
impl ReceiverConfiguration {
    /// Returns the received downstream channel identity.
    /// # Returns
    /// Channel ID shared by all required descriptors.
    pub fn channel_id(&self) -> ChannelId {
        self.channel_id
    }
    /// Copies the pinned received CMTS MAC-domain identity.
    /// # Returns
    /// Source address shared by all required packets; not supplied TX settings.
    pub fn source_mac(&self) -> MacAddress {
        self.source_mac
    }
    /// Returns the OCD's independent generation counter.
    /// # Returns
    /// Received OCD count; per-profile counts remain in their own models.
    pub fn ocd_change_count(&self) -> u8 {
        self.ocd_change_count
    }
    /// Borrows receiver-owned channel geometry and symbol settings.
    /// # Returns
    /// Validated OCD-derived values with explicit minimum-envelope bandwidth semantics.
    pub fn channel(&self) -> &ReceivedChannel {
        &self.channel
    }
    /// Borrows the complete received ProfileA assignments.
    /// # Returns
    /// Immutable physical-bin loading and its independent change count.
    pub fn profile_a(&self) -> &ReceivedProfile {
        &self.profile_a
    }
    /// Borrows the received NCP eligibility/loading assignments.
    /// # Returns
    /// Immutable physical-bin loading, including explicit zero-loaded positions.
    pub fn ncp(&self) -> &ReceivedProfile {
        &self.ncp
    }
    /// Returns the uniform positive NCP constellation selected by the received DPD.
    /// # Returns
    /// QPSK, 16-QAM or 64-QAM; zero-loaded carriers remain in the NCP profile map.
    pub fn ncp_modulation(&self) -> NcpModulation {
        self.ncp_modulation
    }
    /// Borrows the data/scattered-pilot physical-to-reduced carrier association.
    /// # Returns
    /// Ascending physical bins excluding PLC, continuous pilots and exclusions.
    pub fn reduced_bins(&self) -> &[SubcarrierIdx] {
        &self.reduced_bins
    }
    /// Constructs matched cell-interleaver settings from received channel geometry.
    /// # Returns
    /// Validated spacing, reduced-domain length and received depth.
    /// # Errors
    /// Propagates bounds checks; no transmitter interleaver instance is borrowed.
    pub fn interleaver_settings(&self) -> Result<InterleaverSettings, SimulationError> {
        InterleaverSettings::new(
            self.channel.transmission().subcarrier_spacing,
            self.reduced_bins.len(),
            usize::from(self.channel.transmission().time_interleaver_depth),
        )
    }
}
/// Builds a complete candidate using no transmitter layout/profile/settings input.
/// # Arguments
/// * `collection` - Parsed accepted packets, possibly still missing required roles.
/// * `bootstrap_spacing` - Known PLC receive geometry, checked against the OCD.
/// # Returns
/// Some complete settings/assignments, or None while descriptors are missing.
/// Available profiles are validated as soon as OCD geometry is known.
/// # Errors
/// Rejects bootstrap mismatch, profile coverage/consistency or NI bounds.
pub(super) fn try_build(
    collection: &DescriptorCollection,
    bootstrap_spacing: SubcarrierSpacing,
) -> Result<Option<ReceiverConfiguration>, SimulationError> {
    let Some(ocd) = collection.ocd.as_ref() else {
        return Ok(None);
    };
    let channel = channel::decode(&ocd.parsed.tlvs)?;
    if channel.transmission().subcarrier_spacing != bootstrap_spacing {
        return Err(invalid(
            "Received OCD disagrees with PLC bootstrap spacing.",
        ));
    }
    let profile_a = collection
        .profile_a
        .as_ref()
        .map(|data| profile::build(&data.parsed, &channel))
        .transpose()?;
    let ncp = collection
        .ncp
        .as_ref()
        .map(|data| profile::build(&data.parsed, &channel))
        .transpose()?;
    let ncp_modulation = ncp.as_ref().map(ncp_modulation).transpose()?;
    let layout = channel.physical_layout()?;
    let reduced_bins: Vec<_> = layout
        .static_roles()
        .iter()
        .enumerate()
        .filter(|(_, role)| **role == PhysicalCarrierRole::Data)
        .map(|(bin, _)| bin as SubcarrierIdx)
        .collect();
    InterleaverSettings::new(
        bootstrap_spacing,
        reduced_bins.len(),
        usize::from(channel.transmission().time_interleaver_depth),
    )?;
    let (Some(profile_a), Some(ncp), Some(ncp_modulation)) = (profile_a, ncp, ncp_modulation)
    else {
        return Ok(None);
    };
    Ok(Some(ReceiverConfiguration {
        channel_id: ocd.parsed.channel_id,
        source_mac: ocd.packet.source_address(),
        ocd_change_count: ocd.parsed.change_count,
        channel,
        profile_a,
        ncp,
        ncp_modulation,
        reduced_bins,
    }))
}
fn ncp_modulation(profile: &ReceivedProfile) -> Result<NcpModulation, SimulationError> {
    let mut uniform = None;
    for bits in profile
        .bit_loading()
        .iter()
        .flatten()
        .filter(|bits| **bits != 0)
    {
        if uniform.is_some_and(|previous| previous != *bits) {
            return Err(invalid(
                "Received positive NCP bit loading must be uniform.",
            ));
        }
        uniform = Some(*bits);
    }
    match uniform {
        Some(2) => Ok(NcpModulation::Qpsk),
        Some(4) => Ok(NcpModulation::Qam16),
        Some(6) => Ok(NcpModulation::Qam64),
        _ => Err(invalid(
            "Received NCP profile has no supported positive loading.",
        )),
    }
}
