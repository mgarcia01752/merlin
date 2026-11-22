/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded immutable selections over actual paired receiver report owners.
mod models;
mod queries;
mod selection;
use super::*;
pub use crate::signal_processing::{
    modulation::{
        mer::MerResult as FullLoopCellMer, slicing::HardDecision as FullLoopHardDecision,
    },
    receiver::diagnostics::decisions::ScaledDecision as FullLoopCellDecision,
};
pub use models::*;
use selection::{Cell, Selected};

impl FullLoopCellQuery {
    /// Admits a strict bounded receiver selection without touching any report or source.
    /// # Arguments
    /// * `bytes` - At most 4096 UTF-8 JSON bytes with known fields and registered stream names.
    /// # Returns
    /// Owned editable selection; result queries revalidate typed edits before any work.
    /// # Errors
    /// Rejects malformed/unknown fields, unsupported orders, ranges and common query limits.
    pub fn from_json(bytes: &[u8]) -> ApiResult<Self> {
        if bytes.len() > 4096 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Cell query exceeds 4096 bytes",
            ));
        }
        let query: Self = serde_json::from_slice(bytes)
            .map_err(|cause| super::super::error::invalid(cause.to_string()))?;
        query.validate()?;
        Ok(query)
    }

    fn validate(&self) -> ApiResult<()> {
        if self.count > 2048 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Cell view exceeds 2048 rows",
            ));
        }
        if self.first_bin.is_some_and(|value| value > 8191)
            || self.last_bin.is_some_and(|value| value > 8191)
            || self
                .first_bin
                .zip(self.last_bin)
                .is_some_and(|(first, last)| first > last)
            || self
                .first_symbol
                .zip(self.last_symbol)
                .is_some_and(|(first, last)| first > last)
        {
            return Err(super::super::error::invalid(
                "Cell coordinate bounds are reversed or outside the physical grid",
            ));
        }
        if self.modulation_order.is_some_and(|order| {
            crate::signal_processing::modulation::reference_for_order(usize::from(order)).is_none()
        }) {
            return Err(ApiError::new(
                ErrorCode::UnsupportedCapability,
                "Unsupported cell reference order",
            ));
        }
        Ok(())
    }
}
