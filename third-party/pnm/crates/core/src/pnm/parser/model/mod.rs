/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Header output views; concrete capture objects serialize directly.
pub mod pnm_header;

// Preserve the existing API while shared output code lives outside the parser.
pub use crate::common::json::{to_json, DEFAULT_MAX_JSON_BYTES};
