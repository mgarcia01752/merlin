/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded diagnostic inverse transport and receive-time ordering, never receiver inputs.
use super::{
    ConstellationPair, ConstellationResult, ConstellationSelection, ConstellationStatus,
    NcpConstellationPair,
};
use crate::{
    error::invalid,
    signal_processing::interleaving::{InterleaverDirection, SymbolInterleaver},
    simulation::{payload::PayloadReceiveSymbol, waveform::ReceivedSymbol},
    transmission::{
        layout::CarrierLayout, plc::receiver_configuration::ReceiverConfiguration,
        symbols::TransportCell,
    },
    types::{IqPair, SubcarrierIdx},
    SimulationError,
};
use std::collections::BTreeMap;
#[derive(Clone)]
struct Coordinate {
    channel_symbol: u64,
    bin: SubcarrierIdx,
    before: IqPair,
    after: IqPair,
}
struct CoordinateStream {
    origin: u64,
    end_source: u64,
    bins: Vec<SubcarrierIdx>,
    layout: CarrierLayout,
    inverse: SymbolInterleaver<Coordinate>,
    max_pending: usize,
}
pub(crate) struct ConstellationCollector {
    result: ConstellationResult,
    stream: Option<CoordinateStream>,
    pending: BTreeMap<(u64, SubcarrierIdx), ConstellationPair>,
    pending_ncp: BTreeMap<(u64, SubcarrierIdx), NcpConstellationPair>,
}
impl ConstellationCollector {
    /// Creates an empty finite capture before receiver acquisition.
    /// # Arguments
    /// * `selection` - Prevalidated explicit opt-in configuration.
    /// # Returns
    /// Unavailable capture; no inverse stream or samples exist yet.
    pub(crate) fn new(
        selection: &ConstellationSelection,
        mode: crate::signal_processing::receiver::channel_estimation::ReceiverMode,
    ) -> Self {
        Self {
            result: ConstellationResult {
                schema_version: 1,
                selection: selection.clone(),
                status: ConstellationStatus::Unavailable,
                collected_pairs: 0,
                reference: "raw_receiver_iq_shared_unit_rms_no_display_normalization",
                channel_coefficients: if mode==crate::signal_processing::receiver::channel_estimation::ReceiverMode::PilotEstimated {"pilot_estimated_h_receiver_reciprocal"}else{"response_h_receiver_reciprocal"},
                pairs: Vec::new(),
                ncp_pairs: Vec::new(),
            },
            stream: None,
            pending: BTreeMap::new(),
            pending_ncp: BTreeMap::new(),
        }
    }
    /// Freezes diagnostic transport from actual accepted receiver configuration.
    /// # Arguments
    /// * `snapshot` - Decoded OCD/ProfileA/NCP configuration, not a TX plan.
    /// * `origin` - First payload source/channel symbol after acquisition.
    /// * `source_symbols` - Configured source-observation interval length.
    /// # Returns
    /// Unit with an independent matched inverse and bounded reorder storage.
    /// # Errors
    /// Rejects incompatible selection, invalid geometry or overflowing interval.
    pub(crate) fn start(
        &mut self,
        snapshot: &ReceiverConfiguration,
        origin: u64,
        source_symbols: u64,
    ) -> Result<(), SimulationError> {
        let layout = snapshot.channel().physical_layout()?;
        self.result.selection.validate(&layout)?;
        let settings = snapshot.interleaver_settings()?;
        self.stream = Some(CoordinateStream {
            origin,
            end_source: origin
                .checked_add(source_symbols)
                .ok_or_else(|| invalid("Constellation source interval overflow."))?,
            bins: snapshot.reduced_bins().to_vec(),
            layout,
            inverse: SymbolInterleaver::new(settings, InterleaverDirection::Receive)?,
            max_pending: settings.cells() * (settings.pair_delay_symbols() + 1),
        });
        Ok(())
    }
    /// Carries paired received samples through the existing inverse transport and NCP selection.
    /// # Arguments
    /// * `waveform` - Actual same-symbol raw FFT and corrected bins.
    /// * `payload` - Independent receiver outcome and accepted payload data-position ranks.
    /// # Returns
    /// Unit after finite pairwise admission; no decoder or RNG inputs are changed.
    /// # Errors
    /// Rejects internal coordinate/sequence inconsistency; previous capture data is retained.
    pub(crate) fn observe(
        &mut self,
        waveform: &ReceivedSymbol,
        payload: &PayloadReceiveSymbol,
    ) -> Result<(), SimulationError> {
        if self.complete() {
            return Ok(());
        }
        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| invalid("Constellation receiver is not acquired."))?;
        if waveform.symbol_index != payload.channel_symbol_index {
            return Err(invalid("Constellation receive coordinates disagree."));
        }
        let coordinates: Vec<_> = stream
            .bins
            .iter()
            .zip(&waveform.signals.reduced)
            .map(|(&bin, point)| {
                // A missing extracted sample denotes a scattered pilot. Preserve its placeholder.
                point.as_ref().map(|_| Coordinate {
                    channel_symbol: waveform.symbol_index,
                    bin,
                    before: waveform.bins[usize::from(bin)],
                    after: waveform.equalized[usize::from(bin)],
                })
            })
            .collect();
        let restored = stream
            .inverse
            .process_symbol(waveform.symbol_index - stream.origin, &coordinates)?;
        if let Some(source) = payload
            .decoded
            .as_ref()
            .filter(|source| source.error.is_none())
        {
            if (stream.origin..stream.end_source).contains(&source.source_symbol_index) {
                // Only the first consumed high-to-low hypotheses belong to the CRC-accepted
                // field. Remaining hypotheses may be payload and must never be called NCP.
                let ncp_cells: BTreeMap<_, _> = if self.result.selection.capture_ncp {
                    source
                        .ncp
                        .as_ref()
                        .map(|field| {
                            source
                                .ncp_soft
                                .iter()
                                .take(field.consumed_cells)
                                .map(|cell| (cell.data_position, cell))
                                .collect()
                        })
                        .unwrap_or_default()
                } else {
                    BTreeMap::new()
                };
                let mut selected = source
                    .data_soft
                    .iter()
                    .filter(|cell| !cell.llrs.is_empty())
                    .peekable();
                let mut data_position = 0;
                for (cell, coordinate) in payload.restored.iter().zip(restored) {
                    if let Some(TransportCell::Sample(point)) = cell {
                        if let Some(soft) = ncp_cells.get(&data_position) {
                            let coordinate = coordinate
                                .as_ref()
                                .ok_or_else(|| invalid("Missing NCP receive coordinate."))?;
                            if coordinate.after != *point {
                                return Err(invalid("NCP transport differs from received cell."));
                            }
                            let frequency_hz = stream
                                .layout
                                .frequency_hz(coordinate.bin)
                                .ok_or_else(|| invalid("NCP carrier outside geometry."))?;
                            if self.result.selection.contains(frequency_hz)
                                && self.result.ncp_pairs.len()
                                    < self.result.selection.requested_pairs.min(8192)
                            {
                                self.pending_ncp.insert(
                                    (coordinate.channel_symbol, coordinate.bin),
                                    NcpConstellationPair {
                                        channel_symbol_index: coordinate.channel_symbol,
                                        source_symbol_index: source.source_symbol_index,
                                        physical_bin: coordinate.bin,
                                        frequency_hz,
                                        data_position,
                                        modulation_order: 1u16 << soft.llrs.len(),
                                        before_equalization: coordinate.before,
                                        after_equalization: coordinate.after,
                                        llrs: soft.llrs.clone(),
                                    },
                                );
                            }
                        }
                        if selected
                            .peek()
                            .is_some_and(|selected| selected.data_position == data_position)
                        {
                            let soft = selected
                                .next()
                                .ok_or_else(|| invalid("Missing selected payload cell."))?;
                            let modulation_order = 1u16 << soft.llrs.len();
                            let coordinate = coordinate.ok_or_else(|| {
                                invalid("Missing constellation payload coordinate.")
                            })?;
                            if coordinate.after != *point {
                                return Err(invalid(
                                    "Constellation transport does not match received payload.",
                                ));
                            }
                            let frequency_hz =
                                stream.layout.frequency_hz(coordinate.bin).ok_or_else(|| {
                                    invalid("Constellation carrier outside geometry.")
                                })?;
                            if self.result.selection.contains(frequency_hz)
                                && self.result.status != ConstellationStatus::Complete
                            {
                                self.pending.insert(
                                    (coordinate.channel_symbol, coordinate.bin),
                                    ConstellationPair {
                                        channel_symbol_index: coordinate.channel_symbol,
                                        source_symbol_index: source.source_symbol_index,
                                        physical_bin: coordinate.bin,
                                        frequency_hz,
                                        data_position,
                                        profile_id: self.result.selection.profile_id,
                                        modulation_order,
                                        before_equalization: coordinate.before,
                                        after_equalization: coordinate.after,
                                        llrs: soft.llrs.clone(),
                                    },
                                );
                            }
                        }
                        data_position += 1;
                    }
                }
                if selected.next().is_some() {
                    return Err(invalid("Constellation data rank outside restored samples."));
                }
            }
            // All physical cells at or before this source index have now been classified.
            let watermark = source.source_symbol_index;
            self.release(Some(watermark));
        }
        if self.pending.len() + self.pending_ncp.len()
            > self.stream.as_ref().map_or(0, |stream| stream.max_pending)
        {
            return Err(invalid("Constellation pending-coordinate limit exceeded."));
        }
        Ok(())
    }
    fn release(&mut self, watermark: Option<u64>) {
        while self.result.ncp_pairs.len() < self.result.selection.requested_pairs.min(8192) {
            if self
                .pending_ncp
                .first_key_value()
                .is_none_or(|(key, _)| watermark.is_some_and(|limit| key.0 > limit))
            {
                break;
            }
            if let Some((_, pair)) = self.pending_ncp.pop_first() {
                self.result.ncp_pairs.push(pair);
            }
        }
        while self.result.pairs.len() < self.result.selection.requested_pairs {
            if self
                .pending
                .first_key_value()
                .is_none_or(|(key, _)| watermark.is_some_and(|limit| key.0 > limit))
            {
                break;
            }
            if let Some((_, pair)) = self.pending.pop_first() {
                self.result.pairs.push(pair);
            }
        }
        self.result.collected_pairs = self.result.pairs.len();
        self.result.status = if self.result.collected_pairs == self.result.selection.requested_pairs
        {
            ConstellationStatus::Complete
        } else if self.result.collected_pairs == 0 {
            ConstellationStatus::Unavailable
        } else {
            ConstellationStatus::Partial
        };
        if self.result.status == ConstellationStatus::Complete {
            self.pending.clear();
        }
        if self.complete() {
            self.pending_ncp.clear();
            self.stream = None;
        }
    }
    fn complete(&self) -> bool {
        self.result.status == ConstellationStatus::Complete
            && (!self.result.selection.capture_ncp
                || self.result.ncp_pairs.len() == self.result.selection.requested_pairs.min(8192))
    }
    /// Returns the bounded capture collected so far without advancing transport.
    /// # Returns
    /// Borrowed committed pairs; some classified pairs may still await the ordering watermark.
    pub(crate) fn result(&self) -> &ConstellationResult {
        &self.result
    }
    /// Finishes diagnostic retention, preserving confirmed pairs on every job status.
    /// # Returns
    /// Owned result after flushing bounded classified points; no new observations are invented.
    pub(crate) fn finish(mut self) -> ConstellationResult {
        self.release(None);
        self.result
    }
}
