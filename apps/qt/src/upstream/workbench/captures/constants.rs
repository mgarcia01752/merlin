/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent presentation policies; capture validity stays with native APIs.

/// Maximum original records displayed at once, independent of source dimensions.
pub(super) const CAPTURE_PAGE_ROWS: usize = 64;
/// Bound a Qt family tag before copying it; unrelated to draft-token capacity.
pub(crate) const MAX_CAPTURE_FAMILY_CHARACTERS: isize = 16;
/// One new shared source pointee; aliases reuse its existing retention charge.
pub(super) const CAPTURE_SOURCE_SHARED_ROOTS: usize = 1;

/// Exact source points per RxMER plot query, independent of the 64 table rows.
pub(super) const MAX_CAPTURE_PLOT_POINTS: usize = 4096;
/// Horizontal original-carrier tick density; not a physical spacing policy.
pub(super) const CAPTURE_PLOT_CARRIER_TICK_INTERVALS: usize = 4;
/// Independent vertical dB tick density, despite the equal horizontal value.
pub(super) const CAPTURE_PLOT_DB_TICK_INTERVALS: u32 = 4;
/// Display padding in dB for an exactly constant finite set of original readings.
pub(super) const CAPTURE_PLOT_FLAT_PADDING_DB: f64 = 1.0;
