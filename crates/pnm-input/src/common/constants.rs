/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Saved document versions and bounded input policies. These are not PHY limits.

/// Original downstream saved-workbench format, without explicit profile selection.
pub const WORKBENCH_SCHEMA_V1: u32 = 1;
/// Downstream format adding explicit profile selection; all other semantics persist.
pub const WORKBENCH_SCHEMA_V2: u32 = 2;
/// Independently versioned saved additional-loss configuration format.
pub const CHANNEL_MODEL_SCHEMA_VERSION: u32 = 1;
/// Independently versioned saved power-conditioning configuration format.
pub const CHANNEL_POWER_SCHEMA_VERSION: u32 = 1;

/// Inclusive complete saved-document byte budget shared with the desktop adapter.
pub const MAX_WORKBENCH_BYTES: usize = 144 * 1024 * 1024;
/// Inclusive original byte budget for each embedded downstream source or capture.
pub const MAX_EMBEDDED_BYTES: usize = 16 * 1024 * 1024;
/// Inclusive aggregate original byte budget for a saved downstream project.
pub const MAX_TOTAL_ORIGINAL_BYTES: usize = 96 * 1024 * 1024;
/// Inclusive number of original RxMER captures retained in a saved project.
pub const MAX_CAPTURES: usize = 256;
