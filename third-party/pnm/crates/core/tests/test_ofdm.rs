/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent useful-symbol rate and data-bandwidth examples.
use pnm::common::signal_processing::ofdm::{data_bandwidth, useful_symbol_rate};
#[test]
fn ideal_rate_and_bandwidth_known_values_and_bounds() {
    // 100 data carriers at 8 bits, Tu=40 us: 800/40e-6 = 20 Mbit/s.
    assert_eq!(useful_symbol_rate(800, 25_000), Some(20_000_000));
    assert_eq!(data_bandwidth(100, 25_000), Some(2_500_000));
    assert_eq!(useful_symbol_rate(800, 50_000), Some(40_000_000));
    assert_eq!(data_bandwidth(100, 50_000), Some(5_000_000));
    assert_eq!(useful_symbol_rate(0, 25_000), Some(0));
    assert_eq!(data_bandwidth(0, 25_000), Some(0));
    assert_eq!(useful_symbol_rate(800, 0), None);
    assert_eq!(data_bandwidth(100, 0), None);
    assert_eq!(useful_symbol_rate(u64::MAX, 2), None);
    assert_eq!(data_bandwidth(u64::MAX, 25_000), None);
    assert_eq!(data_bandwidth(u64::from(u32::MAX), 1), Some(u32::MAX));
    assert_eq!(data_bandwidth(u64::from(u32::MAX) + 1, 1), None);
}
