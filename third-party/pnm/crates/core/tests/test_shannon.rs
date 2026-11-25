/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::common::signal_processing::shannon::minimum_snr_db;

#[test]
fn independent_capacity_bounds_and_zero() {
    assert_eq!(minimum_snr_db(0), None);
    assert_eq!(minimum_snr_db(1), Some(0.0));
    // Independent 45-digit Decimal log10(M-1) reference values, tolerance1e-10dB for f64 rounding.
    for (bits, expected) in [
        (2, 4.771212547196624),
        (4, 11.760912590556813),
        (8, 24.06540180433955),
        (12, 36.12253906096437),
        (14, 42.14393431255206),
    ] {
        assert!((minimum_snr_db(bits).unwrap() - expected).abs() < 1e-10);
    }
    let mut previous = -1.0;
    for bits in 1..=u8::MAX {
        let value = minimum_snr_db(bits).unwrap();
        assert!(value.is_finite() && value > previous);
        previous = value;
    }
}
