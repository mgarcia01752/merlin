/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded acquisition-safe pilot/PLC receive-pair observations.
use crate::{
    error::invalid,
    simulation::{channel::constellation::ConstellationStatus, waveform::ReceivedSymbol},
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        plc::geometry::PLC_DATA_SYMBOLS,
    },
    types::{FrequencyHz, IqPair, SubcarrierIdx, PLC_FRAME_SYMBOLS},
    SimulationError,
};
use merlin_api::{retention_support::RetainedPayload, RetainedFootprint};
use serde::{Deserialize, Serialize};
/// Combined cap is four streams times this per-stream cap; adapters also bound bytes.
pub const MAX_PAIRS_PER_STREAM: usize = 8192;
/// Finite opt-in capture of all four protected streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProtectedCaptureSelection {
    pub pairs_per_stream: usize,
    pub first_channel_symbol: u64,
    pub end_channel_symbol_exclusive: u64,
}
impl Default for ProtectedCaptureSelection {
    /// Selects a finite2048-pair target per stream over the first1024 symbols.
    /// # Returns
    /// An editable selection; default does not enable capture by itself.
    fn default() -> Self {
        Self {
            pairs_per_stream: 2048,
            first_channel_symbol: 0,
            end_channel_symbol_exclusive: 1024,
        }
    }
}
impl ProtectedCaptureSelection {
    /// Validates per-stream and combined memory/time bounds before any allocation.
    /// # Returns
    /// Unit for four bounded streams and a finite physical-symbol window.
    /// # Errors
    /// Rejects zero/excess counts, reversed windows and windows above one million symbols.
    pub fn validate(&self) -> Result<(), SimulationError> {
        if !(1..=MAX_PAIRS_PER_STREAM).contains(&self.pairs_per_stream)
            || self.first_channel_symbol >= self.end_channel_symbol_exclusive
            || self.end_channel_symbol_exclusive - self.first_channel_symbol > 1_000_000
        {
            return Err(invalid("Invalid protected constellation count/window."));
        }
        Ok(())
    }
}
/// One actual protected carrier, before/after correction, never payload-filtered.
#[derive(Debug, Clone, Serialize)]
pub struct ProtectedPair {
    pub physical_bin: SubcarrierIdx,
    pub frequency_hz: FrequencyHz,
    pub channel_symbol_index: u64,
    pub plc_phase: u8,
    pub before_equalization: IqPair,
    pub after_equalization: IqPair,
    /// Physical TX diagnostic reference, separate from all receiver operations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_iq: Option<IqPair>,
    pub receiver_estimate_ready: bool,
    pub used_in_current_fit: bool,
    pub prior_channel_prediction: Option<IqPair>,
    pub pilot_reference: Option<IqPair>,
}
/// Actual count/status per modulation/role; no sample repetition to fill a target.
#[derive(Debug, Clone, Serialize)]
pub struct ProtectedStream {
    pub modulation: &'static str,
    pub status: ConstellationStatus,
    pub collected_pairs: usize,
    pub pairs: Vec<ProtectedPair>,
}
/// Shared Rust/CLI result; includes acquisition and any later decoding failures.
#[derive(Debug, Clone, Serialize)]
pub struct ProtectedConstellation {
    #[serde(skip)]
    pilot_sequence: Option<Vec<u8>>,
    pub schema_version: u16,
    pub selection: ProtectedCaptureSelection,
    pub receiver_mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    pub continuous_pilots: ProtectedStream,
    pub scattered_pilots: ProtectedStream,
    pub plc_data: ProtectedStream,
    pub plc_preamble: ProtectedStream,
}
impl ProtectedConstellation {
    /// Creates four empty bounded streams after validating the selection.
    /// # Arguments
    /// * `selection` - Per-stream target and physical observation window.
    /// * `receiver_mode` - Explicit supplied-H or pilot-estimated correction.
    /// # Returns
    /// Unavailable streams until actual receive observations arrive.
    /// # Errors
    /// Rejects invalid selection bounds before allocation.
    pub fn new(
        selection: ProtectedCaptureSelection,
        receiver_mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    ) -> Result<Self, SimulationError> {
        selection.validate()?;
        let stream = |modulation| ProtectedStream {
            modulation,
            status: ConstellationStatus::Unavailable,
            collected_pairs: 0,
            pairs: Vec::new(),
        };
        Ok(Self {
            pilot_sequence: None,
            schema_version: 1,
            selection,
            receiver_mode,
            continuous_pilots: stream("boosted_bpsk"),
            scattered_pilots: stream("boosted_bpsk"),
            plc_data: stream("qam16"),
            plc_preamble: stream("bpsk"),
        })
    }
    /// Retains same-bin/time pairs independently of PLC CRC or payload readiness.
    /// # Arguments
    /// * `received` - Completed receiver stages, unchanged by observation.
    /// * `layout` - Receiver bootstrap carrier classification.
    /// # Returns
    /// Unit; full/outside streams stop retaining without affecting science.
    /// # Errors
    /// Rejects inconsistent or nonfinite receive grids before retaining any samples.
    pub fn observe(
        &mut self,
        received: &ReceivedSymbol,
        layout: &CarrierLayout,
    ) -> Result<(), SimulationError> {
        let size = layout.static_roles().len();
        if received.bins.len() != size
            || received.equalized.len() != size
            || received
                .bins
                .iter()
                .chain(&received.equalized)
                .flatten()
                .any(|value| !value.is_finite())
        {
            return Err(invalid(
                "Protected capture requires matching finite physical receiver grids.",
            ));
        }
        let symbol = received.symbol_index;
        if !(self.selection.first_channel_symbol..self.selection.end_channel_symbol_exclusive)
            .contains(&symbol)
        {
            return Ok(());
        }
        let sequence = self.pilot_sequence.get_or_insert_with(|| {
            crate::signal_processing::pilots::PilotSequence::new(layout.subcarrier_spacing())
                .bits()
                .to_vec()
        });
        let roles = layout.symbol_roles(symbol);
        for (bin, role) in roles.iter().enumerate() {
            let stream = match role {
                PhysicalCarrierRole::ContinuousPilot => &mut self.continuous_pilots,
                PhysicalCarrierRole::ScatteredPilot => &mut self.scattered_pilots,
                PhysicalCarrierRole::Plc
                    if symbol % PLC_FRAME_SYMBOLS < PLC_DATA_SYMBOLS as u64 =>
                {
                    &mut self.plc_data
                }
                PhysicalCarrierRole::Plc => &mut self.plc_preamble,
                _ => continue,
            };
            if stream.pairs.len() >= self.selection.pairs_per_stream {
                continue;
            }
            let pilot = received.channel_estimate.as_ref().and_then(|estimate| {
                estimate
                    .pilots
                    .iter()
                    .find(|pilot| usize::from(pilot.bin) == bin)
            });
            stream.pairs.push(ProtectedPair {
                physical_bin: bin as u16,
                frequency_hz: layout.frequency_hz(bin as u16).ok_or_else(|| {
                    invalid("Protected capture bin exceeds validated physical grid.")
                })?,
                channel_symbol_index: symbol,
                plc_phase: (symbol % PLC_FRAME_SYMBOLS) as u8,
                before_equalization: received.bins[bin],
                after_equalization: received.equalized[bin],
                reference_iq: None,
                receiver_estimate_ready: received
                    .channel_estimate
                    .as_ref()
                    .is_none_or(|estimate| estimate.ready),
                used_in_current_fit: pilot.is_some(),
                prior_channel_prediction: pilot.and_then(|pilot| pilot.prior_prediction),
                pilot_reference: pilot.map(|pilot| pilot.reference).or_else(|| {
                    matches!(
                        role,
                        PhysicalCarrierRole::ContinuousPilot | PhysicalCarrierRole::ScatteredPilot
                    )
                    .then_some([2.0 * (1.0 - 2.0 * f64::from(sequence[bin])), 0.0])
                }),
            });
            stream.collected_pairs = stream.pairs.len();
            stream.status = if stream.pairs.len() == self.selection.pairs_per_stream {
                ConstellationStatus::Complete
            } else {
                ConstellationStatus::Partial
            };
        }
        Ok(())
    }
}

