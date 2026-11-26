/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded spatial projection of the authoritative configured resource visits.
use super::{CellRole, ResourcePlan};
use crate::{
    configuration::{UsConfiguration, SAMPLE_RATE_HZ},
    error::{invalid, limit},
    PhyResult,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

/// Application projection budget, independent of execution cells and normative minislot sizes.
pub const MAX_RESOURCE_GRID_CELLS: usize = 16_384;
/// A single initial symbol keeps a full-FFT default viewport within the projection budget.
const DEFAULT_RESOURCE_GRID_SYMBOL_COUNT: usize = 1;

/// Original base-grant frame and physical viewport; zero counts select the remaining extents.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ResourceGridRequest {
    /// Zero-based frame inside the configured base grant.
    pub frame_index: usize,
    /// First original symbol inside the selected frame, zero-based.
    pub first_symbol: usize,
    /// Requested/resolved symbol count; a request zero selects the remaining frame.
    pub symbol_count: usize,
    /// First original physical FFT carrier, without compaction.
    pub first_carrier: usize,
    /// Requested/resolved physical-carrier count; a request zero selects the remaining FFT.
    pub carrier_count: usize,
    /// Positive per-query cell budget, at most MAX_RESOURCE_GRID_CELLS.
    pub max_cells: usize,
}
impl Default for ResourceGridRequest {
    /// Selects the first configured frame/symbol and full original FFT carrier span.
    /// # Returns
    /// A passive one-symbol query with the finite application cell budget; no run starts.
    fn default() -> Self {
        Self {
            frame_index: 0,
            first_symbol: 0,
            symbol_count: DEFAULT_RESOURCE_GRID_SYMBOL_COUNT,
            first_carrier: 0,
            carrier_count: 0,
            max_cells: MAX_RESOURCE_GRID_CELLS,
        }
    }
}

/// Configured allocation role, independent of any measured or simulated I/Q value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceGridRole {
    /// Ordinary loaded data from an authoritative resource visit.
    Data,
    /// Data-carrying complementary pilot from an authoritative visit.
    Complementary,
    /// Known BPSK pilot with no coded-bit contribution.
    KnownPilot,
    /// Selected zero-loaded minislot location with no resource visit.
    ZeroLoaded,
    /// Explicit original physical exclusion from the accepted configuration.
    Excluded,
    /// No grant-selected resource visit, zero-loaded selection or exclusion.
    Unallocated,
}

/// One original physical location; absent visits have no fabricated gain or bit position.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct ResourceGridCell {
    /// Original zero-based symbol inside the selected frame.
    pub frame_symbol: usize,
    /// Original zero-based symbol across frames of this base grant.
    pub grant_symbol: usize,
    /// Original increasing-frequency physical FFT carrier.
    pub physical_carrier: usize,
    /// Absolute configured prefix start in 102.4-MHz sample ticks.
    pub prefix_start_sample: u64,
    /// Absolute configured useful-window start after the accepted cyclic prefix.
    pub useful_start_sample: u64,
    /// Authoritative planned role; no observed I/Q or receiver result is implied.
    pub role: ResourceGridRole,
    /// Original selected-profile inventory position, absent outside that inventory.
    pub minislot_index: Option<usize>,
    /// Original configured minislot pattern, including zero for zero loading.
    pub pilot_pattern: Option<u8>,
    /// Original minislot data bits per cell, separate from complementary loading.
    pub configured_data_loading: Option<u8>,
    /// Whether this original minislot is selected by the configured base grant.
    pub grant_selected: bool,
    /// Original joint-interleaver visit index, not the spatial output position.
    pub visit_index: Option<usize>,
    /// Actual planned visit bits per cell, zero for known pilots or zero-loaded locations.
    pub cell_loading: Option<u8>,
    /// Original pilot/complementary gain before TX Pre-Eq; absent without a resource visit.
    pub amplitude_gain: Option<f64>,
    /// Original coded-bit offset before this visit within one frame.
    pub frame_bit_offset: Option<usize>,
    /// Original coded-bit offset before this visit across configured grant frames.
    pub grant_bit_offset: Option<usize>,
}

