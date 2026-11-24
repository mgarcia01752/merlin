/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite workbench observation choices; public scientific defaults stay unchanged.
use merlin::upstream::{ReceiverObservationPolicy, MAX_RECEIVER_OBSERVATION_SCALARS};

/// One complete received-pilot estimate for the native response instrument.
/// This display-history population is independent of execution grants/frames.
const WORKBENCH_ESTIMATE_FRAMES: usize = 1;

/// Enables finite actual-estimate retention for a newly admitted workbench session.
/// # Returns
/// The existing validated public policy, retaining at most one complete frame
/// within its public scalar ceiling. Source, estimator and receiver behavior do
/// not change; missing or incomplete frames remain unavailable.
/// # Errors
/// Propagates public policy validation without changing an accepted session.
pub(super) fn estimates() -> Result<ReceiverObservationPolicy, String> {
    ReceiverObservationPolicy::new(WORKBENCH_ESTIMATE_FRAMES, MAX_RECEIVER_OBSERVATION_SCALARS)
        .map_err(|error| error.to_string())
}
