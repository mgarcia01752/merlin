/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Label-ordered embedded constellations; PHY I20 AnnexA and PyPNM provenance.
use crate::{
    error::invalid,
    types::{ofdm_modulation::ConstellationOrder, BitsPerSymbol, IqPair},
    SimulationError,
};

/// Integer value of an MSB-first binary cell word;14bits covers16384-QAM.
pub type SymbolLabel = u16;
/// Multiplier applied to integer reference coordinates for unit mean energy.
pub type QamScaleFactor = f64;

/// Borrowed unit-average-energy coordinates indexed by cell-word label.
///
/// Static tables are generated at build time; no runtime filesystem or cache.
#[derive(Debug)]
pub struct QamReference {
    /// Physical constellation size, distinct from a PNM modulation code.
    pub order: ConstellationOrder,
    /// Number of MSB-first bits represented by a label.
    pub bit_loading: BitsPerSymbol,
    /// Reference-coordinate multiplier, never applied to received samples.
    pub scale_factor: QamScaleFactor,
    /// Normalized coordinates; row index equals the binary cell-word label.
    pub iq: &'static [IqPair],
}
include!(concat!(env!("OUT_DIR"), "/qam_references.rs"));

/// Looks up a label-ordered reference, preserving the existing usize interface.
/// # Arguments
/// * `order` - Constellation size:2,4,16,64,128,256,512,1024,2048,4096,8192,16384.
/// # Returns
/// Embedded normalized coordinates, or None for unsupported orders.
/// BPSK/QPSK are control references, not additional downstream data modes.
pub fn reference_for_order(order: usize) -> Option<&'static QamReference> {
    REFERENCES
        .iter()
        .find(|reference| usize::from(reference.order) == order)
}

/// Maps a binary cell-word label using the exact stored constellation ordering.
/// # Arguments
/// * `order` - Physical constellation size, not an OSSI modulation code.
/// * `label` - MSB-first integer value of the cell word, strictly below order.
/// # Returns
/// Unit-average-energy [I,Q] without changing inputs or accessing resources.
/// # Errors
/// Reports unsupported orders and labels outside the selected constellation.
pub fn map_symbol(
    order: ConstellationOrder,
    label: SymbolLabel,
) -> Result<IqPair, SimulationError> {
    let reference = reference_for_order(usize::from(order)).ok_or_else(|| {
        SimulationError::new("UnsupportedModulation", "Unsupported QAM constellation.")
    })?;
    reference
        .iq
        .get(usize::from(label))
        .copied()
        .ok_or_else(|| invalid("Cell-word label exceeds the selected constellation."))
}

/// Maps a four-bit MSB-first label through the shared16-QAM resource.
/// # Arguments
/// * `label` - Integer label in0..16.
/// # Returns
/// Unit-average-energy I/Q, or None for an invalid label.
pub fn map_qam16(label: usize) -> Option<IqPair> {
    reference_for_order(16)?.iq.get(label).copied()
}
