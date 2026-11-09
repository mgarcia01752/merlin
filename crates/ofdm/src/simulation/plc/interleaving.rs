/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Completed nibble/permutation outputs with optional bounded read-only observation.
use crate::{
    error::invalid,
    simulation::test_points::{
        PlcProbeGrid, ProbeContext, ProbeData, ProbeObserver, ProbeOutcome, SignalDomain,
        SignalReference, TestPointId, TestPointProbe,
    },
    transmission::{
        plc::{
            block_interleaving::interleave, geometry::PLC_DATA_SYMBOLS, InterleavedPlcBlock,
            PlcCodewordNibbles, PlcFrameGeometry,
        },
        SubcarrierSpacing,
    },
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
use serde::Serialize;
/// Bounded immutable stage results for one codeword at a known post-preamble phase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlcInterleavingStages {
    first_symbol_index: u64,
    coded: PlcCodewordNibbles,
    interleaved: InterleavedPlcBlock,
}
impl PlcInterleavingStages {
    /// Composes grouping/permutation after validating a whole-block data-phase start.
    /// # Arguments
    /// * `spacing` - Fixed PLC geometry.
    /// * `first_symbol_index` - Absolute post-preamble symbol index; modulo128 must
    ///   be in0..120 and aligned to a6/12-symbol PLC codeword boundary.
    /// * `bits` -384 transmitted coded bits; not mother-code or randomized bits.
    /// # Returns
    /// Owned coded labels and time-major output, before any observation.
    /// # Errors
    /// Rejects invalid start phase, nonbinary values or incorrect bit count.
    pub fn generate(
        spacing: SubcarrierSpacing,
        first_symbol_index: u64,
        bits: &[u8],
    ) -> Result<Self, SimulationError> {
        let phase = (first_symbol_index % PLC_FRAME_SYMBOLS) as usize;
        let geometry = PlcFrameGeometry::new(spacing);
        if phase >= PLC_DATA_SYMBOLS || phase % geometry.data_symbols_per_codeword != 0 {
            log::debug!("plc-interleaver-phase-rejected phase={phase}");
            return Err(invalid(
                "PLC interleaver must start at an aligned data-codeword phase.",
            ));
        }
        let coded = PlcCodewordNibbles::from_bits(bits)?;
        let interleaved = interleave(spacing, &coded);
        Ok(Self {
            first_symbol_index,
            coded,
            interleaved,
        })
    }
    /// Borrows unrandomized codeword-order labels for direct test-point access.
    /// # Returns
    /// Immutable validated96-label block.
    pub fn coded(&self) -> &PlcCodewordNibbles {
        &self.coded
    }
    /// Borrows the physical-time/local-PLC-carrier output for further processing.
    /// # Returns
    /// Immutable6x16 or12x8 block; no physical FFT-bin assignment yet.
    pub fn interleaved(&self) -> &InterleavedPlcBlock {
        &self.interleaved
    }
    /// Dispatches selected completed label windows without changing scientific results.
    /// # Arguments
    /// * `context` - Bounded caller metadata with matching block-start channel index/phase.
    /// * `probe` - Optional selector and finite dispatch budgets.
    /// * `observer` - Caller-owned sink for borrowed events, with no mutable stage state.
    /// # Returns
    /// Coded-nibble and interleaved dispatch outcomes. Full grid metadata survives
    /// selection clipping; event shape still counts selected1-D labels.
    /// # Errors
    /// Rejects inconsistent/invalid context or propagates callbacks without retry.
    /// Completed results remain available on any observation failure.
    pub fn observe(
        &self,
        context: &ProbeContext,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<[ProbeOutcome; 2], SimulationError> {
        context.validate()?;
        if context.channel_symbol_index != Some(self.first_symbol_index)
            || context.plc_phase != Some((self.first_symbol_index % PLC_FRAME_SYMBOLS) as u8)
        {
            return Err(invalid(
                "PLC observation requires matching block-start context.",
            ));
        }
        let mut context = context.clone();
        context.reference = SignalReference::NotApplicable;
        context.plc_grid = Some(PlcProbeGrid::new(self.interleaved.spacing()));
        Ok([
            probe.emit(
                TestPointId::PlcCodedNibbles,
                &context,
                SignalDomain::CodewordCell,
                ProbeData::Labels(self.coded.values()),
                observer,
            )?,
            probe.emit(
                TestPointId::PlcInterleaved,
                &context,
                SignalDomain::PlcCell,
                ProbeData::Labels(self.interleaved.values()),
                observer,
            )?,
        ])
    }
}
