/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Admitted configured resource views from the existing idle public controller.
mod presentation;
#[cfg(test)]
mod tests;

use crate::{
    downstream::inputs::retention::{admit_model, json_children},
    shell::{Retained, RetentionBudget},
    upstream::UpstreamController,
};
use merlin::upstream::ResourceGridRequest;
use serde_json::Value;
use std::mem::size_of;

/// One independently retained JSON viewport; no scientific snapshot is retained.
pub(super) struct Display {
    pub(super) snapshot: Value,
}

/// Queries configured resource roles without running or advancing the source.
///
/// # Arguments
/// * `controller` - Existing idle public session owner; the caller owns canonical
///   application identity checks and executes this finite query off the GUI thread.
/// * `request` - Original physical viewport and existing public cell budget.
/// * `budget` - Canonical counter shared with previous and queued display owners.
///
/// # Returns
/// An admitted owned JSON snapshot preserving public metadata, roles and nulls.
/// Sample/frame identities are lossless decimal strings for QML. The temporary
/// typed public result is released before returning; no simulation state changes.
///
/// # Errors
/// Busy, public viewport/query failure, serialization or actual JSON admission
/// failure. Existing session, report and previously retained views are untouched.
pub(super) fn observe(
    controller: &UpstreamController,
    request: ResourceGridRequest,
    budget: &RetentionBudget,
) -> Result<Retained<Display>, String> {
    let result = controller
        .inspect(|simulation| simulation.resource_grid(request))
        .map_err(|error| format!("{error:?}"))?
        .map_err(|error| error.to_string())?;
    let snapshot = presentation::snapshot(&result)?;
    let mut resources = json_children(&snapshot)?;
    resources.payload_bytes = resources
        .payload_bytes
        .checked_add(size_of::<Display>())
        .ok_or("Upstream resource-grid view size overflowed.")?;
    admit_model(Display { snapshot }, resources, budget)
}
