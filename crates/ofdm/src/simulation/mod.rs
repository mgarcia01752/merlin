/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded source-to-results simulation jobs.
pub mod coding_loop;
pub mod config;
pub mod results;
pub mod runner;
pub mod source;
pub use config::{SimulationSettings, CODING_MODE};
pub use results::{SimulationProgress, SimulationResult};
pub use runner::simulate;
pub use source::SimulationSeed;
// Preserve established imports while DSP belongs to signal_processing.
pub use crate::signal_processing::{
    modulation::{interleaving::transmit_bit_order, map_qam16, qam16_llrs},
    noise::MeasuredNoise,
};
