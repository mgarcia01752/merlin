/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in bounded observation; no filesystem output or scientific state ownership.
use super::{ProbeContext, ProbeData, ProbeEvent, SignalDomain, TestPointId};
use crate::{error::invalid, SimulationError};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Range,
};
const MAX_SELECTIONS: usize = 32;
const MAX_EVENTS: usize = 100_000;
const MAX_EVENT_ELEMENTS: usize = 65_536;
const MAX_TOTAL_ELEMENTS: usize = 1_048_576;
/// One registered stage and finite element/time windows; indices are array offsets.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeSelection {
    pub stage: TestPointId,
    pub elements: Range<usize>,
    /// A missing source/channel index never matches a requested time window.
    pub source_symbols: Option<Range<u64>>,
    pub channel_symbols: Option<Range<u64>>,
}
/// Application limits, independent of PHY limits. No default full-array capture.
#[derive(Clone, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProbeConfig {
    pub selections: Vec<ProbeSelection>,
    pub max_events: usize,
    pub max_elements_per_event: usize,
    pub max_total_elements: usize,
}
/// Explicit observation outcome; limits do not alter a previously computed stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeOutcome {
    Disabled,
    Filtered,
    BudgetExhausted,
    Dispatched { elements: usize },
}
/// Counts calls and selected elements, including callbacks that returned errors.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ProbeStats {
    pub dispatched_events: usize,
    pub dispatched_elements: usize,
    pub callback_errors: usize,
}
/// Caller-owned observation sink. Copies/serialization must respect selected limits.
pub trait ProbeObserver {
    /// Receives an immutable borrowed view; no internal scientific state is supplied.
    /// # Arguments
    /// * `event` - Read-only metadata and selected data valid only during this call.
    /// # Returns
    /// Unit after consuming the event or explicitly copying desired data.
    /// # Errors
    /// Returns a caller-defined observer failure; dispatch budget is still consumed.
    fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError>;
}
/// Job-owned selector and finite budgets; independent of DSP and RNG state.
pub struct TestPointProbe {
    config: Option<ProbeConfig>,
    phase: Option<super::RunPhase>,
    stats: ProbeStats,
    stage_limits: BTreeMap<TestPointId, (usize, usize)>,
}
impl TestPointProbe {
    /// Creates a zero-callback observation path without validating unused metadata.
    /// # Returns
    /// Disabled probe; emit returns Disabled and never invokes the observer.
    pub fn disabled() -> Self {
        Self {
            config: None,
            phase: None,
            stats: ProbeStats::default(),
            stage_limits: BTreeMap::new(),
        }
    }
    /// Validates finite windows and budgets for one observation job.
    /// # Arguments
    /// * `config` -1..32 unique stages; at most100000 events,65536 elements/event
    ///   and1048576 selected elements/job. Every limit must be positive.
    /// # Returns
    /// Owned selector with empty dispatch counters. No output buffers are retained.
    /// # Errors
    /// Rejects duplicates, empty/oversized windows or invalid application limits.
    pub fn new(config: ProbeConfig) -> Result<Self, SimulationError> {
        if config.selections.is_empty()
            || config.selections.len() > MAX_SELECTIONS
            || !(1..=MAX_EVENTS).contains(&config.max_events)
            || !(1..=MAX_EVENT_ELEMENTS).contains(&config.max_elements_per_event)
            || !(1..=MAX_TOTAL_ELEMENTS).contains(&config.max_total_elements)
        {
            return Err(invalid("Invalid bounded probe configuration."));
        }
        let mut stages = BTreeSet::new();
        for selection in &config.selections {
            if !stages.insert(selection.stage)
                || selection.elements.start >= selection.elements.end
                || selection.elements.end - selection.elements.start > config.max_elements_per_event
                || selection
                    .source_symbols
                    .as_ref()
                    .is_some_and(|range| range.start >= range.end)
                || selection
                    .channel_symbols
                    .as_ref()
                    .is_some_and(|range| range.start >= range.end)
            {
                return Err(invalid(
                    "Probe stages must be unique with finite nonempty windows.",
                ));
            }
        }
        Ok(Self {
            config: Some(config),
            phase: None,
            stats: ProbeStats::default(),
            stage_limits: BTreeMap::new(),
        })
    }
    /// Limits one selected stage so early sample streams cannot exhaust later-stage budgets.
    /// # Arguments
    /// * `stage` - Already configured producer identity.
    /// * `events` - Positive per-stage event cap, no larger than100000.
    /// # Returns
    /// Unit; whole events above this cap are filtered without spending the global budget.
    /// # Errors
    /// Rejects an unselected stage, invalid cap or changes after dispatch has started.
    pub fn limit_stage_events(
        &mut self,
        stage: TestPointId,
        events: usize,
    ) -> Result<(), SimulationError> {
        if !(1..=MAX_EVENTS).contains(&events)
            || self.stats.dispatched_events != 0
            || self.config.as_ref().is_none_or(|config| {
                !config
                    .selections
                    .iter()
                    .any(|selection| selection.stage == stage)
            })
        {
            return Err(invalid(
                "Stage event limits require a selected stage before dispatch.",
            ));
        }
        self.stage_limits.insert(stage, (events, 0));
        Ok(())
    }
    /// Filters by run phase before consuming any observation budget.
    /// # Arguments
    /// * `phase` - Exact warmup/observation/drain selection; None admits all phases.
    /// # Returns
    /// Unit; already consumed budgets and scientific state are unchanged.
    pub fn set_phase_filter(&mut self, phase: Option<super::RunPhase>) {
        self.phase = phase;
    }
    /// Returns counters without resetting the selector or budgets.
    /// # Returns
    /// Small owned copy; callback failures remain visible.
    pub fn stats(&self) -> ProbeStats {
        self.stats
    }
    /// Dispatches one selected immutable window of an already computed result.
    /// # Arguments
    /// * `stage` - Registered producer identity; pilot or PLC label stage.
    /// * `context` - Run/time/reference metadata, validated before a callback.
    /// * `domain` - Scientific coordinate domain; selection uses element offsets.
    /// * `data` - Borrowed stage values, never modified or retained here.
    /// * `observer` - Caller sink; receives no mutable engine/RNG state.
    /// # Returns
    /// Disabled, filtered, exhausted or dispatched outcome. Element ranges are
    /// intersected with the available array; empty intersections are filtered.
    /// Budgets skip whole events, never silently truncate a selected window.
    /// # Errors
    /// Invalid selected context or observer failure. Failed callbacks consume budget;
    /// no retry occurs. The caller still owns the unchanged scientific stage result.
    pub fn emit(
        &mut self,
        stage: TestPointId,
        context: &ProbeContext,
        domain: SignalDomain,
        data: ProbeData<'_>,
        observer: &mut impl ProbeObserver,
    ) -> Result<ProbeOutcome, SimulationError> {
        if self.phase.is_some_and(|phase| phase != context.phase) {
            return Ok(ProbeOutcome::Filtered);
        }
        if self
            .stage_limits
            .get(&stage)
            .is_some_and(|(limit, count)| count >= limit)
        {
            return Ok(ProbeOutcome::Filtered);
        }
        let Some(config) = &self.config else {
            return Ok(ProbeOutcome::Disabled);
        };
        let Some(selection) = config
            .selections
            .iter()
            .find(|selection| selection.stage == stage)
        else {
            return Ok(ProbeOutcome::Filtered);
        };
        if !matches_time(&selection.source_symbols, context.source_symbol_index)
            || !matches_time(&selection.channel_symbols, context.channel_symbol_index)
        {
            return Ok(ProbeOutcome::Filtered);
        }
        let start = selection.elements.start;
        let end = selection.elements.end.min(data.len());
        if start >= end {
            return Ok(ProbeOutcome::Filtered);
        }
        context.validate()?;
        let count = end - start;
        if self.stats.dispatched_events >= config.max_events
            || count > config.max_total_elements - self.stats.dispatched_elements
        {
            log::debug!("probe-budget-exhausted stage={stage:?} dispatched-events={} dispatched-elements={}",
                self.stats.dispatched_events, self.stats.dispatched_elements);
            return Ok(ProbeOutcome::BudgetExhausted);
        }
        let selected = data.window(start, end);
        validate_data(stage, context, domain, selected)?;
        let event = ProbeEvent {
            stage,
            direction: stage.direction(),
            context,
            domain,
            units: if stage == TestPointId::RxChannelEstimate {
                super::ProbeUnits::DimensionlessGain
            } else {
                data.units()
            },
            element_start: start,
            total_elements: data.len(),
            shape: [count],
            data: selected,
        };
        self.stats.dispatched_events += 1;
        self.stats.dispatched_elements += count;
        if let Some((_, dispatched)) = self.stage_limits.get_mut(&stage) {
            *dispatched += 1;
        }
        if let Err(error) = observer.observe(&event) {
            self.stats.callback_errors += 1;
            log::debug!(
                "probe-callback-failed stage={stage:?} error-code={}",
                error.code
            );
            return Err(error);
        }
        Ok(ProbeOutcome::Dispatched { elements: count })
    }
}
fn matches_time(window: &Option<Range<u64>>, index: Option<u64>) -> bool {
    window
        .as_ref()
        .is_none_or(|range| index.is_some_and(|value| range.contains(&value)))
}

