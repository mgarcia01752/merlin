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
