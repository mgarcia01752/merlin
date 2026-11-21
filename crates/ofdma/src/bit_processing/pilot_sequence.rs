/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! PHY I20 Figure 35, referenced by §7.4.16.6: reset at physical k=0 per symbol.
use crate::{configuration::Numerology, types::Iq};

/// Generates known BPSK signs for every physical bin of `mode`.
///
/// Returns N owned references from seed 3071, clocking data/excluded bins too.
/// Mapping is bit0→+1 and bit1→-1; no payload/source state is used.
pub fn pilot_reference(mode: Numerology) -> Vec<Iq> {
    let mut state = 3071_u16;
    (0..mode.fft_len())
        .map(|_| {
            let bit = state & 1;
            let feedback = (state ^ (state >> 5) ^ (state >> 8) ^ (state >> 9)) & 1;
            state = (state >> 1) | (feedback << 11);
            Iq {
                re: if bit == 0 { 1.0 } else { -1.0 },
                im: 0.0,
            }
        })
        .collect()
}
