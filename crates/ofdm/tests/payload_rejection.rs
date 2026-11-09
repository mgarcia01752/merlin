/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Received-control failures close recovery without decoding untrusted payload or clearing history.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    signal_processing::{
        modulation::{map_symbol, randomization::CellRandomizer},
        receiver::payload::{collector::PayloadCollector, PayloadSymbolDecoder, SoftDataCell},
    },
    simulation::payload::PayloadReceiver,
    transmission::{
        layout::{ncp::PreInterleavingRole, CarrierLayout},
        ncp::{mapping::labels_from_bits, NcpDataSpan, NcpFieldCodec, NcpMessage},
        packing::{NcpModulation, PackingSettings, SymbolPacker},
        plc::{
            descriptors::{DescriptorIdentity, FixedPlcDescriptors},
            mac_packets::ManagementPacket,
            receiver_configuration::ReceiverConfigurationState,
        },
        symbols::{assemble_packed, TransportCell},
        SubcarrierSpacing,
    },
};
fn settings() -> PackingSettings {
    PackingSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz25,
        interleaver_cells: 200,
        ncp_modulation: NcpModulation::Qpsk,
        data_profile_id: 0,
    }
}
fn filler_message() -> NcpMessage {
    NcpMessage {
        profile_id: 0,
        zero_loaded: true,
        data_profile_odd: false,
        ncp_profile_odd: false,
        last: true,
        tagged: false,
        ncp_profile_update: false,
        pointer: 0,
    }
}
fn received_control(
    message: NcpMessage,
    bad_crc: bool,
    index: u64,
    randomizer: &mut CellRandomizer,
) -> Vec<Option<TransportCell>> {
    let mut field = NcpFieldCodec::new()
        .unwrap()
        .encode_messages(&[message], NcpModulation::Qpsk)
        .unwrap();
    if bad_crc {
        let mut crc = field.crc;
        crc[0] ^= 1;
        let replacement = labels_from_bits(
            &fec_codec::ncp::NcpFecCodec::new().unwrap().encode(&crc),
            NcpModulation::Qpsk,
        )
        .unwrap();
        let start = field.labels.len() - replacement.len();
        field.labels[start..].copy_from_slice(&replacement);
    }
    randomizer.begin_symbol(index).unwrap();
    (0..200)
        .map(|rank| {
            Some(TransportCell::Sample(if rank < 200 - field.labels.len() {
                map_symbol(2, randomizer.zero_loaded_bpsk().unwrap()).unwrap()
            } else {
                let label = field.labels[199 - rank];
                map_symbol(4, randomizer.apply_ncp(4, label).unwrap()).unwrap()
            }))
        })
        .collect()
}
#[test]
fn fec_valid_wrong_crc_rejects_the_whole_field_and_retains_pending_bits() {
    let roles = vec![PreInterleavingRole::Data; 200];
    let mut packer = SymbolPacker::new(settings()).unwrap();
    packer.enqueue(vec![0; 2025]).unwrap();
    let mut randomizer = CellRandomizer::new();
    let first = assemble_packed(
        &packer.pack_symbol(&roles).unwrap(),
        &NcpFieldCodec::new().unwrap(),
        &mut randomizer,
    )
    .unwrap();
    let mut receiver = PayloadSymbolDecoder::new(settings(), 0).unwrap();
    let accepted = receiver.receive(0, &first.cells, &roles, 0.01, 50).unwrap();
    assert!(accepted.error.is_none());
    let pending = accepted.pending_payload_bits;
    assert!(pending > 0);
    let cells = received_control(filler_message(), true, 1, &mut randomizer);
    let rejected = receiver.receive(1, &cells, &roles, 0.01, 50).unwrap();
    assert_eq!(rejected.error.unwrap().code, "NcpCrcMismatch");
    assert!(rejected.ncp.is_none());
    assert!(rejected.recovered.is_empty());
    assert_eq!(rejected.pending_payload_bits, pending);
    assert!(receiver.is_closed());
    assert!(receiver.receive(2, &cells, &roles, 0.01, 50).is_err());
}
#[test]
fn unsupported_profile_activation_is_reported_after_crc_without_substitution() {
    for flag in 0..4 {
        let mut message = filler_message();
        match flag {
            0 => message.data_profile_odd = true,
            1 => message.ncp_profile_odd = true,
            2 => message.ncp_profile_update = true,
            _ => message.tagged = true,
        };
        let mut receiver = PayloadSymbolDecoder::new(settings(), 0).unwrap();
        let cells = received_control(message, false, 0, &mut CellRandomizer::new());
        let result = receiver
            .receive(0, &cells, &[PreInterleavingRole::Data; 200], 0.01, 50)
            .unwrap();
        assert_eq!(result.error.unwrap().code, "UnsupportedProfileActivation");
        assert!(result.ncp.is_some());
        assert!(result.recovered.is_empty());
        assert!(receiver.is_closed());
    }
}
#[test]
fn collector_rejects_orphan_continuation_premature_start_and_filler() {
    let cells = vec![SoftDataCell {
        data_position: 0,
        llrs: vec![10.0; 8],
    }];
    let mut collector = PayloadCollector::new().unwrap();
    let mut span = NcpDataSpan {
        start: 0,
        end_exclusive: 1,
        profile_id: None,
        starts_codeword: false,
        zero_loaded: false,
    };
    assert!(collector.accept_span(&span, &cells, 0, 50).is_err());
    assert_eq!(collector.pending_bits(), 0);
    span.starts_codeword = true;
    span.profile_id = Some(0);
    assert!(collector
        .accept_span(&span, &cells, 0, 50)
        .unwrap()
        .is_none());
    assert_eq!(collector.pending_bits(), 8);
    assert!(collector.accept_span(&span, &cells, 1, 50).is_err());
    assert_eq!(collector.pending_bits(), 8);
    span.zero_loaded = true;
    span.starts_codeword = false;
    assert!(collector.accept_span(&span, &[], 1, 50).is_err());
    assert_eq!(collector.pending_bits(), 8);
}
#[test]
fn uniform64_is_admitted_while_zero_ncp_is_rejected_at_construction() {
    let spacing = SubcarrierSpacing::Khz25;
    let (settings, raw) = acquisition_support::acquisition_setup(spacing, false);
    let layout = CarrierLayout::new(&settings, &raw).unwrap();
    let descriptors = FixedPlcDescriptors::new(
        &settings,
        &layout,
        DescriptorIdentity {
            channel_id: 7,
            change_count: 0,
            source_mac: [0, 0xca, 0, 0, 0, 1],
            primary_capable: true,
            ncp_bits: 4,
        },
    )
    .unwrap();
    for target in [1, 2] {
        let mut packets = descriptors.packets().to_vec();
        let mut body = packets[target].body().to_vec();
        if target == 1 {
            body[5] = 0x26;
        } else {
            body.extend_from_slice(&[5, 5, 0, 1, 144, 1, 144]);
        }
        packets[target] =
            ManagementPacket::new(packets[target].source_address(), 50, &body).unwrap();
        let mut state = ReceiverConfigurationState::new(spacing);
        for packet in packets {
            state.accept_packet(&packet).unwrap();
        }
        assert!(state.require_ready().is_ok());
        if target == 1 {
            assert!(PayloadReceiver::new(&state, 0).is_ok());
        } else {
            assert!(PayloadReceiver::new(&state, 0).is_err());
        }
    }
}
