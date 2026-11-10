/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded source-to-results simulation jobs.
#[cfg(feature = "simulation")]
pub mod coding_loop;
#[cfg(feature = "simulation")]
pub mod config;
#[cfg(feature = "simulation")]
pub mod pilots;
#[cfg(feature = "simulation")]
pub mod plc;
#[cfg(feature = "simulation")]
pub mod results;
#[cfg(feature = "simulation")]
pub mod runner;
pub mod source;
#[cfg(feature = "simulation")]
pub mod test_points;
#[cfg(feature = "simulation")]
pub use config::{SimulationSettings, CODING_MODE};
#[cfg(feature = "simulation")]
pub use results::{SimulationProgress, SimulationResult};
#[cfg(feature = "simulation")]
pub use runner::simulate;
#[cfg(feature = "simulation")]
pub use source::SimulationSeed;
// Preserve established imports while DSP belongs to signal_processing.
#[cfg(feature = "simulation")]
pub use crate::signal_processing::{
    modulation::{interleaving::transmit_bit_order, map_qam16, qam16_llrs},
    noise::MeasuredNoise,
};

#[cfg(feature = "simulation")]
pub mod waveform;

#[cfg(feature = "simulation")]
pub mod payload;

#[cfg(feature = "simulation")]
pub mod channel;
