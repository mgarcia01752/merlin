/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named downstream UI transport policies; backend admission remains authoritative.

/// Inclusive byte ceiling for the existing receiver/run-options editor document.
/// This application transport policy is independent of OCD configuration JSON,
/// even though both currently admit 4096 bytes. It is not a PHY or runtime bound.
pub(crate) const MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES: usize = 4096;
