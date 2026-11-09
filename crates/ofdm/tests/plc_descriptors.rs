/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pinned descriptor fields, independent CRC check strings and received TLV ranges.
use docsis_ofdm_fec::transmission::{
    layout::{CarrierLayout, CarrierLayoutSettings},
    plc::{
        checksums::{header_crc, message_crc},
        descriptors::{parse, DescriptorIdentity, FixedPlcDescriptors},
        mac_packets::ManagementPacket,
        reassembly::PlcMessageReassembler,
        scheduling::PlcMessageScheduler,
    },
    SubcarrierSpacing, TransmissionSettings,
};
fn fixture(spacing: SubcarrierSpacing) -> (TransmissionSettings, CarrierLayout) {
    let divisor = if spacing == SubcarrierSpacing::Khz25 {
        1
    } else {
        2
    };
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        subcarrier_spacing: spacing,
        ..Default::default()
    };
    let layout = CarrierLayout::new(
        &settings,
        &CarrierLayoutSettings {
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
    (settings, layout)
}
fn identity() -> DescriptorIdentity {
    DescriptorIdentity {
        channel_id: 7,
        change_count: 3,
        source_mac: [0, 0xca, 0, 0, 0, 1],
        primary_capable: true,
        ncp_bits: 4,
    }
}
#[test]
fn crc_check_strings_and_packet_checksum_rejection() {
    assert_eq!(header_crc(b"123456789"), 0x906e);
    assert_eq!(message_crc(b"123456789"), 0xcbf43926);
    let packet = ManagementPacket::new(identity().source_mac, 49, &[7, 3]).unwrap();
    assert_eq!(&packet.bytes()[..4], &[0xc2, 0, 0, 26]);
    assert_eq!(&packet.bytes()[6..12], &[1, 0xe0, 0x2f, 0, 0, 1]);
    assert_eq!(&packet.bytes()[18..26], &[0, 8, 0, 0, 3, 5, 49, 0]);
    assert_eq!(
        ManagementPacket::from_bytes(packet.bytes()).unwrap(),
        packet
    );
    for index in 0..packet.bytes().len() {
        let mut corrupted = packet.bytes().to_vec();
        corrupted[index] ^= 1;
        assert!(ManagementPacket::from_bytes(&corrupted).is_err());
    }
    assert!(ManagementPacket::new([0; 6], 49, &[7, 3]).is_err());
}
#[test]
fn fixed_descriptors_include_edges_plc_all_pilots_and_profile_defaults() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (settings, layout) = fixture(spacing);
        let descriptors = FixedPlcDescriptors::new(&settings, &layout, identity()).unwrap();
        let ocd = parse(&descriptors.packets()[0]).unwrap();
        assert_eq!(ocd.channel_id, 7);
        assert_eq!(ocd.change_count, 3);
        assert_eq!(ocd.profile_id, None);
        let get = |kind| &ocd.tlvs.iter().find(|tlv| tlv.kind == kind).unwrap().value;
        assert_eq!(get(0), &[u8::from(spacing == SubcarrierSpacing::Khz25)]);
        assert_eq!(get(1), &[2]);
        assert_eq!(get(2), &[2]);
        assert_eq!(get(4), &[16]);
        assert_eq!(get(6), &[1]);
        assert_eq!(get(3), &100_000_000_u32.to_be_bytes());
        let mut assignment = vec![0; spacing.fft_samples() as usize];
        for tlv in ocd.tlvs.iter().filter(|tlv| tlv.kind == 5) {
            let start = u16::from_be_bytes([tlv.value[1], tlv.value[2]]) as usize;
            let end = u16::from_be_bytes([tlv.value[3], tlv.value[4]]) as usize;
            assignment[start..=end].fill(tlv.value[0]);
        }
        let edge = if spacing == SubcarrierSpacing::Khz25 {
            296
        } else {
            148
        };
        assert!(assignment[..edge].iter().all(|role| *role == 16));
        assert!(assignment[assignment.len() - edge..]
            .iter()
            .all(|role| *role == 16));
        assert_eq!(assignment.iter().filter(|role| **role == 1).count(), 16);
        assert_eq!(
            assignment.iter().filter(|role| **role == 20).count(),
            if spacing == SubcarrierSpacing::Khz25 {
                16
            } else {
                8
            }
        );
        for (packet, profile, bits) in [
            (&descriptors.packets()[1], 0, 8),
            (&descriptors.packets()[2], 255, 4),
        ] {
            let dpd = parse(packet).unwrap();
            assert_eq!(dpd.profile_id, Some(profile));
            let value = &dpd.tlvs[0].value;
            assert_eq!(value[0], 0x20 | bits);
            assert_eq!(&value[1..3], &[0, 0]);
            assert_eq!(
                u16::from_be_bytes([value[3], value[4]]),
                spacing.fft_samples() as u16 - 1
            );
        }
    }
}
#[test]
fn schedule_repeats_and_reassembles_without_tx_packet_boundaries() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (settings, layout) = fixture(spacing);
        let descriptors = FixedPlcDescriptors::new(&settings, &layout, identity()).unwrap();
        let mut scheduler = PlcMessageScheduler::new(&settings, &descriptors).unwrap();
        let period = scheduler.repeat_frames();
        let samples = u64::from(settings.one_second_timing().unwrap().symbol_period_samples) * 128;
        assert!(period * samples <= 40_960_000);
        assert!((period + 1) * samples > 40_960_000);
        let mut receiver = PlcMessageReassembler::new();
        let mut actual = Vec::new();
        for index in 0..period * 2 + 3 {
            let output = scheduler.next_block().unwrap();
            assert_eq!(output.frame_index, index);
            let expected = if spacing == SubcarrierSpacing::Khz25 {
                708
            } else {
                348
            };
            assert_eq!(output.block.encoded_len(), expected);
            for packet in receiver.push(&output.block).unwrap() {
                actual.push(packet);
            }
        }
        assert_eq!(actual.len(), 9);
        for cycle in actual.chunks_exact(3) {
            assert_eq!(cycle, descriptors.packets());
        }
        assert!(receiver.pending().is_empty());
    }
}

