/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Whole-codeword accounting in source time, independent of delayed receive arrival.
use crate::{error::invalid, transmission::DurationTiming, SimulationError};
use serde::Serialize;
/// Half-open source-symbol and sample observation coordinates, not host time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ObservationWindow {
    pub first_source_symbol: u64,
    pub end_source_symbol: u64,
    pub first_sample: u64,
    pub end_sample: u64,
    pub symbol_period_samples: u32,
}
/// Relationship of the complete codeword's source-symbol span to observation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WordPlacement {
    Contained,
    CrossesStart,
    CrossesEnd,
    CrossesBoth,
    Outside,
}
impl ObservationWindow {
    /// Constructs checked absolute coordinates after actual PLC acquisition.
    /// # Arguments
    /// * `origin` - First payload source symbol, independent of RX output delay.
    /// * `timing` - Validated observation sample/symbol plan.
    /// # Returns
    /// Immutable window; unused trailing ticks remain in the sample denominator.
    /// # Errors
    /// Rejects zero/inconsistent timing or absolute symbol/sample overflow.
    pub fn new(origin: u64, timing: DurationTiming) -> Result<Self, SimulationError> {
        let stride = u64::from(timing.symbol_period_samples);
        if stride == 0
            || timing.complete_symbols == 0
            || timing.observation_samples / stride != timing.complete_symbols
            || timing.observation_samples % stride != timing.unused_samples
        {
            return Err(invalid("Inconsistent observation-window timing."));
        }
        let end_source_symbol = origin
            .checked_add(timing.complete_symbols)
            .ok_or_else(|| invalid("Observation symbol overflow."))?;
        let first_sample = origin
            .checked_mul(stride)
            .ok_or_else(|| invalid("Observation sample overflow."))?;
        let end_sample = first_sample
            .checked_add(timing.observation_samples)
            .ok_or_else(|| invalid("Observation sample overflow."))?;
        Ok(Self {
            first_source_symbol: origin,
            end_source_symbol,
            first_sample,
            end_sample,
            symbol_period_samples: timing.symbol_period_samples,
        })
    }
    /// Classifies an inclusive source-symbol span using half-open window edges.
    /// # Arguments
    /// * `first` - First source symbol containing this codeword.
    /// * `last` - Last source symbol containing this codeword, inclusive.
    /// # Returns
    /// Whole-word placement; receiver arrival during drain does not change membership.
    /// # Errors
    /// Rejects reversed spans or an unrepresentable exclusive sample end.
    pub fn classify(&self, first: u64, last: u64) -> Result<WordPlacement, SimulationError> {
        if self.symbol_period_samples == 0
            || self.first_source_symbol >= self.end_source_symbol
            || first > last
            || last
                .checked_add(1)
                .and_then(|end| end.checked_mul(u64::from(self.symbol_period_samples)))
                .is_none()
        {
            return Err(invalid("Invalid received codeword source span."));
        }
        if last < self.first_source_symbol || first >= self.end_source_symbol {
            return Ok(WordPlacement::Outside);
        }
        Ok(
            match (
                first < self.first_source_symbol,
                last >= self.end_source_symbol,
            ) {
                (false, false) => WordPlacement::Contained,
                (true, false) => WordPlacement::CrossesStart,
                (false, true) => WordPlacement::CrossesEnd,
                (true, true) => WordPlacement::CrossesBoth,
            },
        )
    }
}
/// Constant-size source-window totals; no inferred fractional information-bit attribution.
#[derive(Debug, Clone, Serialize)]
pub struct WindowTotals {
    pub coordinates: ObservationWindow,
    pub contained_words: u64,
    pub crossing_start_words: u64,
    pub crossing_end_words: u64,
    pub crossing_both_words: u64,
    pub outside_words: u64,
    pub announced_starts_within: u64,
    pub completed_starts_within: u64,
    pub incomplete_starts_within: u64,
    pub contained_words_recovered_during_drain: u64,
    pub compared_bits: u64,
    pub bit_errors: u64,
    pub excluded_recovered_bits: u64,
    pub ber: Option<f64>,
    /// Whole contained-word payload bits / requested observation time; final runs only.
    pub payload_rate_bps: Option<f64>,
}
impl WindowTotals {
    /// Creates empty counters for an immutable checked source window.
    /// # Arguments
    /// * `coordinates` - Validated absolute source/sample interval.
    /// # Returns
    /// Empty metrics with unavailable BER/rate until observations/finalization.
    pub(crate) fn new(coordinates: ObservationWindow) -> Self {
        Self {
            coordinates,
            contained_words: 0,
            crossing_start_words: 0,
            crossing_end_words: 0,
            crossing_both_words: 0,
            outside_words: 0,
            announced_starts_within: 0,
            completed_starts_within: 0,
            incomplete_starts_within: 0,
            contained_words_recovered_during_drain: 0,
            compared_bits: 0,
            bit_errors: 0,
            excluded_recovered_bits: 0,
            ber: None,
            payload_rate_bps: None,
        }
    }
    /// Adds one actually recovered whole-word comparison to its source category.
    /// # Arguments
    /// * `placement` - Classification from the shared checked window.
    /// * `bits` - Actual compared information bits.
    /// * `errors` - Actual mismatches, bounded by bits at the verifier boundary.
    /// * `during_drain` - Whether receive completion followed source observation.
    /// # Returns
    /// Unit after updating finite cumulative counters.
    pub(crate) fn record(
        &mut self,
        placement: WordPlacement,
        bits: u64,
        errors: u64,
        during_drain: bool,
    ) {
        match placement {
            WordPlacement::Contained => {
                self.contained_words += 1;
                self.completed_starts_within += 1;
                self.compared_bits += bits;
                self.bit_errors += errors;
                self.contained_words_recovered_during_drain += u64::from(during_drain);
            }
            WordPlacement::CrossesStart => self.crossing_start_words += 1,
            WordPlacement::CrossesEnd => {
                self.crossing_end_words += 1;
                self.completed_starts_within += 1;
            }
            WordPlacement::CrossesBoth => self.crossing_both_words += 1,
            WordPlacement::Outside => self.outside_words += 1,
        }
        if placement != WordPlacement::Contained {
            self.excluded_recovered_bits += bits;
        }
        self.incomplete_starts_within = self
            .announced_starts_within
            .saturating_sub(self.completed_starts_within);
        if self.compared_bits > 0 {
            self.ber = Some(self.bit_errors as f64 / self.compared_bits as f64);
        }
    }
    /// Publishes whole-contained-word rate only after successful full completion.
    /// # Arguments
    /// * `timing` - The original fixed positive observation sample budget/clock.
    /// # Returns
    /// Unit with information-bit rate; unused trailing ticks remain in denominator.
    pub(crate) fn finalize(&mut self, timing: DurationTiming) {
        self.payload_rate_bps = Some(
            self.compared_bits as f64 * f64::from(timing.sample_rate_hz)
                / timing.observation_samples as f64,
        );
    }
}
