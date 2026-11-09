/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Post-computation waveform probes reuse shared finite selectors; scientific state is not exposed.
use super::{ReceivedSymbol, TransmittedSymbol};
use crate::{
    error::invalid,
    signal_processing::waveform::{SampleBlock, WaveformGeometry},
    simulation::test_points::{
        PlcProbeGrid, ProbeContext, ProbeData, ProbeObserver, ProbeOutcome, SignalDomain,
        TestPointId, TestPointProbe, WaveformProbeGrid,
    },
    LabError,
};
/// Emits TX transform, prefix, window and overlap boundaries after successful processing.
/// # Arguments
/// * `symbol` - Completed owned TX result, unaffected by callback errors.
/// * `geometry` - Matching per-run geometry for full-size metadata.
/// * `context` - Run identity, matching symbol index and physical frequency origin.
/// * `probe` - Shared bounded stage/time/element selector.
/// * `observer` - Read-only borrowed sink; no transmitter state is accessible.
/// # Returns
/// Four dispatch outcomes with full waveform geometry and absolute sample origins.
/// # Errors
/// Rejects inconsistent metadata or callback failure; completed TX results remain usable.
pub fn observe_transmit(
    symbol: &TransmittedSymbol,
    geometry: &WaveformGeometry,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<Vec<ProbeOutcome>, LabError> {
    if symbol.useful.len() != geometry.fft_samples()
        || symbol.prefixed.len() != geometry.stride_samples()
        || symbol.windowed.len() != geometry.extended_samples()
        || symbol.output.samples().len() != geometry.stride_samples()
    {
        return Err(invalid(
            "TX probe geometry does not match completed boundaries.",
        ));
    }
    let mut context = waveform_context(geometry, context, Some(symbol.symbol_index))?;
    let mut outcomes = Vec::new();
    for (stage, samples, offset) in [
        (
            TestPointId::TxIfft,
            symbol.useful.as_slice(),
            geometry.prefix_samples(),
        ),
        (TestPointId::TxPrefix, symbol.prefixed.as_slice(), 0),
        (TestPointId::TxWindow, symbol.windowed.as_slice(), 0),
        (TestPointId::TxOverlap, symbol.output.samples(), 0),
    ] {
        context.sample_start_index = Some(
            symbol
                .output
                .start_index()
                .checked_add(offset as u64)
                .ok_or_else(|| invalid("Waveform probe sample overflow."))?,
        );
        outcomes.push(probe.emit(
            stage,
            &context,
            SignalDomain::TimeSample,
            ProbeData::Iq(samples),
            observer,
        )?);
    }
    Ok(outcomes)
}
/// Emits known-reference RX framing, prefix, FFT, equalization and extraction boundaries.
/// # Arguments
/// * `symbol` - Completed RX result, containing only actually received samples.
/// * `geometry` - Matching receiver-owned geometry.
/// * `context` - Run identity, symbol index and original physical-bin origin.
/// * `probe` - Finite selector and budgets.
/// * `observer` - Read-only caller sink; failed callbacks cannot change RX state/results.
/// # Returns
/// Seven outcomes with explicit receive direction and appropriate sample/bin domains.
/// # Errors
/// Rejects metadata or callback errors without deleting completed scientific outputs.
pub fn observe_receive(
    symbol: &ReceivedSymbol,
    geometry: &WaveformGeometry,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<Vec<ProbeOutcome>, LabError> {
    if symbol.period.samples().len() != geometry.stride_samples()
        || symbol.useful.len() != geometry.fft_samples()
        || symbol.bins.len() != geometry.fft_samples()
        || symbol.equalized.len() != geometry.fft_samples()
    {
        return Err(invalid(
            "RX probe geometry does not match completed boundaries.",
        ));
    }
    let mut context = waveform_context(geometry, context, Some(symbol.symbol_index))?;
    context.sample_start_index = Some(symbol.period.start_index());
    let mut outcomes = vec![probe.emit(
        TestPointId::RxFramed,
        &context,
        SignalDomain::TimeSample,
        ProbeData::Iq(symbol.period.samples()),
        observer,
    )?];
    context.sample_start_index = Some(
        symbol
            .period
            .start_index()
            .checked_add(geometry.prefix_samples() as u64)
            .ok_or_else(|| invalid("RX probe sample overflow."))?,
    );
    outcomes.push(probe.emit(
        TestPointId::RxUseful,
        &context,
        SignalDomain::TimeSample,
        ProbeData::Iq(&symbol.useful),
        observer,
    )?);
    for (stage, domain, data) in [
        (
            TestPointId::RxFft,
            SignalDomain::PhysicalFftBin,
            ProbeData::Iq(&symbol.bins),
        ),
        (
            TestPointId::RxEqualized,
            SignalDomain::PhysicalFftBin,
            ProbeData::Iq(&symbol.equalized),
        ),
        (
            TestPointId::RxPlc,
            SignalDomain::PlcCell,
            ProbeData::Iq(&symbol.signals.plc),
        ),
        (
            TestPointId::RxPilots,
            SignalDomain::PhysicalFftBin,
            ProbeData::ReceivedPilots(&symbol.signals.pilots),
        ),
        (
            TestPointId::RxReduced,
            SignalDomain::ReducedCarrier,
            ProbeData::OptionalIq(&symbol.signals.reduced),
        ),
    ] {
        context.plc_grid = Some(PlcProbeGrid::for_symbols(
            geometry.settings().subcarrier_spacing,
            1,
        )?);
        outcomes.push(probe.emit(stage, &context, domain, data, observer)?);
    }
    Ok(outcomes)
}
/// Emits one explicit channel-plane sample boundary, including standalone drain blocks.
/// # Arguments
/// * `block` - Already transferred TX-output or RX-input samples.
/// * `receive_plane` - False for channel input; true for channel output/RX input.
/// * `geometry` - Frozen sample-clock/numerology metadata.
/// * `context` - Caller run/time identity; a drain may have no symbol index.
/// * `probe` - Shared finite selector.
/// * `observer` - Read-only sink with bounded selected samples.
/// # Returns
/// One channel-plane dispatch outcome with preserved absolute sample origin.
/// # Errors
/// Rejects inconsistent context or propagates callback error without changing samples.
pub fn observe_channel(
    block: &SampleBlock,
    receive_plane: bool,
    geometry: &WaveformGeometry,
    context: &ProbeContext,
    probe: &mut TestPointProbe,
    observer: &mut impl ProbeObserver,
) -> Result<ProbeOutcome, LabError> {
    let mut context = waveform_context(geometry, context, None)?;
    context.sample_start_index = Some(block.start_index());
    let stage = if receive_plane {
        TestPointId::ChannelOutput
    } else {
        TestPointId::ChannelInput
    };
    probe.emit(
        stage,
        &context,
        SignalDomain::TimeSample,
        ProbeData::Iq(block.samples()),
        observer,
    )
}
fn waveform_context(
    geometry: &WaveformGeometry,
    context: &ProbeContext,
    symbol: Option<u64>,
) -> Result<ProbeContext, LabError> {
    context.validate()?;
    if symbol.is_some_and(|index| {
        context.channel_symbol_index != Some(index)
            || context.plc_phase != Some((index % 128) as u8)
    }) || context
        .physical_grid
        .is_none_or(|grid| grid.spacing != geometry.settings().subcarrier_spacing)
    {
        return Err(invalid(
            "Waveform probe geometry/symbol metadata is inconsistent.",
        ));
    }
    let mut context = context.clone();
    context.waveform_grid = Some(WaveformProbeGrid {
        geometry: geometry.clone(),
        sample_rate_hz: crate::transmission::settings::SAMPLE_RATE_HZ,
    });
    context.plc_grid = None;
    Ok(context)
}
