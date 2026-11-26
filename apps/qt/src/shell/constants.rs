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

/// Logical UTF-16 bytes retained by presentation owners, independent of Rust capacity.
/// This application ceiling is not a Qt allocation, scientific or RSS claim.
pub(crate) const MAX_SESSION_PRESENTATION_UTF16_BYTES: usize = 64 * 1024 * 1024;

/// Logical array/object entries retained by presentation owners, not ordered nodes.
/// Its equal numeric value to the Rust entry ceiling does not share that purpose.
pub(crate) const MAX_SESSION_PRESENTATION_ENTRIES: usize = 1_000_000;

/// Data-retaining presentation views, including hidden or closed owners.
/// This ceiling is independent of the population currently open on screen.
/// Both persistent direction trees retain their existing views simultaneously.
/// Byte/entry/open-view limits remain independent; this is not a scientific limit.
pub(crate) const MAX_SESSION_RETAINED_VIEWS: usize = 256;

/// Presentation views simultaneously open, admitted before opening.
/// Closing releases this population while retained data keeps its other charges.
pub(crate) const MAX_SESSION_OPEN_VIEWS: usize = 32;

/// Existing distinct original-source protection cap, shared by both directions.
/// This is a host path-inventory policy, separate from configuration/error text limits.
pub(crate) const MAX_ORIGINAL_SOURCE_PATHS: usize = 4096;
