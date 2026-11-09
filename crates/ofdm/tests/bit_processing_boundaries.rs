/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent stage vectors plus composition and inverse compatibility.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::signal_processing::modulation::interleaving::{
    demultiplexing, parity, twist, BitInterleaver,
};
#[test]
fn individual_indices_match_independent_parity_twist_and_cell_examples() {
    let parity = parity::permutation();
    assert_eq!(&parity[14400..14404], &[14400, 14405, 14410, 14415]);
    assert_eq!(parity[14760], 14401);
    let twisted = twist::permutation(256).unwrap();
    // Column3 has a one-row rotation: output row0 receives input row2024.
    assert_eq!(twisted[3 * 2025], 3 * 2025 + 2024);
    let demux = demultiplexing::permutation(256).unwrap();
    assert_eq!(
        &demux[..8],
        &[14175, 4050, 8100, 2025, 12150, 6075, 10125, 0]
    );
}
#[test]
fn direct_stage_composition_and_reverse_match_existing_facade_for_both_modes() {
    let bits: Vec<u8> = (0..16200)
        .map(|index| u8::from(index % 7 == 0 || index % 13 == 0))
        .collect();
    for mode in [16, 256] {
        let parity_bits = parity::transmit(&bits).unwrap();
        let twisted = twist::transmit(mode, &parity_bits).unwrap();
        let demux = demultiplexing::transmit(mode, &twisted).unwrap();
        let facade = BitInterleaver::new(mode).unwrap();
        assert_eq!(
            demux,
            facade
                .output_to_codeword()
                .iter()
                .map(|position| bits[*position])
                .collect::<Vec<_>>()
        );
        let likelihoods: Vec<_> = demux
            .iter()
            .map(|bit| if *bit == 0 { 3.25 } else { -9.5 })
            .collect();
        let reversed = parity::receive_llrs(
            &twist::receive_llrs(
                mode,
                &demultiplexing::receive_llrs(mode, &likelihoods).unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(reversed, facade.decode_llrs(&likelihoods).unwrap());
    }
}
#[test]
fn boundary_inputs_are_validated_without_changing_input() {
    assert!(parity::transmit(&[0]).is_err());
    assert!(parity::transmit(&vec![2; 16200]).is_err());
    assert!(parity::receive_llrs(&vec![f64::NAN; 16200]).is_err());
    assert!(twist::transmit(64, &vec![0; 16200]).is_err());
    assert!(demultiplexing::receive_llrs(16384, &vec![0.0; 16200]).is_err());
}
