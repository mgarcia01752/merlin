/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fixed OCD/ProfileA/NCP DPD, MULPI I25 §6.4.40–41, Tables69/70.
use super::mac_packets::{ManagementPacket, DPD_MESSAGE_TYPE, OCD_MESSAGE_TYPE};
use crate::{
    error::invalid,
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES},
        SubcarrierSpacing, TransmissionSettings,
    },
    types::{ChannelId, MacAddress},
    LabError,
};
use serde::Serialize;
const ASSIGNMENT_TLV: u8 = 5;
const CONTINUOUS_PILOT: u8 = 1;
const EXCLUDED: u8 = 16;
const PLC: u8 = 20;
const DEFAULT_ASSIGNMENT: u8 = 0x20;
/// Explicit fixed transmitter identity/configuration, unrelated to host settings.
#[derive(Debug, Clone, Copy)]
pub struct DescriptorIdentity {
    pub channel_id: ChannelId,
    pub change_count: u8,
    pub source_mac: MacAddress,
    pub primary_capable: bool,
    /// Uniform NCP loading:2(QPSK),4(16-QAM) or6(64-QAM).
    pub ncp_bits: u8,
}
/// Three immutable acquisition packets, built from the same validated physical map.
#[derive(Debug, Clone, Serialize)]
pub struct FixedPlcDescriptors {
    packets: Vec<ManagementPacket>,
    settings: TransmissionSettings,
}
impl FixedPlcDescriptors {
    /// Builds OCD and DPDs for uniform256-QAM ProfileA and uniform NCP modulation.
    /// # Arguments
    /// * `settings` - Valid fixed transmission settings.
    /// * `layout` - Same-spacing physical map with all mandatory excluded edges.
    /// * `identity` - Explicit channel/change/source fields and NCP bit loading.
    /// # Returns
    /// OCD,ProfileA,NCP packets in that order, each a bounded single-part MMM.
    /// # Errors
    /// Rejects spacing mismatch, missing edge exclusions, off-grid/zero frequency,
    /// unsupported NCP loading, excessive TLVs or packet size.
    pub fn new(
        settings: &TransmissionSettings,
        layout: &CarrierLayout,
        identity: DescriptorIdentity,
    ) -> Result<Self, LabError> {
        settings.validate()?;
        if settings.subcarrier_spacing != layout.subcarrier_spacing()
            || ![2, 4, 6].contains(&identity.ncp_bits)
        {
            return Err(invalid(
                "Descriptor spacing or NCP loading is inconsistent.",
            ));
        }
        let zero = layout
            .frequency_hz(0)
            .ok_or_else(|| invalid("Missing FFT frequency origin."))?;
        if zero == 0 || zero % settings.subcarrier_spacing.frequency_hz() != 0 {
            return Err(invalid(
                "Descriptor frequency origin must be positive and spacing-aligned.",
            ));
        }
        let roles = layout.static_roles();
        let edge = settings.subcarrier_spacing.mandatory_excluded_edge_bins();
        if roles[..edge]
            .iter()
            .chain(roles[roles.len() - edge..].iter())
            .any(|role| *role != PhysicalCarrierRole::Excluded)
        {
            return Err(invalid(
                "OCD requires mandatory outer6.4MHz edge exclusions.",
            ));
        }
        let mut ocd = vec![identity.channel_id, identity.change_count];
        push_tlv(
            &mut ocd,
            0,
            &[u8::from(
                settings.subcarrier_spacing == SubcarrierSpacing::Khz25,
            )],
        );
        let prefix = CYCLIC_PREFIX_SAMPLES
            .iter()
            .position(|value| *value == settings.cyclic_prefix_samples)
            .ok_or_else(|| invalid("Unsupported descriptor CP."))?;
        let roll = ROLL_OFF_SAMPLES
            .iter()
            .position(|value| *value == settings.roll_off_samples)
            .ok_or_else(|| invalid("Unsupported descriptor rolloff."))?;
        push_tlv(&mut ocd, 1, &[prefix as u8]);
        push_tlv(&mut ocd, 2, &[roll as u8]);
        push_tlv(&mut ocd, 3, &zero.to_be_bytes());
        push_tlv(&mut ocd, 4, &[settings.time_interleaver_depth]);
        let mut start = 0;
        while start < roles.len() {
            let role = roles[start];
            let mut end = start + 1;
            while end < roles.len() && roles[end] == role {
                end += 1;
            }
            let assignment = match role {
                PhysicalCarrierRole::Excluded => Some(EXCLUDED),
                PhysicalCarrierRole::ContinuousPilot => Some(CONTINUOUS_PILOT),
                PhysicalCarrierRole::Plc => Some(PLC),
                _ => None,
            };
            if let Some(value) = assignment {
                push_range(&mut ocd, value, start, end - 1);
            }
            start = end;
        }
        push_tlv(&mut ocd, 6, &[u8::from(identity.primary_capable)]);
        let mut packets = vec![ManagementPacket::new(
            identity.source_mac,
            OCD_MESSAGE_TYPE,
            &ocd,
        )?];
        for (profile, bits) in [(0, 8), (255, identity.ncp_bits)] {
            let mut dpd = vec![identity.channel_id, profile, identity.change_count];
            // Defaults cover all bins; OCD specific roles take precedence (MULPI6.4.41.1).
            push_range(&mut dpd, DEFAULT_ASSIGNMENT | bits, 0, roles.len() - 1);
            packets.push(ManagementPacket::new(
                identity.source_mac,
                DPD_MESSAGE_TYPE,
                &dpd,
            )?);
        }
        Ok(Self {
            packets,
            settings: settings.clone(),
        })
    }
    /// Borrows the fixed numerology used to construct this acquisition set.
    /// # Returns
    /// Immutable source settings for repetition-cadence consistency checks.
    pub fn settings(&self) -> &TransmissionSettings {
        &self.settings
    }

