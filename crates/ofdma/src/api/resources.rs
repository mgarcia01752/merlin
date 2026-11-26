/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned idle resource snapshots; numerical projection stays in the resources owner.
use super::{bounded_json, ApiResult};
pub use crate::resources::view::{
    ResourceGridCell, ResourceGridCounts, ResourceGridRegion, ResourceGridRequest,
    ResourceGridRole, MAX_RESOURCE_GRID_CELLS,
};
use serde::Serialize;

/// Independent JSON transport budget; unrelated to cell counts or execution buffers.
pub const MAX_RESOURCE_GRID_JSON_BYTES: usize = 8 * 1024 * 1024;
pub(super) const RESOURCE_GRID_FORMAT: &str = "merlin.upstream.resource_grid";
pub(super) const RESOURCE_GRID_SCHEMA_VERSION: u32 = 1;

/// Configured allocation with accepted session identity, independent of actual waveform samples.
#[derive(Debug, Clone, Serialize)]
#[non_exhaustive]
pub struct ResourceGridSnapshot {
    /// Export identity, distinct from sample observations, configuration and FEC reports.
    pub format: &'static str,
    /// Resource-grid export schema, independent of DOCSIS and application versions.
    pub schema_version: u32,
    /// Accepted immutable session generation; querying does not allocate another identity.
    pub configuration_generation: u64,
    /// Existing finite grant selector decision; a planned cell does not prove emitted payload.
    pub transmission_planned: bool,
    /// Owned configured allocation, original physical coordinates and finite metadata.
    pub grid: ResourceGridRegion,
}
impl ResourceGridSnapshot {
    /// Serializes this owned configured snapshot without accessing a live session or source.
    /// # Returns
    /// At most eight MiB of lossless JSON, preserving original coordinates and absent values.
    /// # Errors
    /// Returns a typed serialization/transport failure without altering the snapshot.
    pub fn to_json(&self) -> ApiResult<String> {
        bounded_json(self, MAX_RESOURCE_GRID_JSON_BYTES)
    }
}
