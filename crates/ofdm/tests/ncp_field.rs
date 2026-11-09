/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Control-chain discovery, strict rejection and independently expected cell order.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::modulation::{map_symbol, qam_llrs},
    transmission::{
        layout::ncp::PreInterleavingRole,
        ncp::{
            mapping::{labels_from_bits, llrs_from_cells},
            NcpFieldCodec, NcpMessage, NULL_POINTER,
        },
        packing::{NcpModulation, PackingSettings, SymbolPacker},
        SubcarrierSpacing,
    },
};
use fec_codec::ncp::NcpFecCodec;
fn start(pointer: u16, last: bool) -> NcpMessage {
    NcpMessage {
        profile_id: 3,
        zero_loaded: false,
        data_profile_odd: false,
        ncp_profile_odd: false,
        last,
        tagged: false,
        ncp_profile_update: false,
        pointer,
    }
}
fn soft(labels: &[u16], modulation: NcpModulation) -> Vec<Vec<f64>> {
    labels
        .iter()
        .map(|label| {
            qam_llrs(
                modulation.order(),
                map_symbol(modulation.order(), *label).unwrap(),
                0.01,
            )
            .unwrap()
        })
        .collect()
}
#[test]
fn bit_groups_have_normative_direction_not_ordinary_msb_grouping() {
    let mut bits = [0; 48];
    bits[..8].copy_from_slice(&[1, 0, 0, 1, 1, 0, 1, 0]);
    assert_eq!(
        &labels_from_bits(&bits, NcpModulation::Qpsk).unwrap()[..4],
        &[1, 2, 1, 1]
    );
    assert_eq!(
        &labels_from_bits(&bits, NcpModulation::Qam16).unwrap()[..2],
        &[9, 5]
    );
    assert_eq!(
        labels_from_bits(&bits, NcpModulation::Qam64).unwrap()[0],
        25
    );
    let cells = vec![vec![1.0, 2.0]; 24];
    assert_eq!(
        &llrs_from_cells(&cells, NcpModulation::Qpsk).unwrap()[..4],
        &[2.0, 1.0, 2.0, 1.0]
    );
    assert!(labels_from_bits(&[2; 48], NcpModulation::Qpsk).is_err());
    assert!(labels_from_bits(&[0; 47], NcpModulation::Qpsk).is_err());
    assert!(llrs_from_cells(&vec![vec![0.0]; 24], NcpModulation::Qpsk).is_err());
    assert!(llrs_from_cells(&vec![vec![f64::NAN; 2]; 24], NcpModulation::Qpsk).is_err());
}
#[test]
fn all_ncp_modulations_discover_last_crc_and_data_ranges() {
    let mut codec = NcpFieldCodec::new().unwrap();
    let mut messages = vec![start(7, false), start(50, false), start(100, true)];
    messages[2].zero_loaded = true;
    for modulation in [
        NcpModulation::Qpsk,
        NcpModulation::Qam16,
        NcpModulation::Qam64,
    ] {
        let encoded = codec.encode_messages(&messages, modulation).unwrap();
        let mut cells = soft(&encoded.labels, modulation);
        // Receiver is not given exact field length: trailing data is ignored after CRC.
        cells.extend(soft(&[0; 20], modulation));
        let field = codec.decode_field(&cells, modulation, 100).unwrap();
        assert_eq!(field.messages, messages);
        assert_eq!(field.crc, encoded.crc);
        assert_eq!(field.consumed_cells, 4 * modulation.cells_per_block());
        assert_eq!(field.block_iterations.len(), 4);
        let spans = field.data_spans(120).unwrap();
        assert_eq!(spans.len(), 4);
        assert_eq!((spans[0].start, spans[0].end_exclusive), (0, 7));
        assert!(!spans[0].starts_codeword);
        assert_eq!((spans[1].start, spans[1].end_exclusive), (7, 50));
        assert_eq!(spans[1].profile_id, Some(3));
        assert!(spans[1].starts_codeword);
        assert!(spans[3].zero_loaded && spans[3].profile_id.is_none());
        assert!(field.data_spans(100).is_err());
    }
    let null = codec
        .encode_messages(&[start(NULL_POINTER, true)], NcpModulation::Qpsk)
        .unwrap();
    let decoded = codec
        .decode_field(
            &soft(&null.labels, NcpModulation::Qpsk),
            NcpModulation::Qpsk,
            100,
        )
        .unwrap();
    assert!(!decoded.data_spans(60).unwrap()[0].starts_codeword);
    assert!(decoded.data_spans(0).unwrap().is_empty());
}
#[test]
fn malformed_fields_truncation_and_valid_fec_with_bad_crc_are_rejected() {
    let mut codec = NcpFieldCodec::new().unwrap();
    let modulation = NcpModulation::Qpsk;
    assert!(codec.encode_messages(&[], modulation).is_err());
    assert!(codec
        .encode_messages(&[start(0, false)], modulation)
        .is_err());
    assert!(codec
        .encode_messages(&[start(0, true), start(1, true)], modulation)
        .is_err());
    assert!(codec
        .encode_messages(&[start(3, false), start(2, true)], modulation)
        .is_err());
    assert!(codec
        .encode_messages(&[start(NULL_POINTER, false), start(2, true)], modulation)
        .is_err());
    let mut inconsistent = vec![start(0, false), start(2, true)];
    inconsistent[1].ncp_profile_odd = true;
    assert!(codec.encode_messages(&inconsistent, modulation).is_err());
    inconsistent[1].ncp_profile_odd = false;
    inconsistent[1].ncp_profile_update = true;
    assert!(codec.encode_messages(&inconsistent, modulation).is_err());
    let too_many: Vec<_> = (0..12).map(|index| start(index, index == 11)).collect();
    assert!(codec.encode_messages(&too_many, modulation).is_err());
    let field = codec
        .encode_messages(&[start(0, true)], modulation)
        .unwrap();
    let cells = soft(&field.labels, modulation);
    assert_eq!(
        codec
            .decode_field(&cells[..24], modulation, 100)
            .unwrap_err()
            .code,
        "NcpTruncated"
    );
    assert!(codec.decode_field(&cells, modulation, 0).is_err());
    assert!(codec
        .decode_field(&vec![vec![0.0; 2]; 7538], modulation, 100)
        .is_err());
    let contradictory: Vec<_> = (0..48)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    let broken_cells: Vec<Vec<f64>> = contradictory
        .chunks_exact(2)
        .map(|chunk| chunk.iter().rev().copied().collect())
        .collect();
    assert_eq!(
        codec
            .decode_field(&broken_cells, modulation, 1)
            .unwrap_err()
            .code,
        "NcpFecFailure"
    );
    let fec = NcpFecCodec::new().unwrap();
    let wrong_crc = [field.crc[0] ^ 1, field.crc[1], field.crc[2]];
    let mut corrupt = field.labels[..24].to_vec();
    corrupt.extend(labels_from_bits(&fec.encode(&wrong_crc), modulation).unwrap());
    assert_eq!(
        codec
            .decode_field(&soft(&corrupt, modulation), modulation, 100)
            .unwrap_err()
            .code,
        "NcpCrcMismatch"
    );
    // No last marker must hit a bounded reject, never scan indefinitely.
    let raw = start(0, false).to_bytes().unwrap();
    let block = labels_from_bits(&fec.encode(&raw), modulation).unwrap();
    assert!(codec
        .decode_field(&soft(&block.repeat(12), modulation), modulation, 100)
        .is_err());
}
#[test]
fn actual_encoded_cells_fit_packer_reservations_and_reject_bad_indices() {
    let codec = NcpFieldCodec::new().unwrap();
    for modulation in [
        NcpModulation::Qpsk,
        NcpModulation::Qam16,
        NcpModulation::Qam64,
    ] {
        let mut packer = SymbolPacker::new(PackingSettings {
            subcarrier_spacing: SubcarrierSpacing::Khz25,
            interleaver_cells: 5000,
            ncp_modulation: modulation,
            data_profile_id: 3,
        })
        .unwrap();
        packer.enqueue(vec![1; 2025]).unwrap();
        packer.enqueue(vec![2; 2025]).unwrap();
        let mut symbol = packer
            .pack_symbol(&vec![PreInterleavingRole::Data; 5000])
            .unwrap();
        let field = codec.encode_packed(&symbol).unwrap();
        assert_eq!(field.labels.len(), 4 * modulation.cells_per_block());
        assert_eq!(field.labels.len(), symbol.ncp_reserved_cells);
        assert_eq!(field.messages[1].pointer, 2025);
        assert!(field.messages[2].zero_loaded);
        symbol.ncp_reserved_cells += 1;
        assert!(codec.encode_packed(&symbol).is_err());
        symbol.ncp_reserved_cells -= 1;
        symbol.cells.pop();
        assert!(codec.encode_packed(&symbol).is_err());
    }
}
