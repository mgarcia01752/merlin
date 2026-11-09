/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent recurrence expectations, period and recovered-stream accounting.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::simulation::source::{
    prbs::{MAX_PRBS_CHUNK_BITS, PRBS15_PERIOD},
    Prbs15, PrbsVerifier,
};
use fec_codec::parameters::BCH_INPUT_BITS;

#[test]
fn literal_recurrence_vectors_preserve_seed_order_and_feedback() {
    // Locally derived sequence vectors, not CableLabs interoperability vectors:
    // first 15 values are seed bits 0..14; append s[n+14] XOR s[n].
    for (seed, expected) in [
        (0x7fff, "111111111111111010101010101010011001100110011101110111011101001011010010110001101100011011110110"),
        (1, "100000000000000111111111111111010101010101010011001100110011101110111011101001011010010110001101"),
        (0x1234, "001011000100100001101111000111110110101111010100100110101100111000100110111010000111011010011111"),
    ] {
        let literal: Vec<_> = expected.bytes().map(|digit| digit - b'0').collect();
        assert_eq!(Prbs15::new(seed).unwrap().generate(literal.len()).unwrap(), literal);
    }
}

#[test]
fn full_period_has_unique_nonzero_windows_and_expected_balance() {
    let period = usize::from(PRBS15_PERIOD);
    let mut source = Prbs15::new(0x1234).unwrap();
    let bits = source.generate(period + 15).unwrap();
    let mut seen = vec![false; period + 1];
    for window in bits.windows(15).take(period) {
        let state = window
            .iter()
            .enumerate()
            .fold(0usize, |state, (offset, bit)| {
                state | (usize::from(*bit) << offset)
            });
        assert_ne!(state, 0);
        assert!(!seen[state], "sequence repeated before the full period");
        seen[state] = true;
    }
    assert!(seen[1..].iter().all(|value| *value));
    assert_eq!(&bits[..15], &bits[period..]);
    assert_eq!(
        bits[..period].iter().filter(|bit| **bit == 1).count(),
        16_384
    );
    assert_eq!(source.position(), (period + 15) as u64);
}

#[test]
fn chunk_and_payload_boundaries_never_reseed_or_reset_period_position() {
    let mut source = Prbs15::new(1).unwrap();
    let mut chunks = source.generate(7).unwrap();
    assert!(source.generate(0).unwrap().is_empty());
    for _ in 0..4 {
        chunks.extend(source.next_payload().unwrap());
    }
    chunks.extend(source.generate(19).unwrap());
    let expected = Prbs15::new(1)
        .unwrap()
        .generate(4 * BCH_INPUT_BITS + 26)
        .unwrap();
    assert_eq!(chunks, expected);
    assert_eq!(source.position(), expected.len() as u64);
}

#[test]
fn rejected_source_and_verifier_calls_preserve_state() {
    for seed in [0, 0x8000, u16::MAX] {
        assert!(Prbs15::new(seed).is_err());
        assert!(PrbsVerifier::new(seed).is_err());
    }
    let mut source = Prbs15::new(7).unwrap();
    assert!(source.generate(MAX_PRBS_CHUNK_BITS + 1).is_err());
    assert_eq!(source.position(), 0);
    let expected = source.generate(96).unwrap();
    let mut verifier = PrbsVerifier::new(7).unwrap();
    assert!(verifier.compare(&[0, 1, 2]).is_err());
    assert!(verifier.compare(&vec![0; MAX_PRBS_CHUNK_BITS + 1]).is_err());
    assert_eq!(verifier.totals().bits_tested, 0);
    assert_eq!(verifier.compare(&expected).unwrap().bit_errors, 0);
}

#[test]
fn independent_verifier_reports_exact_errors_across_payload_and_period_edges() {
    let mut source = Prbs15::new(0x7fff).unwrap();
    let mut bits = source.generate(4 * BCH_INPUT_BITS).unwrap();
    let errors = [
        0,
        BCH_INPUT_BITS - 1,
        BCH_INPUT_BITS,
        usize::from(PRBS15_PERIOD),
        bits.len() - 1,
    ];
    for offset in errors {
        bits[offset] ^= 1;
    }
    let mut verifier = PrbsVerifier::new(0x7fff).unwrap();
    for (index, chunk) in bits.chunks(BCH_INPUT_BITS).enumerate() {
        let comparison = verifier.compare(chunk).unwrap();
        let start = index * BCH_INPUT_BITS;
        let expected: Vec<_> = errors
            .iter()
            .filter(|offset| (start..start + chunk.len()).contains(offset))
            .collect();
        assert_eq!(comparison.first_bit, start as u64);
        assert_eq!(comparison.bit_errors, expected.len() as u64);
        assert_eq!(
            comparison.first_error_bit,
            expected.first().map(|offset| **offset as u64)
        );
    }
    assert_eq!(verifier.totals().bits_tested, bits.len() as u64);
    assert_eq!(verifier.totals().bit_errors, errors.len() as u64);
    assert_eq!(verifier.totals().first_error_bit, Some(0));
    assert_eq!(
        verifier
            .compare(&source.generate(71).unwrap())
            .unwrap()
            .bit_errors,
        0
    );
}
