/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Observation of completed outputs; no callbacks inside scientific stage computations.
use super::PlcFrameSignals;
use crate::{
    error::invalid,
    simulation::test_points::{
        PhysicalProbeGrid, PlcProbeGrid, ProbeContext, ProbeData, ProbeObserver, ProbeOutcome,
        SignalDomain, SignalReference, TestPointId, TestPointProbe,
    },
    transmission::frame::PhysicalFrequencySymbol,
    types::PLC_FRAME_SYMBOLS,
    SimulationError,
};
impl PlcFrameSignals {
    /// Emits selected PLC stage windows after the complete frame has been generated.
    /// # Arguments
    /// * `context` - Bounded run identity with frame-start channel index and phase0.
    /// * `probe` - Optional finite stage/time/element selector.
    /// * `observer` - Caller sink, receiving borrowed arrays and no mutable stage state.
    /// # Returns
    /// Ordered outcomes for message, five boundaries per word, and eight preamble columns.
    /// Word filters use their first channel symbol; preamble filters use each symbol.
    /// # Errors
    /// Rejects invalid frame context or propagates callback failures. Scientific
    /// results remain owned and unchanged; already consumed budgets are not reset.
    pub fn observe(
        &self,
        context: &ProbeContext,
        probe: &mut TestPointProbe,
        observer: &mut impl ProbeObserver,
    ) -> Result<Vec<ProbeOutcome>, SimulationError> {
        context.validate()?;
        let start = self
            .frame_index
            .checked_mul(PLC_FRAME_SYMBOLS)
            .ok_or_else(|| invalid("PLC observation frame overflow."))?;
        if context.channel_symbol_index != Some(start) || context.plc_phase != Some(0) {
            return Err(invalid(
                "PLC observation requires matching frame-start context.",
            ));
        }
        let mut context = context.clone();
        context.reference = SignalReference::NotApplicable;
        context.plc_grid = None;
        context.physical_grid = None;
        let bytes = self.message.to_bytes();
        let mut outcomes = vec![probe.emit(
            TestPointId::PlcMessage,
            &context,
            SignalDomain::MessageByte,
            ProbeData::Bytes(&bytes),
            observer,
        )?];
        for (word, block) in self.codewords.iter().enumerate() {
            context.channel_symbol_index = Some(block.randomized.first_symbol_index);
            context.plc_phase =
                Some((block.randomized.first_symbol_index % PLC_FRAME_SYMBOLS) as u8);
            context.codeword_index = Some(word as u64);
            context.plc_grid = Some(PlcProbeGrid::new(self.spacing));
            for (stage, domain, data) in [
                (
                    TestPointId::PlcCodedBits,
                    SignalDomain::CodewordBit,
                    ProbeData::Bits(&block.encoded.transmitted_bits),
                ),
                (
                    TestPointId::PlcCodedNibbles,
                    SignalDomain::CodewordCell,
                    ProbeData::Labels(block.coded.values()),
                ),
                (
                    TestPointId::PlcInterleaved,
                    SignalDomain::PlcCell,
                    ProbeData::Labels(block.interleaved.values()),
                ),
                (
                    TestPointId::PlcRandomized,
                    SignalDomain::PlcCell,
                    ProbeData::Labels(block.randomized.values()),
                ),
            ] {
                context.reference = SignalReference::NotApplicable;
                outcomes.push(probe.emit(stage, &context, domain, data, observer)?);
            }
            context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
            outcomes.push(probe.emit(
                TestPointId::PlcIq,
                &context,
                SignalDomain::PlcCell,
                ProbeData::Iq(&block.iq),
                observer,
            )?);
        }
        context.codeword_index = None;
        context.plc_grid = Some(PlcProbeGrid::for_symbols(self.spacing, 1)?);
        for symbol in &self.preamble {
            context.channel_symbol_index = Some(symbol.symbol_index);
            context.plc_phase = Some((symbol.symbol_index % PLC_FRAME_SYMBOLS) as u8);
            outcomes.push(probe.emit(
                TestPointId::PlcPreamble,
                &context,
                SignalDomain::PlcCell,
                ProbeData::Iq(&symbol.iq),
                observer,
            )?);
        }
        Ok(outcomes)
    }
}
/// Emits a selected physical-grid window after protected-bin assembly completes.
/// # Arguments
/// * `symbol` - Completed FFT-sized output, including explicit missing-history cells.
/// * `context` - Run identity with matching channel symbol and PLC phase.
/// * `probe` - Optional finite selector and budgets.
/// * `observer` - Borrowed read-only sink; errors cannot modify the scientific result.
/// # Returns
/// One dispatch outcome with full physical frequency metadata and original offsets.
/// # Errors
/// Rejects inconsistent context or propagates observer failure without retry.
pub fn observe_frequency_symbol(
    symbol: &PhysicalFrequencySymbol,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ProbeOutcome, SimulationError> {
    context.validate()?;
    if context.channel_symbol_index != Some(symbol.symbol_index)
        || context.plc_phase != Some((symbol.symbol_index % PLC_FRAME_SYMBOLS) as u8)
    {
        return Err(invalid(
            "Physical observation context does not match its symbol.",
        ));
    }
    let mut context = context.clone();
    context.plc_grid = None;
    context.reference = SignalReference::OtherSubcarrierRms {
        rms: symbol.reference_rms,
    };
    context.physical_grid = Some(PhysicalProbeGrid {
        spacing: symbol.spacing,
        zero_frequency_hz: symbol.zero_frequency_hz,
    });
    probe.emit(
        TestPointId::PhysicalBins,
        &context,
        SignalDomain::PhysicalFftBin,
        ProbeData::OptionalIq(&symbol.bins),
        observer,
    )
}
