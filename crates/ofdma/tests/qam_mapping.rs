/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Annex A independent source labels, shapes, scaling and soft metric sign.
use docsis_ofdma::{
    signal_processing::{map_symbol, max_log, reference_energy},
    types::Iq,
};
use std::collections::BTreeSet;

#[test]
fn independent_annex_labels_and_every_order_mean_energy() {
    for (bits, label, i, q) in [
        (1, 0, 1, 0),
        (1, 1, -1, 0),
        (2, 0, 1, 1),
        (2, 1, 1, -1),
        (2, 2, -1, 1),
        (2, 3, -1, -1),
        (3, 0, 1, -3),
        (3, 4, -3, -3),
        (3, 6, 3, 3),
        (4, 0, 3, 3),
        (4, 7, 1, -1),
        (4, 10, -1, 3),
        (5, 0, 3, 5),
        (5, 1, 5, 3),
        (5, 2, 1, 5),
        (5, 16, -3, 5),
        (5, 31, -3, -1),
        // Independently read PHY Annex A Figures 104-112, PDF pp199-203.
        (6, 0, 7, 7),
        (6, 0x20, -7, 7),
        (7, 0, 7, 9),
        (7, 2, 7, 11),
        (7, 0x45, -11, 7),
        (8, 0, 15, 15),
        (8, 0x20, 1, 15),
        (8, 0x10, 15, 1),
        (9, 0, 15, 17),
        (9, 8, 15, 23),
        (9, 0x70, 15, 1),
        (9, 0x34, 23, 1),
        (10, 0, 31, 31),
        (10, 0x80, 1, 31),
        (10, 0x40, 31, 1),
        (11, 0, 31, 33),
        (11, 0x20, 31, 47),
        (11, 0x100, 1, 31),
        (11, 0x180, 1, 1),
        (12, 0, 63, 63),
        (12, 0x200, 1, 63),
        (12, 0x100, 63, 1),
    ] {
        let point = map_symbol(bits, label).unwrap();
        let scale = reference_energy(bits).unwrap().sqrt();
        assert!(
            (point.re * scale - i as f64).abs() < 1e-13,
            "bits={bits} label={label:x}: got I={} expected {i}",
            point.re * scale
        );
        assert!(
            (point.im * scale - q as f64).abs() < 1e-13,
            "bits={bits} label={label:x}: got Q={} expected {q}",
            point.im * scale
        );
    }
    for (bits, expected_energy) in [
        (1, 1.0),
        (2, 2.0),
        (3, 10.0),
        (4, 10.0),
        (5, 20.0),
        (6, 42.0),
        (7, 82.0),
        (8, 170.0),
        (9, 330.0),
        (10, 682.0),
        (11, 1322.0),
        (12, 2730.0),
    ] {
        assert_eq!(reference_energy(bits).unwrap(), expected_energy);
        let order = 1_u16 << bits;
        let points: Vec<_> = (0..order)
            .map(|label| map_symbol(bits, label).unwrap())
            .collect();
        assert!(
            (points.iter().map(|z| z.power()).sum::<f64>() / f64::from(order) - 1.0).abs() < 2e-13
        );
        let unique: BTreeSet<_> = points
            .iter()
            .map(|z| (z.re.to_bits(), z.im.to_bits()))
            .collect();
        assert_eq!(unique.len(), order as usize);
        for label in [0, 1, order / 2, order - 1] {
            let metrics = max_log(bits, Some(points[label as usize]), 0.1).unwrap();
            for (b, llr) in metrics.into_iter().enumerate() {
                assert_eq!(u16::from(llr < 0.0), (label >> (bits as usize - 1 - b)) & 1);
            }
        }
        assert!(map_symbol(bits, order).is_err());
    }
    assert!(map_symbol(13, 0).is_err());
    assert!(map_symbol(14, 0).is_err());
}

#[test]
fn maxlog_reference_variance_ties_erasures_and_finite_rejection() {
    assert!((max_log(1, Some(Iq { re: 0.25, im: 0.7 }), 0.5).unwrap()[0] - 2.0).abs() < 1e-14);
    assert_eq!(max_log(1, Some(Iq::default()), 0.5).unwrap(), vec![0.0]);
    assert_eq!(max_log(12, None, 0.5).unwrap(), vec![0.0; 12]);
    assert_eq!(
        max_log(1, Some(Iq { re: 1.0, im: 0.0 }), 0.001).unwrap(),
        vec![50.0]
    );
    for variance in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::MIN_POSITIVE / 10.0] {
        assert!(max_log(2, Some(Iq::default()), variance).is_err());
    }
    assert!(max_log(
        2,
        Some(Iq {
            re: f64::MAX,
            im: 0.0
        }),
        1.0
    )
    .is_err());
}
