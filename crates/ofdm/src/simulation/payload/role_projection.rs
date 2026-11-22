/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Diagnostic role transport uses a separate metadata-only interleaver, never live TX state.
use crate::{
    signal_processing::interleaving::{InterleaverDirection, SymbolInterleaver},
    simulation::test_points::{ProbeContext, TransmitCellRole},
    SimulationError,
};

/// Transports retained packing metadata only for a demonstrably aligned first symbol.
/// # Arguments
/// * `source`, `source_offset` - Original mapped context and absolute event offset.
/// * `target` - Original interleaved context with matching run/configuration and geometry.
/// # Returns
/// Fresh role vector with true delay gaps, or None for insufficient provenance.
/// # Errors
/// Preserves existing interleaver admission; no live stream state is borrowed or changed.
pub(crate) fn first_symbol_roles(
    source: &ProbeContext,
    source_offset: usize,
    target: &ProbeContext,
) -> Result<Option<Vec<Option<TransmitCellRole>>>, SimulationError> {
    let (Some(grid), Some(roles)) = (source.cell_grid, source.transmit_cell_roles.as_ref()) else {
        return Ok(None);
    };
    let aligned = source_offset == 0
        && source.source_symbol_index == Some(grid.origin_symbol)
        && target.channel_symbol_index == Some(grid.origin_symbol)
        && target.source_symbol_index == source.source_symbol_index
        && target.cell_grid == source.cell_grid
        && target.run_id == source.run_id
        && target.configuration_id == source.configuration_id
        && roles.len() == grid.settings.cells();
    if !aligned {
        return Ok(None);
    }
    let input: Vec<_> = roles.iter().copied().map(Some).collect();
    Ok(Some(
        SymbolInterleaver::new(grid.settings, InterleaverDirection::Transmit)?
            .process_symbol(0, &input)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        signal_processing::interleaving::InterleaverSettings,
        simulation::test_points::{CellProbeGrid, RunPhase},
        transmission::{packing::NcpModulation, SubcarrierSpacing},
    };

    fn context() -> ProbeContext {
        let mut context = ProbeContext::new("roles", "input", RunPhase::Observation).unwrap();
        context.cell_grid = Some(CellProbeGrid {
            settings: InterleaverSettings::new(SubcarrierSpacing::Khz50, 8, 2).unwrap(),
            origin_symbol: 128,
            ncp_modulation: NcpModulation::Qpsk,
        });
        context.source_symbol_index = Some(128);
        context.channel_symbol_index = Some(128);
        use TransmitCellRole::*;
        context.transmit_cell_roles = Some(vec![Ncp, Data, Filler, Data, Pilot, Data, Data, Data]);
        context
    }

    #[test]
    fn independent_eight_cell_roles_preserve_delay_gaps_and_frequency_order() {
        let source = context();
        let mut target = source.clone();
        target.transmit_cell_roles = None;
        use TransmitCellRole::*;
        // NI=8: frequency source order 0,4,2,6,1,5,3,7. Depth 2 has no odd-branch history.
        let expected = vec![
            Some(Ncp),
            Some(Pilot),
            Some(Filler),
            Some(Data),
            None,
            None,
            None,
            None,
        ];
        assert_eq!(
            first_symbol_roles(&source, 0, &target).unwrap(),
            Some(expected.clone())
        );
        assert_eq!(
            first_symbol_roles(&source, 0, &target).unwrap(),
            Some(expected)
        );
        assert_eq!(source.transmit_cell_roles.as_ref().unwrap().len(), 8);
    }

    #[test]
    fn later_misaligned_missing_or_other_run_roles_remain_unavailable() {
        let source = context();
        assert!(first_symbol_roles(&source, 1, &source).unwrap().is_none());
        for case in 0..5 {
            let mut target = source.clone();
            match case {
                0 => target.channel_symbol_index = Some(129),
                1 => target.source_symbol_index = None,
                2 => target.cell_grid = None,
                3 => target.run_id = "other".into(),
                _ => target.configuration_id = "other".into(),
            }
            assert!(first_symbol_roles(&source, 0, &target).unwrap().is_none());
        }
        let mut missing = source.clone();
        missing.transmit_cell_roles = None;
        assert!(first_symbol_roles(&missing, 0, &source).unwrap().is_none());
        missing.transmit_cell_roles = Some(vec![TransmitCellRole::Data]);
        assert!(first_symbol_roles(&missing, 0, &source).unwrap().is_none());
    }
}
