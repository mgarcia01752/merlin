/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Residual-power interpretation and controlled Gaussian noise.
mod residual;
pub use residual::equivalent_noise_mer;
#[cfg(feature = "simulation")]
pub mod awgn;
#[cfg(feature = "simulation")]
pub mod measured;
#[cfg(feature = "simulation")]
pub use measured::MeasuredNoise;
