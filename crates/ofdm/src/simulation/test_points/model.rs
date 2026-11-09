/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrowed typed stage observations; registered IDs describe implemented producers.
use crate::transmission::{
    plc::{PlcFrameGeometry, PlcNibble},
    SubcarrierSpacing,
};
use crate::{
    error::invalid,
    signal_processing::pilots::{PilotPower, PilotValue},
    types::{IqPair, ProfileId, PLC_FRAME_SYMBOLS},
    LabError,
};
use serde::Serialize;
const MAX_PROBE_ID_BYTES: usize = 128;
/// Registered observation stage; additional stages can be added without a closed API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[non_exhaustive]
pub enum TestPointId {
    #[serde(rename = "tx.pilots.sequence")]
    PilotSequence,
    #[serde(rename = "tx.pilots.continuous")]
    ContinuousPilots,
    #[serde(rename = "tx.pilots.scattered")]
    ScatteredPilots,
    #[serde(rename = "tx.pilots.power")]
    PilotPower,
    #[serde(rename = "tx.plc.coded_nibbles")]
    PlcCodedNibbles,
    #[serde(rename = "tx.plc.interleaved")]
    PlcInterleaved,
    #[serde(rename = "tx.plc.message")]
    PlcMessage,
    #[serde(rename = "tx.plc.coded")]
    PlcCodedBits,
    #[serde(rename = "tx.plc.randomized")]
    PlcRandomized,
    #[serde(rename = "tx.plc.preamble")]
    PlcPreamble,
    #[serde(rename = "tx.plc.iq")]
    PlcIq,
    #[serde(rename = "tx.frame.bins")]
    PhysicalBins,
    #[serde(rename = "tx.ifft.samples")]
    TxIfft,
    #[serde(rename = "tx.cp.samples")]
    TxPrefix,
    #[serde(rename = "tx.window.samples")]
    TxWindow,
    #[serde(rename = "tx.overlap_add.samples")]
    TxOverlap,
    #[serde(rename = "channel.input")]
    ChannelInput,
    #[serde(rename = "channel.output")]
    ChannelOutput,
    #[serde(rename = "rx.framed.samples")]
    RxFramed,
    #[serde(rename = "rx.cp.samples")]
    RxUseful,
    #[serde(rename = "rx.fft.bins")]
    RxFft,
    #[serde(rename = "rx.equalized.iq")]
    RxEqualized,
    #[serde(rename = "rx.plc.iq")]
    RxPlc,
    #[serde(rename = "rx.pilots.iq")]
    RxPilots,
    #[serde(rename = "rx.reduced.iq")]
    RxReduced,
}
impl TestPointId {
    /// Returns the direction registered for this stage.
    /// # Returns
    /// Transmit or receive according to the registered producer; channel output is RX input.
    pub(super) fn direction(self) -> SignalDirection {
        match self {
            Self::PilotSequence
            | Self::ContinuousPilots
            | Self::ScatteredPilots
            | Self::PilotPower
            | Self::PlcCodedNibbles
            | Self::PlcInterleaved
            | Self::PlcMessage
            | Self::PlcCodedBits
            | Self::PlcRandomized
            | Self::PlcPreamble
            | Self::PlcIq
            | Self::PhysicalBins
            | Self::TxIfft
            | Self::TxPrefix
            | Self::TxWindow
            | Self::TxOverlap
            | Self::ChannelInput => SignalDirection::Transmit,
            Self::ChannelOutput
            | Self::RxFramed
            | Self::RxUseful
            | Self::RxFft
            | Self::RxEqualized
            | Self::RxPlc
            | Self::RxPilots
            | Self::RxReduced => SignalDirection::Receive,
        }
    }
}
/// Transmission direction, distinct from source/channel time identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalDirection {
    Transmit,
    Receive,
}
/// Run interval; startup/drain observations are never automatically payload truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RunPhase {
    Warmup,
    Observation,
    Drain,
}
/// Index domain of observed values; sparse pilot records retain their own bin IDs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalDomain {
    PhysicalFftBin,
    ReducedCarrier,
    CodewordBit,
    MessageByte,
    CodewordCell,
    PlcCell,
    TimeSample,
    Parameter,
}
/// Explicit amplitude reference, never an inferred absolute RF power level.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SignalReference {
    NotApplicable,
    OtherSubcarrierRms { rms: f64 },
}
/// Units of the payload; typed records carry separately named amplitude/power fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeUnits {
    Binary,
    Octet,
    NormalizedIq,
    TypedRecord,
    QamLabel,
}
/// Full PLC block geometry; event element_start/shape identify a selected flat window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PlcProbeGrid {
    spacing: SubcarrierSpacing,
    symbols: usize,
    subcarriers_per_symbol: usize,
}
impl PlcProbeGrid {
    /// Constructs metadata for an explicit PLC symbol window.
    /// # Arguments
    /// * `spacing` - Fixed local carrier geometry.
    /// * `symbols` - Complete source grid height,1..128, before element selection.
    /// # Returns
    /// Immutable full-grid coordinates for preamble, codeword or frame observations.
    /// # Errors
    /// Rejects empty or greater-than-frame grids.
    pub fn for_symbols(spacing: SubcarrierSpacing, symbols: usize) -> Result<Self, LabError> {
        if !(1..=PLC_FRAME_SYMBOLS as usize).contains(&symbols) {
            return Err(invalid("PLC observation grid requires1..128 symbols."));
        }
        Ok(Self {
            spacing,
            symbols,
            subcarriers_per_symbol: PlcFrameGeometry::new(spacing).plc_subcarriers,
        })
    }
    /// Derives unambiguous PLC grid dimensions from known fixed numerology.
    /// # Arguments
    /// * `spacing` -25/50kHz spacing, never inferred from selected window length.
    /// # Returns
    /// Full6x16 or12x8 grid metadata, even when only a few labels are observed.
    pub fn new(spacing: SubcarrierSpacing) -> Self {
        let geometry = PlcFrameGeometry::new(spacing);
        Self {
            spacing,
            symbols: geometry.data_symbols_per_codeword,
            subcarriers_per_symbol: geometry.plc_subcarriers,
        }
    }
}
/// Full physical frequency coordinates retained on selected observation windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PhysicalProbeGrid {
    pub spacing: SubcarrierSpacing,
    pub zero_frequency_hz: crate::types::FrequencyHz,
}
/// Full waveform numerology retained even when only a few samples/bins are selected.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct WaveformProbeGrid {
    pub geometry: crate::signal_processing::waveform::WaveformGeometry,
    pub sample_rate_hz: crate::types::FrequencyHz,
}
/// Caller-assigned run/config identities and optional stage-specific time metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ProbeContext {
    pub run_id: String,
    /// Identifier supplied by the runner, not a computed/authenticated config hash.
    pub configuration_id: String,
    pub phase: RunPhase,
    pub source_symbol_index: Option<u64>,
    pub channel_symbol_index: Option<u64>,
    pub sample_start_index: Option<u64>,
    pub codeword_index: Option<u64>,
    pub profile_id: Option<ProfileId>,
    pub plc_phase: Option<u8>,
    pub reference: SignalReference,
    pub plc_grid: Option<PlcProbeGrid>,
    pub physical_grid: Option<PhysicalProbeGrid>,
    pub waveform_grid: Option<WaveformProbeGrid>,
}
impl ProbeContext {
    /// Creates run metadata with explicitly unknown time/profile fields.
    /// # Arguments
    /// * `run_id` - Nonempty caller identity, at most128 UTF-8 bytes.
    /// * `configuration_id` - Nonempty configuration identity, at most128 bytes.
    /// * `phase` - Warm-up, observation or drain interval.
    /// # Returns
    /// Owned metadata, initially without an I/Q normalization reference.
    /// # Errors
    /// Rejects empty or oversized identifiers.
    pub fn new(run_id: &str, configuration_id: &str, phase: RunPhase) -> Result<Self, LabError> {
        if run_id.is_empty()
            || run_id.len() > MAX_PROBE_ID_BYTES
            || configuration_id.is_empty()
            || configuration_id.len() > MAX_PROBE_ID_BYTES
        {
            return Err(invalid("Probe identities require1..128 bytes."));
        }
        let result = Self {
            run_id: run_id.into(),
            configuration_id: configuration_id.into(),
            phase,
            source_symbol_index: None,
            channel_symbol_index: None,
            sample_start_index: None,
            codeword_index: None,
            profile_id: None,
            plc_phase: None,
            reference: SignalReference::NotApplicable,
            plc_grid: None,
            physical_grid: None,
            waveform_grid: None,
        };
        result.validate()?;
        Ok(result)
    }
    /// Validates metadata after caller field edits, before an enabled observation.
    /// # Returns
    /// Unit for bounded identifiers, valid PLC phase and finite RMS reference.
    /// # Errors
    /// Rejects invalid identifiers, phase or amplitude reference.
    pub(in crate::simulation) fn validate(&self) -> Result<(), LabError> {
        if self.run_id.is_empty()
            || self.run_id.len() > MAX_PROBE_ID_BYTES
            || self.configuration_id.is_empty()
            || self.configuration_id.len() > MAX_PROBE_ID_BYTES
            || self
                .plc_phase
                .is_some_and(|phase| u64::from(phase) >= PLC_FRAME_SYMBOLS)
        {
            return Err(invalid("Invalid probe identities or PLC phase."));
        }
        if let SignalReference::OtherSubcarrierRms { rms } = self.reference {
            if !rms.is_finite() || rms <= 0.0 {
                return Err(invalid("Probe RMS reference must be positive and finite."));
            }
        }
        if self.waveform_grid.as_ref().is_some_and(|grid| {
            grid.sample_rate_hz != crate::transmission::settings::SAMPLE_RATE_HZ
        }) {
            return Err(invalid("Waveform probes require the fixed sample clock."));
        }
        Ok(())
    }
}
/// Borrowed1-D data, available only for the duration of the observer call.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "kind", content = "values", rename_all = "snake_case")]
pub enum ProbeData<'event> {
    Bits(&'event [u8]),
    Pilots(&'event [PilotValue]),
    Power(&'event PilotPower),
    Labels(&'event [PlcNibble]),
    Bytes(&'event [u8]),
    Iq(&'event [IqPair]),
    OptionalIq(&'event [Option<IqPair>]),
    ReceivedPilots(&'event [crate::signal_processing::receiver::extraction::ReceivedPilot]),
}
impl ProbeData<'_> {
    /// Returns the number of selectable values, counting a power record as one.
    /// # Returns
    /// Slice length or1; no payload is copied.
    pub fn len(self) -> usize {
        match self {
            Self::Bits(data) => data.len(),
            Self::Pilots(data) => data.len(),
            Self::ReceivedPilots(data) => data.len(),
            Self::Power(_) => 1,
            Self::Labels(data) => data.len(),
            Self::Bytes(data) => data.len(),
            Self::Iq(data) => data.len(),
            Self::OptionalIq(data) => data.len(),
        }
    }
    /// Reports whether this event contains no selectable values.
    /// # Returns
    /// True for an empty slice; a power record is never empty.
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    /// Returns the data representation's explicit unit tag.
    /// # Returns
    /// Binary values, QAM labels, normalized I/Q records or a typed power record.
    pub(super) fn units(self) -> ProbeUnits {
        match self {
            Self::Bits(_) => ProbeUnits::Binary,
            Self::Pilots(_) | Self::ReceivedPilots(_) => ProbeUnits::NormalizedIq,
            Self::Power(_) => ProbeUnits::TypedRecord,
            Self::Labels(_) => ProbeUnits::QamLabel,
            Self::Bytes(_) => ProbeUnits::Octet,
            Self::Iq(_) | Self::OptionalIq(_) => ProbeUnits::NormalizedIq,
        }
    }
    /// Borrows the selected subrange after the dispatcher has intersected bounds.
    /// # Arguments
    /// * `start` - Inclusive validated element offset.
    /// * `end` - Exclusive validated element offset.
    /// # Returns
    /// Borrowed window; the scalar power record occupies one element.
    pub(super) fn window(self, start: usize, end: usize) -> Self {
        match self {
            Self::Bits(data) => Self::Bits(&data[start..end]),
            Self::Pilots(data) => Self::Pilots(&data[start..end]),
            Self::ReceivedPilots(data) => Self::ReceivedPilots(&data[start..end]),
            Self::Power(data) => Self::Power(data),
            Self::Labels(data) => Self::Labels(&data[start..end]),
            Self::Bytes(data) => Self::Bytes(&data[start..end]),
            Self::Iq(data) => Self::Iq(&data[start..end]),
            Self::OptionalIq(data) => Self::OptionalIq(&data[start..end]),
        }
    }
}
/// Read-only stage event; cloning this structure does not make its buffers owned.
#[derive(Debug, Serialize)]
pub struct ProbeEvent<'event> {
    pub stage: TestPointId,
    pub direction: SignalDirection,
    pub context: &'event ProbeContext,
    pub domain: SignalDomain,
    pub units: ProbeUnits,
    pub element_start: usize,
    pub total_elements: usize,
    /// One-dimensional record shape; pilot records each contain their own I/Q pair.
    pub shape: [usize; 1],
    pub data: ProbeData<'event>,
}
