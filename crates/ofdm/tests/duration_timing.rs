/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent integer sample-clock schedules; available without simulation.
use docsis_ofdm_fec::transmission::{SubcarrierSpacing, TransmissionSettings};
use std::time::Duration;

#[test]
fn duration_plan_matches_independent_sample_arithmetic_and_legacy_schedule() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        for prefix in [192, 256, 512, 768, 1024] {
            let settings = TransmissionSettings {
                subcarrier_spacing: spacing,
                cyclic_prefix_samples: prefix,
                roll_off_samples: 64,
                ..Default::default()
            };
            let fft = if spacing == SubcarrierSpacing::Khz25 {
                8192u64
            } else {
                4096
            };
            let stride = fft + u64::from(prefix);
            for seconds in [1, 2, 60, 3600, 100_000] {
                let plan = settings
                    .duration_timing(Duration::from_secs(seconds))
                    .unwrap();
                assert_eq!(plan.observation_samples, seconds * 204_800_000);
                assert_eq!(plan.complete_symbols, seconds * 204_800_000 / stride);
                assert_eq!(plan.unused_samples, seconds * 204_800_000 % stride);
                assert_eq!(
                    plan.complete_symbols * stride + plan.unused_samples,
                    plan.observation_samples
                );
                assert_eq!(u64::from(plan.windowed_symbol_samples), stride + 64);
            }
            let old = settings.one_second_timing().unwrap();
            let new = settings.duration_timing(Duration::from_secs(1)).unwrap();
            assert_eq!(u64::from(old.complete_symbols), new.complete_symbols);
            assert_eq!(u64::from(old.unused_samples), new.unused_samples);
        }
    }
}

#[test]
fn fractional_ticks_symbol_edges_and_invalid_overflow_are_explicit() {
    let settings = TransmissionSettings::default();
    let stride = 8192 + 512;
    assert_eq!(settings.sample_timing(stride).unwrap().complete_symbols, 1);
    assert!(settings.sample_timing(stride - 1).is_err());
    assert_eq!(
        settings
            .sample_timing(2 * stride - 1)
            .unwrap()
            .unused_samples,
        stride - 1
    );
    let plan = settings
        .duration_timing(Duration::from_nanos(1_000_000_001))
        .unwrap();
    assert_eq!(plan.observation_samples, 204_800_000);
    assert!(settings.duration_timing(Duration::ZERO).is_err());
    assert!(settings.duration_timing(Duration::from_nanos(1)).is_err());
    assert!(settings.duration_timing(Duration::MAX).is_err());
    let invalid = TransmissionSettings {
        cyclic_prefix_samples: 1,
        ..settings
    };
    assert!(invalid.duration_timing(Duration::from_secs(1)).is_err());
}
