/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded single-part acquisition MAC packets, MULPI I25 §6.2.3.2/6.4.1.
use super::checksums::{header_crc, message_crc};
use crate::{error::invalid, types::MacAddress, SimulationError};
use serde::Serialize;
/// Management PDU maximum, including DA/SA and FCS; no extended header supported.
pub const MAX_MANAGEMENT_PDU_BYTES: usize = 2000;
/// Four MAC header octets plus two-byte HCS.
pub const MAC_HEADER_BYTES: usize = 6;
const MANAGEMENT_HEADER_BYTES: usize = 20;
const FCS_BYTES: usize = 4;
pub(super) const MANAGEMENT_FC: u8 = 0xc2;
/// Smallest supported frame, including one descriptor-body byte.
pub const MIN_MANAGEMENT_FRAME_BYTES: usize =
    MAC_HEADER_BYTES + MANAGEMENT_HEADER_BYTES + FCS_BYTES + 1;
/// MULPI I25 Table26 acquisition management message identifiers.
pub const OCD_MESSAGE_TYPE: u8 = 49;
pub const DPD_MESSAGE_TYPE: u8 = 50;
const MANAGEMENT_VERSION: u8 = 5;
/// DOCSIS all-CMs multicast address, MULPI I25 AnnexA.
pub const ALL_CMS: MacAddress = [0x01, 0xe0, 0x2f, 0, 0, 1];
/// An immutable complete MAC frame suitable for the PLC message channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ManagementPacket {
    bytes: Vec<u8>,
}
impl ManagementPacket {
    /// Encapsulates one fixed descriptor with outer MAC header, HCS and message FCS.
    /// # Arguments
    /// * `source` - Caller's synthetic CMTS MAC-domain address, unicast/nonzero.
    /// * `message_type` -49(OCD) or50(DPD), version5, single-part only.
    /// * `body` - Descriptor fixed fields and TLVs, at most1976bytes.
    /// # Returns
    /// Owned complete frame; no extended header, secrets or host identity are added.
    /// # Errors
    /// Rejects unsupported type, invalid source or oversized/empty descriptor.
    pub fn new(source: MacAddress, message_type: u8, body: &[u8]) -> Result<Self, SimulationError> {
        if ![OCD_MESSAGE_TYPE, DPD_MESSAGE_TYPE].contains(&message_type)
            || source[0] & 1 != 0
            || source == [0; 6]
            || body.is_empty()
            || body.len() > MAX_MANAGEMENT_PDU_BYTES - MANAGEMENT_HEADER_BYTES - FCS_BYTES
        {
            return Err(invalid(
                "Invalid fixed PLC management packet fields or size.",
            ));
        }
        let mut pdu = Vec::new();
        pdu.extend_from_slice(&ALL_CMS);
        pdu.extend_from_slice(&source);
        pdu.extend_from_slice(&((6 + body.len()) as u16).to_be_bytes());
        pdu.extend_from_slice(&[0, 0, 3, MANAGEMENT_VERSION, message_type, 0]);
        pdu.extend_from_slice(body);
        pdu.extend_from_slice(&message_crc(&pdu).to_le_bytes());
        let mut bytes = vec![MANAGEMENT_FC, 0];
        bytes.extend_from_slice(&(pdu.len() as u16).to_be_bytes());
        bytes.extend_from_slice(&header_crc(&bytes).to_le_bytes());
        bytes.extend_from_slice(&pdu);
        Ok(Self { bytes })
    }
    /// Validates a complete generated-subset packet, without using transmitter truth.
    /// # Arguments
    /// * `bytes` - One entire outer MAC frame through the message FCS.
    /// # Returns
    /// Owned CRC-validated single-part OCD/DPD packet; TLVs are parsed separately.
    /// # Errors
    /// Rejects size/length mismatch, HCS/FCS, header/type/version/address or multipart.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, SimulationError> {
        if bytes.len() < MAC_HEADER_BYTES + MANAGEMENT_HEADER_BYTES + FCS_BYTES + 1
            || bytes.len() > MAC_HEADER_BYTES + MAX_MANAGEMENT_PDU_BYTES
            || bytes[0..2] != [MANAGEMENT_FC, 0]
        {
            return Err(invalid("Invalid PLC management MAC frame size or format."));
        }
        let pdu = &bytes[MAC_HEADER_BYTES..];
        let header_ok = usize::from(u16::from_be_bytes([bytes[2], bytes[3]])) == pdu.len()
            && bytes[4..6] == header_crc(&bytes[..4]).to_le_bytes();
        let length = usize::from(u16::from_be_bytes([pdu[12], pdu[13]]));
        let fcs_start = pdu.len() - FCS_BYTES;
        if !header_ok
            || length + 14 + FCS_BYTES != pdu.len()
            || pdu[..6] != ALL_CMS
            || pdu[6] & 1 != 0
            || pdu[6..12] == [0; 6]
            || pdu[14..18] != [0, 0, 3, MANAGEMENT_VERSION]
            || ![OCD_MESSAGE_TYPE, DPD_MESSAGE_TYPE].contains(&pdu[18])
            || pdu[19] != 0
            || pdu[fcs_start..] != message_crc(&pdu[..fcs_start]).to_le_bytes()
        {
            log::debug!("plc-management-packet-rejected reason=header-or-crc");
            return Err(invalid(
                "PLC management packet header, length or CRC failed.",
            ));
        }
        Ok(Self {
            bytes: bytes.to_vec(),
        })
    }
    /// Borrows complete transmission-order frame bytes including both checksums.
    /// # Returns
    /// At most2006 bytes tied to this packet's lifetime.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Reports the validated MAC management message type.
    /// # Returns
    ///49(OCD) or50(DPD).
    pub fn message_type(&self) -> u8 {
        self.bytes[MAC_HEADER_BYTES + 18]
    }
    /// Copies the validated packet's CMTS MAC-domain source address.
    /// # Returns
    /// Six source octets from the received management PDU, never the host address.
    pub fn source_address(&self) -> MacAddress {
        let mut source = [0; 6];
        source.copy_from_slice(&self.bytes[MAC_HEADER_BYTES + 6..MAC_HEADER_BYTES + 12]);
        source
    }
    /// Borrows the descriptor payload without MAC/LLC headers or FCS.
    /// # Returns
    /// Unchanged fixed fields and TLV bytes, without interpreting their values.
    pub fn body(&self) -> &[u8] {
        &self.bytes[MAC_HEADER_BYTES + MANAGEMENT_HEADER_BYTES..self.bytes.len() - FCS_BYTES]
    }
}
