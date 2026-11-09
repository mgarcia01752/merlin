/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent profile precedence, protected roles, grid and NCP loading checks.
mod acquisition_support;
use docsis_ofdm_fec::transmission::{
    layout::CarrierLayout,
    plc::{
        descriptors::{DescriptorIdentity, FixedPlcDescriptors},
        mac_packets::ManagementPacket,
        receiver_configuration::ReceiverConfigurationState,
    },
    SubcarrierSpacing,
};
fn base(spacing: SubcarrierSpacing) -> (ReceiverConfigurationState, Vec<ManagementPacket>) {
    let (settings, raw) = acquisition_support::acquisition_setup(spacing, false);
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let packets = FixedPlcDescriptors::new(
        &settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 9,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: 4,
        },
    )
    .unwrap()
    .packets()
    .to_vec();
    (ReceiverConfigurationState::new(spacing), packets)
}
fn range(body: &mut Vec<u8>, bits: u8, first: u16, last: u16) {
    body.extend_from_slice(&[5, 5, bits]);
    body.extend_from_slice(&first.to_be_bytes());
    body.extend_from_slice(&last.to_be_bytes());
}
fn profile(original: &ManagementPacket, body: &[u8]) -> ManagementPacket {
    ManagementPacket::new(original.source_address(), 50, body).unwrap()
}
#[test]
fn specifics_override_defaults_in_either_tlv_order_and_ocd_protection_wins() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let bin = if spacing == SubcarrierSpacing::Khz25 {
            400
        } else {
            200
        };
        for reversed in [false, true] {
            let (mut receiver, packets) = base(spacing);
            let default = &packets[1].body()[3..];
            let mut body = packets[1].body()[..3].to_vec();
            if reversed {
                range(&mut body, 6, bin, bin);
                body.extend_from_slice(default);
            } else {
                body.extend_from_slice(default);
                range(&mut body, 6, bin, bin);
            }
            // A DPD-specific assignment on an excluded bin remains protected by OCD.
            range(&mut body, 14, 0, 0);
            receiver.accept_packet(&packets[0]).unwrap();
            receiver
                .accept_packet(&profile(&packets[1], &body))
                .unwrap();
            receiver.accept_packet(&packets[2]).unwrap();
            let config = receiver.require_ready().unwrap();
            assert_eq!(config.profile_a().bit_loading()[usize::from(bin)], Some(6));
            assert_eq!(config.profile_a().bit_loading()[0], None);
            assert_eq!(
                config.profile_a().bit_loading()[usize::from(bin) + 1],
                Some(8)
            );
        }
    }
}
#[test]
fn duplicates_missing_coverage_and_out_of_grid_are_rejected_before_commit() {
    let (_, packets) = base(SubcarrierSpacing::Khz50);
    let mut duplicate = packets[1].body().to_vec();
    duplicate.extend_from_slice(&packets[1].body()[3..]);
    let mut missing = packets[1].body()[..3].to_vec();
    range(&mut missing, 8, 200, 200);
    let mut out_of_grid = packets[1].body().to_vec();
    range(&mut out_of_grid, 8, 4096, 4096);
    for body in [duplicate, missing, out_of_grid] {
        let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz50);
        receiver.accept_packet(&packets[0]).unwrap();
        let before = receiver.missing();
        assert!(receiver
            .accept_packet(&profile(&packets[1], &body))
            .is_err());
        assert_eq!(receiver.missing(), before);
        assert!(receiver.last_valid().is_none());
        assert!(receiver.require_ready().is_err());
        receiver.accept_packet(&packets[1]).unwrap();
        receiver.accept_packet(&packets[2]).unwrap();
        assert!(receiver.require_ready().is_ok());
    }
}
#[test]
fn ncp_zero_loading_is_preserved_but_positive_orders_must_be_uniform_and_present() {
    let (_, packets) = base(SubcarrierSpacing::Khz25);
    for (default_bits, specific_bits, success) in [(4, 0, true), (4, 6, false), (0, 0, false)] {
        let mut body = packets[2].body()[..3].to_vec();
        range(&mut body, 0x20 | default_bits, 0, 8191);
        range(&mut body, specific_bits, 400, 400);
        let mut receiver = ReceiverConfigurationState::new(SubcarrierSpacing::Khz25);
        receiver.accept_packet(&packets[0]).unwrap();
        receiver.accept_packet(&packets[1]).unwrap();
        let result = receiver.accept_packet(&profile(&packets[2], &body));
        assert_eq!(result.is_ok(), success);
        if success {
            let config = receiver.require_ready().unwrap();
            assert_eq!(config.ncp().bit_loading()[400], Some(0));
            assert_eq!(config.ncp_modulation().order(), 16);
        } else {
            assert!(receiver.require_ready().is_err());
            assert!(receiver.missing().ncp);
        }
    }
}
#[test]
fn late_ocd_checks_cached_dpd_ranges_without_partially_accepting_bad_geometry() {
    let (mut receiver, packets) = base(SubcarrierSpacing::Khz50);
    let mut body = packets[1].body()[..3].to_vec();
    range(&mut body, 0x28, 0, 8191);
    receiver
        .accept_packet(&profile(&packets[1], &body))
        .unwrap();
    receiver.accept_packet(&packets[2]).unwrap();
    assert!(receiver.missing().ocd);
    assert!(receiver.accept_packet(&packets[0]).is_err());
    assert!(receiver.missing().ocd);
    assert!(receiver.last_valid().is_none());
}
