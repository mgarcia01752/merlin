/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Error correction and real LDPC iteration/failure evidence for shortened coding.
use fec_codec::{
    decode_bch,
    shortening::{compare_payload, ShortenedCodec, Shortening},
};
fn payload() -> Vec<u8> {
    (0..6136)
        .map(|index| u8::from((index * 17 + index / 11) % 19 < 9))
        .collect()
}
#[test]
fn shortened_bch_corrects_all_counts_through_twelve_across_information_and_parity() {
    let geometry = Shortening::new(6136).unwrap();
    let codec = ShortenedCodec::new(geometry).unwrap();
    let encoded = codec.encode(&payload()).unwrap();
    for errors in 0..=12 {
        let mut damaged = encoded.bch_mother.clone();
        for ordinal in 0..errors {
            damaged[8096 + (ordinal * 521 + 6303) % 6304] ^= 1;
        }
        let decoded = decode_bch(&damaged).unwrap();
        assert!(decoded.success, "errors={errors}");
        assert_eq!(decoded.corrected_bits, errors as u32);
        assert_eq!(decoded.codeword, encoded.bch_mother);
    }
    let mut damaged = encoded.bch_mother.clone();
    for ordinal in 0..40 {
        damaged[8096 + (ordinal * 157 + 3) % 6304] ^= 1;
    }
    let decoded = decode_bch(&damaged).unwrap();
    assert!(!decoded.success);
    assert_eq!(decoded.codeword, damaged);
}
#[test]
fn weak_wrong_bits_require_real_ldpc_iterations_and_return_original_payload() {
    let mut codec = ShortenedCodec::new(Shortening::new(6136).unwrap()).unwrap();
    let original = payload();
    let encoded = codec.encode(&original).unwrap();
    let mut llrs: Vec<_> = encoded
        .transmitted
        .iter()
        .map(|bit| if *bit == 0 { 6.0 } else { -6.0 })
        .collect();
    for position in [0, 6135, 6303, 6304, 8103] {
        llrs[position] *= -0.1;
    }
    let decoded = codec.decode(&llrs, 50).unwrap();
    assert!(decoded.success());
    assert!(decoded.ldpc.iterations > 0 && decoded.ldpc.iterations <= 50);
    assert_eq!(compare_payload(&original, &decoded).unwrap().bit_errors, 0);
}
#[test]
fn deterministic_overwhelmed_word_retains_actual_failure_and_decisions() {
    let mut codec = ShortenedCodec::new(Shortening::new(6136).unwrap()).unwrap();
    let original = payload();
    let llrs: Vec<_> = (0..8104)
        .map(|index| {
            if (index * 17 + index / 3) % 7 < 3 {
                20.0
            } else {
                -20.0
            }
        })
        .collect();
    let decoded = codec.decode(&llrs, 1).unwrap();
    assert!(!decoded.ldpc.success);
    assert_eq!(decoded.ldpc.iterations, 1);
    assert!(!decoded.success());
    assert_eq!(decoded.payload.len(), 6136);
    assert!(compare_payload(&original, &decoded).unwrap().bit_errors > 0);
}
