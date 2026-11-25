/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::pre_equalization::{apply, reference_db, CorrectionMode};
#[test]
fn multiplication_replacement_and_phase_only_correction_are_distinct() {
    assert_eq!(
        apply(Some([1., 0.]), Some([0., 1.]), CorrectionMode::Adjust),
        Some([0., 1.])
    );
    assert_eq!(
        apply(Some([2., -1.]), Some([0.5, 0.25]), CorrectionMode::Adjust),
        Some([1.25, 0.])
    );
    assert_eq!(
        apply(Some([2., -1.]), Some([0.5, 0.25]), CorrectionMode::Set),
        Some([0.5, 0.25])
    );
    assert_eq!(
        apply(Some([1., -1.]), Some([1., 0.]), CorrectionMode::Adjust),
        Some([1., -1.])
    );
    assert_eq!(
        apply(Some([0., 0.]), Some([1., 0.]), CorrectionMode::Adjust),
        Some([0., 0.])
    );
    assert_eq!(apply(None, Some([1., 0.]), CorrectionMode::Set), None);
    assert_eq!(
        apply(Some([f64::NAN, 0.]), Some([1., 0.]), CorrectionMode::Adjust),
        None
    );
    assert_eq!(CorrectionMode::from_name("automatic"), None);
}
#[test]
fn power_reference_uses_linear_power_and_preserves_undefined_zero() {
    assert_eq!(reference_db(&[Some([1., 0.]), Some([0., 1.])]), Some(0.));
    assert!(
        (reference_db(&[Some([2., 0.]), Some([0., 2.])]).unwrap() - 6.020599913279624).abs()
            < 1e-12
    );
    assert!(
        (reference_db(&[Some([0., 0.]), Some([2., 0.])]).unwrap() - 3.010299956639812).abs()
            < 1e-12
    );
    assert_eq!(reference_db(&[]), None);
    assert_eq!(reference_db(&[Some([0., 0.])]), None);
}
