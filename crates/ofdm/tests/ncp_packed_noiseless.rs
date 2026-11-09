/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Receiver derives boundaries from real NCP FEC+CRC, without transmitter IDs.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::modulation::{
        interleaving::BitInterleaver, map_symbol, qam_llrs, randomization::CellRandomizer,
    },
    simulation::source::SeededSource,
    transmission::{
        layout::ncp::PreInterleavingRole,
        ncp::NcpFieldCodec,
        packing::{NcpModulation, PackedCell, PackingSettings, SymbolPacker},
        SubcarrierSpacing,
    },
};
use fec_codec::concatenated::{compare_payload, ConcatenatedCodec};

#[test]
fn ncp_discovers_four_codewords_across_symbols_and_randomizer_reset() {
    for modulation in [
        NcpModulation::Qpsk,
        NcpModulation::Qam16,
        NcpModulation::Qam64,
    ] {
        let mut packer = SymbolPacker::new(PackingSettings {
            subcarrier_spacing: SubcarrierSpacing::Khz25,
            interleaver_cells: 80,
            ncp_modulation: modulation,
            data_profile_id: 0,
        })
        .unwrap();
        let mut fec = ConcatenatedCodec::new().unwrap();
        let bits = BitInterleaver::new(256).unwrap();
        let mut source = SeededSource::new(41);
        let payloads: Vec<_> = (0..4).map(|_| source.next_payload()).collect();
        for payload in &payloads {
            packer
                .enqueue(
                    bits.encode_labels(&fec.encode(payload).unwrap().ldpc_codeword)
                        .unwrap(),
                )
                .unwrap();
        }
        let mut control = NcpFieldCodec::new().unwrap();
        let mut transmitter = CellRandomizer::new();
        // Both hypotheses clock once per nonpilot position, independent of bit loading.
        // Decode NCP first, then use its pointers to select data-hypothesis LLRs.
        let mut data_receiver = CellRandomizer::new();
        let mut ncp_receiver = CellRandomizer::new();
        let mut pending = Vec::new();
        let mut recovered = 0;
        let mut symbol_index = 0;
        let mut starts = 0;
        while packer.queued_codewords() > 0 {
            assert!(symbol_index < 500);
            let mut roles = vec![PreInterleavingRole::Data; 80];
            for slot in [0, 1 + symbol_index % 20, 79] {
                roles[slot] = PreInterleavingRole::ScatteredPilotPlaceholder;
            }
            let symbol = packer.pack_symbol(&roles).unwrap();
            let field = control.encode_packed(&symbol).unwrap();
            transmitter.begin_symbol(symbol_index as u64).unwrap();
            let mut points = Vec::new();
            // Transmitter may use the plan; receiver below sees only I/Q and pilot mask.
            for cell in symbol.cells {
                let point = match cell {
                    PackedCell::BitData { .. } => {
                        panic!("Legacy fixture selected variable-width mode")
                    }
                    PackedCell::Data { label, .. } => {
                        map_symbol(256, transmitter.apply_data(256, label).unwrap()).unwrap()
                    }
                    PackedCell::NcpReserved { field_cell } => map_symbol(
                        modulation.order(),
                        transmitter
                            .apply_ncp(modulation.order(), field.labels[field_cell])
                            .unwrap(),
                    )
                    .unwrap(),
                    PackedCell::ZeroLoadedFiller => {
                        map_symbol(2, transmitter.zero_loaded_bpsk().unwrap()).unwrap()
                    }
                    PackedCell::ScatteredPilotPlaceholder => {
                        transmitter.skip_scattered_pilot().unwrap();
                        continue;
                    }
                };
                points.push(point);
            }
            data_receiver.begin_symbol(symbol_index as u64).unwrap();
            ncp_receiver.begin_symbol(symbol_index as u64).unwrap();
            let mut data_soft = Vec::new();
            let mut ncp_soft = Vec::new();
            for point in &points {
                data_soft.push(
                    data_receiver
                        .descramble_data_llrs(256, &qam_llrs(256, *point, 0.01).unwrap())
                        .unwrap(),
                );
                ncp_soft.push(
                    ncp_receiver
                        .descramble_ncp_llrs(
                            modulation.order(),
                            &qam_llrs(modulation.order(), *point, 0.01).unwrap(),
                        )
                        .unwrap(),
                );
            }
            assert_eq!(data_receiver.state(), transmitter.state());
            assert_eq!(ncp_receiver.state(), transmitter.state());
            ncp_soft.reverse();
            let decoded = control.decode_field(&ncp_soft, modulation, 100).unwrap();
            let data_count = points.len() - decoded.consumed_cells;
            for span in decoded.data_spans(data_count).unwrap() {
                if span.zero_loaded {
                    assert!(pending.is_empty());
                    continue;
                }
                if span.starts_codeword {
                    assert!(pending.is_empty());
                    assert_eq!(span.profile_id, Some(0));
                    starts += 1;
                } else {
                    assert!(!pending.is_empty(), "Continuation without a received start");
                }
                for cell in &data_soft[span.start..span.end_exclusive] {
                    pending.extend_from_slice(cell);
                    assert!(pending.len() <= 16200);
                    if pending.len() == 16200 {
                        let decoded = fec
                            .decode(&bits.decode_llrs(&pending).unwrap(), 30)
                            .unwrap();
                        assert!(decoded.ldpc.success && decoded.bch.success);
                        assert_eq!(
                            compare_payload(&payloads[recovered], &decoded)
                                .unwrap()
                                .bit_errors,
                            0
                        );
                        recovered += 1;
                        pending.clear();
                    }
                }
            }
            symbol_index += 1;
        }
        assert!(pending.is_empty());
        assert_eq!(starts, 4);
        assert_eq!(recovered, 4);
        // All NCP profiles cross the128-symbol reset, with synthetic small NI only.
        assert!(symbol_index > 128);
    }
}
