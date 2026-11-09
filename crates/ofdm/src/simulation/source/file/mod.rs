/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Exact-byte file traffic, optional finite PRBS fill and independent verification.
mod config;
mod generator;
mod verification;
pub use config::{FilePayload, PrbsFill, SourceSelection, MAX_FILE_BITS, MAX_FILE_BYTES};
pub use generator::FileSource;
pub use verification::{FileSourceSummary, FileVerifier, RecoveredFile};
