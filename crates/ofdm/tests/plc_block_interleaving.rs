/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Full expected coordinate reads transcribed column-wise from Figures66–67.
use docsis_ofdm_fec::transmission::{
    plc::{
        block_interleaving::{deinterleave, deinterleave_llrs, interleave, source_indices},
        InterleavedPlcBlock, PlcCodewordNibbles,
    },
    SubcarrierSpacing,
};
const FOUR_K: [usize; 96] = [
    0, 12, 24, 36, 48, 60, 72, 84, 1, 13, 25, 37, 49, 61, 73, 85, 2, 14, 26, 38, 50, 62, 74, 86, 3,
    15, 27, 39, 51, 63, 75, 87, 4, 16, 28, 40, 52, 64, 76, 88, 5, 17, 29, 41, 53, 65, 77, 89, 6,
    18, 30, 42, 54, 66, 78, 90, 7, 19, 31, 43, 55, 67, 79, 91, 8, 20, 32, 44, 56, 68, 80, 92, 9,
    21, 33, 45, 57, 69, 81, 93, 10, 22, 34, 46, 58, 70, 82, 94, 11, 23, 35, 47, 59, 71, 83, 95,
];
const EIGHT_K: [usize; 96] = [
    0, 6, 12, 18, 24, 30, 36, 42, 48, 54, 60, 66, 72, 78, 84, 90, 1, 7, 13, 19, 25, 31, 37, 43, 49,
    55, 61, 67, 73, 79, 85, 91, 2, 8, 14, 20, 26, 32, 38, 44, 50, 56, 62, 68, 74, 80, 86, 92, 3, 9,
    15, 21, 27, 33, 39, 45, 51, 57, 63, 69, 75, 81, 87, 93, 4, 10, 16, 22, 28, 34, 40, 46, 52, 58,
    64, 70, 76, 82, 88, 94, 5, 11, 17, 23, 29, 35, 41, 47, 53, 59, 65, 71, 77, 83, 89, 95,
];
#[test]
fn full_coordinate_vectors_and_every_nibble_basis_match_both_figures() {
    for (spacing, expected, carriers, symbols) in [
        (SubcarrierSpacing::Khz50, FOUR_K, 8, 12),
        (SubcarrierSpacing::Khz25, EIGHT_K, 16, 6),
    ] {
        assert_eq!(source_indices(spacing), expected);
        let mut sorted = source_indices(spacing);
        sorted.sort();
        assert_eq!(sorted.as_slice(), (0..96).collect::<Vec<_>>());
        for input in 0..96 {
            let mut labels = [0; 96];
            labels[input] = 15;
            let coded = PlcCodewordNibbles::from_values(&labels).unwrap();
            let output = interleave(spacing, &coded);
            let expected_values = expected.map(|source| labels[source]);
            assert_eq!(output.values(), expected_values);
            assert_eq!(deinterleave(&output).unwrap(), coded);
            for symbol in 0..symbols {
                assert_eq!(
                    output.symbol(symbol).unwrap(),
                    &expected_values[symbol * carriers..(symbol + 1) * carriers]
                );
            }
            assert!(output.symbol(symbols).is_none());
            assert!(output.symbol(usize::MAX).is_none());
        }
    }
}
#[test]
fn soft_inverse_preserves_exact_bit_significance_sign_and_magnitude() {
    let original: Vec<_> = (0..384)
        .map(|index| {
            if index % 2 == 0 {
                index as f64 + 0.125
            } else {
                -(index as f64) - 0.125
            }
        })
        .collect();
    for (spacing, expected) in [
        (SubcarrierSpacing::Khz50, FOUR_K),
        (SubcarrierSpacing::Khz25, EIGHT_K),
    ] {
        let mut received = Vec::new();
        for source in expected {
            received.extend_from_slice(&original[source * 4..source * 4 + 4]);
        }
        let before = received.clone();
        assert_eq!(
            deinterleave_llrs(spacing, &received).unwrap().as_slice(),
            original
        );
        assert_eq!(received, before);
    }
}
#[test]
fn received_blocks_and_soft_input_require_complete_finite_values() {
    assert!(InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz25, &[0; 95]).is_err());
    assert!(InterleavedPlcBlock::from_values(SubcarrierSpacing::Khz50, &[16; 96]).is_err());
    assert!(deinterleave_llrs(SubcarrierSpacing::Khz25, &[0.0; 383]).is_err());
    assert!(deinterleave_llrs(SubcarrierSpacing::Khz50, &[f64::NAN; 384]).is_err());
    assert!(deinterleave_llrs(SubcarrierSpacing::Khz25, &[f64::INFINITY; 384]).is_err());
}
