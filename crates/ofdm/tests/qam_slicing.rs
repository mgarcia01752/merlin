/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent decision anchors, deterministic ties and error-coordinate conventions.
use docsis_ofdm_fec::signal_processing::modulation::{map_symbol, slice_symbol};
#[test]
fn decisions_cover_every_order_and_preserve_received_minus_reference_error() {
    for order in [2, 4, 16, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384] {
        for label in [0, 1, order / 2, order - 1] {
            let point = map_symbol(order, label).unwrap();
            let received = [point[0] + 0.0001, point[1] - 0.0002];
            let decision = slice_symbol(order, received).unwrap();
            assert_eq!(decision.label, label);
            assert_eq!(decision.reference_iq, point);
            assert!((decision.error_iq[0] - 0.0001).abs() < 1e-15);
            assert!((decision.error_iq[1] + 0.0002).abs() < 1e-15);
            assert!((decision.distance_squared - 5e-8).abs() < 1e-18);
        }
    }
}
#[test]
fn exact_ties_choose_lowest_label_and_outside_points_still_slice() {
    // PHY AnnexA: BPSK labels0=+1,1=-1; QPSK quadrant labels00,01,10,11.
    assert_eq!(slice_symbol(2, [0.0, 0.0]).unwrap().label, 0);
    assert_eq!(slice_symbol(4, [0.0, 0.0]).unwrap().label, 0);
    assert_eq!(slice_symbol(4, [-0.5, 0.0]).unwrap().label, 2);
    let decision = slice_symbol(2, [-10.0, 4.0]).unwrap();
    assert_eq!(decision.reference_iq, [-1.0, 0.0]);
    assert_eq!(decision.error_iq, [-9.0, 4.0]);
    assert_eq!(decision.distance_squared, 97.0);
}
#[test]
fn malformed_samples_and_unsupported_constellations_reject() {
    for order in [0, 8, 32, 65535] {
        assert_eq!(
            slice_symbol(order, [0.0, 0.0]).unwrap_err().code,
            "UnsupportedModulation"
        );
    }
    for point in [[f64::NAN, 0.0], [0.0, f64::INFINITY], [f64::MAX, 0.0]] {
        assert!(slice_symbol(256, point).is_err());
    }
}
