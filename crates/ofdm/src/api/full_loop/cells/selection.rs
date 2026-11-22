/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrow original report values; only the requested output window allocates owned soft arrays.
use super::*;
use crate::simulation::channel::{
    constellation::ConstellationResult, protected_constellation::ProtectedConstellation,
};
use std::collections::BTreeSet;

pub(super) struct Cell<'a> {
    pub bin: u16,
    pub frequency: u32,
    pub symbol: u64,
    pub source: Option<u64>,
    pub profile: Option<u8>,
    pub order: u16,
    pub amplitude: f64,
    pub before: &'a [f64; 2],
    pub after: &'a [f64; 2],
    pub reference: Option<[f64; 2]>,
    pub llrs: &'a [f64],
}
impl Cell<'_> {
    pub(super) fn owned(&self) -> FullLoopCell {
        FullLoopCell {
            physical_bin: self.bin,
            frequency_hz: self.frequency,
            channel_symbol_index: self.symbol,
            source_symbol_index: self.source,
            profile_id: self.profile,
            modulation_order: self.order,
            reference_amplitude: self.amplitude,
            before_equalization: *self.before,
            after_equalization: *self.after,
            reference_iq: self.reference,
            llrs: self.llrs.to_vec(),
        }
    }
}
pub(super) struct Selected<'a> {
    pub cells: Vec<Cell<'a>>,
    pub info: FullLoopCellSelection,
}
impl<'a> Selected<'a> {
    pub(super) fn new(
        data: Option<&'a ConstellationResult>,
        protected: Option<&'a ProtectedConstellation>,
        query: FullLoopCellQuery,
        maximum_rows: usize,
    ) -> ApiResult<Self> {
        query.validate()?;
        if query.count > maximum_rows {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Cell output exceeds operation row cap",
            ));
        }
        let mut cells: Vec<Cell<'a>> = match query.stream {
            FullLoopCellStream::Payload => data
                .into_iter()
                .flat_map(|report| report.pairs.iter())
                .map(|pair| Cell {
                    bin: pair.physical_bin,
                    frequency: pair.frequency_hz,
                    symbol: pair.channel_symbol_index,
                    source: Some(pair.source_symbol_index),
                    profile: Some(pair.profile_id),
                    order: pair.modulation_order,
                    amplitude: 1.0,
                    before: &pair.before_equalization,
                    after: &pair.after_equalization,
                    reference: pair.reference_iq,
                    llrs: &pair.llrs,
                })
                .collect(),
            FullLoopCellStream::Ncp => data
                .into_iter()
                .flat_map(|report| report.ncp_pairs.iter())
                .map(|pair| Cell {
                    bin: pair.physical_bin,
                    frequency: pair.frequency_hz,
                    symbol: pair.channel_symbol_index,
                    source: Some(pair.source_symbol_index),
                    profile: None,
                    order: pair.modulation_order,
                    amplitude: 1.0,
                    before: &pair.before_equalization,
                    after: &pair.after_equalization,
                    reference: pair.reference_iq,
                    llrs: &pair.llrs,
                })
                .collect(),
            stream => {
                let selection = protected.and_then(|report| match stream {
                    FullLoopCellStream::PlcData => Some((&report.plc_data, 16, 1.0)),
                    FullLoopCellStream::PlcPreamble => Some((&report.plc_preamble, 2, 1.0)),
                    FullLoopCellStream::ContinuousPilots => {
                        Some((&report.continuous_pilots, 2, 2.0))
                    }
                    FullLoopCellStream::ScatteredPilots => Some((&report.scattered_pilots, 2, 2.0)),
                    FullLoopCellStream::Payload | FullLoopCellStream::Ncp => None,
                });
                selection
                    .into_iter()
                    .flat_map(|(report, order, amplitude)| {
                        report.pairs.iter().map(move |pair| Cell {
                            bin: pair.physical_bin,
                            frequency: pair.frequency_hz,
                            symbol: pair.channel_symbol_index,
                            source: None,
                            profile: None,
                            order,
                            amplitude,
                            before: &pair.before_equalization,
                            after: &pair.after_equalization,
                            reference: pair.reference_iq,
                            llrs: &[],
                        })
                    })
                    .collect()
            }
        };
        let retained_cells = cells.len();
        let available_profiles = cells
            .iter()
            .filter_map(|cell| cell.profile)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let available_orders: Vec<_> = cells
            .iter()
            .filter(|cell| {
                query
                    .profile_id
                    .is_none_or(|profile| cell.profile == Some(profile))
            })
            .map(|cell| cell.order)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let selected_order = query
            .modulation_order
            .or_else(|| available_orders.first().copied());
        let bounds = cells.first().map(|first| {
            cells.iter().fold(
                FullLoopCellBounds {
                    first_bin: first.bin,
                    last_bin: first.bin,
                    first_symbol: first.symbol,
                    last_symbol: first.symbol,
                },
                |mut bounds, cell| {
                    bounds.first_bin = bounds.first_bin.min(cell.bin);
                    bounds.last_bin = bounds.last_bin.max(cell.bin);
                    bounds.first_symbol = bounds.first_symbol.min(cell.symbol);
                    bounds.last_symbol = bounds.last_symbol.max(cell.symbol);
                    bounds
                },
            )
        });
        cells.retain(|cell| {
            selected_order == Some(cell.order)
                && query
                    .profile_id
                    .is_none_or(|profile| cell.profile == Some(profile))
                && query.first_bin.is_none_or(|bin| cell.bin >= bin)
                && query.last_bin.is_none_or(|bin| cell.bin <= bin)
                && query
                    .first_symbol
                    .is_none_or(|symbol| cell.symbol >= symbol)
                && query.last_symbol.is_none_or(|symbol| cell.symbol <= symbol)
        });
        let remaining = cells.len().checked_sub(query.start).ok_or_else(|| {
            super::super::super::error::invalid(
                "Cell display offset is outside the selected population",
            )
        })?;
        let displayed_cells = remaining.min(if query.count == 0 {
            maximum_rows
        } else {
            query.count
        });
        let info = FullLoopCellSelection {
            retained_cells,
            selected_cells: cells.len(),
            available_profiles,
            available_orders,
            selected_order,
            bounds,
            display_start: query.start,
            displayed_cells,
            query,
        };
        Ok(Self { cells, info })
    }

    pub(super) fn window(&self) -> &[Cell<'a>] {
        &self.cells[self.info.display_start..self.info.display_start + self.info.displayed_cells]
    }
}
