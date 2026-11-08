/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! CM-SP-PHYv3.1-I20-230419 Annex A Figure102, PDF198.
use crate::types::IqPair;
/// Normalized reference indexed by a four-bit MSB-first symbol label.
pub struct QamReference {
    pub iq: [IqPair; 16],
}
const SCALE: f64 = 0.316_227_766_016_837_94;
static QAM16: QamReference = QamReference {
    iq: [
        [3. * SCALE, 3. * SCALE],
        [3. * SCALE, SCALE],
        [SCALE, 3. * SCALE],
        [SCALE, SCALE],
        [3. * SCALE, -3. * SCALE],
        [3. * SCALE, -SCALE],
        [SCALE, -3. * SCALE],
        [SCALE, -SCALE],
        [-3. * SCALE, 3. * SCALE],
        [-3. * SCALE, SCALE],
        [-SCALE, 3. * SCALE],
        [-SCALE, SCALE],
        [-3. * SCALE, -3. * SCALE],
        [-3. * SCALE, -SCALE],
        [-SCALE, -3. * SCALE],
        [-SCALE, -SCALE],
    ],
};
/// Returns the verified first-mode reference without runtime file access.
/// # Arguments
/// * `order` - Physical constellation size; only16 is currently verified.
/// # Returns
/// Embedded unit-average-energy coordinates, or None for unsupported orders.
pub fn reference_for_order(order: usize) -> Option<&'static QamReference> {
    match order {
        16 => Some(&QAM16),
        _ => None,
    }
}

/// Maps a four-bit MSB-first label using the embedded normalized reference table.
/// # Arguments
/// * `label` - Integer label in0..16; table lineage is recorded in the resources.
/// # Returns
/// Unit-average-power I/Q, or None for an invalid label.
pub fn map_qam16(label: usize) -> Option<[f64; 2]> {
    reference_for_order(16)?.iq.get(label).copied()
}
