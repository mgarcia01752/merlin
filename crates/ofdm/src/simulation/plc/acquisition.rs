/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Encapsulated receive composition: PLC IQ→message reassembly→receiver readiness.
use super::{PlcFrameReceiver, ReceivedPlcFrame};
use crate::{
    transmission::{
        plc::{
            reassembly::PlcMessageReassembler,
            receiver_configuration::{ReceiverConfigurationState, ReceiverConfigurationStatus},
        },
        SubcarrierSpacing,
    },
    types::IqPair,
    SimulationError,
};
use serde::Serialize;
/// One packet admission outcome; rejection does not erase completed FEC diagnostics.
#[derive(Debug, Serialize)]
pub struct DescriptorAdmission {
    /// Received CRC-validated packet retained for bounded caller inspection.
    pub packet: crate::transmission::plc::mac_packets::ManagementPacket,
    pub message_type: u8,
    pub status: ReceiverConfigurationStatus,
    pub error: Option<SimulationError>,
}
/// Completed receive result with separate decode, transport and admission diagnostics.
#[derive(Debug, Serialize)]
pub struct PlcAcquisitionResult {
    pub frame: ReceivedPlcFrame,
    pub admissions: Vec<DescriptorAdmission>,
    pub transport_error: Option<SimulationError>,
    pub configuration_status: ReceiverConfigurationStatus,
}
/// Owns the three independent receive subsystems without any transmitter objects.
pub struct PlcAcquisitionReceiver {
    frame_receiver: PlcFrameReceiver,
    reassembler: PlcMessageReassembler,
    configuration: ReceiverConfigurationState,
}
impl PlcAcquisitionReceiver {
    /// Creates a known-phase PLC receiver with an initially closed data gate.
    /// # Arguments
    /// * `bootstrap_spacing` - Reference PLC spacing known before descriptor recovery.
    /// # Returns
    /// Independent FEC, reassembly and configuration state; no TX profile/layout input.
    /// # Errors
    /// Propagates codec initialization failure.
    pub fn new(bootstrap_spacing: SubcarrierSpacing) -> Result<Self, SimulationError> {
        Ok(Self {
            frame_receiver: PlcFrameReceiver::new(bootstrap_spacing)?,
            reassembler: PlcMessageReassembler::new(),
            configuration: ReceiverConfigurationState::new(bootstrap_spacing),
        })
    }
    /// Borrows the readiness gate and received immutable configuration.
    /// # Returns
    /// Read-only state; callers use require_ready before configuring data/NCP stages.
    pub fn configuration(&self) -> &ReceiverConfigurationState {
        &self.configuration
    }
    /// Receives one equalized known-phase local PLC grid and admits its descriptors.
    /// # Arguments
    /// * `iq` - Complete 128-symbol local PLC grid, data phase 0 first.
    /// * `noise_power` - Positive finite complex demapper variance in normalized units.
    /// * `max_iterations` - Per-word LDPC work limit, 1..100.
    /// # Returns
    /// Completed decoder results and packet decisions, even when message parsing,
    /// transport or individual admission fails. Invalid input closes the data gate.
    /// # Errors
    /// Returns pre-decode shape/numeric/work-limit or codec API failures; accepted
    /// configuration/history is retained, with Rejected status and diagnostic.
    pub fn receive_frame(
        &mut self,
        iq: &[IqPair],
        noise_power: f64,
        max_iterations: usize,
    ) -> Result<PlcAcquisitionResult, SimulationError> {
        let frame = match self.frame_receiver.decode(iq, noise_power, max_iterations) {
            Ok(frame) => frame,
            Err(error) => {
                self.configuration.reject(error.clone());
                return Err(error);
            }
        };
        let mut admissions = Vec::new();
        let mut transport_error = None;
        if let Some(message) = &frame.message {
            match self.reassembler.push(message.message_channel()) {
                Ok(packets) => {
                    for packet in packets {
                        let error = self.configuration.accept_packet(&packet).err();
                        admissions.push(DescriptorAdmission {
                            message_type: packet.message_type(),
                            packet,
                            status: self.configuration.status(),
                            error,
                        });
                    }
                }
                Err(error) => {
                    self.configuration.reject(error.clone());
                    transport_error = Some(error);
                }
            }
        } else if let Some(error) = &frame.message_error {
            self.configuration.reject(error.clone());
        }
        Ok(PlcAcquisitionResult {
            frame,
            admissions,
            transport_error,
            configuration_status: self.configuration.status(),
        })
    }
    /// Starts a fresh fixed session, discarding partial transport and old configuration.
    /// # Returns
    /// Unit with Waiting status; reusable decoder setup and bootstrap spacing remain.
    pub fn reset(&mut self) {
        self.reassembler = PlcMessageReassembler::new();
        self.configuration.reset();
    }
}
