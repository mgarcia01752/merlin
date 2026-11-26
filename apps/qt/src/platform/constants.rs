/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared Linux privacy policy for update staging and restart recovery.

/// Octal owner read/write/traverse; group and other receive no permissions.
/// Used when creating private application directories, not executable files.
pub(crate) const PRIVATE_DIRECTORY_MODE: u32 = 0o700;

/// Group/other read/write/execute bits, all of which private directories reject.
/// Ownership checks are separate; this mask does not establish the owner's UID.
pub(crate) const NON_OWNER_PERMISSION_BITS: u32 = 0o077;
