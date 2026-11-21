/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Figure 8 balanced consecutive blocks, never a 24-plus-small-tail split.
use crate::{error::limit, PhyResult};

/// Partitions `minislots` into ordered blocks of at most 24.
///
/// Returns no blocks for zero; rejects more than the channel's 256 positions.
/// This pure operation preserves physical association through caller indices.
pub fn block_sizes(minislots: usize) -> PhyResult<Vec<usize>> {
    if minislots > 256 {
        return Err(limit("interleaving.minislots"));
    }
    let mut blocks = minislots.div_ceil(24);
    let mut remaining = minislots;
    let mut result = Vec::with_capacity(blocks);
    while blocks > 0 {
        let count = remaining / blocks;
        result.push(count);
        remaining -= count;
        blocks -= 1;
    }
    Ok(result)
}