impl ProtectedConstellation {
    /// Moves the existing final observations into exact retained capacities.
    /// # Returns
    /// Unit preserving all scientific values and the lazily retained pilot cache.
    #[cfg(any(feature = "pnm-input", test))]
    pub(crate) fn normalize_result_storage(&mut self) {
        let Self {
            pilot_sequence,
            schema_version: _,
            selection: _,
            receiver_mode: _,
            continuous_pilots,
            scattered_pilots,
            plc_data,
            plc_preamble,
        } = self;
        if let Some(values) = pilot_sequence {
            *values = std::mem::take(values).into_boxed_slice().into_vec();
        }
        for stream in [continuous_pilots, scattered_pilots, plc_data, plc_preamble] {
            let ProtectedStream {
                modulation: _,
                status: _,
                collected_pairs: _,
                pairs,
            } = stream;
            *pairs = std::mem::take(pairs).into_boxed_slice().into_vec();
        }
    }
}

impl RetainedPayload for ProtectedCaptureSelection {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            pairs_per_stream,
            first_channel_symbol,
            end_channel_symbol_exclusive,
        } = self;
        merlin_api::retention_support::sum([
            pairs_per_stream.retained_children(),
            first_channel_symbol.retained_children(),
            end_channel_symbol_exclusive.retained_children(),
        ])
    }
}

