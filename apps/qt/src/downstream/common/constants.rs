/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named downstream presentation policies; backend admission remains authoritative.

/// Inclusive byte ceiling for the existing receiver/run-options editor document.
/// This application transport policy is independent of OCD configuration JSON,
/// even though both currently admit 4096 bytes. It is not a PHY or runtime bound.
pub(crate) const MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES: usize = 4096;

/// Maximum received-word bars shown by the existing downstream verification view.
/// This display cap is independent of the backend's retained-codeword budget;
/// omitted comparisons remain explicitly counted when more details are retained.
pub(crate) const MAX_VERIFICATION_WORD_BARS: usize = 64;
