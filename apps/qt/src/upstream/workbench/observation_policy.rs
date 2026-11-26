/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Finite workbench observation choices; public scientific defaults stay unchanged.
use merlin::upstream::{
    ReceiverObservationPolicy, SoftObservationPolicy, MAX_RECEIVER_OBSERVATION_SCALARS,
    MAX_SOFT_WORD_QUERY_LLRS,
};

/// One complete received-pilot estimate for the native response instrument.
/// This display-history population is independent of execution grants/frames.
const WORKBENCH_ESTIMATE_FRAMES: usize = 1;
/// Bounded prefix of actual computed cells; independent of scalar table windows.
const WORKBENCH_SOFT_CELLS: usize = 64;
/// Actual completed input histories, independent of the receiver estimate count.
const WORKBENCH_SOFT_WORDS: usize = 2;
/// Each admitted word can consume at most the existing public query ceiling.
const WORKBENCH_SOFT_WORD_LLRS: usize = WORKBENCH_SOFT_WORDS * MAX_SOFT_WORD_QUERY_LLRS;

/// Enables finite actual observations for a newly admitted workbench session.
/// # Returns
/// The existing validated public policy, retaining at most one complete frame
/// within its public scalar ceiling, plus bounded actual cell and LDPC-input
/// histories. Source, estimator and receiver behavior do
/// not change; missing or incomplete frames remain unavailable.
/// # Errors
/// Propagates public policy validation without changing an accepted session.
pub(super) fn instruments() -> Result<ReceiverObservationPolicy, String> {
    let soft = SoftObservationPolicy::new(
        WORKBENCH_SOFT_CELLS,
        WORKBENCH_SOFT_WORDS,
        WORKBENCH_SOFT_WORD_LLRS,
    )
    .map_err(|error| error.to_string())?;
    ReceiverObservationPolicy::new(WORKBENCH_ESTIMATE_FRAMES, MAX_RECEIVER_OBSERVATION_SCALARS)
        .map(|policy| policy.with_soft_metrics(soft))
        .map_err(|error| error.to_string())
}
