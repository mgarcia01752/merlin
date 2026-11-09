/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit coordinate boundaries and zero-LLR priors for deleted information/parity.
use fec_codec::plc::{
    parameters::PLC_TRANSMITTED_COORDINATES,
    puncturing::{puncture, restore_llrs},
};
#[test]
fn puncturing_retains_exact_ascending_specification_coordinates() {
    let expected: Vec<_> = (0..48).chain(96..384).chain(432..480).collect();
    assert_eq!(PLC_TRANSMITTED_COORDINATES.as_slice(), expected);
    assert_eq!(
        [
            expected[47],
            expected[48],
            expected[335],
            expected[336],
            expected[383]
        ],
        [47, 96, 383, 432, 479]
    );
    for coordinate in 0..480 {
        let mut mother = [0; 480];
        mother[coordinate] = 1;
        let transmitted = puncture(&mother).unwrap();
        assert_eq!(
            transmitted.iter().filter(|bit| **bit == 1).count(),
            usize::from(expected.contains(&coordinate))
        );
    }
}
#[test]
fn restoration_assigns_no_information_to_punctures_and_clamps_retained_values() {
    let llrs: Vec<_> = (0..384)
        .map(|index| if index % 2 == 0 { 200.0 } else { -200.0 })
        .collect();
    let original = llrs.clone();
    let restored = restore_llrs(&llrs).unwrap();
    assert_eq!(llrs, original);
    assert!(restored[48..96].iter().all(|llr| *llr == 0.0));
    assert!(restored[384..432].iter().all(|llr| *llr == 0.0));
    for (index, coordinate) in (0..48).chain(96..384).chain(432..480).enumerate() {
        assert_eq!(
            restored[coordinate],
            if index % 2 == 0 { 100.0 } else { -100.0 }
        );
    }
    assert!(puncture(&[0; 479]).is_err());
    assert!(puncture(&[2; 480]).is_err());
    assert!(restore_llrs(&[0.0; 383]).is_err());
    assert!(restore_llrs(&[f64::NAN; 384]).is_err());
    assert!(restore_llrs(&[f64::INFINITY; 384]).is_err());
}
