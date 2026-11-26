/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Passive public resource queries reuse the existing admitted controller slot.
use super::*;
use merlin::upstream::ResourceGridRequest;

impl Pending {
    /// Queries an owned grid away from Qt while preserving the original controller.
    /// # Arguments
    /// * `controller` - Accepted idle scientific owner, never cloned or executed.
    /// * `budget` - Existing canonical counter for the slot, task and retained DTO.
    /// * `expected` - Actual application key, independent of backend generation.
    /// * `request` - Original-coordinate bounded public viewport request.
    /// # Returns
    /// One finite pending completion without acquiring a simulation run lease.
    /// # Errors
    /// Slot/transport/dispatch failure returns the unchanged original controller.
    pub(in crate::upstream::workbench) fn grid(
        controller: UpstreamController,
        budget: RetentionBudget,
        expected: SessionKey,
        request: ResourceGridRequest,
    ) -> Result<Self, (UpstreamController, String)> {
        let (owner, receiver) = super::query::controller_query(
            controller,
            budget,
            "merlin-upstream-grid",
            move |controller, budget| super::super::grid::observe(controller, request, budget),
        )?;
        Ok(Self::Grid {
            expected,
            receiver,
            owner,
        })
    }
}
