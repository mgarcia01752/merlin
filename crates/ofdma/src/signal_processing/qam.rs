/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Exact upstream labels with Table 54 reference normalization once per axis.
use super::qam_tables::coordinates;
use crate::{error::invalid, types::Iq, PhyError, PhyResult};

/// Returns the integer-coordinate mean energy for `bits` from Table 54.
///
/// Supports 1..12 (BPSK is complementary/known-pilot only). Rejects other orders
/// explicitly, including downstream 8192/16384. No signal-dependent scaling.
pub fn reference_energy(bits: u8) -> PhyResult<f64> {
    match bits {
        1 => Ok(1.0),
        2 => Ok(2.0),
        3 | 4 => Ok(10.0),
        5 => Ok(20.0),
        6 => Ok(42.0),
        7 => Ok(82.0),
        8 => Ok(170.0),
        9 => Ok(330.0),
        10 => Ok(682.0),
        11 => Ok(1322.0),
        12 => Ok(2730.0),
        _ => Err(PhyError::Unsupported {
            field: "qam.loading",
        }),
    }
}

/// Maps a checked MSB-first `label` at `bits` loading into complex unit reference.
///
/// Returns exact source coordinates divided by sqrt(E_ref), without mutation.
/// Rejects unsupported loading and labels at/above constellation size.
pub fn map_symbol(bits: u8, label: u16) -> PhyResult<Iq> {
    let energy = reference_energy(bits)?;
    let table = coordinates(bits).ok_or(PhyError::Unsupported { field: "qam.table" })?;
    let [i, q] = *table
        .get(label as usize)
        .ok_or_else(|| invalid("qam.label"))?;
    let scale = energy.sqrt().recip();
    Ok(Iq {
        re: f64::from(i) * scale,
        im: f64::from(q) * scale,
    })
}
