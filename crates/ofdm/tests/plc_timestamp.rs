/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Wire fields and modulo20 carry per MULPI I25 §7.1.5.
use docsis_ofdm_fec::transmission::plc::ExtendedTimestamp;
#[test]
fn timestamp_fields_have_exact_network_order_and_sample_units() {
    let timestamp = ExtendedTimestamp::new(0x0000_0102_0304_0506, 19, 10).unwrap();
    assert_eq!(timestamp.to_bytes(), [0, 2, 4, 6, 8, 10, 13, 0x3a]);
    assert_eq!(timestamp.sample_ticks(), 0x0000_0102_0304_0506 * 20 + 19);
    assert_eq!(
        ExtendedTimestamp::from_bytes(&timestamp.to_bytes()).unwrap(),
        timestamp
    );
    let first = ExtendedTimestamp::from_sample_ticks(19, 15).unwrap();
    let second = first.advance_samples(1).unwrap();
    assert_eq!(first.to_bytes(), [0, 0, 0, 0, 0, 0, 1, 0x3f]);
    assert_eq!(second.to_bytes(), [0, 0, 0, 0, 0, 0, 2, 15]);
    assert_eq!(
        second.advance_samples(204_800_000).unwrap().sample_ticks(),
        204_800_020
    );
}
#[test]
fn invalid_phase_and_overflow_fail_without_wrapping_or_mutation() {
    for phase in 20..32 {
        assert!(ExtendedTimestamp::new(0, phase, 0).is_err());
        let mut bytes = [0; 8];
        bytes[6] = phase >> 4;
        bytes[7] = phase << 4;
        assert!(ExtendedTimestamp::from_bytes(&bytes).is_err());
    }
    assert!(ExtendedTimestamp::new(1 << 55, 0, 0).is_err());
    assert!(ExtendedTimestamp::new(0, 0, 16).is_err());
    assert!(ExtendedTimestamp::from_sample_ticks(u64::MAX, 0).is_err());
    let maximum = ExtendedTimestamp::new((1 << 55) - 1, 19, 15).unwrap();
    assert_eq!(maximum.to_bytes(), [255, 255, 255, 255, 255, 255, 255, 63]);
    assert!(maximum.advance_samples(1).is_err());
    assert!(maximum.advance_samples(u64::MAX).is_err());
    assert_eq!(maximum.advance_samples(0).unwrap(), maximum);
}
