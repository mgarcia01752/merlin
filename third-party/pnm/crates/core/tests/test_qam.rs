/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::qam::{reference_for_modulation_type, reference_for_order};
use std::collections::BTreeSet;

#[test]
fn compiled_geometries_have_unit_energy_unique_points_and_balanced_axes() {
    for (order, bits) in [
        (2, 1),
        (4, 2),
        (16, 4),
        (64, 6),
        (128, 7),
        (256, 8),
        (512, 9),
        (1024, 10),
        (2048, 11),
        (4096, 12),
        (8192, 13),
        (16384, 14),
    ] {
        let reference = reference_for_order(order).unwrap();
        assert_eq!(reference.iq.len(), usize::from(order));
        assert_eq!(reference.bit_loading, bits);
        let unique: BTreeSet<_> = reference
            .iq
            .iter()
            .map(|point| (point[0].to_bits(), point[1].to_bits()))
            .collect();
        assert_eq!(unique.len(), reference.iq.len());
        let energy = reference
            .iq
            .iter()
            .map(|point| point[0] * point[0] + point[1] * point[1])
            .sum::<f64>()
            / f64::from(order);
        assert!((energy - 1.0).abs() < 1e-12, "{order}-QAM energy");
        for axis in 0..2 {
            assert!(
                reference
                    .iq
                    .iter()
                    .map(|point| point[axis])
                    .sum::<f64>()
                    .abs()
                    < 1e-9
            );
        }
    }
    for order in [0, 1, 3, 8, 32, u16::MAX] {
        assert!(reference_for_order(order).is_none());
    }
}
#[test]
fn independent_square_and_cross_qam_scaling_references() {
    // Independent 50-digit Decimal reference calculations; 1e-14 rounding tolerance.
    assert_eq!(
        reference_for_order(2).unwrap().iq,
        &[[-1.0, 0.0], [1.0, 0.0]]
    );
    let qam16 = reference_for_order(16).unwrap();
    assert!((qam16.scale_factor - 0.31622776601683794).abs() < 1e-14);
    assert!((qam16.iq[0][0] - 0.9486832980505138).abs() < 1e-14);
    assert!((qam16.iq[0][1] - 0.9486832980505138).abs() < 1e-14);
    let qam128 = reference_for_order(128).unwrap();
    assert!((qam128.scale_factor - 0.11043152607484655).abs() < 1e-14);
    assert!((qam128.iq[0][0] - 0.7730206825239258).abs() < 1e-14);
    assert!((qam128.iq[0][1] - 0.9938837346736189).abs() < 1e-14);
    assert!((reference_for_order(256).unwrap().scale_factor - 0.07669649888473704).abs() < 1e-14);
}
#[test]
fn header_enum_is_not_profile_assignment_or_literal_qam_order() {
    for (code, order) in [
        (3, 4),
        (4, 16),
        (5, 64),
        (6, 128),
        (7, 256),
        (8, 512),
        (9, 1024),
        (10, 2048),
        (11, 4096),
        (12, 8192),
        (13, 16384),
    ] {
        assert_eq!(reference_for_modulation_type(code).unwrap().order, order);
    }
    for code in [0, 1, 2, 14, 16, 20, 256, u16::MAX] {
        assert!(reference_for_modulation_type(code).is_none());
    }
    assert!(std::ptr::eq(
        reference_for_order(256).unwrap(),
        reference_for_modulation_type(7).unwrap()
    ));
}
