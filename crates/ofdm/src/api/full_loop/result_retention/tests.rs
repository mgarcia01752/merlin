/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Private final-layout fixtures; no fabricated scientific execution claim.
use super::*;

#[test]
fn exact_dimensions_and_one_extra_unit_admit_but_each_missing_unit_rejects() {
    // Independent fixture dimensions exercise admission, not scientific limits.
    let required = RetainedFootprint {
        payload_bytes: 7,
        ordered_entries: 3,
        shared_roots: 2,
    };
    assert!(fits(required, required));
    assert!(fits(
        required,
        RetainedFootprint {
            payload_bytes: required.payload_bytes + 1,
            ordered_entries: required.ordered_entries + 1,
            shared_roots: required.shared_roots + 1
        }
    ));
    for insufficient in [
        RetainedFootprint {
            payload_bytes: required.payload_bytes - 1,
            ..required
        },
        RetainedFootprint {
            ordered_entries: required.ordered_entries - 1,
            ..required
        },
        RetainedFootprint {
            shared_roots: required.shared_roots - 1,
            ..required
        },
        RetainedFootprint {
            shared_roots: 0,
            ..required
        },
    ] {
        assert!(!fits(required, insufficient));
    }
    assert!(fits(
        RetainedFootprint::default(),
        RetainedFootprint::default()
    ));
}

#[test]
fn exact_vector_and_text_storage_preserves_initialized_values() {
    // Spare capacity is intentionally unrelated to the initialized payload.
    const VECTOR_SLOTS: usize = 17;
    const TEXT_BYTES: usize = 31;
    let mut values = Vec::with_capacity(VECTOR_SLOTS);
    values.extend([Some([1.0, -1.0]), None, Some([0.0, 0.0])]);
    let original = values.clone();
    let mut text = String::with_capacity(TEXT_BYTES);
    text.push_str("original UTF-8: µ");
    let original_text = text.clone();
    normalize::vector(&mut values);
    normalize::text(&mut text);
    assert_eq!(values, original);
    assert_eq!(values.capacity(), values.len());
    assert_eq!(text, original_text);
    assert_eq!(text.capacity(), text.len());
}
