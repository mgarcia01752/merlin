/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Cross-symbol FEC recovery using explicit test-oracle boundaries, not NCP decode.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    signal_processing::modulation::{
        interleaving::BitInterleaver, map_symbol, qam_llrs, randomization::CellRandomizer,
    },
    simulation::source::SeededSource,
    transmission::{
        layout::ncp::PreInterleavingRole,
        packing::{NcpModulation, PackedCell, PackingSettings, SymbolPacker},
        SubcarrierSpacing,
    },
};
use fec_codec::concatenated::{compare_payload, ConcatenatedCodec};
use std::collections::BTreeMap;

#[test]
fn four_words_cross_symbols_and_128_phase_boundary_with_zero_payload_errors() {
    // Deliberately small synthetic domain makes every word span many symbols.
    // This is not a complete physical pilot-layout or minimum-bandwidth example.
    let mut packer = SymbolPacker::new(PackingSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz25,
        interleaver_cells: 100,
        ncp_modulation: NcpModulation::Qpsk,
        data_profile_id: 0,
    })
    .unwrap();
    let mut codec = ConcatenatedCodec::new().unwrap();
    let processing = BitInterleaver::new(256).unwrap();
    let mut source = SeededSource::new(41);
    let payloads: Vec<_> = (0..4).map(|_| source.next_payload()).collect();
    for payload in &payloads {
        let encoded = codec.encode(payload).unwrap();
        packer
            .enqueue(processing.encode_labels(&encoded.ldpc_codeword).unwrap())
            .unwrap();
    }
    let mut transmitter = CellRandomizer::new();
    let mut receiver = CellRandomizer::new();
    let mut words: BTreeMap<u64, Vec<f64>> = BTreeMap::new();
    let mut symbols = 0;
    while packer.queued_codewords() > 0 {
        assert!(symbols < 500, "Packer stopped making progress");
        let mut roles = vec![PreInterleavingRole::Data; 100];
        for slot in [0, (symbols % 20) + 1, 99] {
            roles[slot] = PreInterleavingRole::ScatteredPilotPlaceholder;
        }
        let symbol = packer.pack_symbol(&roles).unwrap();
        transmitter.begin_symbol(symbol.symbol_index).unwrap();
        receiver.begin_symbol(symbol.symbol_index).unwrap();
        for cell in symbol.cells {
            match cell {
                PackedCell::BitData { .. } => {
                    panic!("Legacy fixture unexpectedly selected variable-width packing")
                }
                PackedCell::Data {
                    codeword_id,
                    codeword_cell,
                    label,
                } => {
                    let randomized = transmitter.apply_data(256, label).unwrap();
                    let point = map_symbol(256, randomized).unwrap();
                    let soft = qam_llrs(256, point, 0.01).unwrap();
                    let restored = receiver.descramble_data_llrs(256, &soft).unwrap();
                    // Oracle IDs and offsets verify the packer; no wire receiver claim.
                    let output = words.entry(codeword_id).or_default();
                    assert_eq!(output.len(), codeword_cell * 8);
                    output.extend(restored);
                }
                PackedCell::ScatteredPilotPlaceholder => {
                    transmitter.skip_scattered_pilot().unwrap();
                    receiver.skip_scattered_pilot().unwrap();
                }
                PackedCell::ZeroLoadedFiller => {
                    assert_eq!(
                        transmitter.zero_loaded_bpsk().unwrap(),
                        receiver.zero_loaded_bpsk().unwrap()
                    );
                }
                PackedCell::NcpReserved { .. } => {
                    // Synthetic labels only to clock reserved cells. Not NCP messages.
                    let label = transmitter.apply_ncp(4, 0).unwrap();
                    assert_eq!(receiver.apply_ncp(4, label).unwrap(), 0);
                }
            }
        }
        assert_eq!(transmitter.state(), receiver.state());
        symbols += 1;
    }
    assert!(symbols > 128);
    assert_eq!(words.len(), 4);
    for (id, soft) in words {
        assert_eq!(soft.len(), 16200);
        let decoded = codec
            .decode(&processing.decode_llrs(&soft).unwrap(), 30)
            .unwrap();
        assert!(decoded.ldpc.success && decoded.bch.success);
        assert!(decoded.ldpc.iterations <= 30);
        let comparison = compare_payload(&payloads[id as usize], &decoded).unwrap();
        assert_eq!(comparison.bits_tested, 14232);
        assert_eq!(comparison.bit_errors, 0);
    }
}
