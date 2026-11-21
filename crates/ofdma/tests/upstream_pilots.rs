/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Original Figures 35/39–42: masks, late symbols, gains and physical indexing.
use docsis_ofdma::{
    bit_processing::pilot_reference,
    configuration::Numerology,
    resources::{role, CellRole},
};

#[test]
fn source_literals_and_all_family_role_counts() {
    for (pattern, q) in [(1, 8), (8, 16)] {
        let pilots: Vec<_> = (0..6)
            .flat_map(|t| (0..q).map(move |k| (t, k)))
            .filter(|(t, k)| role(pattern, 6, *t, *k, true).unwrap().0 == CellRole::KnownPilot)
            .collect();
        let comp: Vec<_> = (0..6)
            .flat_map(|t| (0..q).map(move |k| (t, k)))
            .filter(|(t, k)| role(pattern, 6, *t, *k, true).unwrap().0 == CellRole::Complementary)
            .collect();
        assert_eq!(pilots, vec![(0, 0), (0, q - 1), (2, 0), (2, q - 1)]);
        assert_eq!(comp, vec![(3, 0), (3, q - 1), (5, 0), (5, q - 1)]);
    }
    for pattern in 1..=14 {
        let q = if pattern <= 7 { 8 } else { 16 };
        let local = if q == 8 { pattern - 1 } else { pattern - 8 };
        for k in 6..=if q == 8 { 36 } else { 18 } {
            for edge in [false, true] {
                let mut p = 0;
                let mut c = 0;
                for t in 0..k {
                    for f in 0..q {
                        let (r, gain) = role(pattern, k, t, f, edge).unwrap();
                        if r == CellRole::KnownPilot {
                            p += 1;
                        }
                        if r == CellRole::Complementary {
                            c += 1;
                        }
                        if local < 4 || r == CellRole::Data {
                            assert_eq!(gain, 1.0);
                        }
                        if local >= 4 && r == CellRole::KnownPilot {
                            assert!((gain * gain - 3.0).abs() < 1e-14);
                        }
                        if t >= if q == 8 { 16 } else { 9 } {
                            assert_eq!(r, CellRole::Data);
                        }
                    }
                }
                let body = match local {
                    0 => 2,
                    1 => 4,
                    2 => 8,
                    3 => 16,
                    4 => 1,
                    5 => 2,
                    _ => 4,
                };
                let edge_extra = if !edge || q == 8 && local == 3 {
                    0
                } else if local < 4 {
                    2
                } else {
                    1
                };
                assert_eq!(p, body + edge_extra, "pattern={pattern},K={k},edge={edge}");
                assert_eq!(c, if local < 4 { 2 } else { 1 } * if edge { 2 } else { 1 });
            }
        }
    }
}

#[test]
fn boosted_short_frame_complement_exception_and_physical_pilot_sequence() {
    for pattern in [6, 7, 13, 14] {
        assert_eq!(
            role(pattern, 6, 3, 0, false).unwrap(),
            (CellRole::Complementary, 1.0)
        );
    }
    let bits = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1];
    for mode in [Numerology::Khz50, Numerology::Khz25] {
        let reference = pilot_reference(mode);
        assert_eq!(reference.len(), mode.fft_len());
        for (i, bit) in bits.into_iter().enumerate() {
            assert_eq!(reference[i].re, if bit == 0 { 1.0 } else { -1.0 });
        }
        assert!(reference.iter().all(|z| z.power() == 1.0 && z.im == 0.0));
        if mode == Numerology::Khz25 {
            assert_eq!(reference[4095], reference[0]);
        }
    }
}
