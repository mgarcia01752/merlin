/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Native transport policy, independent of PHY timing and scientific limits.

/// Interpreter-signal poll interval for both direction workers, in milliseconds.
/// Preserves the existing cadence; this is neither a simulation step nor runtime budget.
pub(crate) const WORKER_SIGNAL_POLL_MS: u64 = 20;

/// Maximum UTF-8 bytes in a registered downstream stage identifier at the native boundary.
/// This transport guard is shared by physical, spectrum and original-event queries.
pub(crate) const MAX_STAGE_IDENTIFIER_BYTES: usize = 128;

/// Independent UTF-8 byte ceiling for receiver-cell stream selectors, not stage identifiers.
pub(crate) const MAX_CELL_STREAM_IDENTIFIER_BYTES: usize = 128;
/// One dispatched downstream request publishes exactly one complete/partial terminal result.
pub(crate) const FULL_LOOP_TERMINAL_CHANNEL_CAPACITY: usize = 1;
