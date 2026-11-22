/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named display units; scientific values and clocks come from the public backend.

/// Samples per megasample for displaying backend sample rates as Msamples/s.
/// This SI display conversion is not a clock, PSD normalization or time policy.
pub(crate) const SAMPLES_PER_MEGASAMPLE: f64 = 1_000_000.0;
