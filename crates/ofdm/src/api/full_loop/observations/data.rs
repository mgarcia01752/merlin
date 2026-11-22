/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned copies of the inspected producer representations; no numerical transformations.
use super::{FullLoopPilotValue, FullLoopTransportCell};
use crate::simulation::test_points::ProbeData;
use serde::Serialize;

/// Actual selected values on the original stage domain, including explicit unavailable cells.
#[derive(Clone, Serialize)]
#[non_exhaustive]
#[serde(tag = "kind", content = "values", rename_all = "snake_case")]
pub enum FullLoopStageData {
    Bits(Vec<u8>),
    Bytes(Vec<u8>),
    Iq(Vec<[f64; 2]>),
    OptionalIq(Vec<Option<[f64; 2]>>),
    QamLabels(Vec<crate::signal_processing::modulation::SymbolLabel>),
    TransportCells(Vec<Option<FullLoopTransportCell>>),
    Pilots(Vec<FullLoopPilotValue>),
    Llrs(Vec<f64>),
}
impl FullLoopStageData {
    /// Reads the number of actual retained selectable values without copying data.
    /// # Returns
    /// Original element count in this owned window; unavailable cells still occupy positions.
    pub fn len(&self) -> usize {
        match self {
            Self::Bits(v) | Self::Bytes(v) => v.len(),
            Self::Iq(v) => v.len(),
            Self::OptionalIq(v) => v.len(),
            Self::QamLabels(v) => v.len(),
            Self::TransportCells(v) => v.len(),
            Self::Pilots(v) => v.len(),
            Self::Llrs(v) => v.len(),
        }
    }
    /// Checks whether a stage copy contains no selectable values.
    /// # Returns
    /// True only for a zero-length owned window, never an inferred unavailable observation.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub(super) fn bytes(data: ProbeData<'_>) -> Option<usize> {
        Some(match data {
            ProbeData::Bits(v) | ProbeData::Bytes(v) => std::mem::size_of_val(v),
            ProbeData::Iq(v) => std::mem::size_of_val(v),
            ProbeData::OptionalIq(v) => std::mem::size_of_val(v),
            ProbeData::QamLabels(v) => std::mem::size_of_val(v),
            ProbeData::TransportCells(v) => std::mem::size_of_val(v),
            ProbeData::Pilots(v) => std::mem::size_of_val(v),
            ProbeData::Llrs(v) => std::mem::size_of_val(v),
            _ => return None,
        })
    }
    pub(super) fn copy(data: ProbeData<'_>) -> Option<Self> {
        Some(match data {
            ProbeData::Bits(v) => Self::Bits(v.to_vec()),
            ProbeData::Bytes(v) => Self::Bytes(v.to_vec()),
            ProbeData::Iq(v) => Self::Iq(v.to_vec()),
            ProbeData::OptionalIq(v) => Self::OptionalIq(v.to_vec()),
            ProbeData::QamLabels(v) => Self::QamLabels(v.to_vec()),
            ProbeData::TransportCells(v) => Self::TransportCells(v.to_vec()),
            ProbeData::Pilots(v) => Self::Pilots(v.to_vec()),
            ProbeData::Llrs(v) => Self::Llrs(v.to_vec()),
            _ => return None,
        })
    }
    pub(super) fn window(&self, start: usize, end: usize) -> Self {
        match self {
            Self::Bits(v) => Self::Bits(v[start..end].to_vec()),
            Self::Bytes(v) => Self::Bytes(v[start..end].to_vec()),
            Self::Iq(v) => Self::Iq(v[start..end].to_vec()),
            Self::OptionalIq(v) => Self::OptionalIq(v[start..end].to_vec()),
            Self::QamLabels(v) => Self::QamLabels(v[start..end].to_vec()),
            Self::TransportCells(v) => Self::TransportCells(v[start..end].to_vec()),
            Self::Pilots(v) => Self::Pilots(v[start..end].to_vec()),
            Self::Llrs(v) => Self::Llrs(v[start..end].to_vec()),
        }
    }
}
