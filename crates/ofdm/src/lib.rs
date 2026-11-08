/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone Rust scientific engine with focused domain and processing modules.
#![forbid(unsafe_code)]
pub mod error;
pub mod profile;
pub mod scenario;
pub mod signal_processing;
#[cfg(feature = "simulation")]
pub mod simulation;
pub mod types;
pub use error::LabError;

/// Compatibility paths; numerical implementations live in signal_processing.
pub mod math {
    pub use crate::signal_processing::modulation::qam;
    pub use crate::signal_processing::{bit_loading, ofdm, shannon};
}
