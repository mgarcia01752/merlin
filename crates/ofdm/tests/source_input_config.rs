/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Source configuration works independently of waveform and FEC compilation.
#![cfg(any(feature = "simulation", feature = "source-input"))]
use docsis_ofdm_fec::simulation::source::{
    file::{FilePayload, PrbsFill, MAX_FILE_BITS},
    Prbs15,
};

#[test]
fn immutable_fill_configuration_shares_bytes_and_rejects_invalid_policies() {
    let original = FilePayload::new(&[0xef, 0xbb, 0xbf, 0, 0xff, 13, 10], None).unwrap();
    let filled = original
        .with_fill(Some(PrbsFill {
            target_bits: 59,
            seed: 1,
        }))
        .unwrap();
    assert_eq!(filled.bytes(), &[0xef, 0xbb, 0xbf, 0, 0xff, 13, 10]);
    assert_eq!(original.bytes().as_ptr(), filled.bytes().as_ptr());
    assert_eq!(original.target_bits(), 56);
    assert_eq!(filled.target_bits(), 59);
    for (target_bits, seed) in [(55, 1), (MAX_FILE_BITS + 1, 1), (59, 0), (59, 32768)] {
        assert!(original
            .with_fill(Some(PrbsFill { target_bits, seed }))
            .is_err());
        assert_eq!(original.target_bits(), 56);
    }
    assert_eq!(filled.with_fill(None).unwrap().target_bits(), 56);
    let empty = FilePayload::new(&[], None).unwrap();
    assert_eq!(
        empty
            .with_fill(Some(PrbsFill {
                target_bits: 3,
                seed: 32767
            }))
            .unwrap()
            .target_bits(),
        3
    );
}

#[test]
fn source_only_prbs_matches_literal_sequence_and_rejects_invalid_seeds() {
    assert!(Prbs15::new(0).is_err());
    assert!(Prbs15::new(32768).is_err());
    let bits = Prbs15::new(32767).unwrap().generate(32).unwrap();
    let literal: Vec<u8> = b"11111111111111101010101010101001"
        .iter()
        .map(|digit| digit - b'0')
        .collect();
    assert_eq!(bits, literal);
}
