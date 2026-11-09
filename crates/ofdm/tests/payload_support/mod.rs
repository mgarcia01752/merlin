/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ready-state fixture built through real management-packet and descriptor validation.
use docsis_ofdm_fec::transmission::{
    layout::{CarrierLayout, CarrierLayoutSettings},
    packing::NcpModulation,
    plc::{
        descriptors::{DescriptorIdentity, FixedPlcDescriptors},
        receiver_configuration::ReceiverConfigurationState,
    },
    TransmissionSettings,
};
/// Acquires the supplied fixed descriptor set without passing TX geometry to the RX constructor.
/// # Arguments
/// * `settings` - Fixture TX numerology/depth.
/// * `raw` - Explicit acquisition-compatible protected assignments.
/// * `ncp` - Positive uniform NCP constellation.
/// # Returns
/// Ready descriptor owner with independent received snapshot.
pub fn ready(
    settings: &TransmissionSettings,
    raw: &CarrierLayoutSettings,
    ncp: NcpModulation,
) -> ReceiverConfigurationState {
    let layout = CarrierLayout::new(settings, raw).unwrap();
    let descriptors = FixedPlcDescriptors::new(
        settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 0,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: ncp.order().ilog2() as u8,
        },
    )
    .unwrap();
    let mut state = ReceiverConfigurationState::new(settings.subcarrier_spacing);
    for packet in descriptors.packets() {
        state.accept_packet(packet).unwrap();
    }
    state
}
