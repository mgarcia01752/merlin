/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Physical masks transcribed from PHY I20 Figures 39–42 pp103–105.
use super::CellRole;
use crate::{error::invalid, PhyResult};

/// Returns the scheduled resource role at a local minislot coordinate.
///
/// `pattern` selects 1..14, `symbols` is K, `symbol`/`carrier` are zero-based,
/// `edge` retains physical gaps. Returns role plus relative amplitude gain.
/// Rejects mismatched width/index/frame bounds; no payload state is consumed.
pub fn role(
    pattern: u8,
    symbols: usize,
    symbol: usize,
    carrier: usize,
    edge: bool,
) -> PhyResult<(CellRole, f64)> {
    let q = if (1..=7).contains(&pattern) {
        8
    } else if (8..=14).contains(&pattern) {
        16
    } else {
        return Err(invalid("pilot.pattern"));
    };
    if !(6..=36).contains(&symbols) || symbol >= symbols || carrier >= q {
        return Err(invalid("pilot.coordinate"));
    }
    let local = if q == 8 { pattern - 1 } else { pattern - 8 };
    let boosted = local >= 4;
    let end = if q == 8 {
        symbols.min(16)
    } else {
        symbols.min(9)
    };
    let known = |row: usize, t: usize| -> bool {
        if !boosted {
            let step = match local {
                0 => q,
                1 => q / 2,
                2 => q / 4,
                _ => {
                    if q == 8 {
                        1
                    } else {
                        2
                    }
                }
            };
            (t == 0 || t == 2) && (row % step == 0 || edge && row == q - 1)
        } else {
            (row == 0 && t == 0)
                || (local == 5 && row == q / 2 && t == 3)
                || (local == 6 && [(q / 4, 1), (q / 2, 3), (3 * q / 4, 4)].contains(&(row, t)))
                || (edge && row == q - 1 && t == 2)
        }
    };
    let resource = if known(carrier, symbol) {
        CellRole::KnownPilot
    } else if carrier == 0 && (symbol == end - 3 || !boosted && symbol == end - 1)
        || edge && carrier == q - 1 && (symbol == end - 1 || !boosted && symbol == end - 3)
    {
        CellRole::Complementary
    } else {
        CellRole::Data
    };
    let gain = if boosted
        && (resource == CellRole::KnownPilot
            || resource == CellRole::Complementary && !(0..q).any(|row| known(row, symbol)))
    {
        3.0_f64.sqrt()
    } else {
        1.0
    };
    Ok((resource, gain))
}
