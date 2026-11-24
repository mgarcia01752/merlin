/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native transport policy, independent of PHY timing and scientific limits.

/// Interpreter-signal poll interval for both direction workers, in milliseconds.
/// Preserves the existing cadence; this is neither a simulation step nor runtime budget.
pub(crate) const WORKER_SIGNAL_POLL_MS: u64 = 20;