/// Counts over the requested rectangle; full-plan coding capacity is reported separately.
#[derive(Debug, Clone, Default, Serialize)]
#[non_exhaustive]
pub struct ResourceGridCounts {
    /// Count of ordinary payload locations in the requested rectangle.
    pub data: usize,
    /// Count of lower-loading complementary data locations in the rectangle.
    pub complementary: usize,
    /// Count of known-pilot locations that consume no coded bits.
    pub known_pilot: usize,
    /// Count of selected zero-loaded minislot locations without resource visits.
    pub zero_loaded: usize,
    /// Count of explicitly excluded physical locations in the rectangle.
    pub excluded: usize,
    /// Count of locations without a grant-selected resource or exclusion.
    pub unallocated: usize,
}

/// Owned configured base-grant region in symbol-major, increasing-physical-carrier order.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct ResourceGridRegion {
    /// Configured CM transmit allocation, independent of actual sample observations.
    pub reference_plane: &'static str,
    /// Original configured base-grant identity; not a generated run identity.
    pub grant_id: u64,
    /// Original base-grant first-prefix sample tick at 102.4 MHz.
    pub grant_epoch_sample: u64,
    /// Original configured first-frame identity, independent of sample-clock origin.
    pub grant_first_frame: u64,
    /// Complete configured base-grant frame count, independent of this viewport.
    pub grant_frames: usize,
    /// Original selected profile identity, independent of array position.
    pub profile_id: u16,
    /// Original selected profile IUC, without inferring modulation or coding from it.
    pub iuc: u8,
    /// Zero-based frame inside the configured base grant.
    pub frame_index: usize,
    /// Original first-frame identity plus the selected relative frame index.
    pub absolute_frame: u64,
    /// Original sample-clock origin of the selected frame first prefix.
    pub frame_prefix_sample: u64,
    /// Complete configured symbols per frame, independent of viewport reduction.
    pub frame_symbols: usize,
    /// Original upstream useful FFT length, without carrier renumbering.
    pub fft_samples: usize,
    /// Configured upstream sample clock in samples per second.
    pub sample_rate_hz: u32,
    /// Original upstream FFT carrier spacing in integer hertz.
    pub spacing_hz: u32,
    /// Accepted cyclic-prefix length in upstream sample ticks.
    pub prefix_samples: usize,
    /// Accepted roll-off/tail length in upstream sample ticks.
    pub rolloff_samples: usize,
    /// Accepted prefix-to-prefix stride, N plus cyclic prefix.
    pub symbol_stride_samples: usize,
    /// First original symbol inside the selected frame, zero-based.
    pub first_symbol: usize,
    /// Requested/resolved symbol count; a request zero selects the remaining frame.
    pub symbol_count: usize,
    /// First original physical FFT carrier, without compaction.
    pub first_carrier: usize,
    /// Requested/resolved physical-carrier count; a request zero selects the remaining FFT.
    pub carrier_count: usize,
    /// Complete ResourcePlan coded capacity per frame, independent of viewport.
    pub frame_bits: usize,
    /// Complete ResourcePlan coded capacity across this finite base grant.
    pub grant_bits: usize,
    /// Complete joint-interleaver visit count per frame, including known pilots.
    pub full_frame_visits: usize,
    /// Role counts over this resolved original-coordinate rectangle.
    pub counts: ResourceGridCounts,
    /// Owned symbol-major, increasing-physical-carrier cells, bounded by the request.
    pub cells: Vec<ResourceGridCell>,
}

