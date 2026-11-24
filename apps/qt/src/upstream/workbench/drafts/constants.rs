/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Configuration presentation/transport policies; scientific domains stay in the API.

/// Maximum changed-field JSON bytes for one Apply, independent of document size.
pub(crate) const MAX_DRAFT_CHANGE_BYTES: usize = 1024 * 1024;
/// Opaque section/token transport bound, checked before copying a Qt string.
pub(crate) const MAX_DRAFT_ID_CHARACTERS: isize = 128;
/// Maximum descriptors retained by one native form.
pub(super) const MAX_DRAFT_FIELDS: usize = 8192;
/// Independent choices per selector; equal to the field ceiling for another purpose.
pub(super) const MAX_DRAFT_CHOICES: usize = 8192;
/// Decimal editing capacity, independent of identity text and scientific limits.
pub(super) const MAX_DECIMAL_TEXT_CHARACTERS: usize = 128;
/// Zero is unissued; process-wide draft identities begin at this nonzero value.
pub(super) const FIRST_DRAFT_ID: u64 = 1;
/// One actual immutable draft Arc pointee, shared through worker completion.
pub(super) const DRAFT_SHARED_ROOTS: usize = 1;
/// Synthetic field edits cannot replace an admitted capture association/replay plan.
pub(in crate::upstream::workbench) const MEASURED_READ_ONLY_MESSAGE: &str =
    "Captured configuration is read-only here. Edit the complete measured plan and reload it.";
