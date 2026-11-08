/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Numeric codec checks, separate from PNM file-format/parser tests.
use fec_codec::{decode_bch, encode_bch, LdpcCodec, BCH_INPUT_BITS, LDPC_CODEWORD_BITS};

fn message() -> Vec<u8> {
    (0..BCH_INPUT_BITS)
        .map(|position| u8::from((position * 17 + position / 11) % 19 < 9))
        .collect()
}
#[test]
fn bch_systematic_noiseless_and_correction_boundary() {
    let message = message();
    let encoded = encode_bch(&message).unwrap();
    assert_eq!(&encoded[..BCH_INPUT_BITS], &message);
    for errors in 0..=12 {
        let mut damaged = encoded.clone();
        for position in 0..errors {
            // Include both information and parity, spread across the full word.
            damaged[(position * 1297 + 14399) % 14400] ^= 1;
        }
        let decoded = decode_bch(&damaged).unwrap();
        assert!(decoded.success, "errors={errors}");
        assert_eq!(decoded.corrected_bits, errors as u32);
        assert_eq!(decoded.codeword, encoded);
    }
}
#[test]
fn bch_reports_a_demonstrated_uncorrectable_pattern() {
    let encoded = encode_bch(&message()).unwrap();
    let mut damaged = encoded.clone();
    for position in 0..40 {
        damaged[(position * 331 + 7) % 14400] ^= 1;
    }
    let decoded = decode_bch(&damaged).unwrap();
    assert!(!decoded.success);
    assert_eq!(decoded.codeword, damaged);
    assert_eq!(decoded.corrected_bits, 0);
}
#[test]
fn ldpc_syndromes_and_noiseless_soft_decoding() {
    let mut codec = LdpcCodec::new().unwrap();
    let information = encode_bch(&message()).unwrap();
    let word = codec.encode(&information).unwrap();
    assert_eq!(word.len(), LDPC_CODEWORD_BITS);
    assert_eq!(&word[..14400], information);
    assert!(codec.valid_syndrome(&word));
    let llrs: Vec<_> = word
        .iter()
        .map(|bit| if *bit == 0 { 20.0 } else { -20.0 })
        .collect();
    let decoded = codec.decode(&llrs, 20).unwrap();
    assert!(decoded.success);
    assert_eq!(decoded.codeword, word);
    let mut wrong = word;
    wrong[91] ^= 1;
    assert!(!codec.valid_syndrome(&wrong));
}
#[test]
fn codec_rejects_invalid_inputs_and_bounded_work() {
    assert!(encode_bch(&[]).is_err());
    assert!(decode_bch(&[]).is_err());
    let mut invalid = message();
    invalid[12] = 2;
    assert!(encode_bch(&invalid).is_err());
    let mut codec = LdpcCodec::new().unwrap();
    assert!(codec.decode(&vec![1.0; LDPC_CODEWORD_BITS], 0).is_err());
    assert!(codec
        .decode(&vec![f64::NAN; LDPC_CODEWORD_BITS], 20)
        .is_err());
}
