/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded RX framing using an explicitly known first symbol boundary.
use crate::{
    error::invalid,
    signal_processing::waveform::{SampleBlock, WaveformGeometry},
    types::IqPair,
    LabError,
};
/// One complete received symbol period, before prefix removal.
#[derive(Debug, Clone)]
pub struct FramedSymbol {
    pub symbol_index: u64,
    pub period: SampleBlock,
}
/// Known-timing sample framer; pending storage stays shorter than one period.
pub struct SymbolFramer {
    geometry: WaveformGeometry,
    next_input: u64,
    next_symbol_start: u64,
    next_symbol: u64,
    pending: Vec<IqPair>,
}
impl SymbolFramer {
    /// Starts reception with explicit sample and symbol identities.
    /// # Arguments
    /// * `geometry` - RX-owned frozen FFT/prefix settings.
    /// * `first_input` - First incoming sample tick.
    /// * `first_symbol_start` - Known first desired symbol boundary, at/after first_input.
    /// * `first_symbol` - PLC-aligned channel symbol index at that boundary.
    /// # Returns
    /// Empty state; samples before the selected boundary are skipped explicitly.
    /// # Errors
    /// Rejects a symbol boundary preceding the supplied stream start.
    pub fn new(
        geometry: &WaveformGeometry,
        first_input: u64,
        first_symbol_start: u64,
        first_symbol: u64,
    ) -> Result<Self, LabError> {
        if first_symbol_start < first_input {
            return Err(invalid("RX symbol boundary precedes stream start."));
        }
        Ok(Self {
            geometry: geometry.clone(),
            next_input: first_input,
            next_symbol_start: first_symbol_start,
            next_symbol: first_symbol,
            pending: Vec::new(),
        })
    }
    /// Accepts one contiguous block and emits only complete symbol periods.
    /// # Arguments
    /// * `input` - Finite bounded RX time samples beginning at the expected tick.
    /// # Returns
    /// At most sixteen complete periods per maximum-size input; partial data stays private.
    /// # Errors
    /// Rejects discontinuity/index overflow atomically; no missing samples are invented.
    pub fn push(&mut self, input: &SampleBlock) -> Result<Vec<FramedSymbol>, LabError> {
        if input.start_index() != self.next_input {
            return Err(invalid("RX sample stream is discontinuous."));
        }
        let skip = self
            .next_symbol_start
            .saturating_sub(input.start_index())
            .min(input.samples().len() as u64) as usize;
        // If a partial symbol exists, its start precedes this block and skip is zero.
        let mut candidate = self.pending.clone();
        candidate.extend_from_slice(&input.samples()[skip..]);
        let stride = self.geometry.stride_samples();
        let completed = candidate.len() / stride;
        let next_symbol = self
            .next_symbol
            .checked_add(completed as u64)
            .ok_or_else(|| invalid("RX symbol index overflow."))?;
        let next_start = self
            .next_symbol_start
            .checked_add((completed * stride) as u64)
            .ok_or_else(|| invalid("RX symbol tick overflow."))?;
        let mut output = Vec::with_capacity(completed);
        for (offset, period) in candidate.chunks_exact(stride).enumerate() {
            output.push(FramedSymbol {
                symbol_index: self.next_symbol + offset as u64,
                period: SampleBlock::new(
                    self.next_symbol_start + (offset * stride) as u64,
                    period.to_vec(),
                )?,
            });
        }
        self.pending = candidate[completed * stride..].to_vec();
        self.next_input = input.end_index();
        self.next_symbol = next_symbol;
        self.next_symbol_start = next_start;
        Ok(output)
    }
    /// Reports incomplete received samples without treating them as recovered data.
    /// # Returns
    /// Fewer than N+CP samples; a final RP drain is reported as a remainder.
    pub fn pending_samples(&self) -> usize {
        self.pending.len()
    }
}
