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
pub(super) const SHORT64: Parameters = Parameters {
    rows: 1350,
    twists: &[0, 0, 0, 2, 2, 2, 3, 3, 3, 6, 7, 7],
    demultiplexer: &[11, 7, 3, 10, 6, 2, 9, 5, 1, 8, 4, 0],
    bits_per_cell: 6,
};
pub(super) const SHORT128: Parameters = Parameters {
    rows: 2315,
    twists: &[0, 1, 2, 2, 2, 3, 3],
    demultiplexer: &[6, 5, 4, 1, 2, 3, 0],
    bits_per_cell: 7,
};
pub(super) const SHORT512: Parameters = Parameters {
    rows: 1800,
    twists: &[0, 1, 2, 3, 5, 6, 7, 9, 11],
    demultiplexer: &[8, 7, 6, 1, 2, 3, 4, 5, 0],
    bits_per_cell: 9,
};
pub(super) const SHORT1024: Parameters = Parameters {
    rows: 810,
    twists: &[0, 0, 0, 2, 2, 2, 2, 2, 5, 5, 5, 5, 5, 7, 7, 7, 7, 8, 8, 10],
    demultiplexer: &[
        8, 3, 7, 10, 19, 4, 9, 5, 17, 6, 14, 11, 2, 18, 16, 15, 0, 1, 13, 12,
    ],
    bits_per_cell: 10,
};
pub(super) const SHORT2048: Parameters = Parameters {
    rows: 1473,
    twists: &[0, 0, 0, 0, 0, 3, 3, 4, 4, 4, 4],
    demultiplexer: &[10, 9, 8, 7, 2, 3, 4, 5, 6, 1, 0],
    bits_per_cell: 11,
};
pub(super) const SHORT4096: Parameters = Parameters {
    rows: 675,
    twists: &[
        0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 7, 9, 9, 9, 10, 10, 10, 10, 10, 11,
    ],
    demultiplexer: &[
        10, 15, 4, 19, 21, 16, 23, 18, 11, 14, 22, 5, 6, 17, 13, 20, 1, 3, 9, 2, 7, 8, 12, 0,
    ],
    bits_per_cell: 12,
};
pub(super) const SHORT8192: Parameters = Parameters {
    rows: 1247,
    twists: &[0, 0, 0, 0, 0, 0, 2, 3, 5, 8, 8, 9, 9],
    demultiplexer: &[12, 11, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 0],
    bits_per_cell: 13,
};
pub(super) const SHORT16384: Parameters = Parameters {
    rows: 1158,
    twists: &[0, 0, 2, 2, 2, 2, 2, 4, 4, 6, 6, 8, 9, 9],
    demultiplexer: &[13, 12, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 1, 0],
    bits_per_cell: 14,
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
        64 => Some(&SHORT64),
        128 => Some(&SHORT128),
        512 => Some(&SHORT512),
        1024 => Some(&SHORT1024),
        2048 => Some(&SHORT2048),
        4096 => Some(&SHORT4096),
        8192 => Some(&SHORT8192),
        16384 => Some(&SHORT16384),
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
pub(super) fn require(
    order: ConstellationOrder,
) -> Result<&'static Parameters, crate::SimulationError> {
    for_order(order).ok_or_else(|| {
        crate::SimulationError::new(
            "UnsupportedCodingMode",
            "Bit processing requires a supported downstream data constellation.",
        )
    })
}