    /// Borrows the complete ordered acquisition set without duplicating payloads.
    /// # Returns
    /// Exactly three CRC-protected management packets.
    pub fn packets(&self) -> &[ManagementPacket] {
        &self.packets
    }
}
fn push_tlv(output: &mut Vec<u8>, kind: u8, value: &[u8]) {
    output.extend_from_slice(&[kind, value.len() as u8]);
    output.extend_from_slice(value);
}
fn push_range(output: &mut Vec<u8>, assignment: u8, start: usize, end: usize) {
    let mut value = vec![assignment];
    value.extend_from_slice(&(start as u16).to_be_bytes());
    value.extend_from_slice(&(end as u16).to_be_bytes());
    push_tlv(output, ASSIGNMENT_TLV, &value);
}

/// A validated TLV in the fixed single-part descriptor subset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DescriptorTlv {
    pub kind: u8,
    pub value: Vec<u8>,
}
/// Received descriptor identity and TLVs, with no transmitter configuration supplied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ParsedDescriptor {
    pub channel_id: ChannelId,
    pub change_count: u8,
    pub profile_id: Option<u8>,
    pub tlvs: Vec<DescriptorTlv>,
}
/// Parses bounded OCD/DPD fixed fields and range TLVs after packet CRC validation.
/// # Arguments
/// * `packet` - Validated generated-subset acquisition packet.
/// # Returns
/// Typed received identity, explicit profile(None for OCD) and preserved TLVs.
/// # Errors
/// Rejects malformed/truncated TLVs, missing mandatory fields or unsupported forms.
pub fn parse(packet: &ManagementPacket) -> Result<ParsedDescriptor, LabError> {
    let body = packet.body();
    let fixed = if packet.message_type() == OCD_MESSAGE_TYPE {
        2
    } else {
        3
    };
    if body.len() < fixed {
        return Err(invalid("Truncated descriptor fixed fields."));
    }
    let mut offset = fixed;
    let mut tlvs = Vec::new();
    while offset < body.len() {
        if offset + 2 > body.len() {
            return Err(invalid("Truncated descriptor TLV header."));
        }
        let kind = body[offset];
        let length = usize::from(body[offset + 1]);
        offset += 2;
        if length == 0 || offset + length > body.len() {
            return Err(invalid("Truncated descriptor TLV value."));
        }
        let value = &body[offset..offset + length];
        let valid = if kind == ASSIGNMENT_TLV {
            length == 5
                && value[0] & 0xc0 == 0
                && u16::from_be_bytes([value[1], value[2]])
                    <= u16::from_be_bytes([value[3], value[4]])
        } else {
            packet.message_type() == OCD_MESSAGE_TYPE
                && match kind {
                    0..=2 | 4 | 6 => length == 1,
                    3 => length == 4,
                    _ => false,
                }
        };
        if !valid {
            return Err(invalid("Unsupported fixed descriptor TLV."));
        }
        tlvs.push(DescriptorTlv {
            kind,
            value: value.to_vec(),
        });
        offset += length;
    }
    if packet.message_type() == OCD_MESSAGE_TYPE {
        for kind in [0, 1, 2, 3, 4, 6] {
            if tlvs.iter().filter(|tlv| tlv.kind == kind).count() != 1 {
                return Err(invalid("OCD requires one of each fixed setting TLV."));
            }
        }
    } else if ![0, 255].contains(&body[1]) || tlvs.iter().all(|tlv| tlv.kind != ASSIGNMENT_TLV) {
        return Err(invalid(
            "Expected ProfileA or NCP descriptor with assignments.",
        ));
    }
    if packet.message_type() == OCD_MESSAGE_TYPE {
        super::receiver_configuration::channel::decode(&tlvs)?;
    } else {
        for tlv in &tlvs {
            let code = tlv.value[0];
            let bits = code & 15;
            let allowed = if body[1] == 255 {
                [0, 2, 4, 6].contains(&bits)
            } else {
                [0, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14].contains(&bits)
            };
            if code & !0x2f != 0
                || !allowed
                || u16::from_be_bytes([tlv.value[3], tlv.value[4]]) > 8191
            {
                return Err(invalid("Reserved DPD bit loading, flags or carrier index."));
            }
        }
    }
    Ok(ParsedDescriptor {
        channel_id: body[0],
        change_count: body[fixed - 1],
        profile_id: if fixed == 3 { Some(body[1]) } else { None },
        tlvs,
    })
}
