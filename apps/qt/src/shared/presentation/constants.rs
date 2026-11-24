/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named display units; scientific values and clocks come from the public backend.

/// Samples per megasample for displaying backend sample rates as Msamples/s.
/// This SI display conversion is not a clock, PSD normalization or time policy.
pub(crate) const SAMPLES_PER_MEGASAMPLE: f64 = 1_000_000.0;

/// Hz per MHz for frequency labels, independent of sample-rate display units.
pub(crate) const HERTZ_PER_MEGAHERTZ: f64 = 1_000_000.0;
/// Hz per kHz for bin-width/ENBW labels; this is not a duration or work budget.
pub(crate) const HERTZ_PER_KILOHERTZ: f64 = 1_000.0;
/// Bits per megabit for gross-rate labels; separate from equal-valued frequency units.
pub(crate) const BITS_PER_MEGABIT: f64 = 1_000_000.0;
/// Microseconds per second for drawing time labels from the backend sample clock.
pub(crate) const MICROSECONDS_PER_SECOND: f64 = 1_000_000.0;

/// Exact seconds in a clock minute for elapsed-time labels, not a runtime budget.
pub(crate) const SECONDS_PER_MINUTE: u64 = 60;
/// Exact minutes in a clock hour; distinct units despite the same value as above.
pub(crate) const MINUTES_PER_HOUR: u64 = 60;
/// Exact seconds in a clock hour, derived from the two authoritative unit scales.
pub(crate) const SECONDS_PER_HOUR: u64 = SECONDS_PER_MINUTE * MINUTES_PER_HOUR;

/// Vertical clipping span of the existing spectrum display, in decibels.
pub(crate) const SPECTRUM_DISPLAY_SPAN_DB: f64 = 100.0;
/// Decibel grid interval used to round the visible top edge.
pub(crate) const SPECTRUM_TOP_GRID_DB: f64 = 10.0;
/// Additional headroom above the rounded peak, separate from the grid interval.
pub(crate) const SPECTRUM_HEADROOM_DB: f64 = 10.0;
/// Maximum horizontal tick intervals in a spectrum viewport.
pub(crate) const SPECTRUM_FREQUENCY_TICK_INTERVALS: usize = 4;
/// Vertical tick intervals, independent of horizontal tick density.
pub(crate) const SPECTRUM_POWER_TICK_INTERVALS: u32 = 5;
/// Minimum viewport width for an SA axis with distinct first/last drawing coordinates.
pub(crate) const MIN_SPECTRUM_DISPLAY_BINS: usize = 2;

/// Complete byte values occupy two hexadecimal digits, independent of row layout.
pub(crate) const HEX_BYTE_DIGITS: usize = 2;
/// Existing packet byte-view row width; not a bit-loading or scientific budget.
pub(crate) const HEX_BYTES_PER_ROW: usize = 16;
/// Minimum address-column padding; larger byte offsets remain fully visible.
pub(crate) const HEX_OFFSET_MIN_DIGITS: usize = 4;

/// Existing maximum complex I/Q markers, distinct from PSD bucket/query budgets.
pub(crate) const MAX_SCATTER_PLOT_POINTS: usize = 2048;
/// Ten-percent drawing margin around the selected I/Q extent; samples are unchanged.
pub(crate) const SCATTER_HEADROOM_RATIO: f64 = 1.1;
/// Midpoint of each normalized scatter axis, independent of physical I/Q zero units.
pub(crate) const SCATTER_AXIS_CENTER: f64 = 0.5;

/// Existing raw FFT power viewport grouping, independent of PSD bucket work limits.
pub(crate) const FFT_POWER_VIEW_GROUPS: usize = 1024;
/// Ten-percent vertical padding around supplied raw-bin power observations.
pub(crate) const FFT_POWER_VIEW_PADDING_RATIO: f64 = 0.1;
/// Minimum raw-power drawing padding in dB; not a physical/noise threshold.
pub(crate) const FFT_POWER_VIEW_MIN_PADDING_DB: f64 = 1.0;
/// Symmetric drawing extent when every raw-bin power is absent/zero.
pub(crate) const FFT_POWER_VIEW_EMPTY_EXTENT_DB: f64 = 1.0;
/// Raw FFT power horizontal tick density; separate from spectrum tick policy.
pub(crate) const FFT_POWER_VIEW_FREQUENCY_INTERVALS: usize = 4;
/// Raw FFT power vertical tick density; independent of horizontal density.
pub(crate) const FFT_POWER_VIEW_POWER_INTERVALS: u32 = 4;

/// Percent units per dimensionless ratio, independent of any admitted editor range.
pub(crate) const PERCENT_PER_UNIT: f64 = 100.0;

/// Borrowed non-Designer fallback when a worker's detailed error cannot be retained.
/// This is application feedback, not a scientific failure category or channel policy.
pub(crate) const WORKER_FAILURE_ADMISSION_MESSAGE: &str =
    "Worker failure details exceed retained-data limits.";

/// Borrowed feedback when a concrete GUI runtime model cannot be admitted.
/// Distinct from rejected worker error details: no simulation/configuration starts.
pub(crate) const RUNTIME_MODEL_ADMISSION_MESSAGE: &str =
    "Session controls are unavailable within the retained-data limit.";
