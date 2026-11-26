/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact final prefix storage fixture; the recovery algorithm is unchanged.
use super::RecoveredFile;

#[test]
fn exact_recovered_prefix_storage_preserves_complete_and_partial_bytes() {
    // Unused capacity is distinct from the meaningful decoded prefix.
    const SPARE_PREFIX_BYTES: usize = 23;
    const ORIGINAL: &[u8] = b"original\0prefix";
    for complete in [false, true] {
        let mut bytes = Vec::with_capacity(ORIGINAL.len() + SPARE_PREFIX_BYTES);
        bytes.extend_from_slice(ORIGINAL);
        let mut report = RecoveredFile {
            bytes,
            original_bytes: ORIGINAL.len() + usize::from(!complete), // One unrecovered byte in the partial fixture.
            recovered_bits: (ORIGINAL.len() * u8::BITS as usize) as u64,
            complete,
        };
        let before = serde_json::to_value(&report).unwrap();
        report.normalize_result_storage();
        assert_eq!(report.bytes(), ORIGINAL);
        assert_eq!(report.bytes.capacity(), ORIGINAL.len());
        assert_eq!(serde_json::to_value(&report).unwrap(), before);
    }
}
