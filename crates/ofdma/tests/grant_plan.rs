/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Source-transcribed thresholds/borrowing and independent integer conservation.
use docsis_ofdma::grant::select;
use fec_codec::upstream_ldpc::DocsisLdpcCode::{
    UpstreamLong as L, UpstreamMedium as M, UpstreamShort as S,
};

#[test]
fn source_threshold_and_two_shortened_word_literals() {
    for (capacity, expected, pad, source, fill) in [
        (700, vec![(S, 420)], 0, 416, 4),
        (1120, vec![(S, 840)], 0, 840, 0),
        (1400, vec![(S, 840)], 280, 840, 0),
        (1401, vec![(S, 420), (S, 421)], 0, 840, 1),
        (3420, vec![(S, 840), (S, 840), (S, 840)], 60, 2520, 0),
        (3421, vec![(M, 2521)], 0, 2520, 1),
        (5940, vec![(M, 5040)], 0, 5040, 0),
        (6221, vec![(M, 4620), (S, 421)], 0, 5040, 1),
        (11880, vec![(M, 5040), (M, 5040)], 0, 10080, 0),
        (11881, vec![(L, 10081)], 0, 10080, 1),
        (16200, vec![(L, 14400)], 0, 14400, 0),
        (16481, vec![(L, 13980), (S, 421)], 0, 14400, 1),
    ] {
        let plan = select(capacity).unwrap();
        assert_eq!(
            plan.codewords
                .iter()
                .map(|word| (word.code, word.information_bits))
                .collect::<Vec<_>>(),
            expected,
            "capacity {capacity}"
        );
        assert_eq!(
            (
                plan.grant_pad_bits,
                plan.source_bits,
                plan.information_fill_bits
            ),
            (pad, source, fill)
        );
    }
    for capacity in [0, 1, 280, 281, 699] {
        let plan = select(capacity).unwrap();
        assert!(!plan.transmit);
        assert!(plan.codewords.is_empty());
        assert_eq!(
            (
                plan.source_bits,
                plan.grant_pad_bits,
                plan.unused_capacity_bits
            ),
            (0, 0, capacity)
        );
    }
}

#[test]
fn complete_bounded_capacity_sweep_conserves_every_role() {
    for capacity in 700..50_000 {
        let plan = select(capacity).unwrap();
        let mut information = 0;
        let mut coded = 0;
        for word in &plan.codewords {
            assert_eq!(
                (word.information_offset, word.coded_offset),
                (information, coded)
            );
            let p = word.code.parameters();
            assert!((1..=p.information_bits).contains(&word.information_bits));
            assert_eq!(
                word.transmitted_bits,
                word.information_bits + p.codeword_bits - p.information_bits
            );
            coded += word.transmitted_bits;
            information += word.information_bits;
        }
        assert_eq!(coded + plan.grant_pad_bits, capacity);
        assert_eq!(plan.source_bits + plan.information_fill_bits, information);
        assert_eq!(plan.source_bits % 8, 0);
    }
    assert!(select(usize::MAX).is_err());
}
