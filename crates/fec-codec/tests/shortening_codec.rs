/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent polynomial/accumulator oracles, not only an encoder/decoder round trip.
#[path = "support/payload.rs"]
mod payload_reference;
use fec_codec::{
    concatenated::ConcatenatedCodec,
    shortening::{compare_payload, ShortenedCodec, Shortening},
    LdpcCodec,
};
#[test]
fn varied_lengths_match_independent_parity_coordinates_and_recover_without_errors() {
    let syndrome = LdpcCodec::new().unwrap();
    for payload_bits in [1, 7, 8, 168, 6136, 14231, 14232] {
        let geometry = Shortening::new(payload_bits).unwrap();
        let mut codec = ShortenedCodec::new(geometry).unwrap();
        for pattern in 0..3 {
            let payload: Vec<_> = (0..payload_bits)
                .map(|index| {
                    u8::from(match pattern {
                        0 => false,
                        1 => index == payload_bits - 1,
                        _ => (index * 17 + index / 11) % 19 < 9,
                    })
                })
                .collect();
            let mut bch = payload.clone();
            bch.extend(payload_reference::bch_parity(&payload));
            let mut systematic = bch.clone();
            systematic.resize(14400, 0);
            let parity = payload_reference::ldpc_parity(&systematic);
            let mut expected = bch.clone();
            expected.extend_from_slice(&parity);
            let encoded = codec.encode(&payload).unwrap();
            assert_eq!(&encoded.bch_mother[14232 - payload_bits..], bch);
            assert_eq!(encoded.ldpc_input, systematic);
            assert_eq!(encoded.transmitted, expected);
            assert_eq!(&encoded.ldpc_mother[14400..], parity);
            assert!(syndrome.valid_syndrome(&encoded.ldpc_mother));
            let decoded = codec.decode_noiseless(&encoded.transmitted, 30).unwrap();
            assert!(decoded.success(), "K={payload_bits} pattern={pattern}");
            assert_eq!(decoded.payload, payload);
            assert_eq!(decoded.ldpc.iterations, 0);
            assert_eq!(compare_payload(&payload, &decoded).unwrap().bit_errors, 0);
            assert_eq!(
                compare_payload(&payload, &decoded).unwrap().bits_tested,
                payload_bits
            );
        }
    }
}
#[test]
fn full_length_is_bit_exact_with_existing_codec_and_truth_does_not_trust_flags() {
    let payload: Vec<_> = (0..14232).map(|index| (index % 2) as u8).collect();
    let mut legacy = ConcatenatedCodec::new().unwrap();
    let mut codec = ShortenedCodec::new(Shortening::new(14232).unwrap()).unwrap();
    let legacy_word = legacy.encode(&payload).unwrap();
    let word = codec.encode(&payload).unwrap();
    assert_eq!(word.bch_mother, legacy_word.bch_codeword);
    assert_eq!(word.transmitted, legacy_word.ldpc_codeword);
    let expected = legacy.decode_noiseless(&word.transmitted, 30).unwrap();
    let mut decoded = codec.decode_noiseless(&word.transmitted, 30).unwrap();
    assert_eq!(decoded.ldpc.codeword, expected.ldpc.codeword);
    assert_eq!(decoded.bch.codeword, expected.bch.codeword);
    decoded.payload[23] ^= 1; // A caller-visible success flag is not payload truth.
    assert!(decoded.success());
    assert_eq!(compare_payload(&payload, &decoded).unwrap().bit_errors, 1);
    decoded.payload.pop();
    assert!(compare_payload(&payload, &decoded).is_err());
}
#[test]
fn bad_inputs_reject_without_poisoning_reusable_codec_state() {
    let mut codec = ShortenedCodec::new(Shortening::new(6136).unwrap()).unwrap();
    assert_eq!(codec.shortening().transmitted_bits(), 8104);
    assert!(codec.encode(&[]).is_err());
    assert!(codec.encode(&vec![2; 6136]).is_err());
    assert!(codec.decode_noiseless(&vec![2; 8104], 30).is_err());
    assert!(codec.decode(&vec![1.0; 8104], 0).is_err());
    assert!(codec.decode(&vec![1.0; 8104], 101).is_err());
    assert!(codec.decode(&vec![f64::NAN; 8104], 30).is_err());
    let original = vec![1; 6136];
    let encoded = codec.encode(&original).unwrap();
    let decoded = codec.decode_noiseless(&encoded.transmitted, 30).unwrap();
    assert!(decoded.success());
    assert_eq!(decoded.payload, original);
}
