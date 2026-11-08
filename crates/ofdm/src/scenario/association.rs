/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact spatial compatibility and explicit, noninterpolating capture-time matching.
use crate::types::CaptureTime;
use crate::{
    error::{invalid, LabError},
    scenario::grid::ScenarioKey,
};
use serde::Serialize;

/// A time association never claims simultaneity unless the timestamp is identical.
#[derive(Debug, Clone, Serialize)]
pub struct TimeAssociation {
    pub source_index: usize,
    pub delta_seconds: i64,
    pub simultaneous: bool,
}

/// Selects the unique nearest compatible observation within a user-set tolerance.
/// # Arguments
/// * `key` - Required device, channel and exact unmodified source grid.
/// * `epoch` - RxMER frame epoch in seconds.
/// * `observations` - Candidate identity/epoch pairs in original catalog order.
/// * `tolerance_seconds` - Explicit maximum timestamp distance; zero means exact.
/// # Returns
/// A unique association, or None when no observation is compatible/in range.
/// # Errors
/// Rejects invalid grids or tied nearest observations instead of guessing.
pub fn match_time(
    key: &ScenarioKey,
    epoch: CaptureTime,
    observations: &[(&ScenarioKey, CaptureTime)],
    tolerance_seconds: u32,
) -> Result<Option<TimeAssociation>, LabError> {
    key.grid.validate()?;
    let mut best: Option<(usize, u32, i64)> = None;
    let mut ambiguous = false;
    for (source_index, (candidate, capture_time)) in observations.iter().enumerate() {
        if *candidate != key {
            continue;
        }
        let distance = epoch.abs_diff(*capture_time);
        if distance > tolerance_seconds {
            continue;
        }
        if best.is_none_or(|(_, previous, _)| distance < previous) {
            best = Some((
                source_index,
                distance,
                i64::from(*capture_time) - i64::from(epoch),
            ));
            ambiguous = false;
        } else if best.is_some_and(|(_, previous, _)| distance == previous) {
            ambiguous = true;
        }
    }
    if ambiguous {
        return Err(invalid(
            "Equally near compatible captures require explicit source selection.",
        ));
    }
    Ok(
        best.map(|(source_index, distance, delta_seconds)| TimeAssociation {
            source_index,
            delta_seconds,
            simultaneous: distance == 0,
        }),
    )
}
