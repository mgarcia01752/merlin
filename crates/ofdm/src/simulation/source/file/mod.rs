/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact-byte file traffic, optional finite PRBS fill and independent verification.
mod config;
#[cfg(feature = "simulation")]
mod generator;
#[cfg(feature = "simulation")]
mod verification;
pub use config::{FilePayload, PrbsFill, SourceSelection, MAX_FILE_BITS, MAX_FILE_BYTES};
#[cfg(feature = "simulation")]
pub use generator::FileSource;
#[cfg(feature = "simulation")]
pub use verification::{FileSourceSummary, FileVerifier, RecoveredFile};
