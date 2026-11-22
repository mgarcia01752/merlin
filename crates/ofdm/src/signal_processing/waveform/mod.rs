/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Centered unitary OFDM transforms and separate TX/RX sample operations.
#[cfg(all(feature = "simulation", feature = "pnm-input"))]
pub(crate) mod bin_power;
pub mod cyclic_prefix;
pub mod fft;
pub mod geometry;
pub mod ifft;
pub mod overlap_add;
pub mod samples;
pub mod spectrum;
#[cfg(all(feature = "simulation", feature = "pnm-input"))]
pub(crate) mod stage_sequence;
pub mod timing;
mod transform;
pub mod windowing;
pub use geometry::WaveformGeometry;
pub use samples::SampleBlock;
