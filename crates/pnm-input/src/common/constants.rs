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
/// Inclusive display-name byte ceiling in an embedded saved downstream original.
/// Empty file-source labels remain valid; scientific captures require provenance names.
pub const MAX_EMBEDDED_NAME_BYTES: usize = 1024;
/// Inclusive base64-text budget for one saved original; independent of decoded size.
pub const MAX_EMBEDDED_BASE64_BYTES: usize = 24 * 1024 * 1024;
/// Inclusive aggregate original byte budget for a saved downstream project.
pub const MAX_TOTAL_ORIGINAL_BYTES: usize = 96 * 1024 * 1024;
/// Inclusive number of original RxMER captures retained in a saved project.
pub const MAX_CAPTURES: usize = 256;

/// Direction-tagged project envelope; legacy downstream versions remain readable.
pub const PROJECT_SCHEMA_VERSION: u32 = 3;
/// One independently owned project per supported direction in the initial workspace.
pub const MAX_PROJECT_COLLECTION_COUNT: usize = 2;
/// Inclusive aggregate serialized project budget, separate from original input storage.
pub const MAX_PROJECT_COLLECTION_BYTES: usize = 288 * 1024 * 1024;
/// Inclusive captured-plan input budget, independent of generic configuration format.
pub const MAX_MEASURED_EXPERIMENT_BYTES: usize = 16 * 1024 * 1024;
/// Inclusive measured provenance report budget; not the input admission ceiling.
pub const MAX_MEASURED_PROVENANCE_BYTES: usize = 16 * 1024 * 1024;

/// Existing Designer baseline collection work ceiling in original carrier values.
/// Independent of equal-valued playback limits: this bounds saved recipe evaluation.
pub const MAX_DESIGNER_COLLECTION_VALUES: usize = 4_000_000;
