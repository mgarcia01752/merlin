/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite copies of existing transmitter boundaries, never simulation inputs.
mod json_storage;
mod plan;
use crate::downstream::common::constants::{
    MAX_INSTRUMENT_TIME_SAMPLES, MAX_TRANSMITTER_MESSAGE_FRAMES, MAX_TX_FRAME_SEQUENCE_SAMPLES,
};
use merlin::downstream::{
    FullLoopStage as TestPointId, FullLoopStageData as ProbeData, FullLoopStageSnapshot,
};
pub(crate) use plan::{Plan, PlanError};
type IqPair = [f64; 2];
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) const STAGES: &[TestPointId] = &[
    TestPointId::SourcePayload,
    TestPointId::PayloadCoded,
    TestPointId::PayloadInterleavedBits,
    TestPointId::PayloadLabels,
    TestPointId::PayloadMapped,
    TestPointId::PayloadInterleaved,
    TestPointId::PhysicalBins,
    TestPointId::PlcMessage,
    TestPointId::PlcIq,
    TestPointId::PlcPreamble,
    TestPointId::ContinuousPilots,
    TestPointId::ScatteredPilots,
    TestPointId::TxIfft,
    TestPointId::TxPrefix,
    TestPointId::TxWindow,
    TestPointId::TxOverlap,
];

/// Presentation order of the four actual TX waveform boundaries.
pub(super) const WAVE_STAGES: &[TestPointId] = &[
    TestPointId::TxIfft,
    TestPointId::TxPrefix,
    TestPointId::TxWindow,
    TestPointId::TxOverlap,
];

pub(crate) struct Snapshot {
    pub data: Value,
    pub total: usize,
    pub offset: usize,
    pub symbol: Option<u64>,
    pub phase: String,
    pub source_symbol: Option<u64>,
}

#[derive(Default)]
pub(crate) struct Capture {
    pub snapshots: BTreeMap<TestPointId, Snapshot>,
    pub waves: [Vec<IqPair>; WAVE_STAGES.len()],
    pub frames: Vec<(u64, Vec<u8>)>,
}
impl Capture {
    /// Copies one selected TX event within fixed per-stage and frame limits.
    /// # Arguments
    /// * `event` - Already bounded immutable observation with original coordinates.
    /// # Returns
    /// Unit; exhausted retention leaves computation and RNG state unchanged.
    pub(crate) fn observe(&mut self, event: &FullLoopStageSnapshot) {
        if !STAGES.contains(&event.stage) {
            return;
        }
        if event.stage == TestPointId::PlcMessage {
            if let ProbeData::Bytes(bytes) = &event.data {
                if self.frames.len() < MAX_TRANSMITTER_MESSAGE_FRAMES
                    && event.element_start == 0
                    && bytes.len() == event.total_elements
                {
                    if let Some(symbol) = event.context.channel_symbol_index {
                        self.frames.push((symbol, bytes.to_vec()));
                    }
                }
            }
            return;
        }
        if let Some(index) = WAVE_STAGES.iter().position(|id| *id == event.stage) {
            if let ProbeData::Iq(values) = &event.data {
                let limit = if event.stage == TestPointId::TxOverlap {
                    MAX_INSTRUMENT_TIME_SAMPLES
                } else {
                    MAX_TX_FRAME_SEQUENCE_SAMPLES
                };
                let count = values.len().min(limit - self.waves[index].len());
                self.waves[index].extend_from_slice(&values[..count]);
            }
        }
        self.snapshots
            .entry(event.stage)
            .or_insert_with(|| Snapshot {
                data: serde_json::to_value(&event.data).unwrap_or(Value::Null),
                total: event.total_elements,
                offset: event.element_start,
                symbol: event.context.channel_symbol_index,
                phase: plan::phase(event.context.phase).to_owned(),
                source_symbol: event.context.source_symbol_index,
            });
    }

    /// Normalizes unchanged copied data into the preplanned exact final storage.
    ///
    /// # Returns
    /// Unit; all owned vectors/text have capacity equal to length. Serializer and
    /// collection builders remain separate finite temporary execution storage.
    pub(crate) fn normalize(&mut self) {
        let Self {
            snapshots,
            waves,
            frames,
        } = self;
        for snapshot in snapshots.values_mut() {
            let Snapshot {
                data,
                total: _,
                offset: _,
                symbol: _,
                phase,
                source_symbol: _,
            } = snapshot;
            json_storage::normalize(data);
            *phase = String::from(std::mem::take(phase).into_boxed_str());
        }
        for wave in waves {
            *wave = Vec::from(std::mem::take(wave).into_boxed_slice());
        }
        for (_, bytes) in frames.iter_mut() {
            *bytes = Vec::from(std::mem::take(bytes).into_boxed_slice());
        }
        *frames = Vec::from(std::mem::take(frames).into_boxed_slice());
    }
}
