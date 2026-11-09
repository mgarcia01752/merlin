/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fixed admission identity, independent generations and readiness atomicity.
mod acquisition_support;
use docsis_ofdm_fec::transmission::{
    layout::{CarrierLayout, PhysicalCarrierRole},
    plc::{
        descriptors::{DescriptorIdentity, FixedPlcDescriptors},
        mac_packets::ManagementPacket,
        receiver_configuration::{ReceiverConfigurationState, ReceiverConfigurationStatus},
    },
    SubcarrierSpacing,
};
fn packets(spacing: SubcarrierSpacing) -> Vec<ManagementPacket> {
    let (settings, raw) = acquisition_support::acquisition_setup(spacing, false);
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    FixedPlcDescriptors::new(
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
    .unwrap()
    .packets()
    .to_vec()
}
#[test]
fn all_orders_wait_for_complete_set_then_build_independent_received_settings() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let packets = packets(spacing);
        for order in [
            [0, 1, 2],
            [0, 2, 1],
            [1, 0, 2],
            [1, 2, 0],
            [2, 0, 1],
            [2, 1, 0],
        ] {
            let mut receiver = ReceiverConfigurationState::new(spacing);
            assert_eq!(receiver.status(), ReceiverConfigurationStatus::Waiting);
            assert!(receiver.require_ready().is_err());
            for (step, index) in order.iter().enumerate() {
                let status = receiver.accept_bytes(packets[*index].bytes()).unwrap();
                if step < 2 {
                    assert_eq!(status, ReceiverConfigurationStatus::Waiting);
                    assert!(receiver.require_ready().is_err());
                }
            }
            let configuration = receiver.require_ready().unwrap();
            assert_eq!(configuration.channel_id(), 7);
            assert_eq!(configuration.source_mac(), [0, 0xca, 0, 0, 0, 1]);
            assert_eq!(configuration.ocd_change_count(), 3);
            assert_eq!(configuration.ncp_modulation().order(), 16);
            assert!(configuration.channel().primary_capable());
            assert_eq!(
                configuration.channel().transmission().subcarrier_spacing,
                spacing
            );
            assert_eq!(
                configuration.channel().transmission().cyclic_prefix_samples,
                512
            );
            assert_eq!(configuration.channel().transmission().roll_off_samples, 128);
            assert_eq!(configuration.interleaver_settings().unwrap().depth(), 16);
            let (_, raw) = acquisition_support::acquisition_setup(spacing, false);
            assert_eq!(
                configuration.channel().layout_settings().zero_frequency_hz,
                100_000_000
            );
            assert_eq!(
                configuration.channel().layout_settings().plc_first_bin,
                raw.plc_first_bin
            );
            let roles = configuration.channel().physical_layout().unwrap();
            for (bin, role) in roles.static_roles().iter().enumerate() {
                if *role == PhysicalCarrierRole::Data {
                    assert_eq!(configuration.profile_a().bit_loading()[bin], Some(8));
                    assert_eq!(configuration.ncp().bit_loading()[bin], Some(4));
                } else {
                    assert_eq!(configuration.profile_a().bit_loading()[bin], None);
                    assert_eq!(configuration.ncp().bit_loading()[bin], None);
                }
            }
            let before = serde_json::to_value(configuration).unwrap();
            for packet in &packets {
                assert_eq!(
                    receiver.accept_packet(packet).unwrap(),
                    ReceiverConfigurationStatus::Ready
                );
            }
            assert_eq!(
                serde_json::to_value(receiver.require_ready().unwrap()).unwrap(),
                before
            );
        }
    }
}
#[test]
fn independent_descriptor_change_counts_are_preserved_not_equated() {
    let mut packets = packets(SubcarrierSpacing::Khz25);
    for (index, count) in [(0, 250), (1, 17), (2, 3)] {
        let mut body = packets[index].body().to_vec();
        let offset = if index == 0 { 1 } else { 2 };
        body[offset] = count;
        packets[index] = ManagementPacket::new(
            packets[index].source_address(),
            packets[index].message_type(),
            &body,
        )
        .unwrap();
    }
    let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz25);
    for packet in &packets {
        receiver.accept_packet(packet).unwrap();
    }
    let config = receiver.require_ready().unwrap();
    assert_eq!(config.ocd_change_count(), 250);
    assert_eq!(config.profile_a().change_count(), 17);
    assert_eq!(config.ncp().change_count(), 3);
}
#[test]
fn rejected_inputs_preserve_last_valid_configuration_and_close_gate_until_valid_repeat() {
    let packets = packets(SubcarrierSpacing::Khz50);
    let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz50);
    for packet in &packets {
        receiver.accept_packet(packet).unwrap();
    }
    let before = serde_json::to_value(receiver.last_valid().unwrap()).unwrap();
    let mut corrupt = packets[0].bytes().to_vec();
    let end = corrupt.len() - 1;
    corrupt[end] ^= 1;
    assert!(receiver.accept_bytes(&corrupt).is_err());
    assert_eq!(receiver.status(), ReceiverConfigurationStatus::Rejected);
    assert!(receiver.require_ready().is_err());
    assert!(receiver.last_rejection().is_some());
    assert_eq!(
        serde_json::to_value(receiver.last_valid().unwrap()).unwrap(),
        before
    );
    receiver.accept_packet(&packets[0]).unwrap();
    assert!(receiver.require_ready().is_ok());
    assert!(receiver.last_rejection().is_none());
    let mut body = packets[1].body().to_vec();
    body[2] = 4;
    let changed = ManagementPacket::new(packets[1].source_address(), 50, &body).unwrap();
    assert_eq!(
        receiver.accept_packet(&changed).unwrap_err().code,
        "ReceiverSessionResetRequired"
    );
    assert_eq!(
        serde_json::to_value(receiver.last_valid().unwrap()).unwrap(),
        before
    );
    receiver.reset();
    assert_eq!(receiver.status(), ReceiverConfigurationStatus::Waiting);
    assert!(receiver.last_valid().is_none());
    assert!(receiver.missing().ocd && receiver.missing().profile_a && receiver.missing().ncp);
}
#[test]
fn source_channel_bootstrap_and_same_generation_conflicts_are_rejected_atomically() {
    let packets = packets(SubcarrierSpacing::Khz25);
    let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz25);
    receiver.accept_packet(&packets[0]).unwrap();
    let missing = receiver.missing();
    let foreign = ManagementPacket::new([0, 0xca, 0, 0, 0, 2], 50, packets[1].body()).unwrap();
    assert!(receiver.accept_packet(&foreign).is_err());
    assert_eq!(receiver.missing(), missing);
    let mut body = packets[1].body().to_vec();
    body[0] = 8;
    let wrong = ManagementPacket::new(packets[1].source_address(), 50, &body).unwrap();
    assert!(receiver.accept_packet(&wrong).is_err());
    assert_eq!(receiver.missing(), missing);
    let mut mismatch = ReceiverConfigurationState::new(SubcarrierSpacing::Khz50);
    assert!(mismatch.accept_packet(&packets[0]).is_err());
    assert!(mismatch.missing().ocd);
    receiver.accept_packet(&packets[1]).unwrap();
    let mut body = packets[1].body().to_vec();
    body[5] = 0x24;
    let conflict = ManagementPacket::new(packets[1].source_address(), 50, &body).unwrap();
    assert_eq!(
        receiver.accept_packet(&conflict).unwrap_err().code,
        "ReceiverSessionResetRequired"
    );
    assert!(receiver.missing().ncp);
}
#[test]
fn inferred_envelope_does_not_claim_transmitter_occupied_bandwidth_is_signaled() {
    let (mut settings, raw) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz25, false);
    settings.occupied_bandwidth_hz = 192_000_000;
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let descriptors = FixedPlcDescriptors::new(
        &settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 0,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: false,
            ncp_bits: 6,
        },
    )
    .unwrap();
    let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz25);
    for packet in descriptors.packets() {
        receiver.accept_packet(packet).unwrap();
    }
    let config = receiver.require_ready().unwrap();
    assert_eq!(config.channel().minimum_occupied_bandwidth_hz(), 24_000_000);
    assert!(!config.channel().primary_capable());
    assert_eq!(config.ncp_modulation().order(), 64);
}
