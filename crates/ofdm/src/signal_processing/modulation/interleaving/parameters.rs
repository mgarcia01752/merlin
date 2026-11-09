/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 Tables24–25 and ETSI EN302769 V1.2.1 Table10(a), short8/9.
use crate::types::ofdm_modulation::ConstellationOrder;

pub(super) struct Parameters {
    pub rows: usize,
    pub twists: &'static [usize],
    pub demultiplexer: &'static [usize],
    pub bits_per_cell: usize,
}
pub(super) const SHORT16: Parameters = Parameters {
    rows: 2025,
    twists: &[0, 0, 0, 1, 7, 20, 20, 21],
    demultiplexer: &[7, 1, 4, 2, 5, 3, 6, 0],
    bits_per_cell: 4,
};
pub(super) const SHORT256: Parameters = Parameters {
    demultiplexer: &[7, 3, 1, 5, 2, 6, 4, 0],
    bits_per_cell: 8,
    ..SHORT16
};
/// Returns only verified non-shortened uniform-modulation configurations.
/// # Arguments
/// * `order` - Physical data constellation size.
/// # Returns
/// Pinned short8/9 parameters, or None rather than a substitute configuration.
pub(super) fn for_order(order: ConstellationOrder) -> Option<&'static Parameters> {
    match order {
        16 => Some(&SHORT16),
        256 => Some(&SHORT256),
        _ => None,
    }
}

/// Requires a verified uniform short8/9 bit-processing configuration.
/// # Arguments
/// * `order` - Physical constellation size.
/// # Returns
/// Shared static parameters for independently callable stages.
/// # Errors
/// Rejects all currently unsupported coding orders.
pub(super) fn require(order: ConstellationOrder) -> Result<&'static Parameters, crate::LabError> {
    for_order(order).ok_or_else(|| {
        crate::LabError::new(
            "UnsupportedCodingMode",
            "Bit processing supports short8/9 uniform16or256-QAM only.",
        )
    })
}
