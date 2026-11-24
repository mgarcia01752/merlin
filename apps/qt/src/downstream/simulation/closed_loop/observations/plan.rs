/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Allocation-free plan of the existing receiver and transmitter presentation selections.
use super::{Data, FullLoopStagePhase, FullLoopStageSnapshot, IqPair, ObservationError, RX_TAPS};
use crate::downstream::common::constants::MAX_INSTRUMENT_TIME_SAMPLES;
use crate::shell::{RetainedResources, RetentionError};
use crate::transmitter_instruments::capture;
use merlin::downstream::FullLoopStage as Stage;
use std::mem::size_of;

#[derive(Default)]
pub(super) struct Plan {
    tx: capture::Plan,
    taps: [(usize, Option<u64>); RX_TAPS.len()],
    responses: usize,
    response_values: usize,
    decoder_llrs: usize,
    warnings: usize,
    warning_bytes: usize,
}

fn add(a: usize, b: usize) -> Result<usize, ObservationError> {
    a.checked_add(b).ok_or(ObservationError::Retention(
        RetentionError::ArithmeticOverflow,
    ))
}
fn product(a: usize, b: usize) -> Result<usize, ObservationError> {
    a.checked_mul(b).ok_or(ObservationError::Retention(
        RetentionError::ArithmeticOverflow,
    ))
}
fn capture_error(error: capture::PlanError) -> ObservationError {
    match error {
        capture::PlanError::Retention(error) => ObservationError::Retention(error),
        capture::PlanError::UnsupportedRepresentation => {
            ObservationError::UnsupportedRepresentation
        }
    }
}

impl Plan {
    /// Includes the existing first/contiguous receiver copies and independent TX selections.
    ///
    /// # Arguments
    /// * `event` - Borrowed admitted event; no typed values or scientific state are copied.
    ///
    /// # Returns
    /// Unit after recording final container populations and exact selected lengths.
    ///
    /// # Errors
    /// Checked arithmetic or an unaudited future public JSON representation.
    pub(super) fn observe(
        &mut self,
        event: &FullLoopStageSnapshot,
    ) -> Result<(), ObservationError> {
        self.tx.observe(event).map_err(capture_error)?;
        if event.context.phase != FullLoopStagePhase::Observation {
            return Ok(());
        }
        match (&event.stage, &event.data) {
            (Stage::RxChannelEstimate, Data::Iq(values)) => {
                self.responses = add(self.responses, 1)?;
                self.response_values = add(self.response_values, values.len())?;
                return Ok(());
            }
            (Stage::PayloadLdpcInput, Data::Llrs(values)) => {
                if self.decoder_llrs == 0 {
                    self.decoder_llrs = values.len();
                }
                return Ok(());
            }
            _ => {}
        }
        let Some(index) = RX_TAPS.iter().position(|stage| *stage == event.stage) else {
            return Ok(());
        };
        let Data::Iq(values) = &event.data else {
            return Ok(());
        };
        let (len, next) = &mut self.taps[index];
        if matches!(event.stage, Stage::RxUseful | Stage::RxFft) && *len != 0 {
            return Ok(());
        }
        if next.is_some() && *next != event.context.sample_start_index {
            return Ok(());
        }
        *len = add(*len, values.len().min(MAX_INSTRUMENT_TIME_SAMPLES - *len))?;
        *next = event
            .context
            .sample_start_index
            .and_then(|start| start.checked_add(values.len() as u64));
        Ok(())
    }

    /// Includes the unchanged original query diagnostic without allocating it yet.
    ///
    /// # Arguments
    /// * `message` - Borrowed public error message repeated by the immutable second pass.
    ///
    /// # Returns
    /// Unit after reservable final warning-string and vector lengths are recorded.
    ///
    /// # Errors
    /// Checked population/byte overflow.
    pub(super) fn warning(&mut self, message: &str) -> Result<(), ObservationError> {
        self.warnings = add(self.warnings, 1)?;
        self.warning_bytes = add(self.warning_bytes, message.len())?;
        Ok(())
    }

    /// Reads all dynamic final projection resources before constructing any copied target.
    ///
    /// # Returns
    /// Exact normalized capacities and ordered map entries; fixed Observations and
    /// its embedded retention guard are included by the separate ResultData layout.
    ///
    /// # Errors
    /// Checked target-layout or population arithmetic overflow.
    pub(super) fn resources(&self) -> Result<RetainedResources, ObservationError> {
        let tx = self.tx.resources().map_err(capture_error)?;
        let mut bytes = add(
            tx.payload_bytes,
            product(self.responses, size_of::<(u64, Vec<IqPair>)>())?,
        )?;
        bytes = add(bytes, product(self.response_values, size_of::<IqPair>())?)?;
        bytes = add(bytes, product(self.decoder_llrs, size_of::<f64>())?)?;
        bytes = add(bytes, product(self.warnings, size_of::<String>())?)?;
        bytes = add(bytes, self.warning_bytes)?;
        for (len, _) in self.taps {
            bytes = add(bytes, product(len, size_of::<IqPair>())?)?;
        }
        Ok(RetainedResources {
            payload_bytes: bytes,
            ordered_entries: tx.ordered_entries,
            shared_roots: 0,
            ..Default::default()
        })
    }
}
