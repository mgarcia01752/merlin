/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded copies of authoritative public observations for existing presentation consumers.
mod plan;
mod queries;
use crate::downstream::common::constants::MAX_INSTRUMENT_TIME_SAMPLES;
use crate::shell::{Retained, RetentionBudget, RetentionError};
use merlin::downstream::{
    ApiError, FullLoopResult, FullLoopStage as Stage, FullLoopStageData as Data,
    FullLoopStagePhase, FullLoopStageSnapshot,
};

const RX_TAPS: &[Stage] = &[
    Stage::ChannelOutput,
    Stage::RxFramed,
    Stage::RxUseful,
    Stage::RxFft,
];
type IqPair = [f64; 2];

#[derive(Default)]
pub(crate) struct Samples {
    pub values: Vec<IqPair>,
    next: Option<u64>,
}

#[derive(Default)]
pub(crate) struct Observations {
    pub tx: crate::transmitter_instruments::capture::Capture,
    pub taps: [Samples; RX_TAPS.len()],
    pub responses: Vec<(u64, Vec<IqPair>)>,
    pub decoder_llrs: Vec<f64>,
    pub decoder_word: Option<u64>,
    pub warnings: Vec<String>,

    // Rust drops fields in declaration order: all charged buffers go first.
    retained: Option<Retained<()>>,
}

/// Presentation admission never changes or rejects the completed scientific owner.
#[derive(Debug)]
pub(crate) enum ObservationError {
    Backend(ApiError),
    Retention(RetentionError),
    UnsupportedRepresentation,
}

fn exact<T>(values: &mut Vec<T>) {
    *values = Vec::from(std::mem::take(values).into_boxed_slice());
}

fn text(value: &mut String) {
    *value = String::from(std::mem::take(value).into_boxed_str());
}

impl Observations {
    /// Builds admitted display copies while retaining every simultaneous query owner.
    ///
    /// # Arguments
    /// * `result` - Immutable completed or partial science, already independently charged.
    /// * `budget` - Canonical host policy shared with source, queued and old results.
    ///
    /// # Returns
    /// Existing instrument values with exact final capacities and an owned dynamic
    /// payload/ordered-entry reservation. Fixed storage belongs to ResultData's
    /// separately reserved layout. Temporary builders keep existing finite policies.
    ///
    /// # Errors
    /// Returns checked metadata, query, unsupported-representation or admission errors.
    /// The caller must keep the scientific result and use an admitted or static
    /// diagnostic; this method allocates no fallback warning on admission failure.
    pub(crate) fn from_result_admitted(
        result: &FullLoopResult,
        budget: &RetentionBudget,
    ) -> Result<Self, ObservationError> {
        let queries = queries::Queries::new(result, budget)?;
        let mut plan = plan::Plan::default();
        queries.visit(|event| match event {
            Ok(event) => plan.observe(event),
            Err(error) => plan.warning(error.message()),
        })?;
        let reservation = budget
            .reserve_resources(plan.resources()?)
            .map_err(ObservationError::Retention)?;
        let mut output = Self::default();
        queries.visit(|event| {
            match event {
                Ok(event) => output.observe(event),
                Err(error) => output.warnings.push(error.message().to_owned()),
            }
            Ok(())
        })?;
        output.normalize();
        output.retained = Some(reservation.retain(()));
        Ok(output)
    }

    /// Measures normalized fixture children independently of the admission planner.
    ///
    /// # Returns
    /// Actual display payload and entries for surrounding owner-lifetime assertions.
    /// This test-only walk is not a production pre-copy bound or admission fallback.
    #[cfg(test)]
    pub(super) fn fixture_resources(&self) -> crate::shell::RetainedResources {
        tests::measured(self)
    }

    fn normalize(&mut self) {
        self.tx.normalize();
        for tap in &mut self.taps {
            exact(&mut tap.values);
        }
        for (_, values) in &mut self.responses {
            exact(values);
        }
        exact(&mut self.responses);
        exact(&mut self.decoder_llrs);
        for warning in &mut self.warnings {
            text(warning);
        }
        exact(&mut self.warnings);
    }

    fn observe(&mut self, event: &FullLoopStageSnapshot) {
        self.tx.observe(event);
        if event.context.phase != FullLoopStagePhase::Observation {
            return;
        }
        match (&event.stage, &event.data) {
            (Stage::RxChannelEstimate, Data::Iq(values)) => {
                // Event count is already bounded by the accepted public observation policy.
                self.responses.push((
                    event.context.channel_symbol_index.unwrap_or(0),
                    values.clone(),
                ));
                return;
            }
            (Stage::PayloadLdpcInput, Data::Llrs(values)) => {
                if self.decoder_llrs.is_empty() {
                    self.decoder_llrs.clone_from(values);
                    self.decoder_word = event.context.codeword_index;
                }
                return;
            }
            _ => {}
        }
        let Some(index) = RX_TAPS.iter().position(|stage| *stage == event.stage) else {
            return;
        };
        let Data::Iq(values) = &event.data else {
            return;
        };
        let target = &mut self.taps[index];
        let first_event_only = matches!(event.stage, Stage::RxUseful | Stage::RxFft);
        if first_event_only && !target.values.is_empty() {
            return;
        }
        if target.next.is_some() && target.next != event.context.sample_start_index {
            return;
        }
        let count = values
            .len()
            .min(MAX_INSTRUMENT_TIME_SAMPLES - target.values.len());
        target.values.extend_from_slice(&values[..count]);
        target.next = event
            .context
            .sample_start_index
            .and_then(|start| start.checked_add(values.len() as u64));
    }
}

#[cfg(test)]
mod tests;
