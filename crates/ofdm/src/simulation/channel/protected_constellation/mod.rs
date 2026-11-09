/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
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
