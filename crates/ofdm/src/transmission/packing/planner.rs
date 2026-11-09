/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded reservation preview; no mutation of the packer's source queue.
use super::{
    model::{NcpPlanEntry, PackedCell, PackingSettings, MAX_NCP_ENTRIES},
    packer::QueuedWord,
};
use crate::{
    error::invalid,
    transmission::layout::ncp::{select_ncp_cells, PreInterleavingRole},
    LabError,
};
use std::collections::VecDeque;

pub(super) struct Preview {
    pub cells: Vec<PackedCell>,
    pub entries: Vec<NcpPlanEntry>,
    pub consumed: usize,
    pub starts: usize,
    pub reserved: usize,
}

/// Previews a complete symbol without changing queue offsets or identities.
/// # Arguments
/// * `settings` - Validated fixed geometry and uniform NCP profile.
/// * `roles` - Per-symbol reduced-domain roles, never physical FFT-bin roles.
/// * `queue` - Bounded complete/partial labels or coded-bit words owned by the packer.
/// * `bit_loading` - Optional NI source-rank bit counts for coded-bit grouping.
/// * `start_budget` - Remaining active NCP starts allowed for this symbol.
/// # Returns
/// Cells and NCP meanings with an exact reservation and a commit consumption count.
/// # Errors
/// Rejects invalid roles or insufficient signaling capacity without side effects.
pub(super) fn plan(
    settings: &PackingSettings,
    roles: &[PreInterleavingRole],
    queue: &VecDeque<QueuedWord>,
    start_budget: usize,
    bit_loading: Option<&[u8]>,
) -> Result<Preview, LabError> {
    if roles.len() != settings.interleaver_cells
        || roles.contains(&PreInterleavingRole::ZeroBitLoaded)
    {
        return Err(invalid(
            "Supply fixed-NI data/pilot roles; zero-loaded data profiles are not supported.",
        ));
    }
    let eligible: Vec<_> = roles
        .iter()
        .enumerate()
        .filter(|(_, role)| **role == PreInterleavingRole::Data)
        .map(|(index, _)| index)
        .collect();
    let block = settings.ncp_modulation.cells_per_block();
    for entry_count in 1..=MAX_NCP_ENTRIES {
        let reserved = (entry_count + 1) * block; // Include the encodedCRC block.
        if reserved > eligible.len() {
            break;
        }
        let capacity = eligible.len() - reserved;
        let mut cells = vec![PackedCell::ScatteredPilotPlaceholder; roles.len()];
        let mut entries = Vec::new();
        let mut word_index = 0;
        let mut offset = queue.front().map_or(0, |word| word.offset);
        let mut consumed = 0;
        let mut starts = 0;
        let mut filler = false;
        let mut started_word = None;
        for (rank, &slot) in eligible[..capacity].iter().enumerate() {
            let next = queue.get(word_index);
            if let Some(word) = next.filter(|word| {
                !filler && (offset != 0 || started_word == Some(word.id) || starts < start_budget)
            }) {
                if offset == 0 && started_word != Some(word.id) {
                    started_word = Some(word.id);
                    entries.push(NcpPlanEntry::CodewordStart {
                        codeword_id: word.id,
                        profile_id: settings.data_profile_id,
                        pointer: rank as u16,
                    });
                    starts += 1;
                }
                let taken = if let Some(loading) = bit_loading {
                    let bits = usize::from(loading[slot]);
                    let taken = bits.min(word.labels.len() - offset);
                    let label = word.labels[offset..offset + taken]
                        .iter()
                        .fold(0, |label, bit| (label << 1) | *bit)
                        << (bits - taken);
                    cells[slot] = PackedCell::BitData {
                        codeword_id: word.id,
                        codeword_bit_offset: offset,
                        label,
                        order: if bits == 0 { 2 } else { 1u16 << bits },
                    };
                    taken
                } else {
                    cells[slot] = PackedCell::Data {
                        codeword_id: word.id,
                        codeword_cell: offset,
                        label: word.labels[offset],
                    };
                    1
                };
                offset += taken;
                consumed += taken;
                if offset == word.labels.len() {
                    word_index += 1;
                    offset = 0;
                }
            } else {
                if !filler {
                    entries.push(NcpPlanEntry::ZeroLoaded {
                        pointer: rank as u16,
                    });
                    filler = true;
                }
                cells[slot] = PackedCell::ZeroLoadedFiller;
            }
        }
        if !filler {
            let terminal_required = entries.is_empty() || (consumed > 0 && offset == 0);
            if entries.len() + 1 == entry_count {
                entries.push(NcpPlanEntry::Null);
            } else if terminal_required {
                continue;
            }
        }
        if entries.is_empty() || entries.len() != entry_count {
            continue;
        }
        for (field_cell, slot) in select_ncp_cells(roles, reserved)?.into_iter().enumerate() {
            cells[slot] = PackedCell::NcpReserved { field_cell };
        }
        return Ok(Preview {
            cells,
            entries,
            consumed,
            starts,
            reserved,
        });
    }
    Err(invalid(
        "No symbol plan fits data, NCP/CRC and terminal-marker requirements.",
    ))
}
