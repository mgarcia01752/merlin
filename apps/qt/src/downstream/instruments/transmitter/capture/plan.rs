/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Allocation-free plan for this concrete transmitter capture owner.
use super::{json_storage, IqPair, ProbeData, Snapshot, TestPointId, STAGES, WAVE_STAGES};
use crate::downstream::common::constants::{
    MAX_INSTRUMENT_TIME_SAMPLES, MAX_TRANSMITTER_MESSAGE_FRAMES, MAX_TX_FRAME_SEQUENCE_SAMPLES,
};
use crate::shell::{RetainedResources, RetentionError};
use merlin::downstream::{FullLoopStagePhase, FullLoopStageSnapshot};
use std::mem::size_of;

#[derive(Debug)]
pub(crate) enum PlanError {
    Retention(RetentionError),
    UnsupportedRepresentation,
}

#[derive(Default)]
pub(crate) struct Plan {
    snapshots: [bool; STAGES.len()],
    waves: [usize; WAVE_STAGES.len()],
    frames: usize,
    frame_bytes: usize,
    snapshot_bytes: usize,
    snapshot_entries: usize,
}

fn add(a: usize, b: usize) -> Result<usize, PlanError> {
    a.checked_add(b)
        .ok_or(PlanError::Retention(RetentionError::ArithmeticOverflow))
}
fn product(a: usize, b: usize) -> Result<usize, PlanError> {
    a.checked_mul(b)
        .ok_or(PlanError::Retention(RetentionError::ArithmeticOverflow))
}

/// Names the existing Debug phase spelling without allocating during planning.
///
/// # Arguments
/// * `phase` - Original public phase, independent of tab or execution ownership.
///
/// # Returns
/// The same text previously produced by formatting that phase with Debug.
pub(super) fn phase(phase: FullLoopStagePhase) -> &'static str {
    match phase {
        FullLoopStagePhase::Warmup => "Warmup",
        FullLoopStagePhase::Observation => "Observation",
        FullLoopStagePhase::Drain => "Drain",
    }
}

impl Plan {
    /// Plans the unchanged capture selections without allocating copied samples or JSON.
    ///
    /// # Arguments
    /// * `event` - Borrowed admitted event with original coordinates and values.
    ///
    /// # Returns
    /// Unit after including only the same first snapshots, frames and waveform prefixes.
    ///
    /// # Errors
    /// Checked overflow or an unaudited JSON representation rejects before target allocation.
    pub(crate) fn observe(&mut self, event: &FullLoopStageSnapshot) -> Result<(), PlanError> {
        let Some(stage) = STAGES.iter().position(|stage| *stage == event.stage) else {
            return Ok(());
        };
        if event.stage == TestPointId::PlcMessage {
            if let ProbeData::Bytes(bytes) = &event.data {
                if self.frames < MAX_TRANSMITTER_MESSAGE_FRAMES
                    && event.element_start == 0
                    && bytes.len() == event.total_elements
                    && event.context.channel_symbol_index.is_some()
                {
                    self.frames = add(self.frames, 1)?;
                    self.frame_bytes = add(self.frame_bytes, bytes.len())?;
                }
            }
            return Ok(());
        }
        if let Some(index) = WAVE_STAGES.iter().position(|id| *id == event.stage) {
            if let ProbeData::Iq(values) = &event.data {
                let limit = if event.stage == TestPointId::TxOverlap {
                    MAX_INSTRUMENT_TIME_SAMPLES
                } else {
                    MAX_TX_FRAME_SEQUENCE_SAMPLES
                };
                self.waves[index] = add(
                    self.waves[index],
                    values.len().min(limit - self.waves[index]),
                )?;
            }
        }
        if !self.snapshots[stage] {
            let json = json_storage::children(&event.data)?;
            self.snapshot_bytes = add(
                self.snapshot_bytes,
                add(
                    size_of::<(TestPointId, Snapshot)>(),
                    add(json.payload_bytes, phase(event.context.phase).len())?,
                )?,
            )?;
            self.snapshot_entries = add(self.snapshot_entries, add(1, json.ordered_entries)?)?;
            self.snapshots[stage] = true;
        }
        Ok(())
    }

    /// Reads dynamic final capture storage; fixed Capture fields belong to ResultData.
    ///
    /// # Returns
    /// Exact payload/ordered-entry target with zero shared scientific roots.
    ///
    /// # Errors
    /// Checked vector-size or aggregate arithmetic overflow.
    pub(crate) fn resources(&self) -> Result<RetainedResources, PlanError> {
        let mut bytes = add(
            self.snapshot_bytes,
            add(
                product(self.frames, size_of::<(u64, Vec<u8>)>())?,
                self.frame_bytes,
            )?,
        )?;
        for count in self.waves {
            bytes = add(bytes, product(count, size_of::<IqPair>())?)?;
        }
        Ok(RetainedResources {
            payload_bytes: bytes,
            ordered_entries: self.snapshot_entries,
            shared_roots: 0,
        })
    }
}
