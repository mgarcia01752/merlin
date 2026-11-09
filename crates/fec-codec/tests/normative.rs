/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent specification arithmetic, not encoder/decoder round trips.
//! Mathematical table values: ETSI EN302769 V1.2.1 Tables4(b)/B.6, pp30/96,
//! referenced by CM-SP-PHYv3.1-I20-230419 section7.5.4.2/Table23.
//! These are independently transcribed parameter checks, not interoperability vectors.
use fec_codec::{encode_bch, LdpcCodec, BCH_INPUT_BITS};
#[path = "support/payload.rs"]
mod payload_reference;
#[test]
fn ldpc_matches_independent_normative_accumulator() {
    let codec = LdpcCodec::new().unwrap();
    for pattern in 0..3 {
        let message: Vec<u8> = (0..14400)
            .map(|index| {
                u8::from(match pattern {
                    0 => index % 997 == 0,
                    1 => index == 14399,
                    _ => (index * 17 + index / 11) % 19 < 9,
                })
            })
            .collect();
        let parity = payload_reference::ldpc_parity(&message);
        let word = codec.encode(&message).unwrap();
        assert_eq!(&word[14400..], &parity);
    }
}
#[test]
fn bch_matches_independent_binary_polynomial_division() {
    for pattern in 0..3 {
        let message: Vec<u8> = (0..BCH_INPUT_BITS)
            .map(|index| {
                u8::from(match pattern {
                    0 => index == 0,
                    1 => index == BCH_INPUT_BITS - 1,
                    _ => (index * 17 + index / 11) % 19 < 9,
                })
            })
            .collect();
        let parity = payload_reference::bch_parity(&message);
        let encoded = encode_bch(&message).unwrap();
        assert_eq!(&encoded[BCH_INPUT_BITS..], &parity);
    }
}
