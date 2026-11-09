/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! PHY I20 section7.5.14.4: NCP selection before time/frequency interleaving.
use crate::{error::invalid, LabError};

const MAX_INTERLEAVER_CELLS: usize = 8192;
/// Cell eligibility in increasing pre-interleaving order, never physical FFT bins.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreInterleavingRole {
    Data,
    ScatteredPilotPlaceholder,
    ZeroBitLoaded,
}

/// Selects NCP cells from the highest eligible index downward without mutation.
/// # Arguments
/// * `roles` - Data/scattered-pilot interleaver domain, with zero-loaded cells
///   explicitly marked. Caller must inverse-map physical pilots first.
/// * `count` - Required NCP cell count supplied by NCP encoding/packing.
/// # Returns
/// Indices in NCP assignment order. These are not transmitted FFT-bin positions.
/// # Errors
/// Rejects oversized domains or insufficient eligible cells, without partial output.
pub fn select_ncp_cells(
    roles: &[PreInterleavingRole],
    count: usize,
) -> Result<Vec<usize>, LabError> {
    if roles.len() > MAX_INTERLEAVER_CELLS || count > roles.len() {
        return Err(invalid(
            "NCP request exceeds the supported interleaver domain.",
        ));
    }
    let selected: Vec<_> = roles
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, role)| **role == PreInterleavingRole::Data)
        .take(count)
        .map(|(index, _)| index)
        .collect();
    if selected.len() != count {
        return Err(invalid("Insufficient data cells for NCP placement."));
    }
    Ok(selected)
}
