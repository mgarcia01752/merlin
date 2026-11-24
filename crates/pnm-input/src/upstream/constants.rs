/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Upstream capture transport bounds, distinct from scientific validity.

/// DA, SA and length precede the UCD message-length-covered region (Figure 30).
pub(super) const UCD_LENGTH_ORIGIN: usize = 14;
/// Largest admitted binary family: UCD's unsigned 16-bit extent after its origin.
pub(super) const MAX_CAPTURE_SOURCE_BYTES: usize = UCD_LENGTH_ORIGIN + u16::MAX as usize;
/// Application policy for lossless JSON text, independent of inspection exports.
pub(super) const MAX_CAPTURE_SOURCE_JSON_BYTES: usize = 1024 * 1024;
/// Identity of the lossless source transport; inspection documents are separate.
pub(super) const CAPTURE_SOURCE_JSON_FORMAT: &str = "merlin.upstream.capture-source";
/// Initial source-envelope schema, independent of binary capture versions.
pub(super) const CAPTURE_SOURCE_JSON_SCHEMA_VERSION: u32 = 1;
