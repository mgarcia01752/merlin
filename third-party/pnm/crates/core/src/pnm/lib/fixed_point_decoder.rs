/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Signed fixed-point conversion, corresponding to PyPNM pnm/lib/fixed_point_decoder.py.
//! Byte order, payload length and unavailable-value rules belong to the parser.
use crate::pnm::data_type::{units, IqPair};

/// Supported signed 16-bit layouts, with the sign bit excluded from integer bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QFormat {
    /// One sign bit, two integer bits and thirteen fractional bits.
    S2_13,
    /// One sign bit, one integer bit and fourteen fractional bits.
    S1_14,
}

/// Converts a signed fixed-point component without rounding or allocation.
///
/// # Arguments
/// * `value` - Already decoded signed 16-bit integer; no byte-order assumption.
/// * `format` - Fractional-bit layout of the encoded value.
/// # Returns
/// The exact binary fraction as f64. All i16 values are accepted; format-specific
/// exclusion markers must be handled by the measurement parser.
pub fn decode_fixed_point(value: i16, format: QFormat) -> f64 {
    let units_per_unit = match format {
        QFormat::S2_13 => units::S2_13_UNITS_PER_UNIT,
        QFormat::S1_14 => units::S1_14_UNITS_PER_UNIT,
    };
    f64::from(value) / units_per_unit
}

/// Converts the real and imaginary components using one fixed-point format.
///
/// # Arguments
/// * `in_phase` - Signed encoded I (real) component.
/// * `quadrature` - Signed encoded Q (imaginary) component.
/// * `format` - Shared component format.
/// # Returns
/// An [I, Q] pair without rounding or allocation.
pub fn decode_iq(in_phase: i16, quadrature: i16, format: QFormat) -> IqPair {
    [
        decode_fixed_point(in_phase, format),
        decode_fixed_point(quadrature, format),
    ]
}
