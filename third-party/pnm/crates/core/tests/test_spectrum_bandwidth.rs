/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent numeric checks for protocol-neutral spectrum bandwidth math.
use pnm::common::signal_processing::spectrum::resolution_bandwidth_hz;

#[test]
fn known_bandwidths_retain_fractional_hz() {
    assert_eq!(resolution_bandwidth_hz(25_000.0, 1.0), Some(25_000.0));
    assert_eq!(resolution_bandwidth_hz(25_000.0, 1.5), Some(37_500.0));
    assert_eq!(resolution_bandwidth_hz(0.25, 0.5), Some(0.125));
    // 1.1 is not exactly representable; tolerance is less than a nanohertz.
    assert!((resolution_bandwidth_hz(29_296.875, 1.1).unwrap() - 32_226.562_5).abs() < 1e-9);
}

#[test]
fn invalid_nonfinite_overflow_and_underflow_are_rejected() {
    for invalid in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(resolution_bandwidth_hz(invalid, 1.0), None);
        assert_eq!(resolution_bandwidth_hz(1.0, invalid), None);
    }
    assert_eq!(resolution_bandwidth_hz(f64::MAX, 2.0), None);
    assert_eq!(resolution_bandwidth_hz(f64::from_bits(1), 0.25), None);
}
