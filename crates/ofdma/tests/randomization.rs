/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Figure 7 first-step and independently derived polynomial impulse anchors.
use docsis_ofdma::bit_processing::Randomizer23;

#[test]
fn literal_mask_and_independent_grant_soft_inverse() {
    let mut mask = Randomizer23::new(0x020000).unwrap();
    let output = mask.apply_bits(&[0; 64]).unwrap();
    let ones: Vec<_> = output
        .iter()
        .enumerate()
        .filter_map(|(i, bit)| (*bit == 1).then_some(i))
        .collect();
    assert_eq!(ones, vec![0, 5, 18, 28, 36, 41, 46, 51, 54]);
    let mut split = Randomizer23::new(0x020000).unwrap();
    let mut joined = split.apply_bits(&[0; 13]).unwrap();
    assert!(split.apply_bits(&[2]).is_err());
    joined.extend(split.apply_bits(&[0; 51]).unwrap());
    assert_eq!(joined, output);
    let soft: Vec<_> = output
        .iter()
        .map(|bit| if *bit == 0 { 3.0 } else { -3.0 })
        .collect();
    assert_eq!(
        Randomizer23::new(0x020000)
            .unwrap()
            .apply_llrs(&soft)
            .unwrap(),
        vec![3.0; 64]
    );
    assert_eq!(
        Randomizer23::new(0).unwrap().apply_bits(&[0; 64]).unwrap(),
        vec![0; 64]
    );
    assert!(Randomizer23::new(0x80_0000).is_err());
}
