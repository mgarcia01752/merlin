/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Noiseless recovery includes punctured information; decoder never receives truth.
use fec_codec::plc::PlcFecCodec;
fn soft(bits: &[u8]) -> Vec<f64> {
    bits.iter()
        .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
        .collect()
}
#[test]
fn all_basis_words_recover_without_errors_including_punctured_information() {
    let mut codec = PlcFecCodec::new().unwrap();
    let mut cases = vec![
        [0; 288],
        [1; 288],
        std::array::from_fn(|index| u8::from((index * 17 + index / 5) % 13 < 6)),
    ];
    for index in 0..288 {
        let mut information = [0; 288];
        information[index] = 1;
        cases.push(information);
    }
    for information in cases {
        let llrs = soft(&codec.encode(&information).unwrap());
        let decoded = codec.decode(&llrs, 100).unwrap();
        assert!(decoded.success, "iterations={}", decoded.iterations);
        assert_eq!(decoded.information, information);
        assert!(codec.valid_syndrome(&decoded.mother_codeword));
        assert!(decoded.iterations <= 100);
        let expected: Vec<_> = information
            .chunks_exact(8)
            .map(|bits| bits.iter().fold(0u8, |byte, bit| (byte << 1) | bit))
            .collect();
        assert_eq!(decoded.information_bytes().as_slice(), expected);
    }
}
#[test]
fn weak_wrong_sign_is_corrected_and_contradictory_input_can_fail() {
    let mut codec = PlcFecCodec::new().unwrap();
    let information = std::array::from_fn::<_, 288, _>(|index| u8::from(index % 7 < 3));
    let mut llrs = soft(&codec.encode(&information).unwrap());
    llrs[0] = -llrs[0].signum() * 0.25;
    let decoded = codec.decode(&llrs, 100).unwrap();
    assert!(decoded.success);
    assert_eq!(decoded.information, information);
    assert!(decoded.iterations > 0);
    let contradictory: Vec<_> = (0..384)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    let failed = codec.decode(&contradictory, 1).unwrap();
    assert!(!failed.success);
    assert_eq!(failed.iterations, 1);
    assert_eq!(failed.information.len(), 288);
    assert_eq!(failed.mother_codeword.len(), 480);
    assert!(codec.decode(&llrs, 0).is_err());
    assert!(codec.decode(&llrs, 101).is_err());
    assert!(codec.decode(&llrs[..383], 10).is_err());
    llrs[0] = f64::NEG_INFINITY;
    assert!(codec.decode(&llrs, 10).is_err());
    llrs[0] = f64::NAN;
    assert!(codec.decode(&llrs, 10).is_err());
}

#[test]
fn a_different_valid_codeword_can_converge_without_recovering_original_truth() {
    let mut codec = PlcFecCodec::new().unwrap();
    // Receiver evidence points to the all-zero codeword, while independent test
    // truth is all ones. A valid syndrome cannot authenticate the original data.
    let other = codec.encode(&[0; 288]).unwrap();
    let decoded = codec.decode(&soft(&other), 100).unwrap();
    assert!(decoded.success);
    assert_eq!(decoded.information, [0; 288]);
    assert_eq!(
        decoded.information.iter().filter(|bit| **bit != 1).count(),
        288
    );
}
