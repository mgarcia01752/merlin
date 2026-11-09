/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
mod payload_support;
use docsis_ofdm_fec::{
    simulation::{
        payload::{PayloadReceiver, PayloadTransmitter},
        source::SeededSource,
    },
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        ncp::activation::{NcpTransitionReceiver, NcpTransitionSchedule, NCP_UPDATE_PATTERN},
        packing::NcpModulation,
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            mac_packets::{ManagementPacket, DPD_MESSAGE_TYPE},
        },
        SubcarrierSpacing,
    },
};
use fec_codec::concatenated::compare_payload;
#[test]
fn exact_announcement_and_rejection_do_not_mutate_received_history() {
    let schedule = NcpTransitionSchedule::new(12, 140, false).unwrap();
    assert!(NcpTransitionSchedule::new(13, 140, false).is_err());
    let mut receiver = NcpTransitionReceiver::new(false);
    let mut packed = 0u128;
    for symbol in 12..140 {
        let (odd, update) = schedule.flags(symbol);
        packed = (packed << 1) | u128::from(update);
        assert!(!receiver.expected_odd());
        assert!(receiver.accept(true, update).is_err());
        receiver.accept(odd, update).unwrap();
    }
    assert_eq!(packed, NCP_UPDATE_PATTERN);
    assert!(receiver.expected_odd());
    assert!(receiver.accept(false, false).is_err());
    receiver.accept(true, false).unwrap();
    assert!(receiver.expected_odd());
    assert_eq!(schedule.flags(140), (true, false));
    let mut corrupt = NcpTransitionReceiver::new(false);
    for index in 0..128 {
        let bit = NCP_UPDATE_PATTERN & (1u128 << (127 - index)) != 0;
        corrupt.accept(false, bit ^ (index == 70)).unwrap();
    }
    assert!(!corrupt.expected_odd());
    assert!(corrupt.accept(true, false).is_err());
}
#[test]
fn received_dpd_switches_data_and_ncp_through_continuous_cell_transport() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (mut settings, raw) = acquisition_support::acquisition_setup(spacing, false);
        settings.time_interleaver_depth = 8;
        let state = payload_support::ready(&settings, &raw, NcpModulation::Qpsk);
        let layout = CarrierLayout::new(&settings, &raw).unwrap();
        let loading: Vec<_> = layout
            .static_roles()
            .iter()
            .map(|role| {
                if *role == PhysicalCarrierRole::Data {
                    6
                } else {
                    0
                }
            })
            .collect();
        let descriptors = FixedPlcDescriptors::with_data_loading(
            &settings,
            &layout,
            DescriptorIdentity {
                channel_id: 7,
                change_count: 1,
                source_mac: [0, 0xca, 0, 0, 0, 1],
                primary_capable: true,
                ncp_bits: 4,
            },
            Some(&loading),
        )
        .unwrap();
        let mut body = descriptors.packets()[1].body().to_vec();
        body[1] = 2;
        let data_packet =
            ManagementPacket::new([0, 0xca, 0, 0, 0, 1], DPD_MESSAGE_TYPE, &body).unwrap();
        let mut tx = PayloadTransmitter::new(&settings, &raw, NcpModulation::Qpsk, 0).unwrap();
        let mut rx = PayloadReceiver::with_dynamic_profiles(&state, 0).unwrap();
        assert!(rx.admit_profile_packet(data_packet.bytes()).unwrap());
        assert!(!rx.admit_profile_packet(data_packet.bytes()).unwrap());
        assert!(rx
            .admit_profile_packet(descriptors.packets()[2].bytes())
            .unwrap());
        // Same parity is not sufficient to replace a still-addressable generation.
        body[2] = 3;
        let conflict =
            ManagementPacket::new([0, 0xca, 0, 0, 0, 1], DPD_MESSAGE_TYPE, &body).unwrap();
        assert!(rx.admit_profile_packet(conflict.bytes()).is_err());
        tx.schedule_ncp_change(128, NcpModulation::Qam16).unwrap();
        let mut source = SeededSource::new(87);
        let payloads: Vec<_> = (0..3).map(|_| source.next_payload()).collect();
        let mut recovered = 0;
        let mut crossed_ncp = false;
        for symbol in 0..150 {
            if symbol == 0 {
                tx.enqueue(&payloads[0]).unwrap();
                assert!(tx.activate_data_profile(2, 1, &loading).is_err());
            }
            if symbol == 20 {
                assert_eq!(tx.queued_codewords(), 0);
                tx.activate_data_profile(2, 1, &loading).unwrap();
                tx.enqueue(&payloads[1]).unwrap();
            }
            if symbol == 126 {
                tx.enqueue(&payloads[2]).unwrap();
            }
            let transmitted = tx.transmit().unwrap();
            assert_eq!(
                transmitted.assembled.ncp.messages[0].ncp_profile_odd,
                symbol >= 128
            );
            let received = rx
                .receive(&state, symbol, &transmitted.channel.reduced, 0.01, 50)
                .unwrap();
            if let Some(decoded) = received.decoded {
                assert!(decoded.error.is_none(), "{:?}", decoded.error);
                for word in decoded.recovered {
                    assert_eq!(word.profile_id, if recovered == 0 { 0 } else { 2 });
                    assert_eq!(word.receive_sequence, recovered as u64);
                    assert!(word.decoded.ldpc.success && word.decoded.bch.success);
                    assert_eq!(
                        compare_payload(&payloads[recovered], &word.decoded)
                            .unwrap()
                            .bit_errors,
                        0
                    );
                    crossed_ncp |= word.first_source_symbol < 128 && word.last_source_symbol >= 128;
                    recovered += 1;
                }
            }
        }
        assert_eq!(recovered, 3);
        assert_eq!(rx.pending_bits(), 0);
        assert!(crossed_ncp);
    }
}

