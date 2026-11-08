/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Codec-loop tests independent of modulation, noise, profiles and input files.
use fec_codec::{
    concatenated::{compare_payload, ConcatenatedCodec},
    parameters::{BCH_INPUT_BITS, LDPC_CODEWORD_BITS, LDPC_INPUT_BITS},
};
fn payload() -> Vec<u8> {
    (0..BCH_INPUT_BITS)
        .map(|offset| u8::from((offset * 17 + offset / 11) % 19 < 9))
        .collect()
}
#[test]
fn isolated_encode_reverse_decode_and_compare_reuses_one_codec() {
    let mut codec = ConcatenatedCodec::new().unwrap();
    let mut edge = vec![0; BCH_INPUT_BITS];
    edge[BCH_INPUT_BITS - 1] = 1;
    for original in [vec![0; BCH_INPUT_BITS], edge, payload()] {
        let encoded = codec.encode(&original).unwrap();
        assert_eq!(encoded.bch_codeword.len(), 14400);
        assert_eq!(encoded.ldpc_codeword.len(), 16200);
        assert_eq!(&encoded.bch_codeword[..BCH_INPUT_BITS], &original);
        let decoded = codec.decode_noiseless(&encoded.ldpc_codeword, 30).unwrap();
        assert!(decoded.ldpc.success);
        assert!(decoded.bch.success);
        assert_eq!(decoded.bch.corrected_bits, 0);
        let comparison = compare_payload(&original, &decoded).unwrap();
        assert_eq!(comparison.bits_tested, 14232);
        assert_eq!(comparison.bit_errors, 0);
    }
}
#[test]
fn receiver_correction_and_truth_comparison_are_independent_of_status_flags() {
    let original = payload();
    let mut codec = ConcatenatedCodec::new().unwrap();
    let mut received = codec.encode(&original).unwrap().ldpc_codeword;
    received[91] ^= 1;
    let mut decoded = codec.decode_noiseless(&received, 30).unwrap();
    assert!(decoded.ldpc.success);
    assert!(decoded.bch.success);
    assert_eq!(compare_payload(&original, &decoded).unwrap().bit_errors, 0);
    // A false success flag cannot hide a wrong recovered payload from comparison.
    decoded.bch.codeword[23] ^= 1;
    assert_eq!(compare_payload(&original, &decoded).unwrap().bit_errors, 1);
    decoded.bch.codeword.pop();
    assert!(compare_payload(&original, &decoded).is_err());
}
#[test]
fn concatenated_input_validation_does_not_modify_sources() {
    let mut codec = ConcatenatedCodec::new().unwrap();
    assert!(codec.encode(&[]).is_err());
    let mut nonbinary = payload();
    nonbinary[0] = 2;
    assert!(codec.encode(&nonbinary).is_err());
    assert_eq!(nonbinary[0], 2);
    assert!(codec
        .decode_noiseless(&vec![0; LDPC_CODEWORD_BITS], 0)
        .is_err());
    assert!(codec
        .decode(&vec![f64::NAN; LDPC_CODEWORD_BITS], 30)
        .is_err());
    assert!(codec
        .decode_noiseless(&vec![0; LDPC_INPUT_BITS], 30)
        .is_err());
}
