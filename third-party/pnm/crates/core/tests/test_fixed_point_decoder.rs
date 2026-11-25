/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent numeric expectations, not values computed with production scales.
use pnm::pnm::lib::fixed_point_decoder::{decode_fixed_point, decode_iq, QFormat};

#[test]
fn signed_formats_decode_known_values_exactly() {
    // Binary fractions are exactly representable in f64, so no tolerance is needed.
    for (encoded, s2_13, s1_14) in [
        (i16::MIN, -4.0, -2.0),
        (-16384, -2.0, -1.0),
        (-8192, -1.0, -0.5),
        (-2048, -0.25, -0.125),
        (-1, -0.0001220703125, -0.00006103515625),
        (0, 0.0, 0.0),
        (1, 0.0001220703125, 0.00006103515625),
        (2048, 0.25, 0.125),
        (8192, 1.0, 0.5),
        (9216, 1.125, 0.5625),
        (16384, 2.0, 1.0),
        (i16::MAX, 3.9998779296875, 1.99993896484375),
    ] {
        assert_eq!(
            decode_fixed_point(encoded, QFormat::S2_13),
            s2_13,
            "s2.13 raw {encoded}"
        );
        assert_eq!(
            decode_fixed_point(encoded, QFormat::S1_14),
            s1_14,
            "s1.14 raw {encoded}"
        );
    }
}

#[test]
fn iq_preserves_component_order_signs_and_numeric_minimum() {
    assert_eq!(decode_iq(9216, -2048, QFormat::S2_13), [1.125, -0.25]);
    assert_eq!(decode_iq(-2048, 9216, QFormat::S1_14), [-0.125, 0.5625]);
    assert_eq!(decode_iq(i16::MIN, i16::MIN, QFormat::S2_13), [-4.0, -4.0]);
    assert_eq!(
        decode_iq(i16::MAX, i16::MIN, QFormat::S1_14),
        [1.99993896484375, -2.0]
    );
    assert_eq!(decode_iq(0, 0, QFormat::S1_14), [0.0, 0.0]);
}

#[test]
fn every_signed_word_has_exact_resolution_and_round_trip() {
    // Check the complete input domain against explicit bit resolutions.
    for (format, fractional_bits, step, minimum) in [
        (QFormat::S2_13, 13, 0.0001220703125, -4.0),
        (QFormat::S1_14, 14, 0.00006103515625, -2.0),
    ] {
        let mut previous = minimum - step;
        for encoded in i16::MIN..=i16::MAX {
            let decoded = decode_fixed_point(encoded, format);
            assert!(decoded.is_finite());
            assert_eq!(decoded - previous, step);
            assert_eq!(
                decoded * f64::from(1_u32 << fractional_bits),
                f64::from(encoded)
            );
            previous = decoded;
        }
    }
}