#[test]
fn dynamic_dpd_is_recovered_over_real_plc_fec_without_closing_ocd_gate() {
    use docsis_ofdm_fec::{
        simulation::plc::{generate_frame, PlcAcquisitionReceiver},
        transmission::plc::{
            message_channel::MessageChannelBlock, ExtendedTimestamp, PlcMessageFrame,
        },
    };
    let (settings, raw) = acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let descriptors = FixedPlcDescriptors::new(
        &settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 0,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: 2,
        },
    )
    .unwrap();
    let mut packets = descriptors.packets().to_vec();
    let mut body = packets[1].body().to_vec();
    body[1] = 2;
    body[2] = 1;
    packets
        .push(ManagementPacket::new(packets[1].source_address(), DPD_MESSAGE_TYPE, &body).unwrap());
    let mut receiver = PlcAcquisitionReceiver::new(SubcarrierSpacing::Khz25).unwrap();
    assert!(receiver.enable_profile_updates().is_err());
    let mut payload = None;
    for (index, packet) in packets.iter().enumerate() {
        let mut segment = packet.bytes().to_vec();
        segment.resize(720 - 12 - 3, 0xff);
        let frame = generate_frame(
            PlcMessageFrame::new(
                SubcarrierSpacing::Khz25,
                ExtendedTimestamp::from_sample_ticks(index as u64 * 1_000_000, 0).unwrap(),
                MessageChannelBlock::new(&segment, Some(0)).unwrap(),
            )
            .unwrap(),
            index as u64,
        )
        .unwrap();
        let received: Vec<_> = (0..128)
            .flat_map(|phase| {
                frame
                    .symbol_iq(index as u64 * 128 + phase)
                    .unwrap()
                    .iter()
                    .copied()
            })
            .collect();
        let result = receiver.receive_frame(&received, 0.01, 50).unwrap();
        assert!(result.transport_error.is_none());
        assert_eq!(result.admissions.len(), 1);
        assert!(
            result.admissions[0].error.is_none(),
            "{:?}",
            result.admissions[0].error
        );
        if index == 2 {
            receiver.enable_profile_updates().unwrap();
            payload =
                Some(PayloadReceiver::with_dynamic_profiles(receiver.configuration(), 0).unwrap());
        }
        if index == 3 {
            assert!(receiver.configuration().require_ready().is_ok());
            assert!(payload
                .as_mut()
                .unwrap()
                .admit_profile_packet(result.admissions[0].packet.bytes())
                .unwrap());
        }
    }
}
