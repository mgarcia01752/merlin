/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::simulation::channel::performance::CarrierMer;
#[test]
fn mer_uses_power_sums_preserving_missing_and_zero_error() {
    let mut mer = CarrierMer::default();
    assert_eq!(mer.mer_db(), None);
    mer.observe([1.0, 0.0], [1.0, 0.0]);
    assert_eq!(mer.samples(), 1);
    assert_eq!(mer.mer_db(), None);
    assert_eq!(mer.error_power(), 0.0);
    mer.observe([f64::NAN, 0.0], [0.0, 0.0]);
    assert_eq!(mer.samples(), 1);
    let mut measured = CarrierMer::default();
    measured.observe([1.0, 0.0], [0.9, 0.0]);
    measured.observe([2.0, 0.0], [1.5, 0.0]);
    assert_eq!(measured.samples(), 2);
    assert!((measured.signal_power() - 5.0).abs() < 1e-12);
    assert!((measured.error_power() - 0.26).abs() < 1e-12);
    assert!((measured.mer_db().unwrap() - 10.0 * (5.0_f64 / 0.26).log10()).abs() < 1e-12);
}
