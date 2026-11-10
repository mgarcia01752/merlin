/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Standalone Rust simulation core with focused domain and processing modules.
#![forbid(unsafe_code)]
pub mod error;
pub mod input_format;
pub mod profile;
pub mod scenario;
pub mod signal_processing;
#[cfg(feature = "simulation")]
pub mod simulation;
pub mod transmission;
pub mod types;
pub use error::SimulationError;

/// Compatibility paths; numerical implementations live in signal_processing.
pub mod math {
    pub use crate::signal_processing::modulation::qam;
    pub use crate::signal_processing::{bit_loading, ofdm, shannon};
}
