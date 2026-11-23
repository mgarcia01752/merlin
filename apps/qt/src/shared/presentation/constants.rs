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
/// Microseconds per second for drawing time labels from the backend sample clock.
pub(crate) const MICROSECONDS_PER_SECOND: f64 = 1_000_000.0;

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
