/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Atomic fixed-session admission and an explicit data-plane readiness gate.
use super::{
    collection::{DescriptorCollection, MissingDescriptors},
    configuration::{self, ReceiverConfiguration},
};
use crate::{
    transmission::{plc::mac_packets::ManagementPacket, SubcarrierSpacing},
    SimulationError,
};
use serde::Serialize;
/// Current admission/readiness state; a rejection closes the data gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiverConfigurationStatus {
    Waiting,
    Ready,
    Rejected,
}
/// Owns accepted descriptors, last complete configuration and rejection diagnostics.
pub struct ReceiverConfigurationState {
    bootstrap_spacing: SubcarrierSpacing,
    collection: DescriptorCollection,
    status: ReceiverConfigurationStatus,
    last_valid: Option<ReceiverConfiguration>,
    last_rejection: Option<SimulationError>,
}
impl ReceiverConfigurationState {
    /// Starts acquisition with PLC bootstrap geometry and no data-plane settings.
    /// # Arguments
    /// * `bootstrap_spacing` - Known local PLC geometry, not a transmitter configuration.
    /// # Returns
    /// Waiting state with no accepted descriptors and a closed data gate.
    pub fn new(bootstrap_spacing: SubcarrierSpacing) -> Self {
        Self {
            bootstrap_spacing,
            collection: DescriptorCollection::default(),
            status: ReceiverConfigurationStatus::Waiting,
            last_valid: None,
            last_rejection: None,
        }
    }
    /// Reports current readiness without exposing mutable collection internals.
    /// # Returns
    /// Waiting, Ready or Rejected; only Ready permits access through require_ready.
    pub fn status(&self) -> ReceiverConfigurationStatus {
        self.status
    }
    /// Reports exactly which required descriptors are still absent.
    /// # Returns
    /// Missing flags; no default profiles are synthesized.
    pub fn missing(&self) -> MissingDescriptors {
        self.collection.missing()
    }
    /// Borrows the most recently accepted complete configuration for diagnostics.
    /// # Returns
    /// Historical valid configuration even after rejection, or None before acquisition.
    /// Data-plane callers must use require_ready rather than treating this as readiness.
    pub fn last_valid(&self) -> Option<&ReceiverConfiguration> {
        self.last_valid.as_ref()
    }
    /// Borrows the last admission/transport rejection without changing state.
    /// # Returns
    /// Diagnostic until a subsequent valid packet restores Waiting/Ready or reset occurs.
    pub fn last_rejection(&self) -> Option<&SimulationError> {
        self.last_rejection.as_ref()
    }
    /// Gates data/NCP configuration on successful acquisition/admission.
    /// # Returns
    /// Immutable complete configuration only in Ready state.
    /// # Errors
    /// Returns ReceiverNotReady for missing or rejected input, preserving history.
    pub fn require_ready(&self) -> Result<&ReceiverConfiguration, SimulationError> {
        if self.status == ReceiverConfigurationStatus::Ready {
            if let Some(configuration) = &self.last_valid {
                return Ok(configuration);
            }
        }
        Err(SimulationError::new(
            "ReceiverNotReady",
            "Data/NCP recovery requires a ready received PLC configuration.",
        ))
    }
    /// Admits a CRC-validated packet and commits only a consistent candidate.
    /// # Arguments
    /// * `packet` - Received OCD/ProfileA/NCP management packet; no TX settings input.
    /// # Returns
    /// Waiting until all roles exist, then Ready. Identical repeats are idempotent
    /// and can restore readiness after a rejected input without changing configuration.
    /// # Errors
    /// Rejects invalid/conflicting packets or configuration. Accepted collection
    /// and last valid configuration are preserved; rejection closes the data gate.
    pub fn accept_packet(
        &mut self,
        packet: &ManagementPacket,
    ) -> Result<ReceiverConfigurationStatus, SimulationError> {
        let result = self.admit(packet);
        if let Err(error) = &result {
            self.reject(error.clone());
        }
        result
    }
    /// Validates raw received MAC bytes before descriptor admission.
    /// # Arguments
    /// * `bytes` - Complete bounded MAC frame including HCS/FCS.
    /// # Returns
    /// Same admission/readiness outcome as accept_packet after CRC validation.
    /// # Errors
    /// Reports packet CRC/format or configuration failures, preserving accepted history.
    pub fn accept_bytes(
        &mut self,
        bytes: &[u8],
    ) -> Result<ReceiverConfigurationStatus, SimulationError> {
        match ManagementPacket::from_bytes(bytes) {
            Ok(packet) => self.accept_packet(&packet),
            Err(error) => {
                self.reject(error.clone());
                Err(error)
            }
        }
    }
    /// Begins a new explicit fixed acquisition session, discarding all old history.
    /// # Returns
    /// Unit with no descriptors/configuration and Waiting status; bootstrap spacing stays fixed.
    pub fn reset(&mut self) {
        self.collection = DescriptorCollection::default();
        self.last_valid = None;
        self.last_rejection = None;
        self.status = ReceiverConfigurationStatus::Waiting;
    }
    /// Records an upstream PLC decode/transport rejection without deleting accepted data.
    /// # Arguments
    /// * `error` - Typed received failure, without capture arrays.
    /// # Returns
    /// Unit with Rejected state and a closed gate; last valid configuration remains readable.
    pub(crate) fn reject(&mut self, error: SimulationError) {
        log::debug!("receiver-configuration-rejected error-code={}", error.code);
        self.last_rejection = Some(error);
        self.status = ReceiverConfigurationStatus::Rejected;
    }
    fn admit(
        &mut self,
        packet: &ManagementPacket,
    ) -> Result<ReceiverConfigurationStatus, SimulationError> {
        let mut candidate = self.collection.clone();
        let added = candidate.admit(packet)?;
        let missing = candidate.missing();
        let complete = !missing.ocd && !missing.profile_a && !missing.ncp;
        if added || self.last_valid.is_none() {
            if let Some(configuration) =
                configuration::try_build(&candidate, self.bootstrap_spacing)?
            {
                self.last_valid = Some(configuration);
            }
        }
        self.collection = candidate;
        self.status = if complete {
            ReceiverConfigurationStatus::Ready
        } else {
            ReceiverConfigurationStatus::Waiting
        };
        self.last_rejection = None;
        Ok(self.status)
    }
}
