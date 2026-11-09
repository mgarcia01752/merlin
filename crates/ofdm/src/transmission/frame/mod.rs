/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Explicit physical frequency-bin assembly before waveform transforms.
pub mod assembly;
pub use assembly::{assemble, PhysicalFrequencySymbol};