impl RetainedPayload for ProtectedPair {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            physical_bin,
            frequency_hz,
            channel_symbol_index,
            plc_phase,
            before_equalization,
            after_equalization,
            reference_iq,
            receiver_estimate_ready,
            used_in_current_fit,
            prior_channel_prediction,
            pilot_reference,
        } = self;
        merlin_api::retention_support::sum([
            physical_bin.retained_children(),
            frequency_hz.retained_children(),
            channel_symbol_index.retained_children(),
            plc_phase.retained_children(),
            before_equalization.retained_children(),
            after_equalization.retained_children(),
            reference_iq.retained_children(),
            receiver_estimate_ready.retained_children(),
            used_in_current_fit.retained_children(),
            prior_channel_prediction.retained_children(),
            pilot_reference.retained_children(),
        ])
    }
}

impl RetainedPayload for ProtectedStream {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            modulation: _,
            status,
            collected_pairs,
            pairs,
        } = self;
        merlin_api::retention_support::sum([
            status.retained_children(),
            collected_pairs.retained_children(),
            pairs.retained_children(),
        ])
    }
}

impl RetainedPayload for ProtectedConstellation {
    /// Counts every owned child beyond the enclosing fixed value.
    ///
    /// # Returns
    /// Complete passive child metadata, or None on checked arithmetic overflow.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            pilot_sequence,
            schema_version,
            selection,
            receiver_mode,
            continuous_pilots,
            scattered_pilots,
            plc_data,
            plc_preamble,
        } = self;
        merlin_api::retention_support::sum([
            pilot_sequence.retained_children(),
            schema_version.retained_children(),
            selection.retained_children(),
            receiver_mode.retained_children(),
            continuous_pilots.retained_children(),
            scattered_pilots.retained_children(),
            plc_data.retained_children(),
            plc_preamble.retained_children(),
        ])
    }
}

#[cfg(test)]
mod result_retention_tests;

#[cfg(test)]
mod retained_footprint_tests {
    use super::{ProtectedCaptureSelection, ProtectedConstellation, ProtectedPair};
    use crate::signal_processing::receiver::channel_estimation::ReceiverMode;
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn skipped_pilot_sequence_and_empty_stream_capacity_are_counted_passively() {
        // Distinct unused capacities isolate the private cache and stream buffer.
        const PILOT_CACHE_SLOTS: usize = 17;
        const STREAM_PAIR_SLOTS: usize = 3;
        let mut report = ProtectedConstellation::new(
            ProtectedCaptureSelection::default(),
            ReceiverMode::CmChannelEstimated,
        )
        .unwrap();
        let serialized = serde_json::to_value(&report).unwrap();
        report.pilot_sequence = Some(Vec::with_capacity(PILOT_CACHE_SLOTS));
        report.continuous_pilots.pairs = Vec::with_capacity(STREAM_PAIR_SLOTS);
        let expected = size_of::<ProtectedConstellation>()
            + report.pilot_sequence.as_ref().unwrap().capacity()
            + report.continuous_pilots.pairs.capacity() * size_of::<ProtectedPair>();
        let footprint = retained_footprint(&report).unwrap();
        assert_eq!(footprint.payload_bytes, expected);
        assert_eq!(footprint.shared_roots, 0);
        assert_eq!(footprint.ordered_entries, 0);
        assert_eq!(retained_footprint(&report).unwrap(), footprint);
        assert_eq!(serde_json::to_value(&report).unwrap(), serialized);
    }
}
