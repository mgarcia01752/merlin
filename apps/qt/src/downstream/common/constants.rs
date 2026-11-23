/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named downstream presentation policies; backend admission remains authoritative.

/// Inclusive byte ceiling for the existing receiver/run-options editor document.
/// This application transport policy is independent of OCD configuration JSON,
/// even though both currently admit 4096 bytes. It is not a PHY or runtime bound.
pub(crate) const MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES: usize = 4096;

/// Maximum received-word bars shown by the existing downstream verification view.
/// This display cap is independent of the backend's retained-codeword budget;
/// omitted comparisons remain explicitly counted when more details are retained.
pub(crate) const MAX_VERIFICATION_WORD_BARS: usize = 64;

/// Fixed adjacent-window graph padding preserving the existing full-view time scale.
/// This prefix allowance is a display policy, not a validator or a claimed PHY limit.
pub(crate) const TIMING_PREFIX_ALLOWANCE_SAMPLES: usize = 1024;
/// Independent roll padding for that same stable graph, not its sampling resolution.
pub(crate) const TIMING_ROLL_ALLOWANCE_SAMPLES: usize = 256;
/// Uniform overview/detail drawing intervals; independent of roll padding above.
pub(crate) const TIMING_DRAWING_INTERVALS: usize = 256;
/// Extra drawing intervals along each taper to keep its shape visible.
pub(crate) const TIMING_TAPER_DRAWING_INTERVALS: usize = 64;
/// Horizontal tick intervals on each window chart.
pub(crate) const TIMING_TICK_INTERVALS: usize = 4;
/// Round the full-view time axis upward in this many microseconds.
pub(crate) const TIMING_OVERVIEW_QUANTUM_MICROSECONDS: f64 = 10.0;

/// One completion envelope for the single idle transmitter-preview worker.
pub(crate) const IDLE_PREVIEW_COMPLETION_CAPACITY: usize = 1;
