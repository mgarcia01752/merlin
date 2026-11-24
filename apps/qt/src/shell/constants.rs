/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Desktop retention policies, independent of scientific execution limits.

/// Two GiB of retained application payload, excluding allocator bookkeeping.
/// This application policy is not a PHY limit, scratch-memory or RSS guarantee.
pub(super) const MAX_SESSION_PAYLOAD_BYTES: usize = 2 * 1024 * 1024 * 1024;

/// Independent population limit for retained ordered-container entries.
pub(super) const MAX_SESSION_ORDERED_ENTRIES: usize = 1_000_000;

/// Independent population limit for retained shared allocation owner edges.
pub(super) const MAX_SESSION_SHARED_ROOTS: usize = 65_536;

/// Simultaneous bounded channel owners, allowing finite retired-worker overlap.
/// This application population ceiling is independent of scientific execution.
pub(super) const MAX_SESSION_CHANNELS: usize = 64;

/// Configured slots across live bounded channels, independent of channel count.
/// Empty slots remain populated policy resources; private std storage is excluded.
pub(super) const MAX_SESSION_CHANNEL_SLOTS: usize = 256;

/// Existing distinct original-source protection cap, shared by both directions.
/// This is a host path-inventory policy, separate from configuration/error text limits.
pub(super) const MAX_ORIGINAL_SOURCE_PATHS: usize = 4096;
