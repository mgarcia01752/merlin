/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pinned PHY timing and boundary tests, independent of codec round trips.
use docsis_ofdm_fec::transmission::{SubcarrierSpacing, TransmissionSettings};

#[test]
fn one_second_8k_schedule_uses_prefix_and_overlapped_rolloff() {
    let settings = TransmissionSettings::default();
    let timing = settings.one_second_timing().unwrap();
    assert_eq!(timing.fft_samples, 8192);
    assert_eq!(timing.symbol_period_samples, 8704);
    assert_eq!(timing.windowed_symbol_samples, 8832);
    assert_eq!(timing.complete_symbols, 23529);
    assert_eq!(timing.unused_samples, 3584);
    assert_eq!(
        timing.complete_symbols * timing.symbol_period_samples + timing.unused_samples,
        204_800_000
    );
    let no_rolloff = TransmissionSettings {
        roll_off_samples: 0,
        ..settings
    };
    assert_eq!(
        no_rolloff.one_second_timing().unwrap().complete_symbols,
        timing.complete_symbols
    );
}

#[test]
fn spacing_specific_depth_and_4k_timing_follow_phy() {
    let mut settings = TransmissionSettings {
        subcarrier_spacing: SubcarrierSpacing::Khz50,
        time_interleaver_depth: 32,
        ..Default::default()
    };
    let timing = settings.one_second_timing().unwrap();
    assert_eq!(timing.fft_samples, 4096);
    assert_eq!(timing.symbol_period_samples, 4608);
    assert_eq!(timing.complete_symbols, 44444);
    assert_eq!(timing.unused_samples, 2048);
    settings.subcarrier_spacing = SubcarrierSpacing::Khz25;
    assert!(settings.validate().is_err());
    settings.time_interleaver_depth = 16;
    settings.validate().unwrap();
    settings.time_interleaver_depth = 0;
    assert!(settings.validate().is_err());
}

#[test]
fn rejects_invalid_bandwidth_prefix_and_rolloff_combinations() {
    for bandwidth in [0, 18_000_000, 25_000_000, 198_000_000, u32::MAX] {
        let settings = TransmissionSettings {
            occupied_bandwidth_hz: bandwidth,
            ..Default::default()
        };
        assert!(settings.one_second_timing().is_err());
    }
    for (prefix, rolloff) in [(193, 64), (512, 65), (192, 192), (192, 256)] {
        let settings = TransmissionSettings {
            cyclic_prefix_samples: prefix,
            roll_off_samples: rolloff,
            ..Default::default()
        };
        assert!(settings.validate().is_err());
    }
    for bandwidth in [24_000_000, 192_000_000] {
        TransmissionSettings {
            occupied_bandwidth_hz: bandwidth,
            ..Default::default()
        }
        .validate()
        .unwrap();
    }
}
