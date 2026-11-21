/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Literal CM-OSSI I27 Tables77/78 and signed fixed-point acceptance.
use pnm_input::upstream::{PreEqCapture, PreEqCaptureKind};

fn fixture(name: &str) -> Vec<u8> {
    let text = match name {
        "applied" => include_str!("../../../examples/measurements/upstream/pre-eq/applied.hex"),
        _ => include_str!("../../../examples/measurements/upstream/pre-eq/last-update.hex"),
    };
    let hex: String = text.split_whitespace().collect();
    hex.as_bytes()
        .chunks_exact(2)
        .map(|p| u8::from_str_radix(std::str::from_utf8(p).unwrap(), 16).unwrap())
        .collect()
}
#[test]
fn literal_scales_headers_and_immutable_originals() {
    for (name, kind, expected) in [
        (
            "applied",
            PreEqCaptureKind::Applied,
            [
                (1.0, 0.0),
                (1.125, -0.25),
                (-4.0, 3.9998779296875),
                (0.0001220703125, -0.0001220703125),
            ],
        ),
        (
            "last-update",
            PreEqCaptureKind::LastUpdate,
            [
                (1.0, 0.0),
                (1.125, -0.25),
                (-2.0, 1.99993896484375),
                (0.00006103515625, -0.00006103515625),
            ],
        ),
    ] {
        let original = fixture(name);
        let capture = PreEqCapture::from_pnm(&original).unwrap();
        assert_eq!(capture.kind(), kind);
        assert_eq!(capture.original_bytes(), original);
        assert_eq!(capture.header().capture_time_epoch_seconds, 0x01234567);
        assert_eq!(capture.header().subcarrier_zero_hz, 12_800_000);
        assert_eq!(capture.header().first_active_subcarrier, 128);
        assert_eq!(capture.header().cm_mac, [2, 0, 0, 0, 0, 1]);
        assert_eq!(capture.header().cmts_mac, [2, 0, 0, 0, 0, 2]);
        for (actual, (re, im)) in capture.coefficients().iter().zip(expected) {
            assert_eq!((actual.re, actual.im), (re, im));
        }
        let exported = capture.to_json().unwrap();
        drop(original);
        assert_eq!(capture.clone().to_json().unwrap(), exported);
    }
}
#[test]
fn malformed_extents_versions_and_every_truncated_prefix_reject() {
    let original = fixture("applied");
    for end in 0..original.len() {
        assert!(PreEqCapture::from_pnm(&original[..end]).is_err(), "{end}");
    }
    for (offset, value) in [(0, 0), (4, 2), (29, 100), (33, 15), (27, 255)] {
        let mut damaged = original.clone();
        damaged[offset] = value;
        assert!(PreEqCapture::from_pnm(&damaged).is_err());
    }
    assert!(PreEqCapture::from_pnm(&vec![0; 16419]).is_err());
    let mut empty = original[..34].to_vec();
    empty[30..34].fill(0);
    assert!(PreEqCapture::from_pnm(&empty)
        .unwrap()
        .coefficients()
        .is_empty());
    for minor in 0..=255 {
        let mut candidate = original.clone();
        candidate[5] = minor;
        assert_eq!(
            PreEqCapture::from_pnm(&candidate)
                .unwrap()
                .header()
                .minor_version,
            minor
        );
    }
}

#[test]
fn pinned_decoder_and_opaque_minor_profiles_keep_literal_coefficients() {
    for (name, expected) in [
        (
            "applied",
            [
                (1.0, 0.0),
                (1.125, -0.25),
                (-4.0, 3.9998779296875),
                (0.0001220703125, -0.0001220703125),
            ],
        ),
        (
            "last-update",
            [
                (1.0, 0.0),
                (1.125, -0.25),
                (-2.0, 1.99993896484375),
                (0.00006103515625, -0.00006103515625),
            ],
        ),
    ] {
        for minor in 0..=255 {
            let mut bytes = fixture(name);
            bytes[5] = minor;
            let capture = PreEqCapture::from_pnm(&bytes).unwrap();
            assert_eq!(capture.original_bytes(), bytes);
            assert_eq!(capture.header().minor_version, minor);
            for (value, (re, im)) in capture.coefficients().iter().zip(expected) {
                assert_eq!((value.re, value.im), (re, im));
            }
        }
    }
}
