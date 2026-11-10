/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite copies of existing transmitter boundaries, never simulation inputs.
use docsis_ofdm_fec::{
    simulation::test_points::{ProbeData, ProbeEvent, TestPointId},
    types::IqPair,
};
use serde_json::Value;
use std::collections::BTreeMap;

pub(crate) const STAGES: [TestPointId; 16] = [
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

pub(crate) struct Snapshot {
    pub data: Value,
    pub total: usize,
    pub offset: usize,
    pub symbol: Option<u64>,
    pub phase: String,
}

#[derive(Default)]
pub(crate) struct Capture {
    pub snapshots: BTreeMap<TestPointId, Snapshot>,
    pub waves: [Vec<IqPair>; 4],
    pub frames: Vec<(u64, Vec<u8>)>,
}
impl Capture {
    /// Copies one selected TX event within fixed per-stage and frame limits.
    /// # Arguments
    /// * `event` - Already bounded immutable observation with original coordinates.
    /// # Returns
    /// Unit; exhausted retention leaves computation and RNG state unchanged.
    pub(crate) fn observe(&mut self, event: &ProbeEvent<'_>) {
        if !STAGES.contains(&event.stage) {
            return;
        }
        if event.stage == TestPointId::PlcMessage {
            if let ProbeData::Bytes(bytes) = event.data {
                if self.frames.len() < 64
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
        if let Some(index) = [
            TestPointId::TxIfft,
            TestPointId::TxPrefix,
            TestPointId::TxWindow,
            TestPointId::TxOverlap,
        ]
        .iter()
        .position(|id| *id == event.stage)
        {
            if let ProbeData::Iq(values) = event.data {
                let limit = if index == 3 { 32768 } else { 16384 };
                let count = values.len().min(limit - self.waves[index].len());
                self.waves[index].extend_from_slice(&values[..count]);
            }
        }
        self.snapshots
            .entry(event.stage)
            .or_insert_with(|| Snapshot {
                data: serde_json::to_value(event.data).unwrap_or(Value::Null),
                total: event.total_elements,
                offset: event.element_start,
                symbol: event.context.channel_symbol_index,
                phase: format!("{:?}", event.context.phase),
            });
    }
}
