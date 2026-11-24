/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local crate ownership and shared DSP/FEC imports.
 */
//! Deterministic per-selected-tap retention within one aggregate budget.
use crate::{
    error::invalid,
    resources::{Cell, CellRole, ResourcePlan},
    types::{Identity, Iq, SampleBlock},
    PhyError, PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Copied actual QAM-point application ceiling shared by raw and normalized queries.
pub const MAX_QAM_QUERY_POINTS: usize = 4096;

/// Stable scientific test point; time samples and frequency cells stay distinct.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Tap {
    /// Physical TX grid after role gain, before Pre-EQ.
    TxGrid,
    /// Physical TX grid after corrective C.
    TxPreEq,
    /// Useful time samples before CP/taper, discontinuous across symbols.
    TxUseful,
    /// Actual indexed assembled transmitter output.
    TxOutput,
    /// Actual channel input, identical reference plane to transmitter output.
    ChannelInput,
    /// Actual channel output, including modeled terminal drain.
    ChannelOutput,
    /// Actual indexed input accepted by the normal receiver.
    RxInput,
    /// Received FFT grid, before received-pilot correction.
    RxFft,
    /// Recovered unit-reference payload cells after correction and role removal.
    RxEqualized,
}

/// Matched physical cell observation in the selected tap's actual reference plane.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct QamPoint {
    /// Actual run/configuration/grant identity.
    pub identity: Identity,
    /// Configured absolute physical frame identity.
    pub frame: u64,
    /// Symbol within that frame.
    pub symbol: usize,
    /// Original physical carrier, without exclusion compaction.
    pub carrier: usize,
    /// Known pilot/data/complementary role.
    pub role: CellRole,
    /// Original payload loading; known pilots carry zero payload bits.
    pub loading: u8,
    /// Actual observed I/Q, never replaced by an ideal point.
    pub value: Iq,
}

/// Owned bounded constellation query result.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct QamSnapshot {
    /// Exact selected stage/reference plane.
    pub tap: Tap,
    /// Actual retained matched cells; no unavailable history is reconstructed.
    pub points: Vec<QamPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct Trace {
    pub(crate) origin: Option<u64>,
    pub(crate) identity: Option<Identity>,
    pub(crate) samples: Vec<Iq>,
    pub(super) points: Vec<QamPoint>,
    // Actual ResourcePlan role gains stay paired with points without changing report JSON.
    #[serde(skip)]
    pub(super) role_gains: Vec<f64>,
}

/// Immutable after-run retention owner; recording is passive and crate-private.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObservationStore {
    limit: usize,
    traces: BTreeMap<Tap, Trace>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) receiver_estimates: Option<super::receiver::ReceiverEstimateStore>,
}

impl ObservationStore {
    /// Splits total `limit` equally across explicit selected `taps`.
    ///
    /// Returns an empty bounded store. Rejects >65536 points, duplicate taps or
    /// more than nine selections; zero disables recording without science changes.
    pub fn new(limit: usize, taps: &[Tap]) -> PhyResult<Self> {
        if limit > 65536 || taps.len() > 9 {
            return Err(invalid("observations.limit"));
        }
        let traces: BTreeMap<_, _> = taps
            .iter()
            .map(|tap| {
                (
                    *tap,
                    Trace {
                        origin: None,
                        identity: None,
                        samples: Vec::new(),
                        points: Vec::new(),
                        role_gains: Vec::new(),
                    },
                )
            })
            .collect();
        if traces.len() != taps.len() {
            return Err(invalid("observations.taps"));
        }
        Ok(Self {
            limit: if taps.is_empty() {
                0
            } else {
                limit / taps.len()
            },
            traces,
            receiver_estimates: None,
        })
    }
    /// Returns an owned bounded QAM snapshot for `tap` and optional `loading`.
    ///
    /// `maximum` is 1..4096. Missing/unretained data is a typed unavailable result,
    /// never a replay. Filtering and query frequency cannot alter scientific state.
    pub fn qam(&self, tap: Tap, loading: Option<u8>, maximum: usize) -> PhyResult<QamSnapshot> {
        if !(1..=MAX_QAM_QUERY_POINTS).contains(&maximum) {
            return Err(invalid("qam_query.maximum"));
        }
        let trace = self.traces.get(&tap).ok_or(PhyError::Unsupported {
            field: "qam_query.unretained_tap",
        })?;
        let points: Vec<_> = trace
            .points
            .iter()
            .filter(|point| loading.is_none_or(|bits| point.loading == bits))
            .take(maximum)
            .cloned()
            .collect();
        if points.is_empty() {
            return Err(PhyError::Unsupported {
                field: "qam_query.unavailable",
            });
        }
        Ok(QamSnapshot { tap, points })
    }
    pub(crate) fn trace(&self, tap: Tap) -> Option<&Trace> {
        self.traces.get(&tap)
    }
    pub(crate) fn record_samples(&mut self, tap: Tap, block: &SampleBlock) {
        let Some(trace) = self.traces.get_mut(&tap) else {
            return;
        };
        if trace.samples.len() >= self.limit || block.samples.is_empty() {
            return;
        }
        if trace
            .identity
            .is_some_and(|identity| identity != block.identity)
        {
            return;
        }
        if let Some(origin) = trace.origin {
            if origin + trace.samples.len() as u64 != block.start {
                return;
            }
        } else {
            trace.origin = Some(block.start);
            trace.identity = Some(block.identity);
        }
        let count = (self.limit - trace.samples.len()).min(block.samples.len());
        trace.samples.extend_from_slice(&block.samples[..count]);
    }
    pub(crate) fn record_grid(
        &mut self,
        tap: Tap,
        identity: Identity,
        frame: u64,
        grid: &[Iq],
        resources: &ResourcePlan,
    ) {
        for cell in &resources.visits {
            if cell.symbol == 0 {
                self.record_cell(tap, identity, frame, cell, grid[cell.carrier]);
            }
        }
    }
    pub(crate) fn record_cell(
        &mut self,
        tap: Tap,
        identity: Identity,
        frame: u64,
        cell: &Cell,
        value: Iq,
    ) {
        let Some(trace) = self.traces.get_mut(&tap) else {
            return;
        };
        if trace.points.len() >= self.limit {
            return;
        }
        trace.points.push(QamPoint {
            identity,
            frame,
            symbol: cell.symbol,
            carrier: cell.carrier,
            role: cell.role,
            loading: cell.loading,
            value,
        });
        trace.role_gains.push(cell.gain);
    }
}

