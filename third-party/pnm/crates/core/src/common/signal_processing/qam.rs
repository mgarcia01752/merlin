/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Build-time-normalized reference constellations from preserved PyPNM tables.
use super::decision_boundary::DecisionBoundary;
use crate::pnm::data_type::{BitsPerSymbol, ConstellationModulationOrder, IqPair};
use serde::Serialize;

/// Number of distinct ideal constellation symbols; not an encoded OSSI enum.
pub type QamOrder = u16;
/// Multiplier applied only to integer reference coordinates to normalize energy.
pub type QamScaleFactor = f64;

/// Borrowed unit-average-power reference points; no file reads or runtime cache.
#[derive(Debug, PartialEq, Serialize)]
pub struct QamReference {
    pub order: QamOrder,
    pub bit_loading: BitsPerSymbol,
    pub scale_factor: QamScaleFactor,
    pub iq: &'static [IqPair],
    pub decision_boundaries: &'static [DecisionBoundary],
}
include!(concat!(env!("OUT_DIR"), "/qam_references.rs"));

/// Finds an embedded ideal constellation by symbol count.
/// # Arguments
/// * `order` - QAM size (for example256); BPSK uses2. Unsupported sizes return None.
/// # Returns
/// Static normalized reference points with their source scaling multiplier.
pub fn reference_for_order(order: QamOrder) -> Option<&'static QamReference> {
    REFERENCES.iter().find(|reference| reference.order == order)
}

/// Interprets the constellation header's DsOfdmModulationType enumeration.
/// # Arguments
/// * `code` - OSSI I27/I13 F.3.2 enum, not a modulation-profile assignment code.
/// # Returns
/// The known data constellation, or None for other/zero/unknown values.
pub fn reference_for_modulation_type(
    code: ConstellationModulationOrder,
) -> Option<&'static QamReference> {
    let order = match code {
        3 => 4,
        4 => 16,
        5 => 64,
        6 => 128,
        7 => 256,
        8 => 512,
        9 => 1024,
        10 => 2048,
        11 => 4096,
        12 => 8192,
        13 => 16384,
        _ => return None,
    };
    reference_for_order(order)
}
