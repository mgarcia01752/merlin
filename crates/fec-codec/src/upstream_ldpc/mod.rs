/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 *
 * Adapted for MERLIN from MORGANA revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: module ownership/imports; codec reuses MERLIN sparse planning.
 */
//! Standalone MERLIN upstream QC-LDPC coding pinned to DOCSIS PHY 3.1 I20 §7.4.3.
//!
//! No BCH, puncturing, grant selection, QAM or waveform processing is implied.
#![forbid(unsafe_code)]

mod codec;
mod definitions;
mod error;

pub use codec::{DecodeResult, LdpcCodec, LdpcSettings, MAX_ITERATIONS};
pub use definitions::{DocsisLdpcCode, LdpcParameters};
pub use error::LdpcError;

#[cfg(test)]
mod tests;
