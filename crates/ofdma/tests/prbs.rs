/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Literal recurrence/period and independent contiguous-source comparison.
use docsis_ofdma::{
    source::{Prbs15, PrbsVerifier},
    types::Identity,
};

#[test]
fn literal_seed_phase_period_and_chunk_continuity() {
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 0,
    };
    let mut source = Prbs15::new(1).unwrap();
    let expected: Vec<_> = b"100000000000000111111111111111"
        .iter()
        .map(|c| c - b'0')
        .collect();
    assert_eq!(source.generate(id, 30).unwrap().bits, expected);
    for seed in [1, 0x1234, 32767] {
        let mut source = Prbs15::new(seed).unwrap();
        let period = source.generate(id, 32767).unwrap().bits;
        assert_eq!(period.iter().filter(|bit| **bit == 1).count(), 16384);
        assert_eq!(source.generate(id, 30).unwrap().bits, period[..30]);
        let mut split = Prbs15::new(seed).unwrap();
        let mut pieces = split.generate(id, 17).unwrap().bits;
        let later = Identity { grant: 1, ..id };
        pieces.extend(split.generate(later, 32750).unwrap().bits);
        assert_eq!(pieces, period);
        assert_eq!(split.position(), 32767);
    }
    assert!(Prbs15::new(0).is_err());
    assert!(Prbs15::new(32768).is_err());
}

#[test]
fn rejected_calls_and_coverage_gaps_never_advance_state() {
    let id = Identity {
        run: 1,
        generation: 1,
        grant: 0,
    };
    let mut source = Prbs15::new(1).unwrap();
    assert!(source.generate(Identity { run: 0, ..id }, 10).is_err());
    assert!(source.generate(id, usize::MAX).is_err());
    assert_eq!(source.position(), 0);
    let chunk = source.generate(id, 90).unwrap();
    let mut verifier = PrbsVerifier::new(1).unwrap();
    assert_eq!(verifier.compare(0, &[]).unwrap().ber, None);
    assert!(verifier.compare(1, &chunk.bits).is_err());
    assert!(verifier.compare(0, &[2]).is_err());
    let mut wrong = chunk.bits.clone();
    wrong[7] ^= 1;
    let comparison = verifier.compare(0, &wrong).unwrap();
    assert_eq!(comparison.bit_errors, 1);
    assert!(comparison.ber.unwrap() > 0.0);
    assert!(verifier.compare(0, &chunk.bits).is_err());
    let next = source.generate(id, 33).unwrap();
    assert_eq!(verifier.compare(90, &next.bits).unwrap().bit_errors, 0);
}
