/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Ten post-computation TX/RX producers; callbacks cannot alter continuous transport state.
use super::{EncodedPayload, PayloadReceiveSymbol, PayloadTransmitSymbol};
use crate::{
    error::invalid,
    simulation::test_points::{
        CellProbeGrid, ProbeContext, ProbeData, ProbeObserver, ProbeOutcome, SignalDomain,
        TestPointId, TestPointProbe,
    },
    LabError,
};
/// Observes completed TX coding outputs after a successful bounded enqueue.
/// # Arguments
/// * `encoded` - Retained TX-only diagnostic word; no receiver consumes its ID.
/// * `grid` - Fixed cell geometry and payload-session origin.
/// * `context` - Run identity; source/channel time is cleared because queue placement is not yet known.
/// * `probe` - Finite selector and dispatch budgets.
/// * `observer` - Borrowed read-only sink.
/// # Returns
/// Coded-bit and label outcomes with diagnostic codeword identity and no fabricated time.
/// # Errors
/// Propagates metadata/callback errors while encoded outputs and the queued word remain unchanged.
pub fn observe_encoded(
    encoded: &EncodedPayload,
    grid: CellProbeGrid,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<Vec<ProbeOutcome>, LabError> {
    let mut context = cell_context(grid, context)?;
    context.codeword_index = Some(encoded.codeword_id);
    context.source_symbol_index = None;
    context.channel_symbol_index = None;
    context.plc_phase = None;
    Ok(vec![
        probe.emit(
            TestPointId::PayloadCoded,
            &context,
            SignalDomain::CodewordBit,
            ProbeData::Bits(&encoded.encoded.ldpc_codeword),
            observer,
        )?,
        probe.emit(
            TestPointId::PayloadLabels,
            &context,
            SignalDomain::CodewordCell,
            ProbeData::QamLabels(&encoded.labels),
            observer,
        )?,
    ])
}
/// Observes source-mapped and channel-interleaved cells without hiding startup history.
/// # Arguments
/// * `symbol` - Completed TX source/channel stages.
/// * `grid` - Matching validated NI/depth/origin.
/// * `context` - Caller run metadata; stage-specific absolute symbols are reconstructed.
/// * `probe` - Shared finite selector.
/// * `observer` - Read-only caller sink.
/// # Returns
/// Two outcomes preserving typed pilot placeholders and outer None history separately.
/// # Errors
/// Rejects wrong NI/origin or callback failure without changing state/results.
pub fn observe_transmit(
    symbol: &PayloadTransmitSymbol,
    grid: CellProbeGrid,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<Vec<ProbeOutcome>, LabError> {
    if symbol.assembled.cells.len() != grid.settings.cells()
        || symbol.channel.cells.len() != grid.settings.cells()
        || grid
            .origin_symbol
            .checked_add(symbol.assembled.local_symbol_index)
            != Some(symbol.source_symbol_index)
    {
        return Err(invalid("Payload TX probe geometry/identity mismatch."));
    }
    let mut context = cell_context(grid, context)?;
    context.source_symbol_index = Some(symbol.source_symbol_index);
    context.channel_symbol_index = Some(symbol.channel.channel_symbol_index);
    context.plc_phase =
        Some((symbol.channel.channel_symbol_index % crate::types::PLC_FRAME_SYMBOLS) as u8);
    context.codeword_index = None;
    Ok(vec![
        probe.emit(
            TestPointId::PayloadMapped,
            &context,
            SignalDomain::ReducedCarrier,
            ProbeData::TransportCells(&symbol.assembled.cells),
            observer,
        )?,
        probe.emit(
            TestPointId::PayloadInterleaved,
            &context,
            SignalDomain::ReducedCarrier,
            ProbeData::TransportCells(&symbol.channel.cells),
            observer,
        )?,
    ])
}
/// Observes actual restored cells, accepted control, soft data and recovered words after RX computation.
/// # Arguments
/// * `symbol` - Retained RX result, including any terminal control diagnostic.
/// * `grid` - RX-owned cell geometry/NCP selection and synchronized origin.
/// * `context` - Caller run identity; receive/source/word coordinates come from the result.
/// * `probe` - Shared finite selector; budget exhaustion skips whole events.
/// * `observer` - Borrowed read-only sink; failed callbacks do not undo prior RX work.
/// # Returns
/// Restore/control/data outcomes and inverse-bit/payload outcomes for each completed word.
/// # Errors
/// Rejects inconsistent metadata/output dimensions or propagates callback failure.
pub fn observe_receive(
    symbol: &PayloadReceiveSymbol,
    grid: CellProbeGrid,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<Vec<ProbeOutcome>, LabError> {
    if symbol.restored.len() != grid.settings.cells() {
        return Err(invalid("Payload RX probe NI mismatch."));
    }
    let mut context = cell_context(grid, context)?;
    context.channel_symbol_index = Some(symbol.channel_symbol_index);
    context.plc_phase = Some((symbol.channel_symbol_index % crate::types::PLC_FRAME_SYMBOLS) as u8);
    context.source_symbol_index = symbol.source_symbol_index;
    context.codeword_index = None;
    let mut outcomes = vec![probe.emit(
        TestPointId::PayloadRestored,
        &context,
        SignalDomain::ReducedCarrier,
        ProbeData::TransportCells(&symbol.restored),
        observer,
    )?];
    if let Some(decoded) = &symbol.decoded {
        outcomes.push(probe.emit(
            TestPointId::NcpSoft,
            &context,
            SignalDomain::CellBit,
            ProbeData::NcpSoft(&decoded.ncp_soft),
            observer,
        )?);
        if let Some(field) = &decoded.ncp {
            outcomes.push(probe.emit(
                TestPointId::NcpAccepted,
                &context,
                SignalDomain::ControlMessage,
                ProbeData::NcpMessages(&field.messages),
                observer,
            )?);
        }
        outcomes.push(probe.emit(
            TestPointId::DataSoft,
            &context,
            SignalDomain::CellBit,
            ProbeData::DataSoft(&decoded.data_soft),
            observer,
        )?);
        for word in &decoded.recovered {
            context.codeword_index = Some(word.receive_sequence);
            context.profile_id = Some(word.profile_id);
            outcomes.push(probe.emit(
                TestPointId::PayloadLdpcInput,
                &context,
                SignalDomain::CodewordBit,
                ProbeData::Llrs(&word.ldpc_llrs),
                observer,
            )?);
            let payload = word
                .decoded
                .bch
                .codeword
                .get(..fec_codec::parameters::BCH_INPUT_BITS)
                .ok_or_else(|| invalid("Malformed retained BCH result."))?;
            outcomes.push(probe.emit(
                TestPointId::PayloadDecoded,
                &context,
                SignalDomain::CodewordBit,
                ProbeData::Bits(payload),
                observer,
            )?);
        }
    }
    Ok(outcomes)
}
fn cell_context(grid: CellProbeGrid, context: &ProbeContext) -> Result<ProbeContext, LabError> {
    context.validate()?;
    let mut context = context.clone();
    context.cell_grid = Some(grid);
    context.plc_grid = None;
    context.waveform_grid = None;
    context.sample_start_index = None;
    context.validate()?;
    Ok(context)
}
