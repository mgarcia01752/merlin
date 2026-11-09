/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded even/odd DPD bank, populated only by CRC-validated received packets.
use super::{configuration::ncp_modulation, profile, ReceivedProfile, ReceiverConfiguration};
use crate::{
    error::invalid,
    transmission::{
        layout::bit_loading::source_bit_loading,
        packing::NcpModulation,
        plc::{descriptors, mac_packets::ManagementPacket},
    },
    SimulationError,
};
use std::collections::BTreeMap;
/// Stable received OCD plus at most two generations for each data/NCP profile.
#[derive(Debug, Clone)]
pub struct ProfileGenerations {
    configuration: ReceiverConfiguration,
    profiles: BTreeMap<(crate::types::ProfileId, bool), ReceivedProfile>,
}
impl ProfileGenerations {
    /// Seeds a bank from a ready receiver snapshot, retaining its independent counters.
    /// # Arguments
    /// * `configuration` - Received-only OCD/ProfileA/NCP configuration.
    /// # Returns
    /// Owned stable geometry and the acquired two profiles; no TX objects are borrowed.
    pub fn new(configuration: &ReceiverConfiguration) -> Self {
        let mut profiles = BTreeMap::new();
        for profile in [configuration.profile_a(), configuration.ncp()] {
            profiles.insert(
                (profile.profile_id(), profile.change_count() & 1 != 0),
                profile.clone(),
            );
        }
        Self {
            configuration: configuration.clone(),
            profiles,
        }
    }
    /// Admits one complete received DPD after identity, CRC, coverage and coding checks.
    /// # Arguments
    /// * `bytes` - Received management packet bytes, including HCS/FCS.
    /// # Returns
    /// True for a new generation, false for an identical retransmission.
    /// Slots never silently replace a distinct same-parity generation.
    /// # Errors
    /// Rejects changed source/channel/OCD, malformed loading or occupied parity slots
    /// atomically. Retire/reacquisition policy for further counter wrap remains explicit.
    pub fn admit(&mut self, bytes: &[u8]) -> Result<bool, SimulationError> {
        let packet = ManagementPacket::from_bytes(bytes)?;
        let descriptor = descriptors::parse(&packet)?;
        if packet.source_address() != self.configuration.source_mac()
            || descriptor.channel_id != self.configuration.channel_id()
        {
            return Err(invalid(
                "DPD generation source/channel differs from the acquired OCD.",
            ));
        }
        let profile = profile::build(&descriptor, self.configuration.channel())?;
        if profile.profile_id() == 255 {
            ncp_modulation(&profile)?;
            if profile
                .bit_loading()
                .iter()
                .flatten()
                .any(|bits| *bits == 0)
            {
                return Err(invalid(
                    "Dynamic NCP requires positive uniform loading on every eligible bin.",
                ));
            }
        } else {
            let loading = self.loading_for(&profile)?;
            crate::signal_processing::modulation::interleaving::mixed::PayloadBitOrder::new(
                &loading,
            )?;
        }
        let key = (profile.profile_id(), profile.change_count() & 1 != 0);
        if let Some(existing) = self.profiles.get(&key) {
            if existing == &profile {
                return Ok(false);
            }
            return Err(SimulationError::new(
                "ProfileGenerationOccupied",
                "Received DPD would replace an occupied parity slot; reacquire the session.",
            ));
        }
        self.profiles.insert(key, profile);
        Ok(true)
    }
    /// Resolves the C-selected data profile into source-rank loading.
    /// # Arguments
    /// * `profile_id` - Received data profile identity0..15.
    /// * `odd` - Received C bit for that codeword start.
    /// # Returns
    /// Owned validated loading using the same inverse frequency mapping as fixed RX.
    /// # Errors
    /// Rejects unavailable generations and control-profile identities.
    pub fn data_loading(
        &self,
        profile_id: crate::types::ProfileId,
        odd: bool,
    ) -> Result<Vec<u8>, SimulationError> {
        if profile_id > 15 {
            return Err(invalid("Invalid data profile identity."));
        }
        self.loading_for(self.get(profile_id, odd)?)
    }
    /// Resolves the N-selected positive uniform NCP constellation.
    /// # Arguments
    /// * `odd` - Generation chosen by received announcement history.
    /// # Returns
    /// QPSK,16-QAM or64-QAM modulation from the received NCP DPD.
    /// # Errors
    /// Rejects an unavailable generation or invalid control loading.
    pub fn ncp_modulation(&self, odd: bool) -> Result<NcpModulation, SimulationError> {
        ncp_modulation(self.get(255, odd)?)
    }
    fn get(
        &self,
        identity: crate::types::ProfileId,
        odd: bool,
    ) -> Result<&ReceivedProfile, SimulationError> {
        self.profiles.get(&(identity, odd)).ok_or_else(|| {
            SimulationError::new(
                "MissingProfileGeneration",
                "Received NCP selects a DPD generation not admitted to this session.",
            )
        })
    }
    fn loading_for(&self, profile: &ReceivedProfile) -> Result<Vec<u8>, SimulationError> {
        let physical: Vec<_> = profile
            .bit_loading()
            .iter()
            .map(|bits| bits.unwrap_or(0))
            .collect();
        source_bit_loading(&self.configuration.channel().physical_layout()?, &physical)
    }
}
