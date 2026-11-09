/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Embed validated QAM tables; there is no runtime resource dependency.
#![forbid(unsafe_code)]
#[path = "build/qam_tables.rs"]
mod qam_tables;

/// Generates immutable constellation resources for this build.
fn main() {
    qam_tables::generate();
}
