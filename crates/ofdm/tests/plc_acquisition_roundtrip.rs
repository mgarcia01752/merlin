/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real descriptor frames cross MC/FEC/IQ boundaries; RX has no TX packet IDs.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::plc::{generate_frame, PlcFrameReceiver},
    transmission::{
        layout::CarrierLayout,
        plc::{
            descriptors::{parse, DescriptorIdentity, FixedPlcDescriptors},
            reassembly::PlcMessageReassembler,
            scheduling::PlcMessageScheduler,
            ExtendedTimestamp, PlcMessageFrame,
        },
        SubcarrierSpacing,
    },
};
#[test]
fn fragmented_acquisition_packets_recover_after_real_fec_qam_and_soft_inverse() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let (settings, raw) = acquisition_support::acquisition_setup(spacing, true);
        let layout = CarrierLayout::new(&settings, &raw).unwrap();
        let identity = DescriptorIdentity {
            channel_id: 7,
            change_count: 3,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: 4,
        };
        let descriptors = FixedPlcDescriptors::new(&settings, &layout, identity).unwrap();
        let mut scheduler = PlcMessageScheduler::new(&settings, &descriptors).unwrap();
        let mut receiver = PlcFrameReceiver::new(spacing).unwrap();
        let mut reassembler = PlcMessageReassembler::new();
        let mut packets = Vec::new();
        let mut saw_continuation = false;
        let frame_samples =
            u64::from(settings.one_second_timing().unwrap().symbol_period_samples) * 128;
        for frame_index in 0..5 {
            let channel = scheduler.next_block().unwrap();
            if frame_index == 1 {
                assert!(channel
                    .block
                    .packet_start_pointer()
                    .is_none_or(|offset| offset > 0));
                saw_continuation = true;
            }
            let timestamp =
                ExtendedTimestamp::from_sample_ticks(frame_index * frame_samples, 0).unwrap();
            let message = PlcMessageFrame::new(spacing, timestamp, channel.block).unwrap();
            let truth = message.to_bytes();
            let frame = generate_frame(message, frame_index).unwrap();
            // Only a copied received local PLC grid enters the receiver.
            let samples: Vec<_> = (0..128)
                .flat_map(|phase| {
                    frame
                        .symbol_iq(frame_index * 128 + phase)
                        .unwrap()
                        .iter()
                        .copied()
                })
                .collect();
            let decoded = receiver.decode(&samples, 0.01, 100).unwrap();
            assert!(decoded.preamble_signs_match);
            assert!(decoded.blocks.iter().all(|word| word.success));
            let received = decoded.message.unwrap();
            assert_eq!(received.to_bytes(), truth);
            let bytes = received.to_bytes();
            let mc =
                docsis_ofdm_fec::transmission::plc::MessageChannelBlock::from_bytes(&bytes[12..])
                    .unwrap();
            packets.extend(reassembler.push(&mc).unwrap());
        }
        assert!(saw_continuation);
        assert_eq!(packets, descriptors.packets());
        assert!(reassembler.pending().is_empty());
        assert_eq!(parse(&packets[0]).unwrap().profile_id, None);
        assert_eq!(parse(&packets[1]).unwrap().profile_id, Some(0));
        assert_eq!(parse(&packets[2]).unwrap().profile_id, Some(255));
    }
}
#[test]
fn receiver_retains_failed_decisions_and_rejects_invalid_limits() {
    let mut rx = PlcFrameReceiver::new(SubcarrierSpacing::Khz50).unwrap();
    let samples = vec![[0.0, 0.0]; 128 * 8];
    let result = rx.decode(&samples, 0.01, 5).unwrap();
    assert!(result.message.is_none());
    assert!(result.message_error.is_some());
    assert_eq!(result.blocks.len(), 10);
    assert!(!result.preamble_signs_match);
    assert!(rx.decode(&samples[..100], 0.01, 5).is_err());
    assert!(rx.decode(&samples, f64::NAN, 5).is_err());
    assert!(rx.decode(&samples, 0.01, 0).is_err());
}
