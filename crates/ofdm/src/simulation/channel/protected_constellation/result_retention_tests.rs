/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Capacity-only private cache fixture; no alternate scientific producer.
use super::*;

#[test]
fn exact_protected_storage_keeps_original_private_pilot_bits() {
    // Independent spare buffers exercise the skipped cache and a protected stream.
    const PILOT_SPARE_BYTES: usize = 19;
    const STREAM_SPARE_PAIRS: usize = 7;
    let bits = crate::signal_processing::pilots::PilotSequence::new(
        crate::transmission::SubcarrierSpacing::Khz50,
    )
    .bits()
    .to_vec();
    let mut cache = Vec::with_capacity(bits.len() + PILOT_SPARE_BYTES);
    cache.extend_from_slice(&bits);
    let mut report = ProtectedConstellation::new(Default::default(), Default::default()).unwrap();
    report.pilot_sequence = Some(cache);
    report.continuous_pilots.pairs = Vec::with_capacity(STREAM_SPARE_PAIRS);
    let before = serde_json::to_value(&report).unwrap();
    report.normalize_result_storage();
    let cache = report.pilot_sequence.as_ref().unwrap();
    assert_eq!(cache, &bits);
    assert_eq!(cache.capacity(), cache.len());
    assert_eq!(report.continuous_pilots.pairs.capacity(), 0);
    assert_eq!(serde_json::to_value(&report).unwrap(), before);
    assert_eq!(
        merlin_api::retention_support::retained_footprint(&report)
            .unwrap()
            .payload_bytes,
        std::mem::size_of::<ProtectedConstellation>() + bits.len()
    );
}