#[test]
fn reassembly_handles_split_headers_continuations_and_crc_failure_atomically() {
    use docsis_ofdm_fec::transmission::plc::MessageChannelBlock;
    let packet = ManagementPacket::new(identity().source_mac, 49, &vec![0x55; 1500]).unwrap();
    let mut receiver = PlcMessageReassembler::new();
    let mut position = 0;
    let mut recovered = Vec::new();
    for width in [1, 2, 1, 17, 300, 400, 600, 400] {
        let end = (position + width).min(packet.bytes().len());
        let pointer = if position == 0 { Some(0) } else { None };
        let block = MessageChannelBlock::new(&packet.bytes()[position..end], pointer).unwrap();
        recovered.extend(receiver.push(&block).unwrap());
        position = end;
        if position == packet.bytes().len() {
            break;
        }
    }
    assert_eq!(recovered, vec![packet.clone()]);
    assert!(receiver.pending().is_empty());
    let first = MessageChannelBlock::new(&packet.bytes()[..100], Some(0)).unwrap();
    receiver.push(&first).unwrap();
    let pending = receiver.pending().to_vec();
    let mut tail = packet.bytes()[100..].to_vec();
    let last = tail.len() - 1;
    tail[last] ^= 1;
    // Split into MC-sized pieces; rejection at the checksum must preserve the
    // pending state from before the failing block, not partially consume it.
    for chunk in tail.chunks(600) {
        let before = receiver.pending().to_vec();
        let block = MessageChannelBlock::new(chunk, None).unwrap();
        if receiver.push(&block).is_err() {
            assert_eq!(receiver.pending(), before);
        }
    }
    assert!(receiver.pending().starts_with(&pending));
    let mut fresh = PlcMessageReassembler::new();
    assert!(fresh
        .push(&MessageChannelBlock::new(&[0xc2, 0, 0, 26], None).unwrap())
        .is_err());
    let block = MessageChannelBlock::new(&[0xc2, 0, 255, 255], Some(0)).unwrap();
    assert!(fresh.push(&block).is_err());
    assert!(fresh.pending().is_empty());
}

#[test]
fn received_descriptor_semantics_and_scheduler_settings_are_checked() {
    let (settings, layout) = fixture(SubcarrierSpacing::Khz25);
    let descriptors = FixedPlcDescriptors::new(&settings, &layout, identity()).unwrap();
    let mut wrong = settings.clone();
    wrong.cyclic_prefix_samples = 256;
    assert!(PlcMessageScheduler::new(&wrong, &descriptors).is_err());
    let mut unsupported = identity();
    unsupported.ncp_bits = 8;
    assert!(FixedPlcDescriptors::new(&settings, &layout, unsupported).is_err());
    // Recompute a valid packet CRC around invalid descriptor contents. Packet
    // integrity must not substitute for semantic FFT/TLV validation.
    let original = &descriptors.packets()[0];
    for (offset, value) in [(4, 2), (7, 255), (10, 255)] {
        let mut body = original.body().to_vec();
        body[offset] = value;
        let packet = ManagementPacket::new(identity().source_mac, 49, &body).unwrap();
        assert!(parse(&packet).is_err());
    }
    let mut dpd = descriptors.packets()[2].body().to_vec();
    dpd[5] = 0x28;
    let packet = ManagementPacket::new(identity().source_mac, 50, &dpd).unwrap();
    assert!(parse(&packet).is_err());
    let packet = ManagementPacket::new(identity().source_mac, 49, &[7, 3, 0, 4, 1]).unwrap();
    assert!(parse(&packet).is_err());
}