/// Copies a finite original-coordinate rectangle from the existing resource-plan authority.
/// # Arguments
/// * `configuration` - Accepted immutable base grant, profile and physical exclusions.
/// * `request` - Original frame/symbol/carrier window and bounded application work budget.
/// # Returns
/// Owned configured roles and original visit/loading/gain/bit/timing metadata; no I/Q is generated.
/// # Errors
/// Rejects invalid extents, work/coordinate overflow or inconsistent duplicate resource visits.
/// All window/budget validation precedes plan construction and output allocation.
pub(crate) fn view(
    configuration: &UsConfiguration,
    request: ResourceGridRequest,
) -> PhyResult<ResourceGridRegion> {
    let waveform = configuration.waveform();
    let grant = configuration.grant();
    let numerology = waveform.numerology();
    let fft_samples = numerology.fft_len();
    let frame_symbols = waveform.frame_symbols();
    if request.frame_index >= grant.frames {
        return Err(invalid("resource_grid.frame_index"));
    }
    if request.max_cells == 0 || request.max_cells > MAX_RESOURCE_GRID_CELLS {
        return Err(limit("resource_grid.max_cells"));
    }
    let symbols = extent(request.first_symbol, request.symbol_count, frame_symbols)?;
    let carriers = extent(request.first_carrier, request.carrier_count, fft_samples)?;
    let total = symbols
        .checked_mul(carriers)
        .filter(|total| *total <= request.max_cells)
        .ok_or_else(|| limit("resource_grid.cells"))?;
    let frame_symbol_origin = request
        .frame_index
        .checked_mul(frame_symbols)
        .ok_or_else(|| limit("resource_grid.symbol_origin"))?;
    let absolute_frame = grant
        .first_frame
        .checked_add(request.frame_index as u64)
        .ok_or_else(|| limit("resource_grid.frame_identity"))?;
    let frame_prefix_sample = prefix_start(configuration, frame_symbol_origin)?;
    let plan = ResourcePlan::new(configuration)?;
    let mut visits = vec![None; total];
    for (visit_index, cell) in plan.visits.iter().enumerate() {
        if (request.first_symbol..request.first_symbol + symbols).contains(&cell.symbol)
            && (request.first_carrier..request.first_carrier + carriers).contains(&cell.carrier)
        {
            let index = (cell.symbol - request.first_symbol) * carriers + cell.carrier
                - request.first_carrier;
            if visits[index].replace((visit_index, cell)).is_some() {
                return Err(invalid("resource_grid.duplicate_visit"));
            }
        }
    }
    let profile = configuration.selected_profile();
    let exclusions: BTreeSet<_> = configuration.excluded().iter().copied().collect();
    let mut cells = Vec::with_capacity(total);
    let mut counts = ResourceGridCounts::default();
    for symbol in request.first_symbol..request.first_symbol + symbols {
        let grant_symbol = frame_symbol_origin
            .checked_add(symbol)
            .ok_or_else(|| limit("resource_grid.grant_symbol"))?;
        let prefix_start_sample = prefix_start(configuration, grant_symbol)?;
        let useful_start_sample = prefix_start_sample
            .checked_add(waveform.prefix() as u64)
            .ok_or_else(|| limit("resource_grid.useful_origin"))?;
        for carrier in request.first_carrier..request.first_carrier + carriers {
            let slot_index = profile
                .minislots()
                .partition_point(|slot| slot.first_carrier <= carrier)
                .checked_sub(1)
                .filter(|index| {
                    carrier
                        < profile.minislots()[*index].first_carrier
                            + numerology.carriers_per_minislot()
                });
            let slot = slot_index.map(|index| &profile.minislots()[index]);
            let selected =
                slot_index.is_some_and(|index| grant.minislots.binary_search(&index).is_ok());
            let visit = visits
                [(symbol - request.first_symbol) * carriers + carrier - request.first_carrier];
            let role = match visit {
                Some((_, visit)) => match visit.role {
                    CellRole::Data => ResourceGridRole::Data,
                    CellRole::Complementary => ResourceGridRole::Complementary,
                    CellRole::KnownPilot => ResourceGridRole::KnownPilot,
                },
                None if exclusions.contains(&carrier) => ResourceGridRole::Excluded,
                None if selected && slot.is_some_and(|slot| slot.loading == 0) => {
                    ResourceGridRole::ZeroLoaded
                }
                None => ResourceGridRole::Unallocated,
            };
            match role {
                ResourceGridRole::Data => counts.data += 1,
                ResourceGridRole::Complementary => counts.complementary += 1,
                ResourceGridRole::KnownPilot => counts.known_pilot += 1,
                ResourceGridRole::ZeroLoaded => counts.zero_loaded += 1,
                ResourceGridRole::Excluded => counts.excluded += 1,
                ResourceGridRole::Unallocated => counts.unallocated += 1,
            }
            let grant_bit_offset = visit
                .map(|(_, visit)| {
                    request
                        .frame_index
                        .checked_mul(plan.frame_bits)
                        .and_then(|origin| origin.checked_add(visit.bit_offset))
                        .ok_or_else(|| limit("resource_grid.bit_origin"))
                })
                .transpose()?;
            cells.push(ResourceGridCell {
                frame_symbol: symbol,
                grant_symbol,
                physical_carrier: carrier,
                prefix_start_sample,
                useful_start_sample,
                role,
                minislot_index: slot_index,
                pilot_pattern: slot.map(|slot| slot.pilot_pattern),
                configured_data_loading: slot.map(|slot| slot.loading),
                grant_selected: selected,
                visit_index: visit.map(|(index, _)| index),
                cell_loading: match visit {
                    Some((_, cell)) => Some(cell.loading),
                    None if role == ResourceGridRole::ZeroLoaded => Some(0),
                    None => None,
                },
                amplitude_gain: visit.map(|(_, cell)| cell.gain),
                frame_bit_offset: visit.map(|(_, cell)| cell.bit_offset),
                grant_bit_offset,
            });
        }
    }
    Ok(ResourceGridRegion {
        reference_plane: "configured_cm_transmit_resources",
        grant_id: grant.id,
        grant_epoch_sample: grant.epoch_sample,
        grant_first_frame: grant.first_frame,
        grant_frames: grant.frames,
        profile_id: profile.id(),
        iuc: profile.iuc(),
        frame_index: request.frame_index,
        absolute_frame,
        frame_prefix_sample,
        frame_symbols,
        fft_samples,
        sample_rate_hz: SAMPLE_RATE_HZ,
        spacing_hz: numerology.spacing_hz(),
        prefix_samples: waveform.prefix(),
        rolloff_samples: waveform.rolloff(),
        symbol_stride_samples: waveform.stride(),
        first_symbol: request.first_symbol,
        symbol_count: symbols,
        first_carrier: request.first_carrier,
        carrier_count: carriers,
        frame_bits: plan.frame_bits,
        grant_bits: plan.grant_bits,
        full_frame_visits: plan.visits.len(),
        counts,
        cells,
    })
}

fn extent(first: usize, count: usize, total: usize) -> PhyResult<usize> {
    let remaining = total
        .checked_sub(first)
        .filter(|remaining| *remaining > 0)
        .ok_or_else(|| invalid("resource_grid.start"))?;
    let count = if count == 0 {
        // Zero requests the original remaining extent rather than an empty view.
        remaining
    } else {
        count
    };
    if count > remaining {
        return Err(invalid("resource_grid.extent"));
    }
    Ok(count)
}

fn prefix_start(configuration: &UsConfiguration, symbol: usize) -> PhyResult<u64> {
    symbol
        .checked_mul(configuration.waveform().stride())
        .and_then(|offset| {
            configuration
                .grant()
                .epoch_sample
                .checked_add(offset as u64)
        })
        .ok_or_else(|| limit("resource_grid.sample_origin"))
}
