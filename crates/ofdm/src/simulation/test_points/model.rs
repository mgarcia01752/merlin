/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrowed typed stage observations; registered IDs describe implemented producers.
use crate::{
    error::invalid,
    signal_processing::pilots::{PilotPower, PilotValue},
    types::{ProfileId, PLC_FRAME_SYMBOLS},
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
}
impl TestPointId {
    /// Returns the direction registered for this stage.
    /// # Returns
    /// Transmit for the current pilot stages; future receive IDs require a new arm.
    pub(super) fn direction(self) -> SignalDirection {
        match self {
            Self::PilotSequence
            | Self::ContinuousPilots
            | Self::ScatteredPilots
            | Self::PilotPower => SignalDirection::Transmit,
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
    NormalizedIq,
    TypedRecord,
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
}
impl ProbeData<'_> {
    /// Returns the number of selectable values, counting a power record as one.
    /// # Returns
    /// Slice length or1; no payload is copied.
    pub fn len(self) -> usize {
        match self {
            Self::Bits(data) => data.len(),
            Self::Pilots(data) => data.len(),
            Self::Power(_) => 1,
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
    /// Binary values, normalized I/Q records or a typed power record.
    pub(super) fn units(self) -> ProbeUnits {
        match self {
            Self::Bits(_) => ProbeUnits::Binary,
            Self::Pilots(_) => ProbeUnits::NormalizedIq,
            Self::Power(_) => ProbeUnits::TypedRecord,
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
            Self::Power(data) => Self::Power(data),
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
