/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent bit significance from PHY I20 Figure65, before any permutation.
use docsis_ofdm_fec::transmission::plc::PlcCodewordNibbles;
#[test]
fn msb_first_groups_and_every_bit_basis_have_exact_labels() {
    let mut bits = [0; 384];
    bits[..16].copy_from_slice(&[1, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1]);
    let labels = PlcCodewordNibbles::from_bits(&bits).unwrap();
    assert_eq!(&labels.values()[..4], &[8, 5, 14, 1]);
    assert_eq!(labels.bits(), bits);
    for bit in 0..384 {
        let mut basis = [0; 384];
        basis[bit] = 1;
        let labels = PlcCodewordNibbles::from_bits(&basis).unwrap();
        assert_eq!(labels.values()[bit / 4], [8, 4, 2, 1][bit % 4]);
        assert_eq!(
            labels.values().iter().filter(|label| **label != 0).count(),
            1
        );
        assert_eq!(labels.bits(), basis);
    }
}
#[test]
fn invalid_sizes_and_values_are_rejected_without_mutation() {
    assert!(PlcCodewordNibbles::from_bits(&[0; 383]).is_err());
    assert!(PlcCodewordNibbles::from_bits(&[2; 384]).is_err());
    assert!(PlcCodewordNibbles::from_values(&[0; 95]).is_err());
    assert!(PlcCodewordNibbles::from_values(&[16; 96]).is_err());
    let mut source = [15; 96];
    let labels = PlcCodewordNibbles::from_values(&source).unwrap();
    source[0] = 0;
    assert_eq!(labels.values()[0], 15);
}
