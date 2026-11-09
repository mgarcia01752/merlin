/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independently calculated short256cell vectors and soft inverse boundaries.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::signal_processing::modulation::interleaving::{
    transmit_bit_order, BitInterleaver,
};

#[test]
fn qam256_cells_match_hand_calculated_parity_twist_and_demux_vectors() {
    let processing = BitInterleaver::new(256).unwrap();
    let order = processing.output_to_codeword();
    // PHY I20 Tables24–25 and ETSI6.1.3/Table10(a). Indices refer to encoded
    // LDPC bits after substituting the parity interleaver's5*offset+group rule.
    assert_eq!(
        &order[..8],
        &[16099, 4050, 10118, 2025, 14155, 8099, 12130, 0]
    );
    assert_eq!(
        &order[8..16],
        &[16104, 4051, 10119, 2026, 14156, 6075, 12131, 1]
    );
    assert_eq!(
        &order[16192..],
        &[16094, 6074, 10117, 4049, 14154, 8098, 12129, 2024]
    );
    for (cell, indices) in [(0, &order[..8]), (2024, &order[16192..])] {
        for (bit, index) in indices.iter().enumerate() {
            let mut codeword = vec![0u8; 16200];
            codeword[*index] = 1;
            let labels = processing.encode_labels(&codeword).unwrap();
            assert_eq!(labels.len(), 2025);
            assert_eq!(labels[cell], 1 << (7 - bit));
            assert_eq!(labels.iter().filter(|label| **label != 0).count(), 1);
        }
    }
}

#[test]
fn supported_permutations_are_bijective_and_inverse_preserves_soft_values() {
    for constellation in [16, 256] {
        let processing = BitInterleaver::new(constellation).unwrap();
        let order = processing.output_to_codeword();
        let mut sorted = order.to_vec();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..16200).collect::<Vec<_>>());
        let original: Vec<_> = (0..16200)
            .map(|index| index as f64 / 16.0 - 500.0)
            .collect();
        let cell_llrs: Vec<_> = order.iter().map(|index| original[*index]).collect();
        assert_eq!(processing.decode_llrs(&cell_llrs).unwrap(), original);
        assert_eq!(
            processing.encode_labels(&vec![0; 16200]).unwrap().len(),
            if constellation == 16 { 4050 } else { 2025 }
        );
    }
    assert_eq!(
        BitInterleaver::new(16).unwrap().output_to_codeword(),
        transmit_bit_order()
    );
    // Independent16-QAM ordering after the common parity/twist stage.
    assert_eq!(
        &transmit_bit_order()[..8],
        &[16099, 2025, 8099, 12130, 4050, 10118, 14155, 0]
    );
    assert_ne!(
        BitInterleaver::new(256).unwrap().output_to_codeword(),
        transmit_bit_order()
    );
}

#[test]
fn unsupported_modes_and_invalid_words_are_rejected_without_input_changes() {
    for order in [0, 4, 8, 32] {
        assert!(BitInterleaver::new(order).is_err());
    }
    let processing = BitInterleaver::new(256).unwrap();
    assert!(processing.encode_labels(&[]).is_err());
    assert!(processing.encode_labels(&vec![0; 16199]).is_err());
    let mut invalid = vec![0; 16200];
    invalid[37] = 2;
    assert!(processing.encode_labels(&invalid).is_err());
    assert_eq!(invalid[37], 2);
    assert!(processing.decode_llrs(&vec![0.0; 16199]).is_err());
    assert!(processing.decode_llrs(&vec![f64::NAN; 16200]).is_err());
}