fn validate_data(
    stage: TestPointId,
    context: &ProbeContext,
    domain: SignalDomain,
    data: ProbeData<'_>,
) -> Result<(), SimulationError> {
    use super::SignalReference;
    let valid = match (stage, domain, data) {
        (TestPointId::SourcePayload, SignalDomain::CodewordBit, ProbeData::Bits(bits)) => {
            context.reference == SignalReference::NotApplicable && bits.iter().all(|bit| *bit <= 1)
        }
        (TestPointId::LegacyNoiseOutput, SignalDomain::CodewordCell, ProbeData::Iq(iq)) => {
            matches!(
                context.reference,
                SignalReference::OtherSubcarrierRms { .. }
            ) && iq.iter().flatten().all(|value| value.is_finite())
        }

        (TestPointId::PilotSequence, SignalDomain::PhysicalFftBin, ProbeData::Bits(bits)) => {
            context.reference == SignalReference::NotApplicable && bits.iter().all(|bit| *bit <= 1)
        }
        (TestPointId::PilotPower, SignalDomain::Parameter, ProbeData::Power(power)) => {
            context.reference
                == SignalReference::OtherSubcarrierRms {
                    rms: power.reference_rms(),
                }
        }
        (
            TestPointId::ContinuousPilots | TestPointId::ScatteredPilots,
            SignalDomain::PhysicalFftBin,
            ProbeData::Pilots(pilots),
        ) => {
            matches!(
                context.reference,
                SignalReference::OtherSubcarrierRms { .. }
            ) && pilots.iter().all(|pilot| {
                pilot.sequence_bit <= 1 && pilot.iq.iter().all(|value| value.is_finite())
            })
        }
        (TestPointId::PlcCodedNibbles, SignalDomain::CodewordCell, ProbeData::Labels(labels))
        | (
            TestPointId::PlcInterleaved | TestPointId::PlcRandomized,
            SignalDomain::PlcCell,
            ProbeData::Labels(labels),
        ) => {
            context.reference == SignalReference::NotApplicable
                && context.plc_grid.is_some()
                && labels.iter().all(|label| {
                    *label < 1 << crate::transmission::plc::geometry::PLC_BITS_PER_CELL
                })
        }
        (TestPointId::PlcMessage, SignalDomain::MessageByte, ProbeData::Bytes(_)) => {
            context.reference == SignalReference::NotApplicable
        }
        (TestPointId::PlcCodedBits, SignalDomain::CodewordBit, ProbeData::Bits(bits)) => {
            context.reference == SignalReference::NotApplicable && bits.iter().all(|bit| *bit <= 1)
        }
        (
            TestPointId::PlcPreamble | TestPointId::PlcIq,
            SignalDomain::PlcCell,
            ProbeData::Iq(iq),
        ) => {
            context.plc_grid.is_some()
                && matches!(
                    context.reference,
                    SignalReference::OtherSubcarrierRms { .. }
                )
                && iq.iter().flatten().all(|value| value.is_finite())
        }
        (TestPointId::PhysicalBins, SignalDomain::PhysicalFftBin, ProbeData::OptionalIq(iq)) => {
            context.physical_grid.is_some()
                && matches!(
                    context.reference,
                    SignalReference::OtherSubcarrierRms { .. }
                )
                && iq.iter().flatten().flatten().all(|value| value.is_finite())
        }
        (
            TestPointId::TxIfft
            | TestPointId::TxPrefix
            | TestPointId::TxWindow
            | TestPointId::TxOverlap
            | TestPointId::ChannelInput
            | TestPointId::ChannelOutput
            | TestPointId::RxFramed
            | TestPointId::RxUseful,
            SignalDomain::TimeSample,
            ProbeData::Iq(iq),
        ) => {
            context.waveform_grid.is_some()
                && context.sample_start_index.is_some()
                && iq.iter().flatten().all(|value| value.is_finite())
        }
        (
            TestPointId::RxFft | TestPointId::RxEqualized | TestPointId::RxChannelEstimate,
            SignalDomain::PhysicalFftBin,
            ProbeData::Iq(iq),
        ) => {
            context.waveform_grid.is_some()
                && context.physical_grid.is_some()
                && iq.iter().flatten().all(|value| value.is_finite())
        }
        (TestPointId::RxPlc, SignalDomain::PlcCell, ProbeData::Iq(iq)) => {
            context.waveform_grid.is_some()
                && context.plc_grid.is_some()
                && iq.iter().flatten().all(|value| value.is_finite())
        }
        (
            TestPointId::RxPilots,
            SignalDomain::PhysicalFftBin,
            ProbeData::ReceivedPilots(pilots),
        ) => {
            context.waveform_grid.is_some()
                && context.physical_grid.is_some()
                && pilots
                    .iter()
                    .all(|pilot| pilot.iq.iter().all(|value| value.is_finite()))
        }
        (TestPointId::RxReduced, SignalDomain::ReducedCarrier, ProbeData::OptionalIq(iq)) => {
            context.waveform_grid.is_some()
                && context.physical_grid.is_some()
                && iq.iter().flatten().flatten().all(|value| value.is_finite())
        }
        (
            TestPointId::PayloadCoded
            | TestPointId::PayloadDecoded
            | TestPointId::PayloadInterleavedBits,
            SignalDomain::CodewordBit,
            ProbeData::Bits(bits),
        ) => context.cell_grid.is_some() && bits.iter().all(|bit| *bit <= 1),
        (TestPointId::PayloadLabels, SignalDomain::CodewordCell, ProbeData::QamLabels(labels)) => {
            context.cell_grid.is_some() && labels.iter().all(|label| *label < 256)
        }
        (
            TestPointId::PayloadMapped
            | TestPointId::PayloadInterleaved
            | TestPointId::PayloadRestored,
            SignalDomain::ReducedCarrier,
            ProbeData::TransportCells(cells),
        ) => {
            context.cell_grid.is_some()
                && cells.iter().flatten().all(|cell| match cell {
                    crate::transmission::symbols::TransportCell::PilotPlaceholder => true,
                    crate::transmission::symbols::TransportCell::Sample(point) => {
                        point.iter().all(|value| value.is_finite())
                    }
                })
        }
        (TestPointId::NcpSoft, SignalDomain::CellBit, ProbeData::NcpSoft(cells)) => {
            context.cell_grid.as_ref().is_some_and(|grid| {
                cells.iter().all(|cell| {
                    cell.data_position < grid.settings.cells()
                        && cell.llrs.len() == grid.ncp_modulation.order().ilog2() as usize
                        && cell.llrs.iter().all(|value| value.is_finite())
                })
            })
        }
        (TestPointId::DataSoft, SignalDomain::CellBit, ProbeData::DataSoft(cells)) => {
            context.cell_grid.as_ref().is_some_and(|grid| {
                cells.iter().all(|cell| {
                    cell.data_position < grid.settings.cells()
                        && cell.llrs.iter().all(|value| value.is_finite())
                })
            })
        }
        (
            TestPointId::NcpAccepted,
            SignalDomain::ControlMessage,
            ProbeData::NcpMessages(messages),
        ) => {
            context.cell_grid.is_some() && messages.iter().all(|message| message.to_bytes().is_ok())
        }
        (TestPointId::PayloadLdpcInput, SignalDomain::CodewordBit, ProbeData::Llrs(llrs)) => {
            context.cell_grid.is_some() && llrs.iter().all(|value| value.is_finite())
        }
        _ => false,
    };
    if !valid {
        return Err(invalid(
            "Probe stage, domain, reference or payload is inconsistent.",
        ));
    }
    Ok(())
}
