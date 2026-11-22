/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! All numerical preparation delegates to the original modulation and receiver owners.
use super::*;
use crate::signal_processing::{
    modulation::mer::{calculate_mer_pairs, MerCalculator, MerStatus},
    receiver::diagnostics::{decisions::decide, llr_statistics},
};

impl FullLoopResult {
    /// Queries actual paired receiver cells and MER across the complete filtered population.
    /// # Arguments
    /// * `query` - Actual role/profile/order/inclusive coordinates and bounded display window.
    /// # Returns
    /// At most 2048 owned cells, original availability and separate raw/corrected nearest-point
    /// and matched-TX metrics. Every selected retained cell contributes before pagination.
    /// No actual TX-reference metric is supplied unless every selected cell has a reference.
    /// # Errors
    /// Rejects invalid query bounds/work/orders or out-of-range display offsets; valid empty
    /// selections return explicit zero counts/unavailable metrics without fabricated samples.
    pub fn qam(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopQamView> {
        let selected = Selected::new(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            query,
            2048,
        )?;
        qam_view(selected, self.job.settings.receiver_mode)
    }

    /// Slices actual corrected physical cells without changing production decoder inputs.
    /// # Arguments
    /// * `query` - Same original-cell selection; this operation permits at most 128 rows.
    /// # Returns
    /// Owned original/normalized physical decisions and actual post-descrambling soft values.
    /// LLR statistics cover at most 2048 selected cells starting at the display offset and
    /// remain separate from the coding-order LDPC-input observation. Protected LLRs stay empty.
    /// # Errors
    /// Rejects invalid work/coordinates/reference order, unavailable display offsets and
    /// nonfinite diagnostic inputs through the existing slicer/statistics owners.
    pub fn cell_decisions(&self, query: FullLoopCellQuery) -> ApiResult<FullLoopCellDecisions> {
        let selected = Selected::new(
            self.job.constellation.as_ref(),
            self.job.protected_constellation.as_ref(),
            query,
            128,
        )?;
        let rows = selected
            .window()
            .iter()
            .map(|cell| {
                Ok(FullLoopCellDecisionRow {
                    cell: cell.owned(),
                    decision: decide(cell.order, cell.amplitude, *cell.after)?,
                })
            })
            .collect::<ApiResult<Vec<_>>>()?;
        let first = selected.info.display_start;
        let soft_cells =
            &selected.cells[first..selected.cells.len().min(first.saturating_add(2048))];
        let values: Vec<_> = soft_cells
            .iter()
            .flat_map(|cell| cell.llrs.iter().copied())
            .collect();
        Ok(FullLoopCellDecisions {
            rows,
            llr_first_selected_cell: first,
            llr_selected_cells: soft_cells.len(),
            llr_statistics: llr_statistics(&values)?,
            selection: selected.info,
        })
    }
}

fn qam_view(selected: Selected<'_>, receiver_mode: ReceiverMode) -> ApiResult<FullLoopQamView> {
    let first = selected.cells.first();
    let calculator = first
        .map(|cell| MerCalculator::new(cell.order, cell.amplitude))
        .transpose()?;
    let nearest = |raw| {
        calculator.as_ref().map_or_else(
            || FullLoopCellMer::unavailable(0, MerStatus::NoSamples),
            |calculator| {
                calculator.calculate(selected.cells.iter().map(|cell| {
                    if raw {
                        *cell.before
                    } else {
                        *cell.after
                    }
                }))
            },
        )
    };
    let references_complete =
        first.is_some() && selected.cells.iter().all(|cell| cell.reference.is_some());
    let reference = |raw| {
        references_complete.then(|| {
            calculate_mer_pairs(selected.cells.iter().filter_map(|cell| {
                cell.reference
                    .map(|reference| (if raw { *cell.before } else { *cell.after }, reference))
            }))
        })
    };
    Ok(FullLoopQamView {
        receiver_mode,
        raw_reference_plane: "rx_before_equalization",
        equalized_reference_plane: "rx_after_equalization",
        cells: selected.window().iter().map(Cell::owned).collect(),
        raw_nearest_mer: nearest(true),
        equalized_nearest_mer: nearest(false),
        raw_tx_reference_mer: reference(true),
        equalized_tx_reference_mer: reference(false),
        selection: selected.info,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::channel::constellation::{
        ConstellationPair, ConstellationResult, ConstellationStatus,
    };
    fn pairs() -> ConstellationResult {
        let pairs: Vec<_> = (0..4096)
            .map(|index| ConstellationPair {
                channel_symbol_index: index as u64 / 32,
                source_symbol_index: index as u64 / 32,
                physical_bin: 320 + (index % 32) as u16,
                frequency_hz: 100_000_000 + (index % 32) as u32 * 25000,
                data_position: index % 32,
                profile_id: 0,
                modulation_order: 2,
                before_equalization: [if index < 2048 { 1.0 } else { 1.1 }, 0.0],
                after_equalization: [1.0, 0.0],
                reference_iq: Some([1.0, 0.0]),
                llrs: vec![5.0],
            })
            .collect();
        ConstellationResult {
            schema_version: 1,
            selection: Default::default(),
            status: ConstellationStatus::Complete,
            collected_pairs: pairs.len(),
            reference: "synthetic independent power vector",
            channel_coefficients: "unity",
            pairs,
            ncp_pairs: vec![],
        }
    }
    #[test]
    fn all_selected_mer_includes_errors_after_display_cap_and_keeps_reference_planes_separate() {
        let original = pairs();
        let selected = Selected::new(Some(&original), None, Default::default(), 2048).unwrap();
        let view = qam_view(selected, ReceiverMode::SuppliedChannel).unwrap();
        assert_eq!(view.cells.len(), 2048);
        assert!(view
            .cells
            .iter()
            .all(|cell| cell.before_equalization == [1.0, 0.0]));
        assert_eq!(view.selection.selected_cells, 4096);
        assert!((view.raw_nearest_mer.mer_db.unwrap() - 23.010299956639813).abs() < 1e-9);
        assert_eq!(view.raw_nearest_mer.sample_count, 4096);
        assert!(
            (view.raw_tx_reference_mer.unwrap().mer_db.unwrap() - 23.010299956639813).abs() < 1e-9
        );
        assert_eq!(
            view.equalized_nearest_mer.calculation_status,
            MerStatus::PerfectReference
        );
        assert_eq!(
            view.equalized_tx_reference_mer.unwrap().calculation_status,
            MerStatus::PerfectReference
        );
        let selected = Selected::new(
            Some(&original),
            None,
            FullLoopCellQuery {
                start: 2048,
                count: 3,
                ..Default::default()
            },
            2048,
        )
        .unwrap();
        let view = qam_view(selected, ReceiverMode::SuppliedChannel).unwrap();
        assert_eq!(view.cells.len(), 3);
        assert_eq!(view.cells[0].before_equalization, [1.1, 0.0]);
        assert!((view.raw_nearest_mer.mer_db.unwrap() - 23.010299956639813).abs() < 1e-9);
        let selected = Selected::new(
            Some(&original),
            None,
            FullLoopCellQuery {
                first_symbol: Some(64),
                ..Default::default()
            },
            2048,
        )
        .unwrap();
        let view = qam_view(selected, ReceiverMode::SuppliedChannel).unwrap();
        assert_eq!(view.selection.selected_cells, 2048);
        assert!((view.raw_nearest_mer.mer_db.unwrap() - 20.0).abs() < 1e-9);
    }
    #[test]
    fn missing_references_empty_filters_and_output_ownership_remain_explicit() {
        let mut original = pairs();
        original.pairs[4095].reference_iq = None;
        let selected = Selected::new(Some(&original), None, Default::default(), 2048).unwrap();
        let mut view = qam_view(selected, ReceiverMode::SuppliedChannel).unwrap();
        assert!(view.raw_tx_reference_mer.is_none());
        assert!(view.equalized_tx_reference_mer.is_none());
        assert_eq!(view.raw_nearest_mer.sample_count, 4096);
        view.cells[0].llrs[0] = 999.0;
        assert_eq!(original.pairs[0].llrs, [5.0]);
        assert_eq!(view.selection.available_profiles, [0]);
        assert_eq!(view.selection.available_orders, [2]);
        let bounds = view.selection.bounds.unwrap();
        assert_eq!(
            (
                bounds.first_bin,
                bounds.last_bin,
                bounds.first_symbol,
                bounds.last_symbol
            ),
            (320, 351, 0, 127)
        );
        let selected = Selected::new(
            Some(&original),
            None,
            FullLoopCellQuery {
                profile_id: Some(255),
                ..Default::default()
            },
            2048,
        )
        .unwrap();
        let view = qam_view(selected, ReceiverMode::SuppliedChannel).unwrap();
        assert_eq!(view.selection.retained_cells, 4096);
        assert_eq!(view.selection.selected_cells, 0);
        assert!(view.cells.is_empty());
        assert_eq!(
            view.raw_nearest_mer.calculation_status,
            MerStatus::NoSamples
        );
        assert!(view.raw_tx_reference_mer.is_none());
    }
    #[test]
    fn strict_query_admission_and_per_operation_windows_reject_without_changing_originals() {
        for data in [
            b"{".as_slice(),
            b"{\"unknown\":1}".as_slice(),
            b"{\"stream\":\"unknown\"}".as_slice(),
            b"{\"modulation_order\":8}".as_slice(),
            b"{\"first_bin\":8192}".as_slice(),
            b"{\"first_symbol\":3,\"last_symbol\":2}".as_slice(),
            b"{\"count\":2049}".as_slice(),
        ] {
            assert!(FullLoopCellQuery::from_json(data).is_err());
        }
        assert!(FullLoopCellQuery::from_json(&vec![b' '; 4097]).is_err());
        let original = pairs();
        for (query, limit) in [
            (
                FullLoopCellQuery {
                    start: usize::MAX,
                    ..Default::default()
                },
                2048,
            ),
            (
                FullLoopCellQuery {
                    count: 129,
                    ..Default::default()
                },
                128,
            ),
            (
                FullLoopCellQuery {
                    first_bin: Some(350),
                    last_bin: Some(340),
                    ..Default::default()
                },
                2048,
            ),
        ] {
            assert!(Selected::new(Some(&original), None, query, limit).is_err());
        }
        let empty = Selected::new(None, None, Default::default(), 2048).unwrap();
        assert_eq!(empty.info.retained_cells, 0);
        assert!(empty.info.bounds.is_none());
        assert!(empty.window().is_empty());
    }
}
