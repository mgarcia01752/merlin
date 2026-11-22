/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Typed inspection of the existing validated fixed descriptor subset.
use super::*;
use crate::transmission::plc::{
    mac_packets::{MAC_HEADER_BYTES, MANAGEMENT_HEADER_BYTES},
    receiver_configuration::channel,
};
use crate::types::ofdm_modulation::OfdmModulation;

/// Exact wire TLV and offset plus semantic values supplied by the descriptor owner.
#[derive(Serialize)]
#[non_exhaustive]
pub struct DescriptorField {
    pub kind: u8,
    /// Absolute type-byte offset in the CRC-validated complete MAC packet.
    pub offset: usize,
    pub value: Vec<u8>,
    pub meaning: DescriptorMeaning,
}
/// Accepted fixed-subset meanings, separate from display labels and endpoint capabilities.
#[derive(Serialize)]
#[non_exhaustive]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DescriptorMeaning {
    Numerology {
        fft_samples: u32,
        spacing_hz: u32,
    },
    CyclicPrefix {
        samples: u16,
    },
    RollOff {
        samples: u16,
    },
    SubcarrierZeroFrequency {
        frequency_hz: u32,
    },
    InterleaverDepth {
        symbols: u8,
    },
    PrimaryCapability {
        enabled: bool,
    },
    PhysicalAssignment {
        role: PhysicalCarrierRole,
        first: u16,
        last: u16,
    },
    ProfileAssignment {
        default: bool,
        bits_per_carrier: u8,
        /// None denotes explicit zero loading, never an unsupported substituted constellation.
        modulation_order: Option<u16>,
        first: u16,
        last: u16,
    },
}
/// Owned decoded identity/fields from a CRC-validated actual management packet.
#[derive(Serialize)]
#[non_exhaustive]
pub struct DescriptorInspection {
    pub channel_id: ChannelId,
    pub profile_id: Option<u8>,
    pub change_count: u8,
    pub fields: Vec<DescriptorField>,
}