impl RetainedPayload for QamPoint {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            identity,
            frame,
            symbol,
            carrier,
            role,
            loading,
            value,
        } = self;
        sum([
            identity.retained_children(),
            frame.retained_children(),
            symbol.retained_children(),
            carrier.retained_children(),
            role.retained_children(),
            loading.retained_children(),
            value.retained_children(),
        ])
    }
}

impl RetainedPayload for Trace {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            origin,
            identity,
            samples,
            points,
            role_gains,
        } = self;
        sum([
            origin.retained_children(),
            identity.retained_children(),
            samples.retained_children(),
            points.retained_children(),
            role_gains.retained_children(),
        ])
    }
}

impl RetainedPayload for ObservationStore {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            limit,
            traces,
            receiver_estimates,
        } = self;
        sum([
            limit.retained_children(),
            traces.retained_children(),
            receiver_estimates.retained_children(),
        ])
    }
}

impl RetainedPayload for Tap {
    /// Audits every inline variant without counting borrowed or fixed data twice.
    ///
    /// # Returns
    /// Zero dynamic children; the enclosing root/container owns the fixed bytes.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        match self {
            Self::TxGrid
            | Self::TxPreEq
            | Self::TxUseful
            | Self::TxOutput
            | Self::ChannelInput
            | Self::ChannelOutput
            | Self::RxInput
            | Self::RxFft
            | Self::RxEqualized => Some(RetainedFootprint::default()),
        }
    }
}

impl ObservationStore {
    /// Bounds future trace payload under the already admitted aggregate point policy.
    ///
    /// # Returns
    /// Checked extra storage beyond the prebuilt empty map; each tap conservatively
    /// allows samples, cells and private gains without assuming vector growth.
    pub(crate) fn report_retained_extra(limit: usize, taps: &[Tap]) -> Option<RetainedFootprint> {
        let slots = if taps.is_empty() {
            0
        } else {
            (limit / taps.len()).checked_mul(taps.len())?
        };
        sum([
            merlin_api::retention_support::capacity_bytes::<Iq>(slots),
            merlin_api::retention_support::capacity_bytes::<QamPoint>(slots),
            merlin_api::retention_support::capacity_bytes::<f64>(slots),
        ])
    }

    /// Moves every actual trace and private cache into exact final capacities.
    ///
    /// # Returns
    /// Unit; values, missing history, keys and recording order remain unchanged.
    pub(crate) fn compact_report_payload(&mut self) {
        let Self {
            limit: _,
            traces,
            receiver_estimates,
        } = self;
        for trace in traces.values_mut() {
            let Trace {
                origin: _,
                identity: _,
                samples,
                points,
                role_gains,
            } = trace;
            *samples = std::mem::take(samples).into_boxed_slice().into_vec();
            *points = std::mem::take(points).into_boxed_slice().into_vec();
            *role_gains = std::mem::take(role_gains).into_boxed_slice().into_vec();
        }
        if let Some(store) = receiver_estimates {
            store.compact_report_payload();
        }
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use super::*;
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn ordered_trace_payload_and_serialization_skipped_role_gains_are_complete() {
        const SPARE_ROLE_GAINS: usize = 7;
        let mut store = ObservationStore::new(0, &[Tap::RxEqualized]).unwrap();
        let before = retained_footprint(&store).unwrap();
        assert_eq!(before.ordered_entries, 1);
        assert_eq!(
            before.payload_bytes,
            size_of::<ObservationStore>() + size_of::<Tap>() + size_of::<Trace>()
        );
        let json = serde_json::to_vec(&store).unwrap();
        let trace = store.traces.get_mut(&Tap::RxEqualized).unwrap();
        trace.role_gains.reserve_exact(SPARE_ROLE_GAINS);
        let requested = trace.role_gains.capacity() * size_of::<f64>();
        assert_eq!(
            retained_footprint(&store).unwrap().payload_bytes - before.payload_bytes,
            requested
        );
        assert_eq!(serde_json::to_vec(&store).unwrap(), json);
    }
    #[test]
    fn report_normalization_preserves_json_and_removes_private_gain_spare_capacity() {
        const SPARE_GAIN_SLOTS: usize = 29;
        let mut store = ObservationStore::new(0, &[Tap::RxEqualized]).unwrap();
        store
            .traces
            .get_mut(&Tap::RxEqualized)
            .unwrap()
            .role_gains
            .reserve_exact(SPARE_GAIN_SLOTS);
        let before = retained_footprint(&store).unwrap();
        let json = serde_json::to_vec(&store).unwrap();
        store.compact_report_payload();
        let after = retained_footprint(&store).unwrap();
        assert!(before.payload_bytes > after.payload_bytes);
        assert_eq!(before.ordered_entries, after.ordered_entries);
        assert_eq!(serde_json::to_vec(&store).unwrap(), json);
        assert_eq!(store.traces[&Tap::RxEqualized].role_gains.capacity(), 0);
    }
}
