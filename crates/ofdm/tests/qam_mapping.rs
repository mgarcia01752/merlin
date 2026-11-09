/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent PHY I20 label vectors, unit energy and shared soft-bit conventions.
use docsis_ofdm_fec::signal_processing::modulation::{
    map_qam16, map_symbol, qam16_llrs, qam_llrs, reference_for_order,
};

#[test]
fn control_labels_match_phy_figures99_and100_independently_of_resource_order() {
    assert_eq!(map_symbol(2, 0).unwrap(), [1.0, 0.0]);
    assert_eq!(map_symbol(2, 1).unwrap(), [-1.0, 0.0]);
    let scale = 1.0 / 2_f64.sqrt();
    for (label, point) in [[1.0, 1.0], [1.0, -1.0], [-1.0, 1.0], [-1.0, -1.0]]
        .iter()
        .enumerate()
    {
        assert_eq!(
            map_symbol(4, label as u16).unwrap(),
            [point[0] * scale, point[1] * scale]
        );
    }
}

#[test]
fn every_256qam_label_matches_figure108_and_quadrant_reflections() {
    // Independently transcribed Figure108, PDF201: rows Q=15..1, columns I=1..15.
    let labels: [[u16; 8]; 8] = [
        [0x20, 0x22, 0x2a, 0x28, 0x08, 0x0a, 0x02, 0x00],
        [0x21, 0x23, 0x2b, 0x29, 0x09, 0x0b, 0x03, 0x01],
        [0x25, 0x27, 0x2f, 0x2d, 0x0d, 0x0f, 0x07, 0x05],
        [0x24, 0x26, 0x2e, 0x2c, 0x0c, 0x0e, 0x06, 0x04],
        [0x34, 0x36, 0x3e, 0x3c, 0x1c, 0x1e, 0x16, 0x14],
        [0x35, 0x37, 0x3f, 0x3d, 0x1d, 0x1f, 0x17, 0x15],
        [0x31, 0x33, 0x3b, 0x39, 0x19, 0x1b, 0x13, 0x11],
        [0x30, 0x32, 0x3a, 0x38, 0x18, 0x1a, 0x12, 0x10],
    ];
    let scale = 1.0 / 170_f64.sqrt(); // AnnexA.2 Table54, PDF206.
    for (row, columns) in labels.iter().enumerate() {
        for (column, label) in columns.iter().enumerate() {
            for quadrant in 0..4 {
                let sign_i = if quadrant & 2 == 0 { 1.0 } else { -1.0 };
                let sign_q = if quadrant & 1 == 0 { 1.0 } else { -1.0 };
                let point = map_symbol(256, label | (quadrant << 6)).unwrap();
                assert_eq!(
                    point,
                    [
                        sign_i * (2 * column + 1) as f64 * scale,
                        sign_q * (15 - 2 * row) as f64 * scale
                    ]
                );
            }
        }
    }
}

#[test]
fn embedded_tables_have_normative_energy_and_unique_coordinates_through_16k() {
    // Table54, PDF206; label0 anchors Figures104–114, PDF199–205.
    let expected = [
        (16, 10.0, [3.0, 3.0]),
        (64, 42.0, [7.0, 7.0]),
        (128, 82.0, [7.0, 9.0]),
        (256, 170.0, [15.0, 15.0]),
        (512, 330.0, [15.0, 17.0]),
        (1024, 682.0, [31.0, 31.0]),
        (2048, 1322.0, [31.0, 33.0]),
        (4096, 2730.0, [63.0, 63.0]),
        (8192, 5290.0, [63.0, 65.0]),
        (16384, 10922.0, [127.0, 127.0]),
    ];
    for (order, energy, anchor) in expected {
        let reference = reference_for_order(order).unwrap();
        assert_eq!(reference.iq.len(), order);
        assert_eq!(usize::from(reference.bit_loading), order.ilog2() as usize);
        let scale = 1.0 / f64::sqrt(energy);
        assert_eq!(reference.scale_factor, scale);
        assert_eq!(reference.iq[0], [anchor[0] * scale, anchor[1] * scale]);
        let mean = reference
            .iq
            .iter()
            .map(|point| point[0].powi(2) + point[1].powi(2))
            .sum::<f64>()
            / order as f64;
        assert!((mean - 1.0).abs() < 1e-12);
        let unique: std::collections::BTreeSet<_> = reference
            .iq
            .iter()
            .map(|point| (point[0].to_bits(), point[1].to_bits()))
            .collect();
        assert_eq!(unique.len(), order);
    }
}

#[test]
fn all_256qam_cells_recover_eight_msb_first_bits_without_channel_noise() {
    for label in 0..256 {
        let point = map_symbol(256, label).unwrap();
        let llrs = qam_llrs(256, point, 0.01).unwrap();
        assert_eq!(llrs.len(), 8);
        for (bit, llr) in llrs.iter().enumerate() {
            assert_ne!(*llr, 0.0);
            assert_eq!(u16::from(*llr < 0.0), (label >> (7 - bit)) & 1);
        }
    }
}

#[test]
fn higher_orders_and_control_references_use_same_softbit_contract() {
    for order in [
        2u16, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384,
    ] {
        let bits = order.ilog2() as usize;
        for label in [0, 1, order / 2, order - 1] {
            let point = map_symbol(order, label).unwrap();
            let llrs = qam_llrs(order, point, 0.01).unwrap();
            assert_eq!(llrs.len(), bits);
            for (bit, llr) in llrs.iter().enumerate() {
                assert!(llr.is_finite() && llr.abs() <= 100.0);
                assert_ne!(*llr, 0.0);
                assert_eq!(u16::from(*llr < 0.0), (label >> (bits - 1 - bit)) & 1);
            }
        }
    }
    let point = map_symbol(256, 0x35).unwrap();
    let first = qam_llrs(256, point, 1.0).unwrap();
    let second = qam_llrs(256, point, 2.0).unwrap();
    for (first, second) in first.iter().zip(second) {
        assert!((first / 2.0 - second).abs() < 1e-12);
    }
    for label in 0..16 {
        let point = map_qam16(label).unwrap();
        assert_eq!(
            qam_llrs(16, point, 0.1).unwrap(),
            qam16_llrs(point, 0.1).unwrap()
        );
    }
}

#[test]
fn invalid_orders_labels_and_nonfinite_metrics_return_errors() {
    for order in [0, 8, 32, 65535] {
        assert!(reference_for_order(usize::from(order)).is_none());
        assert_eq!(
            map_symbol(order, 0).unwrap_err().code,
            "UnsupportedModulation"
        );
        assert!(qam_llrs(order, [0.0, 0.0], 1.0).is_err());
    }
    assert!(map_symbol(256, 256).is_err());
    assert!(map_symbol(16384, u16::MAX).is_err());
    for variance in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert!(qam_llrs(256, [0.0, 0.0], variance).is_err());
    }
    for point in [[f64::NAN, 0.0], [0.0, f64::INFINITY], [f64::MAX, 0.0]] {
        assert!(qam_llrs(256, point, 1.0).is_err());
    }
}