pub(crate) fn inspect(packet: &ManagementPacket) -> Result<DescriptorInspection, SimulationError> {
    let descriptor = parse(packet)?;
    let received = if descriptor.profile_id.is_none() {
        Some(channel::decode(&descriptor.tlvs)?)
    } else {
        None
    };
    let mut offset = MAC_HEADER_BYTES
        + MANAGEMENT_HEADER_BYTES
        + if descriptor.profile_id.is_none() {
            2
        } else {
            3
        };
    let mut fields = Vec::with_capacity(descriptor.tlvs.len());
    for tlv in descriptor.tlvs {
        let meaning = if tlv.kind == ASSIGNMENT_TLV {
            let code = tlv.value[0];
            let first = u16::from_be_bytes([tlv.value[1], tlv.value[2]]);
            let last = u16::from_be_bytes([tlv.value[3], tlv.value[4]]);
            if received.is_some() {
                let role = match code {
                    CONTINUOUS_PILOT => PhysicalCarrierRole::ContinuousPilot,
                    EXCLUDED => PhysicalCarrierRole::Excluded,
                    PLC => PhysicalCarrierRole::Plc,
                    _ => {
                        return Err(invalid("Unsupported validated OCD assignment"));
                    }
                };
                DescriptorMeaning::PhysicalAssignment { role, first, last }
            } else {
                let bits_per_carrier = code & 15;
                let modulation_order = match bits_per_carrier {
                    0 => None,
                    2 => Some(4),
                    bits => Some(
                        OfdmModulation::from_bits(bits)
                            .ok_or_else(|| invalid("Unsupported validated DPD modulation"))?
                            .order,
                    ),
                };
                DescriptorMeaning::ProfileAssignment {
                    default: code & 0x20 != 0,
                    bits_per_carrier,
                    modulation_order,
                    first,
                    last,
                }
            }
        } else {
            let received = received
                .as_ref()
                .ok_or_else(|| invalid("DPD contains an OCD field"))?;
            let settings = received.transmission();
            match tlv.kind {
                0 => DescriptorMeaning::Numerology {
                    fft_samples: settings.subcarrier_spacing.fft_samples(),
                    spacing_hz: settings.subcarrier_spacing.frequency_hz(),
                },
                1 => DescriptorMeaning::CyclicPrefix {
                    samples: settings.cyclic_prefix_samples,
                },
                2 => DescriptorMeaning::RollOff {
                    samples: settings.roll_off_samples,
                },
                3 => DescriptorMeaning::SubcarrierZeroFrequency {
                    frequency_hz: received.layout_settings().zero_frequency_hz,
                },
                4 => DescriptorMeaning::InterleaverDepth {
                    symbols: settings.time_interleaver_depth,
                },
                6 => DescriptorMeaning::PrimaryCapability {
                    enabled: received.primary_capable(),
                },
                _ => {
                    return Err(invalid("Unsupported validated OCD field"));
                }
            }
        };
        let length = tlv.value.len();
        fields.push(DescriptorField {
            kind: tlv.kind,
            offset,
            value: tlv.value,
            meaning,
        });
        offset += 2 + length;
    }
    Ok(DescriptorInspection {
        channel_id: descriptor.channel_id,
        profile_id: descriptor.profile_id,
        change_count: descriptor.change_count,
        fields,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fixed_ocd_views_reuse_received_timing_and_preserve_all_original_wire_offsets() {
        for (spacing, divisor, fft, spacing_hz) in [
            (SubcarrierSpacing::Khz25, 1, 8192, 25000),
            (SubcarrierSpacing::Khz50, 2, 4096, 50000),
        ] {
            // Shared independent 24 MHz physical fixture from plc_descriptors.rs.
            let settings = TransmissionSettings {
                occupied_bandwidth_hz: 24_000_000,
                subcarrier_spacing: spacing,
                ..Default::default()
            };
            let layout = CarrierLayout::new(
                &settings,
                &crate::transmission::layout::CarrierLayoutSettings {
                    zero_frequency_hz: 100_000_000,
                    first_active_bin: 320 / divisor,
                    active_end_exclusive: 1280 / divisor,
                    plc_first_bin: 752 / divisor,
                    excluded_bins: vec![1200 / divisor],
                    additional_continuous_pilots: [352, 448, 544, 920, 1008, 1080, 1176, 1256]
                        .map(|bin| bin / divisor)
                        .to_vec(),
                },
            )
            .unwrap();
            let descriptors = FixedPlcDescriptors::new(
                &settings,
                &layout,
                DescriptorIdentity {
                    channel_id: 7,
                    change_count: 3,
                    source_mac: [0, 0xca, 0, 0, 0, 1],
                    primary_capable: true,
                    ncp_bits: 4,
                },
            )
            .unwrap();
            let packet = &descriptors.packets()[0];
            let view = inspect(packet).unwrap();
            assert_eq!(view.channel_id, 7);
            assert_eq!(view.change_count, 3);
            assert_eq!(view.profile_id, None);
            assert!(
                matches!(view.fields[0].meaning,DescriptorMeaning::Numerology {
                fft_samples,spacing_hz:hz} if fft_samples==fft && hz==spacing_hz)
            );
            assert!(matches!(
                view.fields[1].meaning,
                DescriptorMeaning::CyclicPrefix { samples: 512 }
            ));
            assert!(matches!(
                view.fields[2].meaning,
                DescriptorMeaning::RollOff { samples: 128 }
            ));
            assert!(matches!(
                view.fields[3].meaning,
                DescriptorMeaning::SubcarrierZeroFrequency {
                    frequency_hz: 100_000_000
                }
            ));
            assert!(matches!(
                view.fields[4].meaning,
                DescriptorMeaning::InterleaverDepth { symbols: 16 }
            ));
            assert!(matches!(
                view.fields.last().unwrap().meaning,
                DescriptorMeaning::PrimaryCapability { enabled: true }
            ));
            assert_eq!(view.fields[0].offset, 28);
            assert_eq!(view.fields[1].offset, 31);
            for field in &view.fields {
                assert_eq!(packet.bytes()[field.offset], field.kind);
                assert_eq!(packet.bytes()[field.offset + 1] as usize, field.value.len());
                assert_eq!(
                    &packet.bytes()[field.offset + 2..field.offset + 2 + field.value.len()],
                    field.value
                );
            }
            let mut bad = packet.body().to_vec();
            bad[7] = 255;
            let bad = ManagementPacket::new(packet.source_address(), packet.message_type(), &bad)
                .unwrap();
            assert!(inspect(&bad).is_err());
        }
    }

    #[test]
    fn descriptor_zero_loading_and_ncp_qpsk_keep_distinct_typed_meanings() {
        for (profile, code, expected) in [
            (0, 0x20, None),
            (255, 0x22, Some(4)),
            (0, 0x2e, Some(16384)),
        ] {
            let packet = ManagementPacket::new(
                [0, 0xca, 0, 0, 0, 1],
                50,
                &[7, profile, 3, 5, 5, code, 0, 0, 0, 31],
            )
            .unwrap();
            let view = inspect(&packet).unwrap();
            assert!(
                matches!(view.fields[0].meaning,DescriptorMeaning::ProfileAssignment {
                modulation_order,default:true,first:0,last:31,..} if modulation_order==expected)
            );
        }
    }
}
