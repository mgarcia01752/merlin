/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! PHY I20 §7.4.5.1 Figures 11/12 joint address state machine.
use crate::{error::invalid, PhyResult};

fn reversed(counter: &mut usize, size: usize, modulus: usize) -> usize {
    let width = modulus.trailing_zeros();
    loop {
        let coordinate = counter.reverse_bits() >> (usize::BITS - width);
        if coordinate < size {
            return coordinate;
        }
        *counter = (*counter + 1) % modulus;
    }
}

/// Enumerates every `(symbol, compact_carrier)` once for an accepted block.
///
/// `symbols` is K in 6..36; `carriers` is L in 1..384. Returns K*L joint
/// visits, including pilot coordinates. Rejects bounds before allocation.
/// K1 is >=K; L1 is strictly >L, including at power-of-two boundaries.
pub fn addresses(symbols: usize, carriers: usize) -> PhyResult<Vec<(usize, usize)>> {
    if !(6..=36).contains(&symbols) || !(1..=384).contains(&carriers) {
        return Err(invalid("interleaving.geometry"));
    }
    let kt = symbols.next_power_of_two();
    let lf = (carriers + 1).next_power_of_two();
    let (mut count_t, mut count_f, mut diagonal) = (0, 0, 0);
    let mut visits = Vec::with_capacity(symbols * carriers);
    for _ in 0..symbols {
        for _ in 0..carriers {
            let t = reversed(&mut count_t, symbols, kt);
            let f = reversed(&mut count_f, carriers, lf);
            visits.push((t, f));
            count_t = (count_t + 1) % kt;
            count_f = (count_f + 1) % lf;
        }
        diagonal = (diagonal + 1) % kt;
        reversed(&mut diagonal, symbols, kt);
        count_t = diagonal;
        count_f = 0;
    }
    Ok(visits)
}
